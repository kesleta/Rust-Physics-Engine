use std::ops::{Add, Mul};

use super::OdeSolver;

struct Euler;
impl OdeSolver for Euler {
    fn solve<S: Copy + Add<Output = S> + Mul<f64, Output = S>, F: Fn(&S) -> S>(
        curr_state: &S,
        f: F,
        dt: f64,
    ) -> S {
        *curr_state + f(&curr_state) * dt
    }
}
