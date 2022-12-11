mod constraint;
mod constraint_solver;
mod force;
mod object;
mod ode_solver;
mod physics_system;
mod state;

use constraint::{anchor::Anchor, rod::Rod};
use force::{
    gravity::{self, Gravity},
    linear_damping::LinearDamper,
    quad_damping::QuadDamper,
    spring::Spring,
};
use nannou::{color::gradient, prelude::*};
use object::ball::Ball;
use ode_solver::rk4::RK4;
use physics_system::PhysicsSystem;
use state::pose::Pose;

struct Model {
    physics_system: PhysicsSystem<RK4>,
}

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    // app.set_loop_mode(LoopMode::rate_fps(1.0)); Not effective for some reason: https://github.com/nannou-org/nannou/issues/708

    let ball1 = Ball::new(Pose::new(0.0, 100.0, 0.0), Pose::zero(), 1.0);
    let ball2 = Ball::new(Pose::new(400.0, 100.0, 0.0), Pose::zero(), 1.0);

    let spring = Spring::new(0, 1, 300.0, 30.0);
    let gravity = Gravity::new(1000.0);
    let damping = LinearDamper::new(0.04);

    let anchor = Anchor::new(0, Pose::new(0.0, 100.0, 0.0));
    let rod = Rod::new(0, 1, 400.0);

    Model {
        physics_system: PhysicsSystem::new(
            vec![Box::new(ball1), Box::new(ball2)],
            vec![Box::new(gravity), Box::new(damping)],
            vec![Box::new(anchor), Box::new(rod)],
        ),
    }
}

fn event(_app: &App, model: &mut Model, event: Event) {
    match event {
        Event::Update(u) => {
            let ups = 20000;
            let sub_divisions = ups / 60;
            for _ in 0..sub_divisions {
                model
                    .physics_system
                    .update(u.since_last.as_secs_f64() / (sub_divisions as f64));
            }
            // println!("{}", model.physics_system.get_energy());
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
        .x_y(state1.pose.position.x as f32, state1.pose.position.y as f32)
        .radius(10.0) // TODO!!!!!
        .color(object_color)
        .stroke(BLACK)
        .stroke_weight(outline_weight);
    draw.ellipse()
        .x_y(state2.pose.position.x as f32, state2.pose.position.y as f32)
        .radius(10.0) // TODO!!!!!
        .color(object_color)
        .stroke(BLACK)
        .stroke_weight(outline_weight);
    draw.line()
        .points(
            Vec2::new(state1.pose.position.x as f32, state1.pose.position.y as f32),
            Vec2::new(state2.pose.position.x as f32, state2.pose.position.y as f32),
        )
        .weight(4.0);

    draw.to_frame(app, &frame).unwrap();
}

//Todo:
//  -Add drift correction terms to constraint solver
//  -Add more constraint types
//      -Distance
//      -x/y/theta axis fixed
//      -Rotation
//  -Allow forces to interact with object masses
//  -Add local coordinate system for objects
//  -Add even more constraints that connect two objects using local coordinates
//      -Nail (completly fixed together t some relative point on both objects, rotation and all)
//      -Pin (Same as nail but freely rotating)
//  -Collisions somehow, no idea where to start
//      -Possibly add non-overlapping constraint, and have some sort of reation when two bodys are touching
