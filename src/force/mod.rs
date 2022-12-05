pub mod sum_force;
pub mod gravity;
pub mod spring;

use crate::state::object_set_state::ObjectSetState;

pub trait ForceGenerator<const N: usize> {
    fn get_force(&self, system_state: &ObjectSetState<N>) -> ObjectSetState<N>;
}
