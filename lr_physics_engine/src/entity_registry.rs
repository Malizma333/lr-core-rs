mod bone;
mod entity;
mod entity_state;
mod joint;
mod mount_phase;
mod point;
mod remount_version;
mod skeleton;

use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
};

pub use bone::{EntityBone, EntityBoneBuilder};
pub(crate) use entity::Entity;
pub(crate) use entity_state::EntityPointState;
pub use entity_state::EntityState;
pub use joint::{EntityJoint, EntityJointBuilder};
pub use mount_phase::MountPhase;
pub use point::{EntityPoint, EntityPointBuilder};
pub use remount_version::RemountVersion;
pub use skeleton::{
    EntityBoneId, EntityJointId, EntityPointId, EntityTemplate, EntityTemplateBuilder,
};
use vector2d::Vector2Df;

use crate::{PhysicsMoment, line_registry::LineRegistry};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntityTemplateId(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntityId(usize);

pub(crate) struct EntityRegistry {
    // TODO Should these be slot maps? How do we verify order of insertion impacting order of remount processing?
    entity_templates: HashMap<EntityTemplateId, EntityTemplate>,
    entities: BTreeMap<EntityId, Entity>,
    latest_synced_frame: u32,
}

impl EntityRegistry {
    pub(crate) fn new() -> Self {
        Self {
            entity_templates: HashMap::new(),
            entities: BTreeMap::new(),
            latest_synced_frame: 0,
        }
    }

    pub(crate) fn add_entity_template(&mut self, template: EntityTemplate) -> EntityTemplateId {
        let id = EntityTemplateId(self.entity_templates.len());
        self.entity_templates.insert(id, template);
        id
    }

    pub(crate) fn create_entity(&mut self, template_id: EntityTemplateId) -> EntityId {
        self.clear_cache();
        let template = self.entity_templates.get_mut(&template_id).unwrap();
        let entity = Entity::new(template_id, template);
        let id = EntityId(self.entities.len());
        self.entities.insert(id, entity);
        id
    }

    pub(crate) fn get_entity_initial_offset(&self, entity_id: EntityId) -> Vector2Df {
        self.entities.get(&entity_id).unwrap().initial_offset()
    }

    pub(crate) fn set_entity_initial_offset(&mut self, entity_id: EntityId, offset: Vector2Df) {
        self.clear_cache();
        let entity = self.entities.get_mut(&entity_id).unwrap();
        let template = self.entity_templates.get(&entity.template_id()).unwrap();
        entity.set_initial_offset(offset, template)
    }

    pub(crate) fn get_entity_initial_velocity(&self, entity_id: EntityId) -> Vector2Df {
        self.entities.get(&entity_id).unwrap().initial_velocity()
    }

    pub(crate) fn set_entity_initial_velocity(&mut self, entity_id: EntityId, velocity: Vector2Df) {
        self.clear_cache();
        let entity = self.entities.get_mut(&entity_id).unwrap();
        let template = self.entity_templates.get(&entity.template_id()).unwrap();
        entity.set_initial_velocity(velocity, template)
    }

    pub(crate) fn remove_entity(&mut self, entity_id: EntityId) {
        self.clear_cache();
        self.entities.remove(&entity_id);
    }

    pub(crate) fn clear_cache(&mut self) {
        // TODO this should be better modularized on a per-invalidation basis
        self.latest_synced_frame = 0;
        for entity in self.entities.values_mut() {
            entity.truncate_cache(0);
        }
    }

    // This is a pretty delicate function that manages entity states and cache
    pub(crate) fn compute_frame(
        &mut self,
        frame: u32,
        _moment: PhysicsMoment,
        line_registry: &LineRegistry,
    ) -> Vec<EntityState> {
        let mut entity_states = Vec::new();

        // TODO this is inefficient if we don't need to reset the cache
        // but checking if we don't need to is hard
        for entity in self.entities.values_mut() {
            entity.truncate_cache(self.latest_synced_frame);
            let state = entity
                .cached_states()
                .last()
                .unwrap_or(entity.initial_state())
                .clone();
            entity_states.push(state);
        }

        while self.latest_synced_frame < frame {
            let mut state_index = 0;

            for entity in self.entities.values() {
                let template = self.entity_templates.get(&entity.template_id()).unwrap();

                let mut state = entity_states.get(state_index).unwrap().clone();

                state = entity.process_frame(state, template, line_registry);

                entity_states[state_index] = state.clone();
                state_index += 1;
            }

            state_index = 0;

            for entity in self.entities.values() {
                let template = self.entity_templates.get(&entity.template_id()).unwrap();

                let mut state = entity_states.get(state_index).unwrap().clone();

                if !state.dismounted_this_frame() {
                    state = entity.process_mount_phase(state, template, &mut entity_states);
                }

                entity_states[state_index] = state.clone();
                state_index += 1;
            }

            state_index = 0;

            for entity in self.entities.values_mut() {
                let state = entity_states.get(state_index).unwrap().clone();
                entity.push_to_cache(state);
                state_index += 1;
            }

            self.latest_synced_frame += 1;
        }

        entity_states
    }
}
