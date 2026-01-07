use geometry::Point;
use lr_physics_line_store::ColliderState;
use vector2d::Vector2Df;

#[derive(Debug, Clone)]
pub struct EntityPointState {
    position: Point,
    velocity: Vector2Df,
    previous_position: Point,
}

impl ColliderState for EntityPointState {
    fn position(&self) -> Point {
        self.position
    }

    fn velocity(&self) -> Vector2Df {
        self.velocity
    }

    fn previous_position(&self) -> Point {
        self.previous_position
    }
}

impl EntityPointState {
    pub(crate) fn new(position: Point, velocity: Vector2Df, previous_position: Point) -> Self {
        EntityPointState {
            position,
            velocity,
            previous_position,
        }
    }

    pub(crate) fn update(
        &mut self,
        new_position: Option<Point>,
        new_velocity: Option<Vector2Df>,
        new_previous_position: Option<Point>,
    ) {
        self.position = new_position.unwrap_or(self.position);
        self.velocity = new_velocity.unwrap_or(self.velocity);
        self.previous_position = new_previous_position.unwrap_or(self.previous_position);
    }

    pub(crate) fn position(&self) -> Point {
        self.position
    }

    pub(crate) fn velocity(&self) -> Vector2Df {
        self.velocity
    }

    pub(crate) fn previous_position(&self) -> Point {
        self.previous_position
    }
}
