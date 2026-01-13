use crate::entity_registry::{EntityState, EntityTemplate, EntityTemplateId};
use vector2d::Vector2Df;

struct InitialProps {
    offset: Vector2Df,
    velocity: Vector2Df,
}

pub(crate) struct Entity {
    cached_states: Vec<EntityState>,
    initial_state: EntityState,
    initial_props: InitialProps,
    associated_template_id: EntityTemplateId,
}

impl Entity {
    pub(super) fn new(template_id: EntityTemplateId, template: &EntityTemplate) -> Self {
        let mut entity = Self {
            cached_states: Vec::new(),
            initial_props: InitialProps {
                offset: Vector2Df::zero(),
                velocity: Vector2Df::zero(),
            },
            initial_state: EntityState::new(template, Vector2Df::zero(), Vector2Df::zero()),
            associated_template_id: template_id,
        };
        entity.regenerate_initial_state(template);
        entity
    }

    pub(super) fn cached_states(&self) -> &Vec<EntityState> {
        &self.cached_states
    }

    pub(super) fn push_to_cache(&mut self, state: EntityState) {
        self.cached_states.push(state);
    }

    pub(super) fn truncate_cache(&mut self, size: u32) {
        self.cached_states.truncate(size as usize);
    }

    pub(super) fn initial_state(&self) -> &EntityState {
        &self.initial_state
    }

    pub(super) fn initial_offset(&self) -> Vector2Df {
        self.initial_props.offset
    }

    pub(super) fn set_initial_offset(&mut self, offset: Vector2Df, template: &EntityTemplate) {
        self.initial_props.offset = offset;
        self.regenerate_initial_state(template);
    }

    pub(super) fn initial_velocity(&self) -> Vector2Df {
        self.initial_props.velocity
    }

    pub(super) fn set_initial_velocity(&mut self, velocity: Vector2Df, template: &EntityTemplate) {
        self.initial_props.velocity = velocity;
        self.regenerate_initial_state(template);
    }

    pub(super) fn template_id(&self) -> EntityTemplateId {
        self.associated_template_id
    }

    fn regenerate_initial_state(&mut self, template: &EntityTemplate) {
        self.initial_state =
            EntityState::new(template, self.initial_offset(), self.initial_velocity());
        self.cached_states.clear();
    }
}
