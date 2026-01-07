use geometry::{Line, Point};
use vector2d::Vector2Df;

use crate::{ColliderProps, ColliderState};

const DEFAULT_HEIGHT: f64 = 10.0;

pub struct PhysicsLine {
    endpoints: Line,
    flipped: bool,
    left_extension: bool,
    right_extension: bool,
    height: f64,
    multiplier: f64,

    // Computed props
    inverse_length_squared: f64,
    normal_unit: Vector2Df,
    left_limit: f64,
    right_limit: f64,
    acceleration_vector: Vector2Df,
}

impl PhysicsLine {
    pub fn new(
        endpoints: Line,
        flipped: bool,
        left_extension: bool,
        right_extension: bool,
    ) -> PhysicsLine {
        let mut line = PhysicsLine {
            endpoints,
            flipped,
            left_extension,
            right_extension,
            height: DEFAULT_HEIGHT,
            multiplier: 0.0,

            inverse_length_squared: 0.0,
            normal_unit: Vector2Df::zero(),
            left_limit: 0.0,
            right_limit: 0.0,
            acceleration_vector: Vector2Df::zero(),
        };
        line.recompute_props();
        line
    }

    pub fn endpoints(&self) -> Line {
        self.endpoints
    }

    pub fn set_endpoints(&mut self, endpoints: Line) {
        self.endpoints = endpoints;
        self.recompute_props();
    }

    pub fn flipped(&self) -> bool {
        self.flipped
    }

    pub fn set_flipped(&mut self, flipped: bool) {
        self.flipped = flipped;
        self.recompute_props();
    }

    pub fn left_extension(&self) -> bool {
        self.left_extension
    }

    pub fn set_left_extension(&mut self, left_extension: bool) {
        self.left_extension = left_extension;
        self.recompute_props();
    }

    pub fn right_extension(&self) -> bool {
        self.right_extension
    }

    pub fn set_right_extension(&mut self, right_extension: bool) {
        self.right_extension = right_extension;
        self.recompute_props();
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn set_height(&mut self, height: f64) {
        self.height = height;
        self.recompute_props();
    }

    pub fn multiplier(&self) -> f64 {
        self.multiplier
    }

    pub fn set_accel_multiplier(&mut self, multiplier: f64) {
        self.multiplier = multiplier;
        self.recompute_props();
    }

    /** Returns the new (position, previous position) to update a point with if it interacts with this line\
    *(The previous position is not necessarily `position - velocity`, it represents how much force is applied
    on the momentum tick due to forces such as friction)*
    */
    pub fn check_interaction<T: ColliderProps, U: ColliderState>(
        &self,
        point: &T,
        point_state: &U,
    ) -> Option<(Point, Point)> {
        if !point.can_collide() {
            return None;
        }

        let offset_from_point = point_state.position().vector_from(self.endpoints.p0());
        let moving_into_line = Vector2Df::dot(self.normal_unit, point_state.velocity()) > 0.0;
        let distance_from_line_top = Vector2Df::dot(self.normal_unit, offset_from_point);
        let position_between_ends = Vector2Df::dot(self.endpoints.get_vector(), offset_from_point)
            * self.inverse_length_squared;

        if moving_into_line
            && 0.0 < distance_from_line_top
            && distance_from_line_top < self.height
            && self.left_limit <= position_between_ends
            && position_between_ends <= self.right_limit
        {
            let new_position = point_state
                .position()
                .translated_by(-1.0 * self.normal_unit * distance_from_line_top);

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

            let initial_friction_vector =
                (self.normal_unit.rotate_cw() * point.friction()) * distance_from_line_top;

            let friction_vector = Vector2Df::new(
                friction_x_flipped * initial_friction_vector.x(),
                friction_y_flipped * initial_friction_vector.y(),
            );

            let new_previous_position = point_state
                .previous_position()
                .translated_by(friction_vector)
                .translated_by(-1.0 * self.acceleration_vector);

            Some((new_position, new_previous_position))
        } else {
            None
        }
    }

    fn recompute_props(&mut self) {
        let vector = self.endpoints.get_vector();
        let length = vector.length();
        self.inverse_length_squared = 1.0 / vector.length_squared();
        let unit = vector * (1.0 / length);

        self.normal_unit = if self.flipped {
            unit.rotate_cw()
        } else {
            unit.rotate_ccw()
        };

        const MAX_EXTENSION_SIZE: f64 = 0.25;
        let extension_ratio = MAX_EXTENSION_SIZE.min(self.height / length);

        self.left_limit = if self.left_extension {
            -extension_ratio
        } else {
            0.0
        };

        self.right_limit = if self.right_extension {
            1.0 + extension_ratio
        } else {
            1.0
        };

        const ACCELERATION_FACTOR: f64 = 0.1;
        self.acceleration_vector = unit * (self.multiplier * ACCELERATION_FACTOR);
    }
}
