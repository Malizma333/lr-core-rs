use crate::{
    EntitySkeletonInitialProperties, MountPhase,
    engine::state::EngineState,
    entity::{
        joint::entity::EntityJoint,
        point::state::EntityPointState,
        registry::{EntityRegistry, EntitySkeletonId, EntitySkeletonTemplateId},
        skeleton::{builder::EntitySkeletonBuilder, state::EntitySkeletonState},
    },
    grid::{Grid, LineId},
    line::Hitbox,
};
use geometry::Line;
use std::collections::HashMap;
use vector2d::Vector2Df;
mod builder;
mod defaults;
mod moment;
mod state;
mod view;
pub use builder::EngineBuilder;
pub use moment::PhysicsMoment;
pub use view::EngineView;

const GRAVITY_MULTIPLIER: f64 = 0.175;

pub struct Engine {
    grid: Grid,
    line_lookup: HashMap<LineId, Box<dyn Hitbox>>,
    registry: EntityRegistry,
    // The initial state of the engine as a reference point
    initial_state: EngineState,
    // A list of cached state snapshots we can jump to
    state_snapshots: Vec<EngineState>,
    get_gravity_at_time: fn(u32) -> Vector2Df,
    get_skeleton_frozen_at_time: fn(EntitySkeletonId, u32) -> bool,
}

impl Engine {
    pub fn view_frame(&mut self, frame: u32) -> EngineView {
        self.fill_snapshots_up_to_frame(frame);
        let state = self
            .state_snapshots
            .get(frame as usize)
            .unwrap_or(&self.initial_state);
        EngineView::new(&self.registry, state)
    }

    pub fn view_moment(&mut self, frame: u32, moment: PhysicsMoment) -> EngineView {
        self.fill_snapshots_up_to_frame(frame);
        let target_frame_state = self
            .state_snapshots
            .get(frame as usize)
            .unwrap_or(&self.initial_state)
            .clone();
        let state = self.get_next_state(target_frame_state, frame, Some(moment));
        EngineView::new(&self.registry, &state)
    }

    pub fn define_gravity(&mut self, function: fn(u32) -> Vector2Df) {
        self.get_gravity_at_time = function;
    }

    pub fn define_skeleton_frozen(&mut self, function: fn(EntitySkeletonId, u32) -> bool) {
        self.get_skeleton_frozen_at_time = function;
    }

    pub fn create_line(&mut self, line: Box<dyn Hitbox>) -> LineId {
        let line_points = &Line::from_tuple(line.properties().endpoints());
        let id = self.grid.add_line(line_points);
        self.line_lookup.insert(id, line);
        self.invalidate_snapshots();
        id
    }

    pub fn move_line(&mut self, line_id: LineId, new_points: Line) {
        let line = self.line_lookup.get(&line_id);
        if let Some(line) = line {
            let line_points = &Line::from_tuple(line.properties().endpoints());
            self.grid.move_line(line_id, line_points, &new_points);
            self.invalidate_snapshots();
        }
    }

    pub fn delete_line(&mut self, line_id: LineId) {
        let line = self.line_lookup.remove(&line_id);
        if let Some(line) = line {
            let line_points = &Line::from_tuple(line.properties().endpoints());
            self.grid.remove_line(line_id, line_points);
            self.invalidate_snapshots();
        }
    }

    fn invalidate_snapshots(&mut self) {
        self.state_snapshots.truncate(0);
        self.state_snapshots.push(self.initial_state.clone());
    }

    pub fn build_skeleton(&mut self) -> EntitySkeletonBuilder<'_> {
        self.registry.skeleton_template_builder()
    }

    pub fn add_skeleton(
        &mut self,
        skeleton_template_id: EntitySkeletonTemplateId,
    ) -> EntitySkeletonId {
        let skeleton_id = self.registry.create_skeleton(skeleton_template_id);
        let skeleton = self.registry.get_skeleton(skeleton_id);

        self.initial_state.skeletons_mut().insert(
            skeleton_id,
            EntitySkeletonState::new(MountPhase::Mounted, true),
        );

        for point_id in skeleton.points() {
            let point = self.registry.get_point(*point_id);
            let offset = point.initial_position();
            self.initial_state.points_mut().insert(
                *point_id,
                EntityPointState::new(offset, Vector2Df::zero(), offset),
            );
        }

        self.invalidate_snapshots();
        skeleton_id
    }

    pub fn set_skeleton_initial_properties(
        &mut self,
        skeleton_id: EntitySkeletonId,
        initial_properties: EntitySkeletonInitialProperties,
    ) {
        let skeleton = self.registry.get_skeleton(skeleton_id);

        for point_id in skeleton.points() {
            let point = self.registry.get_point(*point_id);
            let local_offset = point.initial_position();
            let position = local_offset + initial_properties.start_position();
            let velocity = initial_properties.start_velocity();
            self.initial_state
                .points_mut()
                .get_mut(point_id)
                .unwrap()
                .update(Some(position), Some(velocity), Some(position - velocity));
        }

        self.invalidate_snapshots();
    }

    pub fn remove_skeleton(&mut self, skeleton_id: EntitySkeletonId) {
        let skeleton = self.registry.get_skeleton(skeleton_id);

        self.initial_state.skeletons_mut().remove(&skeleton_id);

        for point_id in skeleton.points() {
            self.initial_state.points_mut().remove(point_id);
        }

        self.registry.delete_skeleton(skeleton_id);

        self.invalidate_snapshots();
    }

    fn fill_snapshots_up_to_frame(&mut self, target_frame: u32) {
        let mut current_state = self
            .state_snapshots
            .last()
            .unwrap_or(&self.initial_state)
            .clone();

        while (self.state_snapshots.len() as u32) < target_frame + 1 {
            let next_state =
                self.get_next_state(current_state, self.state_snapshots.len() as u32, None);
            self.state_snapshots.push(next_state.clone());
            current_state = next_state.clone();
        }
    }

    // The main loop of the physics engine
    fn get_next_state(
        &mut self,
        mut current_state: EngineState,
        frame: u32,
        _moment: Option<PhysicsMoment>,
    ) -> EngineState {
        let mut dismount_flags = Vec::new();

        // Physics step
        for (skeleton_id, skeleton) in self.registry.skeletons() {
            let mut dismounted_this_frame = false;

            // Check if frozen skeleton
            if !(self.get_skeleton_frozen_at_time)(*skeleton_id, frame) {
                // momentum
                for point_id in skeleton.points() {
                    let point = self.registry.get_point(*point_id);
                    let point_state = current_state.points_mut().get_mut(point_id).unwrap();
                    point.process_initial_step(
                        point_state,
                        GRAVITY_MULTIPLIER * (self.get_gravity_at_time)(frame),
                    );
                }

                let initial_mount_phase = {
                    let skeleton_state = current_state.skeletons().get(skeleton_id).unwrap();
                    skeleton_state.mount_phase()
                };

                for _ in 0..6 {
                    // bones
                    for bone_id in skeleton.bones() {
                        let bone = self.registry.get_bone(*bone_id);
                        let point_states = (
                            current_state.points().get(&bone.points().0).unwrap(),
                            current_state.points().get(&bone.points().1).unwrap(),
                        );
                        let mount_phase = if skeleton.remount_version().lra() {
                            initial_mount_phase
                        } else {
                            let skeleton_state =
                                current_state.skeletons().get(skeleton_id).unwrap();
                            skeleton_state.mount_phase()
                        };

                        if !bone.is_breakable() {
                            let adjustment =
                                bone.get_adjustment(point_states, mount_phase.remounting());
                            {
                                let bone_point0_state = current_state
                                    .points_mut()
                                    .get_mut(&bone.points().0)
                                    .unwrap();
                                bone_point0_state.update(Some(adjustment.0), None, None);
                            }
                            {
                                let bone_point1_state = current_state
                                    .points_mut()
                                    .get_mut(&bone.points().1)
                                    .unwrap();
                                bone_point1_state.update(Some(adjustment.1), None, None);
                            }
                        } else if mount_phase.remounting() || mount_phase.mounted() {
                            let intact = bone.get_intact(point_states, mount_phase.remounting());
                            if !dismounted_this_frame {
                                if intact {
                                    let adjustment =
                                        bone.get_adjustment(point_states, mount_phase.remounting());
                                    {
                                        let bone_point0_state = current_state
                                            .points_mut()
                                            .get_mut(&bone.points().0)
                                            .unwrap();
                                        bone_point0_state.update(Some(adjustment.0), None, None);
                                    }
                                    {
                                        let bone_point1_state = current_state
                                            .points_mut()
                                            .get_mut(&bone.points().1)
                                            .unwrap();
                                        bone_point1_state.update(Some(adjustment.1), None, None);
                                    }
                                } else {
                                    dismounted_this_frame = true;
                                    let skeleton_state_mut =
                                        current_state.skeletons_mut().get_mut(skeleton_id).unwrap();
                                    let next_mount_phase = if !skeleton.remount_enabled() {
                                        MountPhase::Dismounted {
                                            frames_until_remounting: 0,
                                        }
                                    } else if mount_phase.mounted() {
                                        MountPhase::Dismounting {
                                            frames_until_dismounted: skeleton.dismounted_timer(),
                                        }
                                    } else if mount_phase.remounting() {
                                        MountPhase::Dismounted {
                                            frames_until_remounting: skeleton.remounting_timer(),
                                        }
                                    } else {
                                        mount_phase
                                    };
                                    skeleton_state_mut.set_mount_phase(next_mount_phase);
                                }
                            }
                        }
                    }

                    // line collisions
                    for point_id in skeleton.points() {
                        let point = self.registry.get_point(*point_id);
                        let point_state = current_state.points_mut().get_mut(point_id).unwrap();
                        let interacting_lines =
                            self.grid.get_lines_near_point(point_state.position());
                        for line_id in interacting_lines {
                            let line = &self.line_lookup[&line_id];
                            if let Some((new_position, new_previous_position)) =
                                line.check_interaction(point, point_state)
                            {
                                point_state.update(
                                    Some(new_position),
                                    None,
                                    Some(new_previous_position),
                                );
                            }
                        }
                    }
                }

                // flutter bones (like scarf)
                for bone_id in skeleton.bones() {
                    let bone = self.registry.get_bone(*bone_id);
                    if bone.is_flutter() {
                        let point_states = (
                            current_state.points().get(&bone.points().0).unwrap(),
                            current_state.points().get(&bone.points().1).unwrap(),
                        );
                        let mount_phase = {
                            let skeleton_state =
                                current_state.skeletons().get(skeleton_id).unwrap();
                            skeleton_state.mount_phase()
                        };
                        let adjustment =
                            bone.get_adjustment(point_states, mount_phase.remounting());
                        current_state
                            .points_mut()
                            .get_mut(&bone.points().0)
                            .unwrap()
                            .update(Some(adjustment.0), None, None);
                    }
                }

                // check dismount
                let mount_phase = {
                    let skeleton_state = current_state.skeletons().get(skeleton_id).unwrap();
                    skeleton_state.mount_phase()
                };
                if mount_phase.mounted() || mount_phase.remounting() {
                    for joint_id in skeleton.joints() {
                        let joint = self.registry.get_joint(*joint_id);
                        if joint.is_mount()
                            && self.get_joint_should_break(joint, &current_state)
                            && !dismounted_this_frame
                        {
                            dismounted_this_frame = true;
                            let skeleton_state_mut =
                                current_state.skeletons_mut().get_mut(skeleton_id).unwrap();
                            let next_mount_phase = if !skeleton.remount_enabled() {
                                MountPhase::Dismounted {
                                    frames_until_remounting: 0,
                                }
                            } else if mount_phase.mounted() {
                                MountPhase::Dismounting {
                                    frames_until_dismounted: skeleton.dismounted_timer(),
                                }
                            } else if mount_phase.remounting() {
                                MountPhase::Dismounted {
                                    frames_until_remounting: skeleton.remounting_timer(),
                                }
                            } else {
                                mount_phase
                            };
                            skeleton_state_mut.set_mount_phase(next_mount_phase);
                            // LRA also breaks sled on mount joint break
                            if skeleton.remount_version().lra() {
                                skeleton_state_mut.set_sled_intact(false);
                            }
                        }
                    }
                }

                // check skeleton break (like sled break)
                let mount_phase = {
                    let skeleton_state = current_state.skeletons().get(skeleton_id).unwrap();
                    skeleton_state.mount_phase()
                };
                let sled_intact = {
                    let skeleton_state = current_state.skeletons().get(skeleton_id).unwrap();
                    skeleton_state.sled_intact()
                };
                if !((skeleton.remount_version().lra() || skeleton.remount_version().comv1())
                    && !(mount_phase.mounted() || mount_phase.remounting()))
                {
                    for joint_id in skeleton.joints() {
                        let joint = self.registry.get_joint(*joint_id);
                        if !joint.is_mount()
                            && self.get_joint_should_break(joint, &current_state)
                            && sled_intact
                        {
                            let skeleton_state_mut =
                                current_state.skeletons_mut().get_mut(skeleton_id).unwrap();
                            skeleton_state_mut.set_sled_intact(false);
                        }
                    }
                }
            }

            dismount_flags.push(dismounted_this_frame);
        }

        let mut dismount_flag_index = 0;

        // Remount step
        for (skeleton_id, skeleton) in self.registry.skeletons() {
            let dismounted_this_frame = dismount_flags[dismount_flag_index];
            dismount_flag_index += 1;

            if !(self.get_skeleton_frozen_at_time)(*skeleton_id, frame)
                && skeleton.remount_enabled()
                && !dismounted_this_frame
            {
                let mount_phase = {
                    let skeleton_state = current_state.skeletons().get(skeleton_id).unwrap();
                    skeleton_state.mount_phase()
                };

                let sled_intact = {
                    let skeleton_state = current_state.skeletons().get(skeleton_id).unwrap();
                    skeleton_state.sled_intact()
                };

                let skeleton_state_mut =
                    current_state.skeletons_mut().get_mut(skeleton_id).unwrap();

                if skeleton.remount_version().lra() {
                    if !sled_intact {
                        skeleton_state_mut.set_mount_phase(MountPhase::Dismounted {
                            frames_until_remounting: 0,
                        });
                    } else {
                        if mount_phase.dismounting() {
                            if mount_phase.timer() == 0 {
                                skeleton_state_mut.set_mount_phase(MountPhase::Dismounted {
                                    frames_until_remounting: skeleton.remounting_timer(),
                                });
                            } else {
                                skeleton_state_mut.set_mount_phase(MountPhase::Dismounting {
                                    frames_until_dismounted: skeleton_state_mut
                                        .timer()
                                        .saturating_sub(1),
                                });
                            }
                        } else if mount_phase.dismounted() {
                            let can_enter_remounting = false;
                            // TODO
                            // if self.state.can_enter_remounting(self, other_entities):
                            if can_enter_remounting {
                                if mount_phase.timer() == 0 {
                                    skeleton_state_mut.set_mount_phase(MountPhase::Remounting {
                                        frames_until_mounted: skeleton.mounted_timer(),
                                    });
                                } else {
                                    skeleton_state_mut.set_mount_phase(MountPhase::Dismounted {
                                        frames_until_remounting: skeleton_state_mut
                                            .timer()
                                            .saturating_sub(1),
                                    });
                                }
                            } else {
                                skeleton_state_mut.set_mount_phase(MountPhase::Dismounted {
                                    frames_until_remounting: skeleton.remounting_timer(),
                                });
                            }
                        } else if mount_phase.remounting() {
                            let can_enter_mounted = false;
                            // TODO
                            // if self.state.can_enter_mount_phase(self, MountPhase.MOUNTED):
                            if can_enter_mounted {
                                if mount_phase.timer() == 0 {
                                    skeleton_state_mut.set_mount_phase(MountPhase::Mounted);
                                } else {
                                    skeleton_state_mut.set_mount_phase(MountPhase::Remounting {
                                        frames_until_mounted: skeleton_state_mut
                                            .timer()
                                            .saturating_sub(1),
                                    });
                                }
                            } else {
                                skeleton_state_mut.set_mount_phase(MountPhase::Remounting {
                                    frames_until_mounted: skeleton.mounted_timer(),
                                });
                            }
                        }
                    }
                } else {
                    if mount_phase.dismounting() {
                        skeleton_state_mut.set_mount_phase(MountPhase::Dismounting {
                            frames_until_dismounted: skeleton_state_mut.timer().saturating_sub(1),
                        });

                        if skeleton_state_mut.timer() == 0 {
                            skeleton_state_mut.set_mount_phase(MountPhase::Dismounted {
                                frames_until_remounting: skeleton.remounting_timer(),
                            });
                        }
                    } else if mount_phase.dismounted() {
                        // TODO
                        // if self.state.can_enter_remounting(self, other_entities):
                        let can_enter_remounting = false;
                        if can_enter_remounting {
                            skeleton_state_mut.set_mount_phase(MountPhase::Dismounted {
                                frames_until_remounting: skeleton_state_mut
                                    .timer()
                                    .saturating_sub(1),
                            });
                        } else {
                            skeleton_state_mut.set_mount_phase(MountPhase::Dismounted {
                                frames_until_remounting: skeleton.remounting_timer(),
                            });
                        }

                        if skeleton_state_mut.timer() == 0 {
                            skeleton_state_mut.set_mount_phase(MountPhase::Remounting {
                                frames_until_mounted: skeleton.mounted_timer(),
                            });
                        }
                    } else if mount_phase.remounting() {
                        let can_enter_mounted = false;
                        // TODO
                        // if self.state.can_enter_mount_phase(self, MountPhase.MOUNTED):
                        if can_enter_mounted {
                            skeleton_state_mut.set_mount_phase(MountPhase::Remounting {
                                frames_until_mounted: skeleton_state_mut.timer().saturating_sub(1),
                            });
                        } else {
                            skeleton_state_mut.set_mount_phase(MountPhase::Remounting {
                                frames_until_mounted: skeleton.mounted_timer(),
                            });
                        }

                        if skeleton_state_mut.timer() == 0 {
                            skeleton_state_mut.set_mount_phase(MountPhase::Mounted);
                        }
                    }
                }
            }
        }

        current_state
    }

    fn get_joint_should_break(&self, joint: &EntityJoint, current_state: &EngineState) -> bool {
        let bones = (
            self.registry.get_bone(joint.bones().0),
            self.registry.get_bone(joint.bones().1),
        );
        let bone0_p0 = current_state.points().get(&bones.0.points().0).unwrap();
        let bone0_p1 = current_state.points().get(&bones.0.points().1).unwrap();
        let bone1_p0 = current_state.points().get(&bones.1.points().0).unwrap();
        let bone1_p1 = current_state.points().get(&bones.1.points().1).unwrap();
        let bone_vectors = (
            bone0_p1.position() - bone0_p0.position(),
            bone1_p1.position() - bone1_p0.position(),
        );
        joint.should_break(bone_vectors)
    }
}
