mod constraint;
mod force;
mod object;
mod ode_solver;
mod physics_system;
mod state;

use constraint::anchor::Anchor;
use force::{quad_damping::QuadDamper, spring::Spring};
use nannou::prelude::*;
use object::ball::Ball;
use ode_solver::rk4::RK4;
use physics_system::PhysicsSystem;

pub type Scalar = f64;
pub type V2 = vek::vec2::Vec2<Scalar>;

struct Model {
    physics_system: PhysicsSystem<RK4>,
}

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    // app.set_loop_mode(LoopMode::rate_fps(1.0)); Not effective for some reason: https://github.com/nannou-org/nannou/issues/708

    let ball1 = Ball::new(V2::new(-200.0, 0.0), V2::zero(), 1.0, 50.0);
    let ball2 = Ball::new(V2::new(200.0, 0.0), V2::zero(), 1.0, 50.0);

    let spring = Spring::new(0, 1, 300.0, 5.0);
    let damping = QuadDamper::new(0.001);

    let anchor = Anchor::new(0, V2::new(-200.0, 0.0));
    Model {
        physics_system: PhysicsSystem::new(
            vec![Box::new(ball1), Box::new(ball2)],
            vec![Box::new(spring), Box::new(damping)],
            vec![Box::new(anchor)],
        ),
    }
}

fn event(_app: &App, model: &mut Model, event: Event) {
    match event {
        Event::Update(u) => {
            let ups = 10000;
            let sub_divisions = ups / 60;
            for _ in 0..sub_divisions {
                model
                    .physics_system
                    .update(u.since_last.as_secs_f64() / (sub_divisions as f64));
            }
            println!("{}", model.physics_system.get_energy());
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
