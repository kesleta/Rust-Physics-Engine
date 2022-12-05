use crate::{force::spring::Spring, object::ball::Ball, physics_system::PhysicsSystem};

mod force;
mod object;
mod ode_solver;
mod physics_system;
mod state;

use ode_solver::euler::Euler;

pub type Scalar = f64;
pub type V2 = vek::vec2::Vec2<Scalar>;

fn main() {
    let mut ball1 = Ball::new(V2::new(-100.0, 0.0), V2::zero(), 1.0, 20.0);
    let mut ball2 = Ball::new(V2::new(100.0, 0.0), V2::zero(), 1.0, 20.0);
    let spring = Spring::new(0, 1, 190.0, 1.0);
    let mut physics_system: PhysicsSystem<2, Euler> =
        PhysicsSystem::new([&mut ball1, &mut ball2], vec![&spring]);

    physics_system.update(0.01);
    physics_system.update(0.01);
    physics_system.update(0.01);
    physics_system.update(0.01);
    println!("Hello, world!");
}
