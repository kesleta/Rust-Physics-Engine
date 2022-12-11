use crate::state::{object_set_state::ObjectSetState, object_state::ObjectState};

use super::Object;

pub struct ObjectSet {
    pub objects: Vec<Box<dyn Object>>,
}

impl ObjectSet {
    pub fn new(objects: Vec<Box<dyn Object>>) -> Self {
        Self { objects }
    }

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

    pub fn get_masses(&self) -> Vec<f64> {
        self.objects
            .iter()
            .map(|o| o.get_mass())
            .collect::<Vec<f64>>()
            .try_into()
            .unwrap()
    }

    pub fn get_kinetic(&self) -> f64 {
        self.objects
            .iter()
            .map(|o| (1.0 / 2.0) * o.get_mass() * o.get_state().vel.position.magnitude_squared())
            .sum()
    }
}
