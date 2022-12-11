use nalgebra::DVector;

use crate::state::{self, object_set_state::ObjectSetState, pose::Pose};

use super::Constraint;

#[derive(Debug, Clone)]
pub struct Rod {
    object_1_index: usize,
    object_2_index: usize,
    length: f64,
}

impl Rod {
    pub fn new(object_1_index: usize, object_2_index: usize, length: f64) -> Self {
        Self {
            object_1_index,
            object_2_index,
            length,
        }
    }
}

impl Constraint for Rod {
    fn constrain_pose(&self, state: &ObjectSetState) -> f64 {
        let position1 = state.states[self.object_1_index].pose.position;
        let position2 = state.states[self.object_2_index].pose.position;
        (position2 - position1).magnitude() - self.length
    }

    fn constrain_vel(&self, state: &ObjectSetState) -> f64 {
        let position1 = state.states[self.object_1_index].pose.position;
        let position2 = state.states[self.object_2_index].pose.position;
        let velocity1 = state.states[self.object_1_index].vel.position;
        let velocity2 = state.states[self.object_2_index].vel.position;

        2.0 * (position2 - position1).dot(velocity2 - velocity1)
    }

    fn get_jacobian_block(&self, state: &ObjectSetState) -> Vec<(usize, usize, f64)> {
        let position1 = state.states[self.object_1_index].pose.position;
        let position2 = state.states[self.object_2_index].pose.position;

        let dist = (position2 - position1).magnitude();
        let d_x = position2.x - position1.x;
        let d_y = position2.y - position1.y;

        vec![
            (0, self.object_1_index * 3 + 0, 1.0),//2.0 * d_x / dist),
            (1, self.object_1_index * 3 + 1, 2.0),//2.0 * d_y / dist),
            (0, self.object_2_index * 3 + 0, 3.0),//2.0 * d_x / dist),
            (1, self.object_2_index * 3 + 1, 4.0),//2.0 * d_y / dist),
        ]
    }

    fn get_jacobian_dot_block(&self, state: &ObjectSetState) -> Vec<(usize, usize, f64)> {
        let position1 = state.states[self.object_1_index].pose.position;
        let position2 = state.states[self.object_2_index].pose.position;

        vec![
            (
                0,
                self.object_1_index + 0,
                2.0 * (position2.x - position1.x),
            ),
            (
                1,
                self.object_1_index + 1,
                2.0 * (position2.y - position1.x),
            ),
            (
                0,
                self.object_2_index + 0,
                -2.0 * (position2.x - position1.x),
            ),
            (
                1,
                self.object_2_index + 1,
                -2.0 * (position2.y - position1.x),
            ),
        ]
    }
}
