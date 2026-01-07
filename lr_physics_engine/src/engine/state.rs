use std::collections::BTreeMap;

use crate::entity::{
    point::state::EntityPointState,
    registry::{EntityPointId, EntitySkeletonId},
    skeleton::state::EntitySkeletonState,
};

#[derive(Debug, Clone)]
pub(super) struct EngineState {
    point_states: BTreeMap<EntityPointId, EntityPointState>,
    skeleton_states: BTreeMap<EntitySkeletonId, EntitySkeletonState>,
}

impl EngineState {
    pub(super) fn new() -> Self {
        Self {
            point_states: BTreeMap::new(),
            skeleton_states: BTreeMap::new(),
        }
    }

    pub(super) fn points(&self) -> &BTreeMap<EntityPointId, EntityPointState> {
        &self.point_states
    }

    pub(super) fn points_mut(&mut self) -> &mut BTreeMap<EntityPointId, EntityPointState> {
        &mut self.point_states
    }

    pub(super) fn skeletons(&self) -> &BTreeMap<EntitySkeletonId, EntitySkeletonState> {
        &self.skeleton_states
    }

    pub(super) fn skeletons_mut(&mut self) -> &mut BTreeMap<EntitySkeletonId, EntitySkeletonState> {
        &mut self.skeleton_states
    }
}
