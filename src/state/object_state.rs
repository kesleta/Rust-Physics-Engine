use std::ops::{Add, Mul};

use crate::V2;

use super::State;

#[derive(Clone, Copy)]
pub struct ObjectState {
    pub position: V2,
    pub velocity: V2,
}

impl ObjectState {
    pub fn new(position: V2, velocity: V2) -> Self {
        Self { position, velocity }
    }
}

impl Add for ObjectState {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            position: self.position + rhs.position,
            velocity: self.velocity + rhs.velocity,
        }
    }
}

impl Mul<f64> for ObjectState {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            position: self.position * rhs,
            velocity: self.velocity * rhs,
        }
    }
}

impl State for ObjectState {}
