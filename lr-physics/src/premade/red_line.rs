use geometry::Point;

use crate::line::{computed::ComputedLineProperties, hitbox::Hitbox};

const ACCELERATION_FACTOR: f64 = 0.1;

pub struct RedLine {
    endpoints: (Point, Point),
    flipped: bool,
    left_extension: bool,
    right_extension: bool,
    acceleration: f64,
}

impl ComputedLineProperties for RedLine {
    fn endpoints(&self) -> (Point, Point) {
        self.endpoints
    }

    fn extended_left(&self) -> bool {
        self.left_extension
    }

    fn extended_right(&self) -> bool {
        self.right_extension
    }

    fn flipped(&self) -> bool {
        self.flipped
    }
}

impl Hitbox for RedLine {
    fn interact(
        &self,
        point: &crate::entity::point::EntityPoint,
        distance_from_line_top: f64,
        _position_between_ends: f64,
    ) -> Option<(Point, Point)> {
        let new_position = point.position() - (self.normal_unit() * distance_from_line_top);

        let mut friction_vector =
            (self.normal_unit().rotate_cw() * point.friction()) * distance_from_line_top;

        if point.previous_position().x >= new_position.x {
            friction_vector.x *= -1.0;
        }

        if point.previous_position().y < new_position.y {
            friction_vector.y *= -1.0;
        }

        let acceleration_vector = self.unit() * (self.acceleration * ACCELERATION_FACTOR);

        let new_previous_position =
            point.previous_position() + friction_vector - acceleration_vector;

        Some((new_position, new_previous_position))
    }
}
