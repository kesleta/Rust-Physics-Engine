pub mod euler;
pub mod runge_kutta_4;

use crate::state::{Multiplier, State};

pub trait OdeSolver {
    //S is the state, F is the right hand side of the first order differential equation
    fn solve<M, S, F>(curr_state: &S, f: F, dt: f64) -> S
    where
        M: Multiplier,
        S: State<Multiplier = M>,
        F: Fn(&S) -> S;
}
