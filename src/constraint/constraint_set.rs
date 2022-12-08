use nannou::winit::dpi::Position;

use crate::{state::object_set_state::ObjectSetState, Scalar, V2};

use super::Constraint;

pub struct ConstraintSet {
    pub constraints: Vec<Box<dyn Constraint>>,
}

impl ConstraintSet {
    pub fn new(constraints: Vec<Box<dyn Constraint>>) -> Self {
        Self { constraints }
    }

    pub fn check_initial(&self, initial_state: &ObjectSetState) -> bool {
        let positions: Vec<_> = initial_state.states.iter().map(|s| s.position).collect();
        let velocities: Vec<_> = initial_state.states.iter().map(|s| s.velocity).collect();
        self.constraints
            .iter()
            .map(|c| {
                c.constrain_position(&positions).abs() + c.constrain_velocities(&velocities).abs()
            })
            .sum::<Scalar>()
            == 0.0
    }

    pub fn compute_constraint_vector(&self, positions: &Vec<V2>) -> Vec<Scalar> {
        self.constraints
            .iter()
            .map(|c| c.constrain_position(positions))
            .collect()
    }

    pub fn compute_constraint_vector_dot(&self, velocities: &Vec<V2>) -> Vec<Scalar> {
        self.constraints
            .iter()
            .map(|c| c.constrain_velocities(velocities))
            .collect()
    }

    pub fn get_jacobian(&self, positions: &Vec<V2>) -> Vec<(usize, usize, V2)> {
        self.constraints
            .iter()
            .enumerate()
            .map(|(c_i, c)| {
                c.get_jacobian_slice(&positions)
                    .into_iter()
                    .map(move |(i, v)| (i, c_i, v))
            })
            .flatten()
            .collect()
    }

    pub fn get_jacobian_dot(&self, velocities: &Vec<V2>) -> Vec<(usize, usize, V2)> {
        self.constraints
            .iter()
            .enumerate()
            .map(|(c_i, c)| {
                c.get_jacobian_dot_slice(&velocities)
                    .into_iter()
                    .map(move |(i, v)| (c_i, i, v))
            })
            .flatten()
            .collect()
    }
}
