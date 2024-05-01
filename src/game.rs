use std::time::Duration;

use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::prelude::*;

use self::{
    events::{Bounce, ScoreGoal},
    systems::{
        accelerate_with_time, move_paddles, rebound, respawn_ball_on_lock, score_goal, scoring,
        spawn_ball, spawn_camera,
        ui::{display_score, update_score},
        update_last_hit,
    },
};

mod components;
mod events;
mod systems;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, spawn_ball, display_score))
            .add_plugins(PhysicsPlugins::default())
            .add_systems(
                Update,
                (
                    move_paddles,
                    rebound,
                    score_goal,
                    scoring,
                    update_score,
                    respawn_ball_on_lock,
                ),
            )
            .add_systems(FixedUpdate, (accelerate_with_time, update_last_hit))
            .add_plugins(InputManagerPlugin::<GameAction>::default())
            .insert_resource(Score::default())
            .insert_resource(Gravity(Vec2::ZERO))
            .insert_resource(LastPong::default())
            .add_event::<ScoreGoal>()
            .add_event::<Bounce>();
    }
}

#[derive(Resource, Default)]
pub struct LastPong {
    pub last: Duration,
}

#[derive(Resource, Default)]
pub struct Score {
    pub left: u32,
    pub right: u32,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Actionlike, Reflect, Copy)]
pub enum GameAction {
    PaddleUp,
    PaddleDown,
}
