pub mod ball;
pub mod object_set;

use crate::{state::object_state::ObjectState, Scalar};

pub trait Object {
    fn get_state(&self) -> ObjectState;
    fn set_state(&mut self, state: ObjectState);
    fn get_mass(&self) -> Scalar;
}
