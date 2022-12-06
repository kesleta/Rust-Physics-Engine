use crate::state::State;

use super::OdeSolver;

pub struct Euler;
impl OdeSolver for Euler {
    fn solve<S, F>(curr_state: &S, f: F, dt: f64) -> S
    where
        S: State,
        F: Fn(&S) -> S,
    {
        curr_state.clone() + f(&curr_state) * dt
    }
}
