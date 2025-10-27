use geometry::Point;
use vector2d::Vector2Df;

pub(crate) struct EntityPointSnapshot {
    contact: bool,
    contact_friction: f64,
    air_friction: f64,
    position: Point,
    velocity: Vector2Df,
    previous_position: Point,
}

impl EntityPointSnapshot {
    pub fn new(
        position: Point,
        velocity: Vector2Df,
        previous_position: Point,
        contact_friction: f64,
        air_friction: f64,
        contact: bool,
    ) -> Self {
        EntityPointSnapshot {
            contact,
            contact_friction,
            air_friction,
            position,
            velocity,
            previous_position,
        }
    }

    pub fn is_contact(&self) -> bool {
        self.contact
    }

    pub fn contact_friction(&self) -> f64 {
        self.contact_friction
    }

    pub fn air_friction(&self) -> f64 {
        self.air_friction
    }

    pub fn position(&self) -> Point {
        self.position
    }

    pub fn velocity(&self) -> Vector2Df {
        self.velocity
    }

    pub fn previous_position(&self) -> Point {
        self.previous_position
    }

    fn get_initial_step(&self, gravity: Vector2Df) -> (Point, Vector2Df, Point) {
        let computed_velocity = self.position - self.previous_position;
        let new_velocity = computed_velocity * (1.0 - self.air_friction) + gravity;
        let new_position = self.position + new_velocity;
        (new_position, new_velocity, self.position)
    }
}

#[cfg(test)]
mod tests {
    use geometry::Point;
    use vector2d::Vector2Df;

    use crate::entity::point::snapshot::EntityPointSnapshot;

    #[test]
    fn initial_step_zero_gravity() {
        let point = EntityPointSnapshot::new(
            Point::zero(),
            Vector2Df::zero(),
            Vector2Df::zero(),
            0.0,
            0.0,
            true,
        );
        let result = point.get_initial_step(Vector2Df::zero());
        assert!(result.0 == Point::zero(), "Position should be zero");
        assert!(result.1 == Vector2Df::zero(), "Velocity should be zero");
        assert!(
            result.2 == Vector2Df::zero(),
            "Previous should copy last position"
        );

        let point =
            EntityPointSnapshot::new(Point::up(), Point::up(), Point::zero(), 0.0, 0.0, true);
        let result = point.get_initial_step(Vector2Df::zero());
        assert!(result.0 == Point::up() * 2.0, "Position should increase");
        assert!(result.1 == Vector2Df::up(), "Velocity should stay the same");
        assert!(
            result.2 == Vector2Df::up(),
            "Previous should copy last position"
        );
    }

    #[test]
    fn initial_step_normal_gravity() {
        let point =
            EntityPointSnapshot::new(Point::up(), Point::up(), Point::zero(), 0.0, 0.0, true);
        let result = point.get_initial_step(Vector2Df::down());
        assert!(result.0 == Point::up(), "Position should be the same");
        assert!(result.1 == Vector2Df::zero(), "Velocity should be zero");
        assert!(
            result.2 == Vector2Df::up(),
            "Previous should copy last position"
        );
    }

    #[test]
    fn initial_step_air_friction() {
        let point =
            EntityPointSnapshot::new(Point::up(), Point::up(), Point::down(), 0.0, 0.5, true);
        let result = point.get_initial_step(Vector2Df::down());
        assert!(result.0 == Point::up(), "Position should be the same");
        assert!(result.1 == Vector2Df::zero(), "Velocity should be zero");
        assert!(
            result.2 == Vector2Df::up(),
            "Previous position should copy last position"
        );
    }
}
