use std::collections::HashMap;

use crate::{
    engine::state::{point::EntityPointState, skeleton::EntitySkeletonState},
    entity::registry::{EntityPointId, EntitySkeletonId},
};

mod point;
mod skeleton;

pub struct EngineState {
    point_states: HashMap<EntityPointId, EntityPointState>,
    skeleton_states: HashMap<EntitySkeletonId, EntitySkeletonState>,
}

impl Clone for EngineState {
    fn clone(&self) -> Self {
        Self {
            // hashmap clone is implemented as a deep copy
            point_states: self.point_states.clone(),
            skeleton_states: self.skeleton_states.clone(),
        }
    }
}

impl EngineState {
    pub fn new() -> Self {
        Self {
            point_states: HashMap::new(),
            skeleton_states: HashMap::new(),
        }
    }
}
