use super::ForceGenerator;
use crate::state::{object_set_state::ObjectSetState, pose::Pose};
use std::iter::repeat;

#[derive(Debug, Clone)]
pub struct Spring {
    pub p1_index: usize,
    pub p2_index: usize,
    pub target_length: f64,
    pub strength: f64,
}

impl Spring {
    pub fn new(p1_index: usize, p2_index: usize, target_length: f64, strength: f64) -> Self {
        Self {
            p1_index,
            p2_index,
            target_length,
            strength,
        }
    }
}

impl ForceGenerator for Spring {
    fn get_force(&self, system_state: &ObjectSetState) -> Vec<Pose> {
        assert!(self.p1_index < system_state.states.len());
        assert!(self.p2_index < system_state.states.len());
        let p1_state = system_state.states[self.p1_index];
        let p2_state = system_state.states[self.p2_index];

        let p1_p2 = p2_state.pose.position - p1_state.pose.position;
        let f = p1_p2.normalized() * (p1_p2.magnitude() - self.target_length) * self.strength;

        let mut result: Vec<_> = repeat(Pose::zero())
            .take(system_state.states.len())
            .collect();
        result[self.p1_index] = Pose::from_vector(&f);
        result[self.p2_index] = Pose::from_vector(&-f);
        result
    }

    fn get_potential(&self, system_state: &ObjectSetState) -> f64 {
        assert!(self.p1_index < system_state.states.len());
        assert!(self.p2_index < system_state.states.len());
        let p1_state = system_state.states[self.p1_index];
        let p2_state = system_state.states[self.p2_index];

        let displacement =
            (p1_state.pose.position - p2_state.pose.position).magnitude() - self.target_length;

        (1.0 / 2.0) * self.strength * displacement.powi(2)
    }
}
