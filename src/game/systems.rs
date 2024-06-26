use std::time::Duration;

use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::PrimaryWindow,
};
use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::prelude::*;

use super::{
    components::{Ball, LeftWall, Paddle, RightWall, Side},
    events::ScoreGoal,
    GameAction, LastPong, Score,
};

pub mod ui;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..default()
        },
        BloomSettings::default(),
    ));
}

pub fn accelerate_with_time(
    mut balls: Query<&mut LinearVelocity, With<Ball>>,
    time: Res<Time<Fixed>>,
) {
    for mut ball_velocity in &mut balls {
        ball_velocity.0 *= Vec2::splat(time.delta_seconds()) * 0.05 + Vec2::splat(1.0);
    }
}

pub fn update_last_hit(mut last_pong: ResMut<LastPong>, time: Res<Time<Fixed>>) {
    last_pong.last += time.delta();
}

pub fn respawn_ball_on_lock(
    mut commands: Commands,
    mut last_pong: ResMut<LastPong>,
    balls: Query<Entity, With<Ball>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let lock_maximum = Duration::from_secs_f32(20.);

    if last_pong.last > lock_maximum {
        for ball in balls.iter() {
            info!("Despawning ball");
            commands.entity(ball).despawn();
        }

        last_pong.last = Duration::default();

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(20.)).into(),
                material: materials.add(Color::rgb(7.5, 0.0, 7.5)),
                ..default()
            },
            Ball,
            RigidBody::Dynamic,
            Collider::circle(20.),
            LinearVelocity::from(Vec2::new(100., 33.)),
            Restitution::PERFECTLY_ELASTIC,
        ));
    }
}

pub fn score_goal(
    mut collision_events: EventReader<Collision>,
    left_walls: Query<Entity, With<LeftWall>>,
    right_walls: Query<Entity, With<RightWall>>,
    balls: Query<Entity, With<Ball>>,
    mut score_event: EventWriter<ScoreGoal>,
) {
    for Collision(contact) in collision_events.read() {
        if balls.get(contact.entity1).is_err() && balls.get(contact.entity2).is_err() {
            debug!("No ball in collision event");
            continue;
        }

        if left_walls
            .get(contact.entity1)
            .or_else(|_| left_walls.get(contact.entity2))
            .is_ok()
        {
            score_event.send(ScoreGoal {
                goal_for: Side::Right,
            });

            continue;
        }

        if right_walls
            .get(contact.entity1)
            .or_else(|_| right_walls.get(contact.entity2))
            .is_ok()
        {
            score_event.send(ScoreGoal {
                goal_for: Side::Left,
            });

            continue;
        }
    }
}

pub fn scoring(mut score_event: EventReader<ScoreGoal>, mut score: ResMut<Score>) {
    for ScoreGoal { goal_for } in score_event.read() {
        match goal_for {
            Side::Left => {
                info!("Right scored");
                score.right += 1;
            }
            Side::Right => {
                info!("Left scored");
                score.left += 1;
            }
        }
    }
}

pub fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window.single();

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(20.)).into(),
            material: materials.add(Color::rgb(7.5, 0.0, 7.5)),
            ..default()
        },
        Ball,
        RigidBody::Dynamic,
        Collider::circle(20.),
        LinearVelocity::from(Vec2::new(100., 33.)),
        Restitution::PERFECTLY_ELASTIC,
    ));

    // Walls
    let square_sprite = Sprite {
        color: Color::rgb(0.7, 0.7, 0.8),
        custom_size: Some(Vec2::splat(50.0)),
        ..default()
    };

    let window_width = window.width();
    let window_height = window.height();

    let half_width = window_width / 2.;
    let half_height = window_height / 2.;

    commands.spawn((
        SpriteBundle {
            sprite: square_sprite.clone(),
            transform: Transform::from_xyz(0.0, half_height, 0.0).with_scale(Vec3::new(
                window_width,
                1.0,
                1.0,
            )),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(window_width, 50.0),
        Restitution::PERFECTLY_ELASTIC,
    ));

    // Floor
    commands.spawn((
        SpriteBundle {
            sprite: square_sprite.clone(),
            transform: Transform::from_xyz(0.0, -half_height, 0.0).with_scale(Vec3::new(
                window_width,
                1.0,
                1.0,
            )),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(window_width, 50.0),
        Restitution::PERFECTLY_ELASTIC,
    ));

    // Left wall
    commands.spawn((
        SpriteBundle {
            sprite: square_sprite.clone(),
            transform: Transform::from_xyz(half_width, half_height, 0.0).with_scale(Vec3::new(
                1.0,
                window.width(),
                1.0,
            )),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(50.0, window_height),
        Restitution::PERFECTLY_ELASTIC,
        LeftWall,
    ));

    // Right wall
    commands.spawn((
        SpriteBundle {
            sprite: square_sprite,
            transform: Transform::from_xyz(-half_width, half_height, 0.0).with_scale(Vec3::new(
                1.0,
                window.width(),
                1.0,
            )),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(50.0, window_height),
        Restitution::PERFECTLY_ELASTIC,
        RightWall,
    ));

    let paddle = Paddle {
        width: 20.,
        height: 210.,
    };

    // Paddles

    let right_input_map = InputMap::new([
        (GameAction::PaddleUp, KeyCode::KeyY),
        (GameAction::PaddleDown, KeyCode::KeyI),
    ]);

    let left_input_map = InputMap::new([
        (GameAction::PaddleUp, KeyCode::KeyZ),
        (GameAction::PaddleDown, KeyCode::KeyR),
    ]);

    commands.spawn((
        MaterialMesh2dBundle {
            transform: Transform::from_translation(Vec3::new(-half_width + 80., 0., 0.)),
            mesh: meshes
                .add(Rectangle::new(paddle.width, paddle.height))
                .into(),
            material: materials.add(Color::rgb(0.0, 7.5, 7.5)),
            ..default()
        },
        RigidBody::Kinematic,
        Collider::rectangle(paddle.width, paddle.height),
        Restitution::PERFECTLY_ELASTIC,
        paddle,
        InputManagerBundle::with_map(left_input_map),
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            transform: Transform::from_translation(Vec3::new(half_width - 80., 0., 0.)),
            mesh: meshes
                .add(Rectangle::new(paddle.width, paddle.height))
                .into(),
            material: materials.add(Color::rgb(7.5, 7.5, 0.0)),
            ..default()
        },
        RigidBody::Kinematic,
        Collider::rectangle(paddle.width, paddle.height),
        Restitution::PERFECTLY_ELASTIC,
        paddle,
        InputManagerBundle::with_map(right_input_map),
    ));
}

pub fn rebound(
    mut collision_events: EventReader<Collision>,
    mut balls: Query<(&Transform, &mut LinearVelocity), With<Ball>>,
    mut last_pong: ResMut<LastPong>,
    paddles: Query<(Entity, &Transform), With<Paddle>>,
) {
    for Collision(contact) in collision_events.read() {
        let Ok((paddle_entity, paddle_transform)) = paddles
            .get(contact.entity1)
            .or_else(|_| paddles.get(contact.entity2))
        else {
            continue;
        };

        let ball_entity = if paddle_entity == contact.entity1 {
            contact.entity2
        } else {
            contact.entity1
        };

        let Ok((ball_transform, mut velocity)) = balls.get_mut(ball_entity) else {
            continue;
        };

        let paddle_to_ball_direction = ball_transform.translation - paddle_transform.translation;

        last_pong.last = Duration::default();
        velocity.0 += paddle_to_ball_direction.normalize().xy() * 100.0;
    }
}

pub fn move_paddles(
    mut paddles: Query<(&ActionState<GameAction>, &mut LinearVelocity), With<Paddle>>,
) {
    let speed = 500.;

    for (action_state, mut velocity) in paddles.iter_mut() {
        velocity.0 = Vec2::ZERO;

        if action_state.pressed(&GameAction::PaddleUp) {
            velocity.0 += Vec2::Y * speed;
        }
        if action_state.pressed(&GameAction::PaddleDown) {
            velocity.0 -= Vec2::Y * speed;
        }
    }
}
