pub mod sum_force;
pub mod gravity;
pub mod spring;
pub mod linear_damping;
pub mod quad_damping;

use crate::{state::object_set_state::ObjectSetState, V2, Scalar};

pub trait ForceGenerator {
    fn get_force(&self, system_state: &ObjectSetState) -> Vec<V2>;
    fn get_potential(&self, system_state: &ObjectSetState) -> Scalar;
}
