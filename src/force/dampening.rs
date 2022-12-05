use crate::{
    state::{object_set_state::ObjectSetState, object_state::ObjectState},
    Scalar, V2,
};

use super::ForceGenerator;

pub struct Dampening {
    pub strength: Scalar,
}

impl Dampening {
    pub fn new(strength: Scalar) -> Self {
        Self { strength }
    }
}

impl<const N: usize> ForceGenerator<N> for Dampening {
    fn get_force(&self, system_state: &ObjectSetState<N>) -> ObjectSetState<N> {
        let iter = system_state.states.iter().map(|s| {
            ObjectState::new(
                s.velocity,
                s.velocity.try_normalized().unwrap_or(V2::zero())
                    * -(s.velocity.magnitude_squared() * self.strength),
            )
        });
        ObjectSetState::new(iter.collect::<Vec<_>>().try_into().unwrap())
    }
}
