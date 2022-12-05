use super::ForceGenerator;
use crate::{
    state::{object_set_state::ObjectSetState, object_state::ObjectState},
    Scalar,
};

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

impl<'a, const N: usize> ForceGenerator<N> for Spring {
    fn get_force(&self, system_state: &ObjectSetState<N>) -> ObjectSetState<N> {
        assert!(self.p1_index < N);
        assert!(self.p2_index < N);
        let p1_state = system_state.states[self.p1_index];
        let p2_state = system_state.states[self.p2_index];

        let p1_p2 = p2_state.position - p1_state.position;
        let f = p1_p2.normalized() * (p1_p2.magnitude() - self.target_length) * self.strength;

        let mut result = ObjectSetState::zero();
        result.states[self.p1_index] = ObjectState::new(p1_state.velocity, f);
        result.states[self.p2_index] = ObjectState::new(p1_state.velocity, -f);
        result
    }
}
