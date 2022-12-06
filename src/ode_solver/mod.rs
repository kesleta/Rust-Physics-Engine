pub mod euler;
pub mod rk4;

use crate::state::State;

pub trait OdeSolver {
    //S is the state, F is the right hand side of the first order differential equation
    fn solve<S, F>(curr_state: &S, f: F, dt: f64) -> S
    where
        S: State,
        F: Fn(&S) -> S;
}
