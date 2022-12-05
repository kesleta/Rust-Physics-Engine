use crate::state::object_state::ObjectState;

use super::ForceGenerator;

pub struct SumForce<'a, F1: ForceGenerator + ?Sized, F2: ForceGenerator + ?Sized> {
    pub f1: &'a F1,
    pub f2: &'a F2,
}

impl<'a, F1: ForceGenerator + ?Sized, F2: ForceGenerator + ?Sized> SumForce<'a, F1, F2> {
    pub fn new(f1: &'a F1, f2: &'a F2) -> Self {
        Self { f1, f2 }
    }
}

impl<'a, F1: ForceGenerator, F2: ForceGenerator> ForceGenerator for SumForce<'a, F1, F2> {
    fn get_force(&self, curr_state: &ObjectState) -> ObjectState {
        self.f1.get_force(curr_state) + self.f2.get_force(curr_state)
    }
}
