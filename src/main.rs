mod force;
mod object;
mod ode_solver;
mod physics_system;
mod state;

use force::{spring::Spring, dampening::{self, Dampening}};
use nannou::prelude::*;
use object::ball::Ball;
use ode_solver::{euler::Euler, runge_kutta_4::RungeKutta4};
use physics_system::PhysicsSystem;

pub type Scalar = f64;
pub type V2 = vek::vec2::Vec2<Scalar>;

struct Model {
    ball1: Ball,
    ball2: Ball,
}

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    Model {
        ball1: Ball::new(V2::new(-200.0, 0.0), V2::zero(), 1.0, 50.0),
        ball2: Ball::new(V2::new(200.0, 0.0), V2::zero(), 1.0, 50.0),
    }
}

fn event(_app: &App, model: &mut Model, event: Event) {
    match event {
        Event::Update(u) => {
            let spring = Spring::new(0, 1, 300.0, 1.0);
            let dampening = Dampening::new(0.1);
            let mut physics_system: PhysicsSystem<2, RungeKutta4> =
                PhysicsSystem::new([&mut model.ball1, &mut model.ball2], vec![&spring, &dampening]);
            physics_system.update(u.since_last.as_secs_f64());
        }
        _ => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let background_color = rgb(0.3, 0.3, 0.3);
    let object_color = rgb(0.2, 0.2, 0.2);
    let outline_weight = 1.5;

    draw.background().color(background_color);

    let ball1_pos = model.ball1.position;
    let ball2_pos = model.ball2.position;

    draw.ellipse()
        .x_y(ball1_pos.x as f32, ball1_pos.y as f32)
        .radius(model.ball1.radius as f32)
        .color(object_color)
        .stroke(BLACK)
        .stroke_weight(outline_weight);
    draw.ellipse()
        .x_y(ball2_pos.x as f32, ball1_pos.y as f32)
        .color(object_color)
        .radius(model.ball2.radius as f32)
        .stroke(BLACK)
        .stroke_weight(outline_weight);
    draw.line()
        .points(
            Vec2::new(ball1_pos.x as f32, ball1_pos.y as f32),
            Vec2::new(ball2_pos.x as f32, ball2_pos.y as f32),
        )
        .weight(4.0);

    draw.to_frame(app, &frame).unwrap();
}
