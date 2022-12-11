use crate::state::{object_set_state::ObjectSetState, pose::Pose};

use super::ForceGenerator;

#[derive(Debug, Clone)]
pub struct Gravity {
    pub strength: f64,
}

impl Gravity {
    pub fn new(strength: f64) -> Self {
        Self { strength }
    }
}

impl ForceGenerator for Gravity {
    fn get_force(&self, system_state: &ObjectSetState) -> Vec<Pose> {
        system_state
            .states
            .iter()
            .map(|_| Pose::new(0.0, -self.strength, 0.0))
            .collect()
    }

    fn get_potential(&self, system_state: &ObjectSetState) -> f64 {
        system_state.states.iter().map(|o| o.pose.position.y).sum()
    }
}
