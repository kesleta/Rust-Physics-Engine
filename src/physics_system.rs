use std::marker::PhantomData;

use sprs::{CsMat, CsVec, TriMat};

use crate::{
    constraint::{constraint_set::ConstraintSet, Constraint},
    force::{sum_force::SumForce, ForceGenerator},
    object::{object_set::ObjectSet, Object},
    ode_solver::OdeSolver,
    state::{object_set_state::ObjectSetState, object_state::ObjectState},
    Scalar, V2,
};

pub struct PhysicsSystem<S: OdeSolver> {
    pub object_set: ObjectSet,
    pub force_set: SumForce,
    pub constraint_set: ConstraintSet,
    ode_solver: PhantomData<S>,
}

impl<S: OdeSolver> PhysicsSystem<S> {
    pub fn new(
        objects: Vec<Box<dyn Object>>,
        forces: Vec<Box<dyn ForceGenerator>>,
        constraints: Vec<Box<dyn Constraint>>,
    ) -> Self {
        let object_set = ObjectSet::new(objects);
        let constraint_set = ConstraintSet::new(constraints);
        if !constraint_set.check_initial(&object_set.get_states()) {
            panic!("Initial conditions to not fit constraints");
        };
        Self {
            object_set,
            force_set: SumForce::new(forces),
            constraint_set,
            ode_solver: PhantomData,
        }
    }

    pub fn update(&mut self, dt: f64) {
        let state = self.object_set.get_states();
        let masses = self.object_set.get_masses();
        
        let constraint_forces: Vec<V2> = vec![]; //TODO
        let new_state = S::solve(
            &state,
            |set_state| {
                ObjectSetState::new(
                    self.force_set
                        .get_force(set_state)
                        .iter()
                        .zip(&constraint_forces)
                        .zip(&masses)
                        .zip(state.get_velocities())
                        .map(|(((f, c), m), v)| ObjectState::new(v, (*f + *c) * m.recip()))
                        .collect(),
                )
            },
            dt,
        );
        self.object_set.set_states(new_state);
    }

    pub fn get_energy(&self) -> Scalar {
        self.force_set.get_potential(&self.object_set.get_states()) + self.object_set.get_kinetic()
    }
}
