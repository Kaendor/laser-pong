use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::PrimaryWindow,
};

use self::events::Bounce;

mod events;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, spawn_ball))
            .add_systems(Update, (screen_rebound, collide_ball_paddle))
            .add_systems(FixedUpdate, move_object)
            .add_event::<Bounce>();
    }
}

fn spawn_camera(mut commands: Commands) {
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

#[derive(Component, Clone, Copy)]
struct Ball;

#[derive(Component, Copy, Clone)]
struct Paddle {
    width: f32,
    height: f32,
}

#[derive(Component, Clone)]
struct Velocity(Vec2);

fn spawn_ball(
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
        Velocity(Vec2::new(4., 2.)),
    ));

    let half_width = window.width() / 2.;

    info!("Half width: {}", half_width);

    let paddle = Paddle {
        width: 20.,
        height: 320.,
    };

    commands.spawn((
        MaterialMesh2dBundle {
            transform: Transform::from_translation(Vec3::new(-half_width + 80., 0., 0.)),
            mesh: meshes
                .add(Rectangle::new(paddle.width, paddle.height))
                .into(),
            material: materials.add(Color::rgb(0.0, 7.5, 7.5)),
            ..default()
        },
        paddle,
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
        paddle,
    ));
}

fn move_object(mut balls: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in balls.iter_mut() {
        transform.translation += velocity.0.extend(0.);
    }
}

fn screen_rebound(
    mut balls: Query<(&Transform, &mut Velocity), With<Ball>>,
    mut bounce_events: EventWriter<Bounce>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window.single();

    let height = window.height();
    let width = window.width();

    for (transform, mut velocity) in balls.iter_mut() {
        if transform.translation.x > width / 2. || transform.translation.x < -width / 2. {
            bounce_events.send(Bounce {
                position: transform.translation,
            });

            velocity.0.x = -velocity.0.x;
        }
        if transform.translation.y > height / 2. || transform.translation.y < -height / 2. {
            bounce_events.send(Bounce {
                position: transform.translation,
            });
            velocity.0.y = -velocity.0.y;
        }
    }
}

fn collide_ball_paddle(
    paddles: Query<(&Transform, &Paddle)>,
    mut balls: Query<(&Transform, &mut Velocity), With<Ball>>,
    mut bounce_events: EventWriter<Bounce>,
) {
    for (ball_transform, mut velocity) in &mut balls {
        for (paddle_tranrform, baddle) in &paddles {
            let ball = ball_transform.translation;
            let paddle = paddle_tranrform.translation;

            let ball_radius = 20.;
            let paddle_width = baddle.width;
            let paddle_height = baddle.height;

            let paddle_x = paddle.x - paddle_width / 2.;
            let paddle_y = paddle.y - paddle_height / 2.;

            let paddle_x_max = paddle_x + paddle_width;
            let paddle_y_max = paddle_y + paddle_height;

            if ball.x + ball_radius > paddle_x
                && ball.x - ball_radius < paddle_x_max
                && ball.y + ball_radius > paddle_y
                && ball.y - ball_radius < paddle_y_max
            {
                bounce_events.send(Bounce {
                    position: ball_transform.translation,
                });

                velocity.0.x = -velocity.0.x;
            }
        }
    }
}
