use std::{
    iter::Sum,
    ops::{Add, Mul},
};

use crate::{Scalar, V2};

use super::{object_state::ObjectState, State};

#[derive(Clone, Debug)]
pub struct ObjectSetState {
    pub states: Vec<ObjectState>,
}

impl ObjectSetState {
    pub fn new(states: Vec<ObjectState>) -> Self {
        Self { states }
    }

    pub fn get_positions(&self) -> Vec<V2> {
        self.states.iter().map(|s| s.position).collect()
    }

    pub fn get_velocities(&self) -> Vec<V2> {
        self.states.iter().map(|s| s.velocity).collect()
    }
}

impl Add for ObjectSetState {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        ObjectSetState {
            states: self
                .states
                .iter()
                .zip(rhs.states)
                .map(|(a, b)| *a + b)
                .collect::<Vec<ObjectState>>()
                .try_into()
                .unwrap(),
        }
    }
}

impl Sum for ObjectSetState {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|a, b| a + b).unwrap()
    }
}

impl Mul<Scalar> for ObjectSetState {
    type Output = Self;

    fn mul(self, rhs: Scalar) -> Self::Output {
        ObjectSetState {
            states: self.states.iter().map(|s| *s * rhs).collect(),
        }
    }
}

impl State for ObjectSetState {}
