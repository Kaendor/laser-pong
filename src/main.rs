use bevy::{app::App, DefaultPlugins};
use game::GamePlugin;

mod game;

fn main() {
    App::new().add_plugins((DefaultPlugins, GamePlugin)).run();
}
