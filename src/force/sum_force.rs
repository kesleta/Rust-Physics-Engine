use crate::state::{object_set_state::ObjectSetState, pose::Pose};

use super::ForceGenerator;

#[derive(Debug)]
pub struct SumForce {
    pub forces: Vec<Box<dyn ForceGenerator>>,
}

impl SumForce {
    pub fn new(forces: Vec<Box<dyn ForceGenerator>>) -> Self {
        Self { forces }
    }
}

impl ForceGenerator for SumForce {
    fn get_force(&self, curr_state: &ObjectSetState) -> Vec<Pose> {
        let fs = self.forces.iter().map(|f| f.get_force(curr_state));
        fs.fold(
            curr_state.states.iter().map(|_| Pose::zero()).collect(),
            |v1, v2| v1.iter().zip(v2.iter()).map(|(a, b)| *a + *b).collect(),
        )
    }

    fn get_potential(&self, system_state: &ObjectSetState) -> f64 {
        self.forces
            .iter()
            .map(|f| f.get_potential(system_state))
            .sum()
    }
}
