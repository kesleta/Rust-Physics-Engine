use std::ops::{Add, Mul};

use crate::Scalar;

pub mod object_set_state;
pub mod object_state;

pub trait Multiplier {
    fn from_scalar(scaler: Scalar) -> Self;
}

impl Multiplier for Scalar {
    fn from_scalar(scaler: Scalar) -> Self {
        scaler
    }
}

pub trait State:
    Copy + Add<Output = Self> + Mul<Scalar, Output = Self> + Mul<Self::Multiplier, Output = Self>
{
    type Multiplier: Multiplier;
}
