use crate::{state::object_set_state::ObjectSetState, Scalar};

use super::Constraint;

pub struct ConstraintSet {
    constraints: Vec<Box<dyn Constraint>>,
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
}
