pub mod anchor;
pub mod constraint_set;

use crate::{Scalar, V2};

pub trait Constraint {
    fn constrain_position(&self, positions: &Vec<V2>) -> Scalar;
    fn constrain_velocities(&self, velocities: &Vec<V2>) -> Scalar;
    fn constrain_acceleration(&self, accelerations: &Vec<V2>) -> Scalar;

    fn get_jacobian_slice(&self, positions: &Vec<V2>) -> Vec<(usize, V2)>;
    fn get_jacobian_dot_slice(&self, velocities: &Vec<V2>) -> Vec<(usize, V2)>;
}
