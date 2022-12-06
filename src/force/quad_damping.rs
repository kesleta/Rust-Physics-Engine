use crate::{state::object_set_state::ObjectSetState, Scalar, V2};

use super::ForceGenerator;

#[derive(Debug, Clone)]
pub struct QuadDamper {
    pub strength: Scalar,
}

impl QuadDamper {
    pub fn new(strength: Scalar) -> Self {
        Self { strength }
    }
}

impl ForceGenerator for QuadDamper {
    fn get_force(&self, system_state: &ObjectSetState) -> Vec<V2> {
        system_state
            .states
            .iter()
            .map(|s| {
                s.velocity.try_normalized().unwrap_or(V2::zero())
                    * s.velocity.magnitude_squared()
                    * -self.strength
            })
            .collect()
    }

    fn get_potential(&self, _system_state: &ObjectSetState) -> Scalar {
        0.0 // lost as heat
    }
}
