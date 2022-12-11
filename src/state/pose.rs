use std::ops::{Add, Mul, Neg};

use super::State;

type V2 = vek::Vec2<f64>;

#[derive(Clone, Copy, Debug)]
pub struct Pose {
    pub position: V2,
    pub angle: f64,
}

impl Pose {
    pub fn new(x: f64, y: f64, angle: f64) -> Self {
        Self {
            position: V2::new(x, y),
            angle,
        }
    }

    pub fn from_vector(v: &V2) -> Self {
        Self {
            position: *v,
            angle: 0.0,
        }
    }

    pub fn zero() -> Pose {
        Self {
            position: V2::zero(),
            angle: 0.0,
        }
    }
}

impl Add for Pose {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            position: self.position + rhs.position,
            angle: self.angle + rhs.angle,
        }
    }
}

impl Mul<f64> for Pose {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            position: self.position * rhs,
            angle: self.angle * rhs,
        }
    }
}

impl Neg for Pose {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl State for Pose {}
