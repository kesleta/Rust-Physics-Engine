use crate::{
    state::{object_set_state::ObjectSetState},
    Scalar, V2,
};

use super::ForceGenerator;

#[derive(Debug, Clone)]
pub struct Gravity {
    pub strength: Scalar,
}

impl Gravity {
    pub fn new(strength: Scalar) -> Self {
        Self { strength }
    }
}

impl ForceGenerator for Gravity {
    fn get_force(&self, system_state: &ObjectSetState) -> Vec<V2> {
        system_state.states.iter().map(|_| V2::new(0.0, -self.strength)).collect()
    }

    fn get_potential(&self, system_state: &ObjectSetState) -> Scalar {
        system_state.states.iter().map(|o| o.position.y).sum()
    }
}
