use std::marker::PhantomData;

use crate::{
    force::{sum_force::SumForce, ForceGenerator},
    object::{object_set::ObjectSet, Object},
    ode_solver::OdeSolver,
};

pub struct PhysicsSystem<'a, const N: usize, S: OdeSolver> {
    pub object_set: ObjectSet<'a, N>,
    pub force_set: SumForce<'a, N>,
    ode_solver: PhantomData<S>,
}

impl<'a, const N: usize, S: OdeSolver> PhysicsSystem<'a, N, S> {
    pub fn new(os: [&'a mut dyn Object; N], fs: Vec<&'a dyn ForceGenerator<N>>) -> Self {

        Self {
            object_set: ObjectSet {
                objects: os,
            },
            force_set: SumForce {
                forces: fs,
            },
            ode_solver: PhantomData,
        }
    }

    pub fn update(&mut self, dt: f64) {
        let state = self.object_set.get_states();
        let masses = self.object_set.get_masses();
        let new_state = S::solve(
            &state,
            |s| self.force_set.get_force(s) * (masses.map(|m| 1.0 / m)),
            dt,
        );
        self.object_set.set_states(new_state);
    }
}