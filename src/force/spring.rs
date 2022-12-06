use super::ForceGenerator;
use crate::{state::object_set_state::ObjectSetState, Scalar, V2};
use std::iter::repeat;

#[derive(Debug, Clone)]
pub struct Spring {
    pub p1_index: usize,
    pub p2_index: usize,
    pub target_length: Scalar,
    pub strength: Scalar,
}

impl Spring {
    pub fn new(p1_index: usize, p2_index: usize, target_length: Scalar, strength: Scalar) -> Self {
        Self {
            p1_index,
            p2_index,
            target_length,
            strength,
        }
    }
}

impl ForceGenerator for Spring {
    fn get_force(&self, system_state: &ObjectSetState) -> Vec<V2> {
        assert!(self.p1_index < system_state.states.len());
        assert!(self.p2_index < system_state.states.len());
        let p1_state = system_state.states[self.p1_index];
        let p2_state = system_state.states[self.p2_index];

        let p1_p2 = p2_state.position - p1_state.position;
        let f = p1_p2.normalized() * (p1_p2.magnitude() - self.target_length) * self.strength;

        let mut result: Vec<_> = repeat(V2::zero()).take(system_state.states.len()).collect();
        result[self.p1_index] = f;
        result[self.p2_index] = -f;
        result
    }

    fn get_potential(&self, system_state: &ObjectSetState) -> Scalar {
        assert!(self.p1_index < system_state.states.len());
        assert!(self.p2_index < system_state.states.len());
        let p1_state = system_state.states[self.p1_index];
        let p2_state = system_state.states[self.p2_index];

        let displacement = (p1_state.position - p2_state.position).magnitude() - self.target_length;

        (1.0 / 2.0) * self.strength * displacement.powi(2)
    }
}
