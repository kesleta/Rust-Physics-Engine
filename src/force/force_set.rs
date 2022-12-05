use crate::{state::object_state::ObjectState};

use super::ForceGenerator;

struct ForceSet<'a, const N: usize> {
    forces: [&'a dyn ForceGenerator; N],
}

impl<'a, const N: usize> ForceSet<'a, N> {
    fn get_forces(&self, curr_states: [ObjectState; N]) -> [ObjectState; N] {
        self.forces
            .iter()
            .zip(curr_states)
            .map(|(f, s)| f.get_force(&s))
            .collect::<Vec<ObjectState>>()
            .try_into()
            .unwrap()
    }
}
