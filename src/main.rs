mod force;
mod object;
mod ode_solver;
mod physics_system;
mod state;

use force::{
    linear_damping::LinearDamper,
    spring::Spring,
};
use nannou::prelude::*;
use object::ball::Ball;
use ode_solver::{euler::Euler, runge_kutta_4::RungeKutta4};
use physics_system::PhysicsSystem;

pub type Scalar = f64;
pub type V2 = vek::vec2::Vec2<Scalar>;

struct Model {
    physics_system: PhysicsSystem<RungeKutta4>,
}

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    let ball1 = Ball::new(V2::new(-200.0, 0.0), V2::zero(), 1.0, 50.0);
    let ball2 = Ball::new(V2::new(200.0, 0.0), V2::zero(), 1.0, 50.0);
    let spring = Spring::new(0, 1, 300.0, 5.0);
    let dampening = LinearDamper::new(0.2);
    Model {
        physics_system: PhysicsSystem::new(
            vec![Box::new(ball1), Box::new(ball2)],
            vec![Box::new(spring), Box::new(dampening)],
        ),
    }
}

fn event(_app: &App, model: &mut Model, event: Event) {
    match event {
        Event::Update(u) => {
            model.physics_system.update(u.since_last.as_secs_f64());
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

    let state1 = model.physics_system.object_set.objects[0].get_state();
    let state2 = model.physics_system.object_set.objects[1].get_state();

    draw.ellipse()
        .x_y(state1.position.x as f32, state1.position.y as f32)
        .radius(40.0) // TODO!!!!!
        .color(object_color)
        .stroke(BLACK)
        .stroke_weight(outline_weight);
    draw.ellipse()
        .x_y(state2.position.x as f32, state2.position.y as f32)
        .radius(40.0) // TODO!!!!!
        .color(object_color)
        .stroke(BLACK)
        .stroke_weight(outline_weight);
    draw.line()
        .points(
            Vec2::new(state1.position.x as f32, state1.position.y as f32),
            Vec2::new(state2.position.x as f32, state2.position.y as f32),
        )
        .weight(4.0);

    draw.to_frame(app, &frame).unwrap();
}
