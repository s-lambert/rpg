use bevy::prelude::*;

use crate::{GameState, TILE_SIZE};

pub struct BattlePlugin;

fn place_characters(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        transform: Transform {
            scale: Vec3::new(0.25, 0.25, 1.0),
            ..default()
        },
        ..default()
    });

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            flip_x: true,
            ..default()
        },
        texture: asset_server.load("player.png"),
        transform: Transform::from_translation(Vec3::new(-TILE_SIZE, 0.0, 0.0)),
        ..default()
    });

    commands.spawn(SpriteBundle {
        texture: asset_server.load("goblin.png"),
        transform: Transform::from_translation(Vec3::new(TILE_SIZE, 0.0, 0.0)),
        ..default()
    });
}

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(place_characters.in_schedule(OnEnter(GameState::Battle)));
    }
}
