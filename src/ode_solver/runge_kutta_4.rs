use super::OdeSolver;

struct RungeKutta4;
impl OdeSolver for RungeKutta4 {
    fn solve<
        S: Copy + std::ops::Add<Output = S> + std::ops::Mul<f64, Output = S>,
        F: Fn(&S) -> S,
    >(
        curr_state: &S,
        f: F,
        dt: f64,
    ) -> S {
        let k1 = f(curr_state);
        let k2 = f(&(*curr_state + k1 * (dt / 2.0)));
        let k3 = f(&(*curr_state + k2 * (dt / 2.0)));
        let k4 = f(&(*curr_state + k3 * dt));
        *curr_state + (k1 + (k2 * 2.0) + (k3 * 2.0) + k4) * (1.0 / 6.0) * dt
    }
}
