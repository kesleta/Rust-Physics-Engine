use std::ops::{Add, Mul};

use crate::Scalar;

pub mod object_set_state;
pub mod object_state;

pub trait Multiplier {
    fn from_scalar(scaler: Scalar) -> Self;
}

pub trait State: Clone + Add<Output = Self> + Mul<Scalar, Output = Self> {}
