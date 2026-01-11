mod point_state;
mod skeleton_state;

pub(crate) use point_state::EntityPointState;
pub(crate) use skeleton_state::EntitySkeletonState;

use std::collections::BTreeMap;

use geometry::Point;
use vector2d::Vector2Df;

use crate::entity_registry::{EntityPointId, EntityTemplate, MountPhase};

#[derive(Debug)]
pub struct EntityState {
    dismounted_this_frame: bool,
    skeleton_state: EntitySkeletonState,
    // TODO not a fan of this map being cloned, see if we can static allocate since we know how many points there are
    // or allocate as a Vec for less expensive clone, then have a map of Id to vec index
    point_states: BTreeMap<EntityPointId, EntityPointState>,
}

impl Clone for EntityState {
    fn clone(&self) -> Self {
        Self {
            dismounted_this_frame: false,
            skeleton_state: self.skeleton_state.clone(),
            point_states: self.point_states.clone(),
        }
    }
}

impl EntityState {
    // TODO add feature flag to apply bone rest lengths after initial state applied
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
            dismounted_this_frame: false,
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
        self.point_states.get(point_id).unwrap()
    }

    pub(crate) fn point_state_mut(&mut self, point_id: &EntityPointId) -> &mut EntityPointState {
        // TODO audit unwrap usage (and other panics)
        // Look into using slotmap for guaranteed safe key usage
        self.point_states.get_mut(point_id).unwrap()
    }

    pub(crate) fn dismounted_this_frame(&self) -> bool {
        self.dismounted_this_frame
    }

    pub(crate) fn dismount_this_frame(&mut self) {
        self.dismounted_this_frame = true;
    }
}
