use crate::state::State;

use super::OdeSolver;

pub struct RK4;
impl OdeSolver for RK4 {
    fn solve<S, F>(curr_state: &S, f: F, dt: f64) -> S
    where
        S: State,
        F: Fn(&S) -> S,
    {
        let k1 = f(curr_state);
        let k2 = f(&(curr_state.clone() + k1.clone() * (dt / 2.0)));
        let k3 = f(&(curr_state.clone() + k2.clone() * (dt / 2.0)));
        let k4 = f(&(curr_state.clone() + k3.clone() * dt));
        curr_state.clone() + (k1 + (k2 * 2.0) + (k3 * 2.0) + k4) * (1.0 / 6.0) * dt
    }
}
