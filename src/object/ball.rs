use super::Object;
use crate::{state::object_state::ObjectState, Scalar, V2};

#[derive(Debug, Clone)]
pub struct Ball {
    pub position: V2,
    pub velocity: V2,
    pub mass: Scalar,
    pub radius: Scalar,
}

impl Object for Ball {
    fn get_state(&self) -> ObjectState {
        ObjectState::new(self.position, self.velocity)
    }

    fn set_state(&mut self, state: ObjectState) {
        self.position = state.position;
        self.velocity = state.velocity;
    }

    fn get_mass(&self) -> Scalar {
        self.mass
    }
}

impl Ball {
    pub fn new(position: V2, velocity: V2, mass: Scalar, radius: Scalar) -> Self {
        Self {
            position,
            velocity,
            mass,
            radius,
        }
    }
}
