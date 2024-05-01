use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct Ball;

#[derive(Component, Copy, Clone)]
pub struct Paddle {
    pub width: f32,
    pub height: f32,
}

#[derive(Component, Clone)]
pub struct LeftWall;

#[derive(Component, Clone)]
pub struct RightWall;

#[derive(Debug, Clone, Copy, Component)]
pub enum Side {
    Left,
    Right,
}
