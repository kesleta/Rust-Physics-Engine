use crate::state::{object_set_state::ObjectSetState, pose::Pose};

use super::Constraint;

pub struct ConstraintSet {
    pub constraints: Vec<Box<dyn Constraint>>,
}

impl ConstraintSet {
    pub fn new(constraints: Vec<Box<dyn Constraint>>) -> Self {
        Self { constraints }
    }

    pub fn check_initial(&self, initial_state: &ObjectSetState) -> bool {
        self.constraints
            .iter()
            .map(|c| c.constrain_pose(&initial_state).abs() + c.constrain_vel(&initial_state).abs())
            .sum::<f64>()
            == 0.0
    }

    pub fn get_constraint_vector(&self, state: &ObjectSetState) -> Vec<f64> {
        self.constraints
            .iter()
            .map(|c| c.constrain_pose(state))
            .collect()
    }

    pub fn get_constraint_dot_vector(&self, state: &ObjectSetState) -> Vec<f64> {
        self.constraints
            .iter()
            .map(|c| c.constrain_vel(state))
            .collect()
    }

    pub fn get_jacobian_matrix(&self, state: &ObjectSetState) -> Vec<(usize, usize, f64)> {
        self.constraints
            .iter()
            .enumerate()
            .map(|(c_i, c)| {
                c.get_jacobian_block(&state)
                    .into_iter()
                    .map(move |(i, j, p)| (i + 3 * c_i, j, p))
            })
            .flatten()
            .collect()
    }

    pub fn get_jacobian_dot_matrix(&self, state: &ObjectSetState) -> Vec<(usize, usize, f64)> {
        self.constraints
            .iter()
            .enumerate()
            .map(|(c_i, c)| {
                c.get_jacobian_dot_block(&state)
                    .into_iter()
                    .map(move |(i, j, p)| (i + 3 * c_i, j, p))
            })
            .flatten()
            .collect()
    }
}
