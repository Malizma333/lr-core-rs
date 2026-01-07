use geometry::Point;
use vector2d::Vector2Df;

pub trait ColliderProps {
    fn can_collide(&self) -> bool;

    fn friction(&self) -> f64;
}

pub trait ColliderState {
    fn position(&self) -> Point;

    fn velocity(&self) -> Vector2Df;

    fn external_velocity(&self) -> Point;
}
