use std::ops::{Add, Mul};

pub mod object_set_state;
pub mod object_state;
pub mod pose;

pub trait State: Clone + Add<Output = Self> + Mul<f64, Output = Self> + std::fmt::Debug {}
