use bevy::prelude::*;

use super::components::Side;

#[derive(Debug, Clone, Copy, Event)]
pub struct Bounce {
    pub position: Vec3,
}

#[derive(Debug, Clone, Copy, Event)]
pub struct ScoreGoal {
    pub goal_for: Side,
}
