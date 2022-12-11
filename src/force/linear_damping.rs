use crate::state::{object_set_state::ObjectSetState, pose::Pose};

use super::ForceGenerator;

#[derive(Debug, Clone)]
pub struct LinearDamper {
    pub strength: f64,
}

impl LinearDamper {
    pub fn new(strength: f64) -> Self {
        Self { strength }
    }
}

impl ForceGenerator for LinearDamper {
    fn get_force(&self, system_state: &ObjectSetState) -> Vec<Pose> {
        system_state
            .states
            .iter()
            .map(|s| s.vel * -self.strength)
            .collect()
    }

    fn get_potential(&self, _system_state: &ObjectSetState) -> f64 {
        0.0 // lost as heat
    }
}
