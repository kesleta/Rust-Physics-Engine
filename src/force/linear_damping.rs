use crate::{
    state::{object_set_state::ObjectSetState},
    Scalar, V2,
};

use super::ForceGenerator;

pub struct LinearDamper {
    pub strength: Scalar,
}

impl LinearDamper {
    pub fn new(strength: Scalar) -> Self {
        Self { strength }
    }
}

impl ForceGenerator for LinearDamper {
    fn get_force(&self, system_state: &ObjectSetState) -> Vec<V2> {
        system_state.states.iter().map(|s| s.velocity * -self.strength).collect()
    }
}
