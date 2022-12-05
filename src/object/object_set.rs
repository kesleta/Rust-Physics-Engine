use crate::{state::object_state::ObjectState, Scalar};

use super::Object;

struct ObjectSet<'a, const N: usize> {
    objects: [&'a mut dyn Object; N],
}

impl<'a, const N: usize> ObjectSet<'a, N> {
    fn get_states(&self) -> [ObjectState; N] {
        self.objects
            .iter()
            .map(|o| o.get_state())
            .collect::<Vec<ObjectState>>()
            .try_into()
            .unwrap()
    }

    fn set_states(&mut self, state: [ObjectState; N]) {
        self.objects
            .iter_mut()
            .zip(state)
            .map(|(o, s)| o.set_state(s))
            .collect()
    }

    fn get_masses(&self) -> [Scalar; N] {
        self.objects
            .iter()
            .map(|o| o.get_mass())
            .collect::<Vec<Scalar>>()
            .try_into()
            .unwrap()
    }
}
