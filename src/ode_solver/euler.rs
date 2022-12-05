use std::ops::{Add, Mul};

use crate::state::{Multiplier, State};

use super::OdeSolver;

pub struct Euler;
impl OdeSolver for Euler {
    fn solve<M, S, F>(curr_state: &S, f: F, dt: f64) -> S
    where
        M: Multiplier,
        S: State<Multiplier = M>,
        F: Fn(&S) -> S,
    {
        *curr_state + f(&curr_state) * dt
    }
}
