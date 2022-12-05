pub mod gravity;
pub mod spring;
pub mod sum_force;
pub mod force_set;

use crate::state::object_state::ObjectState;

pub use self::sum_force::SumForce;

pub trait ForceGenerator {
    fn get_force(&self, curr_state: &ObjectState) -> ObjectState;
}
