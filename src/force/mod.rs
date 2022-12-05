pub mod gravity;
pub mod spring;
pub mod sum_force;

use crate::state::object_state::ObjectState;

pub use self::sum_force::SumForce;

pub trait ForceGenerator {
    fn get_force(&self, curr_state: &ObjectState) -> ObjectState;

    fn add<'a, F: ForceGenerator>(
        &'a self,
        rhs: &'a F,
    ) -> SumForce<Self, F> {
        SumForce { f1: self, f2: rhs }
    }
}
