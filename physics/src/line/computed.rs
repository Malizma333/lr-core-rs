use geometry::Point;
use vector2d::Vector2Df;

pub struct ComputedProperties {
    inverse_length_squared: f64,
    normal_unit: Vector2Df,
    left_limit: f64,
    right_limit: f64,
    acceleration_vector: Vector2Df,
}

impl ComputedProperties {
    pub(super) fn new(
        endpoints: (Point, Point),
        flipped: bool,
        left_extension: bool,
        right_extension: bool,
        height: f64,
        multiplier: f64,
    ) -> Self {
        let vector = endpoints.1 - endpoints.0;
        let length = vector.length();
        let inverse_length_squared = 1.0 / vector.length_squared();
        let unit = vector * (1.0 / length);

        let normal_unit = if flipped {
            unit.rotate_cw()
        } else {
            unit.rotate_ccw()
        };

        const MAX_EXTENSION_SIZE: f64 = 0.25;
        let extension_ratio = MAX_EXTENSION_SIZE.min(height / length);

        let left_limit = if left_extension {
            -extension_ratio
        } else {
            0.0
        };

        let right_limit = if right_extension {
            1.0 + extension_ratio
        } else {
            1.0
        };

        const ACCELERATION_FACTOR: f64 = 0.1;
        let acceleration_vector = unit * (multiplier * ACCELERATION_FACTOR);

        Self {
            inverse_length_squared,
            normal_unit,
            left_limit,
            right_limit,
            acceleration_vector,
        }
    }

    pub(super) fn inverse_length_squared(&self) -> f64 {
        self.inverse_length_squared
    }

    pub(super) fn normal_unit(&self) -> Vector2Df {
        self.normal_unit
    }

    pub(super) fn left_limit(&self) -> f64 {
        self.left_limit
    }

    pub(super) fn right_limit(&self) -> f64 {
        self.right_limit
    }

    pub(super) fn acceleration_vector(&self) -> Vector2Df {
        self.acceleration_vector
    }
}
