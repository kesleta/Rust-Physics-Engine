use super::Object;
use crate::state::{object_state::ObjectState, pose::Pose};

#[derive(Debug, Clone)]
pub struct Ball {
    pub pose: Pose,
    pub vel: Pose,
    pub mass: f64,
}

impl Object for Ball {
    fn get_state(&self) -> ObjectState {
        ObjectState::new(self.pose, self.vel)
    }

    fn set_state(&mut self, state: ObjectState) {
        self.pose = state.pose;
        self.vel = state.vel;
    }

    fn get_mass(&self) -> f64 {
        self.mass
    }
}

impl Ball {
    pub fn new(pose: Pose, vel: Pose, mass: f64) -> Self {
        Self { pose, vel, mass }
    }
}
