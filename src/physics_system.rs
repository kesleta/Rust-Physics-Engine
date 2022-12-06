use std::marker::PhantomData;

use crate::{
    force::{sum_force::SumForce, ForceGenerator},
    object::{object_set::ObjectSet, Object},
    ode_solver::OdeSolver,
    state::{object_set_state::ObjectSetState, object_state::ObjectState}, Scalar,
};

pub struct PhysicsSystem<S: OdeSolver> {
    pub object_set: ObjectSet,
    pub force_set: SumForce,
    ode_solver: PhantomData<S>,
}

impl<S: OdeSolver> PhysicsSystem<S> {
    pub fn new(objects: Vec<Box<dyn Object>>, forces: Vec<Box<dyn ForceGenerator>>) -> Self {
        Self {
            object_set: ObjectSet { objects },
            force_set: SumForce { forces },
            ode_solver: PhantomData,
        }
    }

    pub fn update(&mut self, dt: f64) {
        let state = self.object_set.get_states();
        let masses = self.object_set.get_masses();
        let new_state = S::solve(
            &state,
            |set_state| {
                ObjectSetState::new(
                    self.force_set
                        .get_force(set_state)
                        .iter()
                        .zip(state.states.iter().map(|s| s.velocity))
                        .zip(masses.iter())
                        .map(|((a, v), m)| ObjectState::new(v, *a * m.recip()))
                        .collect(),
                )
            },
            dt,
        );
        self.object_set.set_states(new_state);
    }

    pub fn get_energy(&self) -> Scalar{
        self.force_set.get_potential(&self.object_set.get_states()) + self.object_set.get_kinetic()
    }
}
