use crate::{state::object_set_state::ObjectSetState, V2};

use super::ForceGenerator;

pub struct SumForce {
    pub forces: Vec<Box<dyn ForceGenerator>>,
}

impl SumForce {
    pub fn new(forces: Vec<Box<dyn ForceGenerator>>) -> Self {
        Self { forces }
    }
}

impl ForceGenerator for SumForce {
    fn get_force(&self, curr_state: &ObjectSetState) -> Vec<V2> {
        let fs = self.forces.iter().map(|f| f.get_force(curr_state));
        fs.reduce(|v1, v2| v1.iter().zip(v2.iter()).map(|(a, b)| a + b).collect())
            .unwrap()
    }

    fn get_potential(&self, system_state: &ObjectSetState) -> crate::Scalar {
        self.forces
            .iter()
            .map(|f| f.get_potential(system_state))
            .sum()
    }
}
