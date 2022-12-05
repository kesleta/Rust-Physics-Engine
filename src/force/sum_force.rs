use crate::state::object_set_state::ObjectSetState;

use super::ForceGenerator;

pub struct SumForce<'a, const N: usize> {
    pub forces: Vec<&'a dyn ForceGenerator<N>>,
}

impl<'a, const N: usize> ForceGenerator<N> for SumForce<'a, N> {
    fn get_force(&self, curr_state: &ObjectSetState<N>) -> ObjectSetState<N> {
        self.forces.iter().map(|f| f.get_force(curr_state)).sum()
    }
}
