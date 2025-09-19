use geometry::Point;
use vector2d::Vector2Df;

use crate::{entity::ContactPoint, line::computed::ComputedLineProperties};

pub(crate) const HITBOX_HEIGHT: f64 = 10.0;

pub trait Hitbox: ComputedLineProperties {
    fn interact(
        &self,
        point: &mut ContactPoint,
        distance_from_line_top: f64,
        position_between_ends: f64,
    );

    fn check_interaction(&self, point: &mut ContactPoint) {
        let offset_from_point = point.position() - self.endpoints().0;
        let moving_into_line = Vector2Df::dot(self.normal_unit(), point.velocity()) > 0.0;
        let distance_from_line_top = Vector2Df::dot(self.normal_unit(), offset_from_point);
        let position_between_ends =
            Vector2Df::dot(self.vector(), offset_from_point) * self.inverse_length_squared();

        if moving_into_line
            && 0.0 < distance_from_line_top
            && distance_from_line_top < HITBOX_HEIGHT
            && self.left_limit() <= position_between_ends
            && position_between_ends <= self.right_limit()
        {
            self.interact(point, distance_from_line_top, position_between_ends);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn interaction() {
        todo!()
        // moving into line inside hitbox should interact
        // moving out of line inside hitbox should not interact
        // moving into line above hitbox should not interact
        // moving into line below hitbox should not interact
        // moving into line to left of hitbox should not interact
        // moving into line to left of hitbox w/ left extension should interact
        // moving into line to right of hitbox should not interact
        // moving into line to right of hitbox w/ right extension should interact
    }
}

// Put line-specific implementations somewhere else?
//             let new_position = point.position() - (self.normal_unit() * distance_from_line_top);
//
//             let friction_vector =
//                 (self.normal_unit().rotate_cw() * point.friction) * distance_from_line_top;
//
//             if point.previous_position().x() >= new_position.x() {
//                 friction_vector.x *= -1;
//             }
//
//             if point.previous_position().y() < new_position.y() {
//                 friction_vector.y *= -1;
//             }
//
//             let new_previous_position =
//                 point.base.previous_position + friction_vector - self.acceleration_vector;
//
//             (new_position, new_previous_position)
