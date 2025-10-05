use geometry::Point;
use vector2d::Vector2Df;

pub struct EntityPointProps {
    contact: bool,
    contact_friction: f64,
    air_friction: f64,
}

pub struct EntityPointState {
    position: Point,
    velocity: Vector2Df,
    previous_position: Point,
}

impl Clone for EntityPointState {
    fn clone(&self) -> Self {
        Self {
            position: self.position.clone(),
            velocity: self.velocity.clone(),
            previous_position: self.previous_position.clone(),
        }
    }
}

pub struct EntityPoint {
    props: EntityPointProps,
    state: EntityPointState,
}

pub struct EntityPointTemplate {
    initial_position: Point,
    contact: bool,
    contact_friction: Option<f64>,
    air_friction: Option<f64>,
}

impl EntityPointTemplate {
    pub fn new(initial_position: Point) -> EntityPointTemplate {
        EntityPointTemplate {
            initial_position,
            contact: false,
            contact_friction: None,
            air_friction: None,
        }
    }

    pub fn contact(&mut self) -> &mut Self {
        self.contact = true;
        self
    }

    pub fn contact_friction(&mut self, friction: f64) -> &mut Self {
        self.contact_friction = Some(friction);
        self
    }

    pub fn air_friction(&mut self, friction: f64) -> &mut Self {
        self.air_friction = Some(friction);
        self
    }

    pub fn build(&self) -> EntityPoint {
        EntityPoint {
            state: EntityPointState {
                position: self.initial_position,
                velocity: Vector2Df::zero(),
                previous_position: self.initial_position,
            },
            props: EntityPointProps {
                contact: self.contact,
                contact_friction: self.contact_friction.unwrap_or(0.0),
                air_friction: self.air_friction.unwrap_or(0.0),
            },
        }
    }
}

impl EntityPoint {
    pub fn update(
        &mut self,
        new_position: Point,
        new_velocity: Vector2Df,
        new_previous_position: Point,
    ) {
        self.state.position = new_position;
        self.state.velocity = new_velocity;
        self.state.previous_position = new_previous_position;
    }

    pub fn position(&self) -> Point {
        self.state.position
    }

    pub fn velocity(&self) -> Vector2Df {
        self.state.velocity
    }

    pub fn previous_position(&self) -> Point {
        self.state.previous_position
    }

    pub fn friction(&self) -> f64 {
        self.props.contact_friction
    }

    pub fn air_friction(&self) -> f64 {
        self.props.air_friction
    }

    pub fn is_contact(&self) -> bool {
        self.props.contact
    }
}
