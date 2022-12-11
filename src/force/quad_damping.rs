use crate::state::{object_set_state::ObjectSetState, pose::Pose};

use super::ForceGenerator;

#[derive(Debug, Clone)]
pub struct QuadDamper {
    pub strength: f64,
}

impl QuadDamper {
    pub fn new(strength: f64) -> Self {
        Self { strength }
    }
}

impl ForceGenerator for QuadDamper {
    fn get_force(&self, system_state: &ObjectSetState) -> Vec<Pose> {
        system_state
            .states
            .iter()
            .map(|s| {
                Pose::from_vector(
                    &(s.vel
                        .position
                        .try_normalized()
                        .unwrap_or(Pose::zero().position)
                        * s.vel.position.magnitude_squared()
                        * -self.strength),
                )
            })
            .collect()
    }

    fn get_potential(&self, _system_state: &ObjectSetState) -> f64 {
        0.0 // lost as heat
    }
}
