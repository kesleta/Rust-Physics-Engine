pub mod gravity;
pub mod linear_damping;
pub mod quad_damping;
pub mod spring;
pub mod sum_force;

use crate::state::{object_set_state::ObjectSetState, pose::Pose};

pub trait ForceGenerator: std::fmt::Debug {
    fn get_force(&self, system_state: &ObjectSetState) -> Vec<Pose>;
    fn get_potential(&self, system_state: &ObjectSetState) -> f64;
}
