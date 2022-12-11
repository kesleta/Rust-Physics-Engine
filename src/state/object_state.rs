use std::ops::{Add, Mul};

use super::{pose::Pose, State};

#[derive(Clone, Copy, Debug)]
pub struct ObjectState {
    pub pose: Pose,
    pub vel: Pose,
}

impl ObjectState {
    pub fn new(vel: Pose, pose_dot: Pose) -> Self {
        Self {
            pose: vel,
            vel: pose_dot,
        }
    }
}

impl Add for ObjectState {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            pose: self.pose + rhs.pose,
            vel: self.vel + rhs.vel,
        }
    }
}

impl Mul<f64> for ObjectState {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            pose: self.pose * rhs,
            vel: self.vel * rhs,
        }
    }
}

impl State for ObjectState {}
