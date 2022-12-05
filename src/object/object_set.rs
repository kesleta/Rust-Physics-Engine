use crate::{
    state::{object_set_state::ObjectSetState, object_state::ObjectState},
    Scalar,
};

use super::Object;

pub struct ObjectSet<'a, const N: usize> {
    pub objects: [&'a mut dyn Object; N],
}

impl<'a, const N: usize> ObjectSet<'a, N> {
    pub fn get_states(&self) -> ObjectSetState<N> {
        ObjectSetState {
            states: self
                .objects
                .iter()
                .map(|o| o.get_state())
                .collect::<Vec<ObjectState>>()
                .try_into()
                .unwrap(),
        }
    }

    pub fn set_states(&mut self, state: ObjectSetState<N>) {
        self.objects
            .iter_mut()
            .zip(state.states)
            .map(|(o, s)| o.set_state(s))
            .collect()
    }

    pub fn get_masses(&self) -> [Scalar; N] {
        self.objects
            .iter()
            .map(|o| o.get_mass())
            .collect::<Vec<Scalar>>()
            .try_into()
            .unwrap()
    }
}
