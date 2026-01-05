use geometry::Point;

use crate::{PhysicsLine, line::computed::ComputedProperties};

const DEFAULT_HEIGHT: f64 = 10.0;

pub struct PhysicsLineBuilder {
    endpoints: (Point, Point),
    flipped: bool,
    left_extension: bool,
    right_extension: bool,
    height: f64,
    multiplier: f64,
}

impl PhysicsLineBuilder {
    pub fn new(endpoints: (Point, Point)) -> Self {
        Self {
            endpoints,
            flipped: false,
            left_extension: false,
            right_extension: false,
            height: DEFAULT_HEIGHT,
            multiplier: 0.0,
        }
    }

    pub fn flipped(&mut self, flipped: bool) -> &mut Self {
        self.flipped = flipped;
        self
    }

    pub fn left_extension(&mut self, left_extension: bool) -> &mut Self {
        self.left_extension = left_extension;
        self
    }

    pub fn right_extension(&mut self, right_extension: bool) -> &mut Self {
        self.right_extension = right_extension;
        self
    }

    pub fn height(&mut self, height: f64) -> &mut Self {
        self.height = height;
        self
    }

    pub fn acceleration_multiplier(&mut self, multiplier: f64) -> &mut Self {
        self.multiplier = multiplier;
        self
    }

    pub fn build(&self) -> PhysicsLine {
        PhysicsLine {
            endpoints: self.endpoints,
            height: self.height,
            computed: ComputedProperties::new(
                self.endpoints,
                self.flipped,
                self.left_extension,
                self.right_extension,
                self.height,
                self.multiplier,
            ),
        }
    }
}
