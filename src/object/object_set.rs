use crate::{
    state::{object_set_state::ObjectSetState, object_state::ObjectState},
    Scalar,
};

use super::Object;

pub struct ObjectSet {
    pub objects: Vec<Box<dyn Object>>,
}

impl ObjectSet {
    pub fn get_states(&self) -> ObjectSetState {
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

    pub fn set_states(&mut self, state: ObjectSetState) {
        self.objects
            .iter_mut()
            .zip(state.states)
            .map(|(o, s)| o.set_state(s))
            .collect()
    }

    pub fn get_masses(&self) -> Vec<Scalar> {
        self.objects
            .iter()
            .map(|o| o.get_mass())
            .collect::<Vec<Scalar>>()
            .try_into()
            .unwrap()
    }
}
