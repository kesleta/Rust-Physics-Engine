use std::f64::consts::PI;

use crate::state::{object_set_state::ObjectSetState, pose::Pose};

use super::Constraint;

pub struct Anchor {
    pub object_index: usize,
    pub pose: Pose,
}

impl Anchor {
    pub fn new(object_index: usize, pose: Pose) -> Self {
        Self { object_index, pose }
    }
}

//I seperate constraint equation into components for clairty
impl Constraint for Anchor {
    fn constrain_pose(&self, state: &ObjectSetState) -> f64 {
        let object_pose = state.states[self.object_index].pose;
        (object_pose.position.x - self.pose.position.x).abs()
            + (object_pose.position.y - self.pose.position.y).abs()
    }

    fn constrain_vel(&self, state: &ObjectSetState) -> f64 {
        let object_vel = state.states[self.object_index].vel;
        (object_vel.position.x).abs() + (object_vel.position.y).abs()
    }

    fn get_jacobian_block(&self, state: &ObjectSetState) -> Vec<(usize, usize, f64)> {
        vec![
            (0, self.object_index * 3 + 0, 1.0),
            (1, self.object_index * 3 + 1, 1.0),
        ]
    }

    fn get_jacobian_dot_block(&self, _state: &ObjectSetState) -> Vec<(usize, usize, f64)> {
        vec![]
    }
}
