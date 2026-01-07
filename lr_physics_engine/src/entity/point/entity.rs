use geometry::Point;
use lr_physics_line_store::ColliderProps;
use vector2d::Vector2Df;

use crate::entity::point::state::EntityPointState;

pub struct EntityPoint {
    pub(super) initial_position: Point,
    pub(super) is_contact: bool,
    pub(super) contact_friction: f64,
    pub(super) air_friction: f64,
}

impl ColliderProps for EntityPoint {
    fn can_collide(&self) -> bool {
        self.is_contact
    }

    fn friction(&self) -> f64 {
        self.contact_friction
    }
}

impl EntityPoint {
    pub(crate) fn initial_position(&self) -> Point {
        self.initial_position
    }

    pub(crate) fn process_initial_step(&self, state: &mut EntityPointState, gravity: Vector2Df) {
        let computed_velocity = state.position().vector_from(state.external_velocity());
        let new_velocity = computed_velocity * (1.0 - self.air_friction) + gravity;
        let new_position = state.position().translated_by(new_velocity);
        state.update(
            Some(new_position),
            Some(new_velocity),
            Some(state.position()),
        );
    }
}
