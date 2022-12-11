pub mod anchor;
pub mod constraint_set;
pub mod rod;

use crate::state::{pose::Pose, object_set_state::ObjectSetState};

pub trait Constraint {
    fn constrain_pose(&self, state: &ObjectSetState) -> f64;
    fn constrain_vel(&self, state: &ObjectSetState) -> f64;

    fn get_jacobian_block(&self, state: &ObjectSetState) -> Vec<(usize, usize, f64)>;
    fn get_jacobian_dot_block(&self, state: &ObjectSetState) -> Vec<(usize, usize, f64)>;
}
