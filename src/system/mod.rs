pub mod ball;

use crate::{state::object_state::ObjectState, Scalar};

pub trait Object {
    fn get_state(&self) -> ObjectState;
    fn set_state(&mut self, state: ObjectState);
    fn get_mass(&self) -> Scalar;
}

pub trait System {
    type State;
    type Masses;

    fn get_state(&self) -> Self::State;
    fn set_state(&mut self, state: Self::State);
    fn get_masses(&self) -> Self::Masses;
}

impl<T: Object> System for T {
    type State = ObjectState;
    type Masses = Scalar;

    fn get_state(&self) -> Self::State {
        self.get_state()
    }

    fn set_state(&mut self, state: Self::State) {
        self.set_state(state)
    }

    fn get_masses(&self) -> Self::Masses {
        self.get_mass()
    }
}