use crate::{Scalar, V2};

use super::Constraint;

pub struct Anchor {
    pub object_index: usize,
    pub position: V2,
}

impl Anchor {
    pub fn new(object_index: usize, position: V2) -> Self {
        Self {
            object_index,
            position,
        }
    }
}

impl Constraint for Anchor {
    fn constrain_position(&self, positions: &Vec<V2>) -> Scalar {
        assert!(self.object_index < positions.len());
        let pos = positions[self.object_index];
        (pos - self.position.x).magnitude()
    }

    fn constrain_velocities(&self, velocities: &Vec<V2>) -> Scalar {
        assert!(self.object_index < velocities.len());
        let vel = velocities[self.object_index];
        vel.magnitude()
    }

    fn constrain_acceleration(&self, accelerations: &Vec<V2>) -> Scalar {
        assert!(self.object_index < accelerations.len());
        let acc = accelerations[self.object_index];
        acc.magnitude()
    }
}
