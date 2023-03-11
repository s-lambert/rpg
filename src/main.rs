mod battle;

use battle::*;
use bevy::prelude::*;

static TILE_SIZE: f32 = 16.0;

#[derive(States, Default, Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum GameState {
    #[default]
    Battle,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "RPG!".to_string(),
                        resolution: (32.0 * TILE_SIZE, 24.0 * TILE_SIZE).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_system(bevy::window::close_on_esc)
        .add_state::<GameState>()
        .add_plugin(BattlePlugin)
        .run();
}
