mod force;
mod state;
mod ode_solver;
mod object;

pub type Scalar = f64;
pub type V2 = vek::vec2::Vec2<Scalar>;
use force::spring::Spring;
use object::ball::Ball;

fn main() {
    let mut ball = Ball::new(V2::new(0.0, -200.0), V2::zero(), 1.0, 1.0);
    let origin = V2::new(0.0, 0.0);
    let spring = Spring::new(&origin, &ball.position, 200.0, 1.0);
    println!("Hello, world!");
}
