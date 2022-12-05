use std::{
    iter::Sum,
    ops::{Add, Mul},
};

use crate::{Scalar, V2};

use super::{object_state::ObjectState, Multiplier, State};

#[derive(Clone, Copy)]
pub struct ObjectSetState<const N: usize> {
    pub states: [ObjectState; N],
}

impl<const N: usize> ObjectSetState<N> {
    pub fn new(states: [ObjectState; N]) -> Self {
        Self { states }
    }

    pub fn zero() -> Self {
        Self {
            states: [ObjectState::zero(); N],
        }
    }
}

impl<const N: usize> Add for ObjectSetState<N> {
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

impl<const N: usize> Sum for ObjectSetState<N> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |a, b| a + b)
    }
}

impl<const N: usize> Multiplier for [Scalar; N] {
    fn from_scalar(scaler: Scalar) -> Self {
        [scaler; N]
    }
}

impl<const N: usize> Mul<[Scalar; N]> for ObjectSetState<N> {
    type Output = Self;

    fn mul(self, rhs: [Scalar; N]) -> Self::Output {
        ObjectSetState {
            states: self
                .states
                .iter()
                .zip(rhs)
                .map(|(s, m)| *s * m)
                .collect::<Vec<ObjectState>>()
                .try_into()
                .unwrap(),
        }
    }
}

impl<const N: usize> Mul<Scalar> for ObjectSetState<N> {
    type Output = Self;

    fn mul(self, rhs: Scalar) -> Self::Output {
        ObjectSetState {
            states: self.states.map(|s| s * rhs),
        }
    }
}

impl<const N: usize> State for ObjectSetState<N> {
    type Multiplier = [Scalar; N];
}
