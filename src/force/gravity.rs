use crate::{
    state::{object_set_state::ObjectSetState, object_state::ObjectState},
    Scalar, V2,
};

use super::ForceGenerator;

pub struct Gravity {
    pub strength: Scalar,
}

impl Gravity {
    pub fn new(strength: Scalar) -> Self {
        Self { strength }
    }
}

impl<const N: usize> ForceGenerator<N> for Gravity {
    fn get_force(&self, system_state: &ObjectSetState<N>) -> ObjectSetState<N> {
        let mut result: Vec<_> = Vec::new();
        for obj_state in system_state.states {
            result.push(ObjectState::new(
                obj_state.velocity,
                V2::new(0.0, -self.strength),
            ))
        }
        ObjectSetState::new(result.try_into().unwrap())
    }
}
