pub mod euler;
mod runge_kutta_4;

use std::ops::{Add, Mul};

pub trait OdeSolver {
    //S is the state, F is the right hand side of the first order differential equation
    fn solve<S, F>(curr_state: &S, f: F, dt: f64) -> S
    where
        S: Copy + Add<Output = S> + Mul<f64, Output = S>,
        F: Fn(&S) -> S;
}
