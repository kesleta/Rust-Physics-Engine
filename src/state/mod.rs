use std::ops::{Add, Mul};

use crate::Scalar;

pub mod object_state;

trait State: Copy + Add<Output = Self> + Mul<Scalar, Output = Self> {}
