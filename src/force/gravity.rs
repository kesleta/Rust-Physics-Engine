use crate::{state::object_state::ObjectState, Scalar, V2};

use super::ForceGenerator;

struct Gravity {
    pub strength: Scalar,
}

impl Gravity {
    fn new(strength: Scalar) -> Self {
        Self { strength }
    }
}

impl ForceGenerator for Gravity {

    fn get_force(&self, curr_state: &ObjectState) -> ObjectState {
        ObjectState::new(curr_state.velocity, V2::new(0.0, -self.strength))
    }
}
