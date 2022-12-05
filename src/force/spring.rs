use super::ForceGenerator;
use crate::{state::object_state::ObjectState, Scalar, V2};

pub struct Spring<'a> {
    pub f1: SpringForce<'a>,
    pub f2: SpringForce<'a>,
}

impl<'a> Spring<'a> {
    pub fn new(p1: &'a V2, p2: &'a V2, target_length: Scalar, strength: Scalar) -> Self {
        Self {
            f1: SpringForce::new(p2, target_length, strength),
            f2: SpringForce::new(p1, target_length, strength),
        }
    }
}

pub struct SpringForce<'a> {
    other_point: &'a V2,
    target_length: Scalar,
    strength: Scalar,
}

impl<'a> SpringForce<'a> {
    pub fn new(other_point: &'a V2, target_length: Scalar, strength: Scalar) -> Self {
        Self {
            other_point,
            target_length,
            strength,
        }
    }
}

impl<'a> ForceGenerator for SpringForce<'a> {

    fn get_force(&self, curr_state: &ObjectState) -> ObjectState {
        let a_b = self.other_point - curr_state.position;
        let f = a_b.normalized() * (a_b.magnitude() - self.target_length) * self.strength;
        ObjectState::new(curr_state.velocity, f)
    }
}
