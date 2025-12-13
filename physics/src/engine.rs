use crate::{
    engine::state::EngineState,
    entity::{
        joint::entity::EntityJoint,
        registry::{EntityRegistry, EntitySkeletonId},
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
    // TODO this should return a view struct rather than the direct state
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
        self.invalidate_frames();
        id
    }

    pub fn move_line(&mut self, line_id: LineId, new_points: Line) {
        let line = self.line_lookup.get(&line_id);
        if let Some(line) = line {
            let line_points = &Line::from_tuple(line.properties().endpoints());
            self.grid.move_line(line_id, line_points, &new_points);
            self.invalidate_frames();
        }
    }

    pub fn delete_line(&mut self, line_id: LineId) {
        let line = self.line_lookup.remove(&line_id);
        if let Some(line) = line {
            let line_points = &Line::from_tuple(line.properties().endpoints());
            self.grid.remove_line(line_id, line_points);
            self.invalidate_frames();
        }
    }

    fn invalidate_frames(&mut self) {
        self.state_snapshots.truncate(1);
    }

    pub fn registry(&mut self) -> &mut EntityRegistry {
        &mut self.registry
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
    // TODO moment unused
    fn get_next_state(
        &mut self,
        mut current_state: EngineState,
        frame: u32,
        moment: Option<PhysicsMoment>,
    ) -> EngineState {
        let mut dismounted_this_frame = false;

        // Physics step
        for (skeleton_id, skeleton) in self.registry.skeletons() {
            // Check if frozen skeleton
            if (self.get_skeleton_frozen_at_time)(*skeleton_id, frame) {
                continue;
            }

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
                    let mount_phase = if skeleton.use_initial_mount_phase_during_bones() {
                        initial_mount_phase
                    } else {
                        let skeleton_state = current_state.skeletons().get(skeleton_id).unwrap();
                        skeleton_state.mount_phase()
                    };

                    if !bone.is_breakable() {
                        let adjustment =
                            bone.get_adjustment(point_states, mount_phase.remounting());
                    } else if mount_phase.remounting() || mount_phase.mounted() {
                        let intact = bone.get_intact(point_states, mount_phase.remounting());
                        if !dismounted_this_frame {
                            if intact {
                                let adjustment =
                                    bone.get_adjustment(point_states, mount_phase.remounting());
                            } else {
                                dismounted_this_frame = true;
                                let skeleton_state_mut =
                                    current_state.skeletons_mut().get_mut(skeleton_id).unwrap();
                                skeleton_state_mut.set_mount_phase(
                                    skeleton.get_phase_after_dismount(mount_phase),
                                );
                            }
                        }
                    }
                }

                // line collisions
                for point_id in skeleton.points() {
                    let point = self.registry.get_point(*point_id);
                    let point_state = current_state.points_mut().get_mut(point_id).unwrap();
                    let interacting_lines = self.grid.get_lines_near_point(point_state.position());
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
                        let skeleton_state = current_state.skeletons().get(skeleton_id).unwrap();
                        skeleton_state.mount_phase()
                    };
                    let adjustment = bone.get_adjustment(point_states, mount_phase.remounting());
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
                        skeleton_state_mut
                            .set_mount_phase(skeleton.get_phase_after_dismount(mount_phase));
                        // LRA also breaks sled on mount joint break
                        // TODO
                        // self.state.remount_version == RemountVersion.LRA:
                        let lra_remount_version = false;
                        if lra_remount_version {
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
            // TODO
            // self.state.remount_version == RemountVersion.LRA
            // or self.state.remount_version == RemountVersion.COM_V1
            let some_remount_version = false;
            if !(some_remount_version && !(mount_phase.mounted() || mount_phase.remounting())) {
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

        // Remount step
        // TODO
        for (skeleton_id, skeleton) in self.registry.skeletons() {}

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
