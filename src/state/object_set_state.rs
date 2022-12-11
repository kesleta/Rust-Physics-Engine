use std::{
    iter::Sum,
    ops::{Add, Mul},
};

use super::{object_state::ObjectState, pose::Pose, State};

#[derive(Clone, Debug)]
pub struct ObjectSetState {
    pub states: Vec<ObjectState>,
}

impl ObjectSetState {
    pub fn new(states: Vec<ObjectState>) -> Self {
        Self { states }
    }

    pub fn get_positions(&self) -> Vec<Pose> {
        self.states.iter().map(|s| s.pose).collect()
    }

    pub fn get_velocities(&self) -> Vec<Pose> {
        self.states.iter().map(|s| s.vel).collect()
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

impl Mul<f64> for ObjectSetState {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        ObjectSetState {
            states: self.states.iter().map(|s| *s * rhs).collect(),
        }
    }
}

impl State for ObjectSetState {}
