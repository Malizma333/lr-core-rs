mod point_state;
mod skeleton_state;

pub(crate) use point_state::EntityPointState;
pub(crate) use skeleton_state::EntitySkeletonState;

use std::collections::BTreeMap;

use geometry::Point;
use vector2d::Vector2Df;

use crate::{
    entity_registry::{EntityPointId, EntityTemplate, MountPhase, RemountVersion},
    line_registry::LineRegistry,
};

const MAX_ITERATION: u8 = 6;

#[derive(Debug, Clone)]
pub struct EntityState {
    skeleton_state: EntitySkeletonState,
    // Cloning a BTreeMap is 5x slower than cloning a Vec, but at this scale it's a difference of nanoseconds
    point_states: BTreeMap<EntityPointId, EntityPointState>,
}

impl EntityState {
    pub(super) fn new(
        template: &EntityTemplate,
        initial_offset: Vector2Df,
        initial_velocity: Vector2Df,
    ) -> Self {
        let skeleton_state = EntitySkeletonState::new(MountPhase::Mounted, true);
        let mut point_states = BTreeMap::new();

        for (point_id, point_template) in template.points() {
            let position = point_template
                .initial_position()
                .translated_by(initial_offset);
            let velocity = initial_velocity;
            let point_state =
                EntityPointState::new(position, velocity, position.translated_by(-velocity));
            point_states.insert(*point_id, point_state);
        }

        Self {
            skeleton_state,
            point_states,
        }
    }

    pub fn point_positions(&self) -> Vec<Point> {
        self.point_states
            .iter()
            .map(|point| point.1.position())
            .collect()
    }

    pub fn point_velocities(&self) -> Vec<Vector2Df> {
        self.point_states
            .iter()
            .map(|point| point.1.velocity())
            .collect()
    }

    pub fn mount_phase(&self) -> MountPhase {
        self.skeleton_state.mount_phase()
    }

    pub fn sled_intact(&self) -> bool {
        self.skeleton_state.sled_intact()
    }

    pub(crate) fn skeleton_state(&self) -> &EntitySkeletonState {
        &self.skeleton_state
    }

    pub(crate) fn skeleton_state_mut(&mut self) -> &mut EntitySkeletonState {
        &mut self.skeleton_state
    }

    pub(crate) fn point_state(&self, point_id: &EntityPointId) -> &EntityPointState {
        self.point_states
            .get(point_id)
            .expect("Point state should exist when retrieved internally")
    }

    pub(crate) fn point_state_mut(&mut self, point_id: &EntityPointId) -> &mut EntityPointState {
        self.point_states
            .get_mut(point_id)
            .expect("Point state should exist when retrieved internally")
    }

    // This is the main physics loop that transforms an entity state
    // Returns whether the rider dismounted
    pub(super) fn process_frame(
        &mut self,
        template: &EntityTemplate,
        line_registry: &LineRegistry,
    ) -> bool {
        let mut dismounted = false;

        for (point_id, point) in template.points() {
            const GRAVITY_MULTIPLIER: f64 = 0.175;
            let gravity = Vector2Df::down() * GRAVITY_MULTIPLIER;

            let point_state = self.point_state_mut(point_id);
            let computed_velocity = point_state
                .position()
                .vector_from(point_state.computed_previous_position());
            let new_velocity =
                computed_velocity * (1.0 - point.air_friction()) + gravity.flipped_vertical();
            let new_position = point_state.position().translated_by(new_velocity);
            point_state.update(
                Some(new_position),
                Some(new_velocity),
                Some(point_state.position()),
            );
        }

        let initial_mount_phase = self.mount_phase();

        for _ in 0..MAX_ITERATION {
            for bone in template.bones().values() {
                if !bone.is_flutter() {
                    let point_states = (
                        self.point_state(&bone.point_ids().0),
                        self.point_state(&bone.point_ids().1),
                    );

                    let mount_phase = match template.remount_version() {
                        RemountVersion::LRA => initial_mount_phase,
                        _ => self.mount_phase(),
                    };

                    if !bone.is_breakable() {
                        let adjusted = bone.get_adjusted(point_states, mount_phase.is_remounting());
                        self.point_state_mut(&bone.point_ids().0).update(
                            Some(adjusted.0),
                            None,
                            None,
                        );
                        self.point_state_mut(&bone.point_ids().1).update(
                            Some(adjusted.1),
                            None,
                            None,
                        );
                    } else if (mount_phase.is_remounting() || mount_phase.is_mounted())
                        && !dismounted
                    {
                        if bone.get_intact(point_states, mount_phase.is_remounting()) {
                            let adjusted =
                                bone.get_adjusted(point_states, mount_phase.is_remounting());
                            self.point_state_mut(&bone.point_ids().0).update(
                                Some(adjusted.0),
                                None,
                                None,
                            );
                            self.point_state_mut(&bone.point_ids().1).update(
                                Some(adjusted.1),
                                None,
                                None,
                            );
                        } else {
                            dismounted = true;

                            let next_mount_phase = match template.remount_version() {
                                RemountVersion::None => MountPhase::Dismounted {
                                    frames_until_remounting: 0,
                                },
                                _ => {
                                    if mount_phase.is_mounted() {
                                        MountPhase::Dismounting {
                                            frames_until_dismounted: template.dismounted_timer(),
                                        }
                                    } else if mount_phase.is_remounting() {
                                        MountPhase::Dismounted {
                                            frames_until_remounting: template.remounting_timer(),
                                        }
                                    } else {
                                        mount_phase
                                    }
                                }
                            };

                            self.skeleton_state_mut().set_mount_phase(next_mount_phase);
                        }
                    } else {
                        // Don't process bone
                    }
                }
            }

            for (point_id, point) in template.points() {
                if point.is_contact() {
                    let point_state = self.point_state_mut(point_id);
                    for line in line_registry.lines_near_point(point_state.position()) {
                        if let Some((new_position, new_computed_previous_position)) =
                            line.check_interaction(point, point_state)
                        {
                            point_state.update(
                                Some(new_position),
                                None,
                                Some(new_computed_previous_position),
                            );
                        }
                    }
                }
            }
        }

        for bone in template.bones().values() {
            if bone.is_flutter() {
                let point_states = (
                    self.point_state(&bone.point_ids().0),
                    self.point_state(&bone.point_ids().1),
                );
                let mount_phase = self.skeleton_state().mount_phase();
                let adjusted = bone.get_adjusted(point_states, mount_phase.is_remounting());
                self.point_state_mut(&bone.point_ids().0)
                    .update(Some(adjusted.0), None, None);
                self.point_state_mut(&bone.point_ids().1)
                    .update(Some(adjusted.1), None, None);
            }
        }

        let mount_phase = self.skeleton_state().mount_phase();

        if mount_phase.is_mounted() || mount_phase.is_remounting() {
            for joint in template.joints().values() {
                if joint.should_break(self, template) && !dismounted {
                    dismounted = true;

                    let next_mount_phase = match template.remount_version() {
                        RemountVersion::None => MountPhase::Dismounted {
                            frames_until_remounting: 0,
                        },
                        _ => {
                            if mount_phase.is_mounted() {
                                MountPhase::Dismounting {
                                    frames_until_dismounted: template.dismounted_timer(),
                                }
                            } else {
                                MountPhase::Dismounted {
                                    frames_until_remounting: template.remounting_timer(),
                                }
                            }
                        }
                    };

                    self.skeleton_state_mut().set_mount_phase(next_mount_phase);

                    if template.remount_version().is_lra() {
                        self.skeleton_state_mut().set_sled_intact(false);
                    }
                }
            }
        }

        let mount_phase = self.skeleton_state().mount_phase();
        let sled_intact = self.skeleton_state().sled_intact();

        if mount_phase.is_mounted()
            || mount_phase.is_remounting()
            || template.remount_version().is_none()
            || template.remount_version().is_comv2()
        {
            for joint in template.joints().values() {
                if !joint.is_mount() && joint.should_break(self, template) && sled_intact {
                    self.skeleton_state_mut().set_sled_intact(false);
                }
            }
        }

        dismounted
    }

    // This retrieves the next mount phase
    pub(super) fn process_mount_phase(
        &mut self,
        template: &EntityTemplate,
        other_states: &mut Vec<EntityState>,
    ) {
        let current_mount_phase = self.skeleton_state().mount_phase();
        let sled_intact = self.skeleton_state().sled_intact();

        let next_mount_phase = match template.remount_version() {
            RemountVersion::LRA => {
                if !sled_intact {
                    MountPhase::Dismounted {
                        frames_until_remounting: 0,
                    }
                } else {
                    match current_mount_phase {
                        MountPhase::Dismounting {
                            frames_until_dismounted,
                        } => {
                            if frames_until_dismounted == 0 {
                                MountPhase::Dismounted {
                                    frames_until_remounting: template.remounting_timer(),
                                }
                            } else {
                                MountPhase::Dismounting {
                                    frames_until_dismounted: frames_until_dismounted
                                        .saturating_sub(1),
                                }
                            }
                        }
                        MountPhase::Dismounted {
                            frames_until_remounting,
                        } => {
                            let mut can_swap = false;

                            for other_state in other_states {
                                if self.can_swap_sleds(template, other_state) {
                                    can_swap = true;
                                    break;
                                }
                            }

                            if can_swap {
                                if frames_until_remounting == 0 {
                                    MountPhase::Remounting {
                                        frames_until_mounted: template.mounted_timer(),
                                    }
                                } else {
                                    MountPhase::Dismounted {
                                        frames_until_remounting: frames_until_remounting
                                            .saturating_sub(1),
                                    }
                                }
                            } else {
                                MountPhase::Dismounted {
                                    frames_until_remounting: template.remounting_timer(),
                                }
                            }
                        }
                        MountPhase::Remounting {
                            frames_until_mounted,
                        } => {
                            if self.skeleton_can_enter_phase(template, false) {
                                if frames_until_mounted == 0 {
                                    MountPhase::Mounted
                                } else {
                                    MountPhase::Remounting {
                                        frames_until_mounted: frames_until_mounted
                                            .saturating_sub(1),
                                    }
                                }
                            } else {
                                MountPhase::Remounting {
                                    frames_until_mounted: template.mounted_timer(),
                                }
                            }
                        }
                        MountPhase::Mounted => MountPhase::Mounted,
                    }
                }
            }
            RemountVersion::ComV1 | RemountVersion::ComV2 => match current_mount_phase {
                MountPhase::Dismounting {
                    frames_until_dismounted,
                } => {
                    let next_timer = frames_until_dismounted.saturating_sub(1);
                    if next_timer == 0 {
                        MountPhase::Dismounted {
                            frames_until_remounting: template.remounting_timer(),
                        }
                    } else {
                        MountPhase::Dismounting {
                            frames_until_dismounted: next_timer,
                        }
                    }
                }
                MountPhase::Dismounted {
                    frames_until_remounting,
                } => {
                    let mut can_swap = false;

                    for other_state in other_states {
                        if self.can_swap_sleds(template, other_state) {
                            can_swap = true;
                            break;
                        }
                    }

                    let next_timer = if can_swap {
                        frames_until_remounting.saturating_sub(1)
                    } else {
                        template.remounting_timer()
                    };

                    if next_timer == 0 {
                        MountPhase::Remounting {
                            frames_until_mounted: template.mounted_timer(),
                        }
                    } else {
                        MountPhase::Dismounted {
                            frames_until_remounting: next_timer,
                        }
                    }
                }
                MountPhase::Remounting {
                    frames_until_mounted,
                } => {
                    let next_timer = if self.skeleton_can_enter_phase(template, false) {
                        frames_until_mounted.saturating_sub(1)
                    } else {
                        template.mounted_timer()
                    };

                    if next_timer == 0 {
                        MountPhase::Mounted
                    } else {
                        MountPhase::Remounting {
                            frames_until_mounted: next_timer,
                        }
                    }
                }
                MountPhase::Mounted => MountPhase::Mounted,
            },
            RemountVersion::None => current_mount_phase,
        };

        self.skeleton_state_mut().set_mount_phase(next_mount_phase);
    }

    pub(super) fn can_swap_sleds(
        &mut self,
        template: &EntityTemplate,
        other_state: &mut EntityState,
    ) -> bool {
        if other_state.sled_intact() && other_state.mount_phase().is_dismounted() {
            // Swap sleds to check entity can safely remount
            self.swap_skeleton_sleds(template, other_state);

            if self.skeleton_can_enter_phase(template, true) {
                return true;
            }

            // Swap sleds back if we failed
            self.swap_skeleton_sleds(template, other_state);
        }

        false
    }

    fn swap_skeleton_sleds(&mut self, template: &EntityTemplate, other_state: &mut EntityState) {
        match template.remount_version() {
            RemountVersion::ComV2 | RemountVersion::LRA => {
                let sled_intact = self.skeleton_state().sled_intact();
                let other_sled_intact = other_state.skeleton_state().sled_intact();
                other_state
                    .skeleton_state_mut()
                    .set_sled_intact(sled_intact);
                self.skeleton_state_mut().set_sled_intact(other_sled_intact);
            }
            _ => {}
        }

        // Assumes sled points are in same order, because they originate from same template
        for point_id in template.sled_points() {
            let point_state = self.point_state(point_id).clone();
            let other_point_state = other_state.point_state(point_id).clone();

            other_state.point_state_mut(point_id).update(
                Some(point_state.position()),
                Some(point_state.velocity()),
                Some(point_state.computed_previous_position()),
            );
            self.point_state_mut(point_id).update(
                Some(other_point_state.position()),
                Some(other_point_state.velocity()),
                Some(other_point_state.computed_previous_position()),
            );
        }
    }

    pub(super) fn skeleton_can_enter_phase(
        &self,
        template: &EntityTemplate,
        target_phase_is_remounting: bool,
    ) -> bool {
        for bone in template.bones().values() {
            let point_states = (
                self.point_state(&bone.point_ids().0),
                self.point_state(&bone.point_ids().1),
            );

            if bone.is_breakable() && !bone.get_intact(point_states, target_phase_is_remounting) {
                return false;
            }
        }

        match template.remount_version() {
            RemountVersion::ComV1 | RemountVersion::ComV2 => {
                for joint in template.joints().values() {
                    if !joint.is_mount() && joint.should_break(self, template) {
                        return false;
                    }
                }

                for joint in template.joints().values() {
                    if joint.is_mount() && joint.should_break(self, template) {
                        return false;
                    }
                }
            }
            _ => {}
        }

        true
    }
}
