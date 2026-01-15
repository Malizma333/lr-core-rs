use geometry::Point;
use vector2d::Vector2Df;

use crate::entity_registry::EntityPointState;

/// Constructed point that holds props after building
pub struct EntityPoint {
    initial_position: Point,
    is_contact: bool,
    contact_friction: f64,
    air_friction: f64,
}

impl EntityPoint {
    pub(crate) fn initial_position(&self) -> Point {
        self.initial_position
    }

    pub(crate) fn is_contact(&self) -> bool {
        self.is_contact
    }

    pub(crate) fn contact_friction(&self) -> f64 {
        self.contact_friction
    }

    pub(crate) fn apply_momentum(&self, state: &mut EntityPointState, gravity: Vector2Df) {
        let computed_velocity = state
            .position()
            .vector_from(state.computed_previous_position());
        let new_velocity =
            computed_velocity * (1.0 - self.air_friction) + gravity.flipped_vertical();
        let new_position = state.position().translated_by(new_velocity);
        state.update(
            Some(new_position),
            Some(new_velocity),
            Some(state.position()),
        );
    }
}

pub struct EntityPointBuilder {
    initial_position: Point,
    is_contact: bool,
    contact_friction: f64,
    air_friction: f64,
}

/// Point builder for custom skeletons
impl EntityPointBuilder {
    pub(crate) fn new(initial_position: Point) -> EntityPointBuilder {
        Self {
            initial_position,
            is_contact: false,
            contact_friction: 0.0,
            air_friction: 0.0,
        }
    }

    pub fn is_contact(mut self, is_contact: bool) -> Self {
        self.is_contact = is_contact;
        self
    }

    pub fn contact_friction(mut self, friction: f64) -> Self {
        self.contact_friction = friction;
        self
    }

    pub fn air_friction(mut self, friction: f64) -> Self {
        self.air_friction = friction;
        self
    }

    pub(crate) fn build(self) -> EntityPoint {
        EntityPoint {
            initial_position: self.initial_position,
            is_contact: self.is_contact,
            contact_friction: self.contact_friction,
            air_friction: self.air_friction,
        }
    }
}

impl From<EntityPoint> for EntityPointBuilder {
    fn from(point: EntityPoint) -> Self {
        EntityPointBuilder {
            initial_position: point.initial_position,
            is_contact: point.is_contact,
            contact_friction: point.contact_friction,
            air_friction: point.air_friction,
        }
    }
}
