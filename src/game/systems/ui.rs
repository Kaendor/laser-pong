use bevy::prelude::*;

use crate::game::{components::Side, Score};

pub fn display_score(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceAround,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "0",
                    TextStyle {
                        font_size: 24.,
                        color: Color::rgb(0.0, 7.5, 7.5),
                        ..default()
                    },
                ),
                Side::Left,
                Label,
            ));
            parent.spawn((
                TextBundle::from_section(
                    "0",
                    TextStyle {
                        font_size: 24.,
                        color: Color::rgb(7.5, 7.5, 0.0),
                        ..default()
                    },
                ),
                Side::Right,
                Label,
            ));
        });
}

pub fn update_score(score: Res<Score>, mut labels: Query<(&mut Text, &Side)>) {
    if !score.is_changed() {
        return;
    }

    for (mut label, side) in &mut labels {
        let text = match side {
            Side::Left => {
                format!("{}", score.left / 2)
            }
            Side::Right => {
                format!("{}", score.right / 2)
            }
        };

        label.sections[0].value = text;
    }
}
