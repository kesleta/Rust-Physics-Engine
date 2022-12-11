use nalgebra::{DMatrix, DVector};

use crate::{
    force::ForceGenerator, ode_solver::OdeSolver, physics_system::PhysicsSystem, state::pose::Pose,
};

pub fn solve<T: OdeSolver>(physics_system: &PhysicsSystem<T>) -> Vec<Pose> {
    let state = physics_system.object_set.get_states();
    let poses: Vec<Pose> = state.states.iter().map(|s| s.pose).collect();
    let vels: Vec<Pose> = state.states.iter().map(|s| s.vel).collect();

    let masses = physics_system.object_set.get_masses();
    let net_force = physics_system
        .force_set
        .get_force(&physics_system.object_set.get_states());
    let constraints = &physics_system.constraint_set.constraints;

    if constraints.len() == 0 {
        return poses.iter().map(|_| Pose::zero()).collect();
    }

    let mut jacobian_matrix = DMatrix::from_element(constraints.len() * 3, poses.len() * 3, 0.0);
    for (c_i, c) in constraints.iter().enumerate() {
        for (r, c, x) in c.get_jacobian_block(&state) {
            jacobian_matrix[(c_i * 3 + r, c)] = x;
        }
    }

    let inverse_mass_vector = DVector::from_iterator(
        masses.len() * 3,
        masses
            .iter()
            .map(|m| [m.recip(), m.recip(), m.recip()])
            .flatten(),
    );
    let inverse_mass_matrix = DMatrix::from_diagonal(&inverse_mass_vector);

    let left_hand_matrix =
        &jacobian_matrix * &(&inverse_mass_matrix * &jacobian_matrix.transpose());
    //----------------------------------------------------------------------
    let mut jacobian_dot_matrix = DMatrix::from_element(constraints.len() * 3, vels.len() * 3, 0.0);
    for (c_i, c) in constraints.iter().enumerate() {
        for (r, c, x) in c.get_jacobian_dot_block(&state) {
            jacobian_dot_matrix[(c_i * 3 + r, c)] = x;
        }
    }

    let vels_vector = DVector::from_iterator(
        vels.len() * 3,
        vels.iter()
            .map(|v| [v.position.x, v.position.y, v.angle])
            .flatten(),
    );

    let term_1 = -(&jacobian_dot_matrix * &vels_vector); //-J'q'

    let force_vector = DVector::from_iterator(
        poses.len() * 3,
        net_force
            .iter()
            .map(|f| [f.position.x, f.position.y, f.angle])
            .flatten(),
    );

    let term_2 = -(&jacobian_matrix * &(&inverse_mass_matrix * &force_vector)); //-JWQ

    let mut right_hand_vector = DVector::from_element(constraints.len() * 3, 0.0);
    term_1.add_to(&term_2, &mut right_hand_vector);


    println!("J: {}", &jacobian_matrix);
    // println!("JWJT: {}", &left_hand_matrix);
    dbg!(&left_hand_matrix.determinant());

    let lambda = left_hand_matrix
        .lu()
        .solve(&right_hand_vector)
        .expect("Failed to solve linear system");

    let result: Vec<_> = (&jacobian_matrix.transpose() * &lambda)
        .iter()
        .map(|x| *x)
        .collect();
    result
        .chunks(3)
        .map(|vs| Pose::new(vs[0], vs[1], vs[2]))
        .collect()
}
