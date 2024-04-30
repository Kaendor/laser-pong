use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Event)]
pub struct Bounce {
    pub position: Vec3,
}
