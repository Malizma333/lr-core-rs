use vector2d::Vector2Df;

use crate::entity::point::snapshot::EntityPointSnapshot;

pub(crate) mod snapshot;
pub(crate) mod state;
pub(crate) mod template;

pub(crate) struct EntityPoint {
    contact: bool,
    contact_friction: f64,
    air_friction: f64,
}

impl EntityPoint {
    pub fn get_snapshot(&self) -> EntityPointSnapshot {
        // TODO
        EntityPointSnapshot::new(
            Vector2Df::zero(),
            Vector2Df::zero(),
            Vector2Df::zero(),
            self.contact_friction,
            self.air_friction,
            self.contact,
        )
    }
}
