use std::ops::{Add, Mul};

use crate::{Scalar, V2};

use super::{object_state::ObjectState, Multiplier, State};

#[derive(Clone, Copy)]
struct ObjectSetState<const N: usize> {
    states: [ObjectState; N],
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

impl<const N: usize> Multiplier for [Scalar; N] {
    fn from_scalar(scaler: Scalar) -> Self {
        [scaler; N]
    }
}

impl<const N: usize> State for ObjectSetState<N> {
    type Multiplier = [Scalar; N];
}
