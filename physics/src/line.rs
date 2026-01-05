mod builder;
mod computed;

pub use builder::PhysicsLineBuilder;
use geometry::Point;
use vector2d::Vector2Df;

use crate::{
    entity::point::{entity::EntityPoint, state::EntityPointState},
    line::computed::ComputedProperties,
};

pub struct PhysicsLine {
    endpoints: (Point, Point),
    height: f64,
    computed: ComputedProperties,
}

impl PhysicsLine {
    pub(crate) fn endpoints(&self) -> (Vector2Df, Vector2Df) {
        self.endpoints
    }

    /** Returns the new (position, previous position) to update a point with after it interacts with this line\
    (The previous position is not necessarily `position - velocity`, it represents how much force is applied
    on the momentum tick due to forces such as friction)
    */
    fn interact(
        &self,
        point: &EntityPoint,
        point_state: &EntityPointState,
        distance_from_line_top: f64,
        _position_between_ends: f64,
    ) -> (Point, Point) {
        let new_position =
            point_state.position() - (self.computed.normal_unit() * distance_from_line_top);

        let friction_x_flipped = if point_state.previous_position().x() >= new_position.x() {
            -1.0
        } else {
            1.0
        };

        let friction_y_flipped = if point_state.previous_position().y() < new_position.y() {
            -1.0
        } else {
            1.0
        };

        let initial_friction_vector = (self.computed.normal_unit().rotate_cw()
            * point.contact_friction())
            * distance_from_line_top;

        let friction_vector = Vector2Df::new(
            friction_x_flipped * initial_friction_vector.x(),
            friction_y_flipped * initial_friction_vector.y(),
        );

        let new_previous_position =
            point_state.previous_position() + friction_vector - self.computed.acceleration_vector();

        (new_position, new_previous_position)
    }

    pub(crate) fn check_interaction(
        &self,
        point: &EntityPoint,
        point_state: &EntityPointState,
    ) -> Option<(Point, Point)> {
        if !point.is_contact() {
            return None;
        }

        let offset_from_point = point_state.position() - self.endpoints.0;
        let moving_into_line =
            Vector2Df::dot(self.computed.normal_unit(), point_state.velocity()) > 0.0;
        let distance_from_line_top = Vector2Df::dot(self.computed.normal_unit(), offset_from_point);
        let position_between_ends =
            Vector2Df::dot(self.endpoints.1 - self.endpoints.0, offset_from_point)
                * self.computed.inverse_length_squared();

        if moving_into_line
            && 0.0 < distance_from_line_top
            && distance_from_line_top < self.height
            && self.computed.left_limit() <= position_between_ends
            && position_between_ends <= self.computed.right_limit()
        {
            Some(self.interact(
                point,
                point_state,
                distance_from_line_top,
                position_between_ends,
            ))
        } else {
            None
        }
    }
}
