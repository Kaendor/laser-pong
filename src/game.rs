use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::prelude::*;

use self::{
    events::Bounce,
    systems::{move_paddles, rebound, spawn_ball, spawn_camera},
};

mod components;
mod events;
mod systems;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, spawn_ball))
            .add_plugins(PhysicsPlugins::default())
            .add_systems(Update, (move_paddles, rebound))
            .add_plugins(InputManagerPlugin::<GameAction>::default())
            .insert_resource(Gravity(Vec2::ZERO))
            .add_event::<Bounce>();
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Actionlike, Reflect, Copy)]
pub enum GameAction {
    PaddleUp,
    PaddleDown,
}
