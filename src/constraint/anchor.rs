use crate::{Scalar, V2};

use super::Constraint;

pub struct Anchor {
    pub anchor_index: usize,
    pub position: V2,
}

impl Anchor {
    pub fn new(object_index: usize, position: V2) -> Self {
        Self {
            anchor_index: object_index,
            position,
        }
    }
}

impl Constraint for Anchor {
    fn constrain_position(&self, positions: &Vec<V2>) -> Scalar {
        assert!(self.anchor_index < positions.len());
        let pos = positions[self.anchor_index];
        (pos - self.position).magnitude()
    }

    fn constrain_velocities(&self, velocities: &Vec<V2>) -> Scalar {
        assert!(self.anchor_index < velocities.len());
        let vel = velocities[self.anchor_index];
        vel.magnitude()
    }

    fn constrain_acceleration(&self, accelerations: &Vec<V2>) -> Scalar {
        assert!(self.anchor_index < accelerations.len());
        let acc = accelerations[self.anchor_index];
        acc.magnitude()
    }

    fn get_jacobian_slice(&self, positions: &Vec<V2>) -> Vec<(usize, V2)> {
        let pos = positions[self.anchor_index];
        vec![(self.anchor_index, todo!())]
    }

    fn get_jacobian_dot_slice(&self, velocities: &Vec<V2>) -> Vec<(usize, V2)> {
        vec![(self.anchor_index, todo!())]
    }
}
