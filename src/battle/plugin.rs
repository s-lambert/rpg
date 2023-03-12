use bevy::prelude::*;

use crate::{GameState, Position};

pub struct BattlePlugin;

fn place_characters(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        transform: Transform {
            scale: Vec3::new(0.25, 0.25, 1.0),
            ..default()
        },
        ..default()
    });

    commands.spawn((
        Position {
            x: -1,
            y: 0,
            layer: 0,
        },
        SpriteBundle {
            sprite: Sprite {
                flip_x: true,
                ..default()
            },
            texture: asset_server.load("player.png"),
            ..default()
        },
    ));

    commands.spawn((
        Position {
            x: 1,
            y: 0,
            layer: 0,
        },
        SpriteBundle {
            texture: asset_server.load("goblin.png"),
            ..default()
        },
    ));
}

fn position_to_translation(
    mut changed_positions_query: Query<(&mut Transform, &Position), Changed<Position>>,
) {
    for (mut transform, position) in changed_positions_query.iter_mut() {
        transform.translation = position.to_translation();
        dbg!(transform.translation);
    }
}

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(place_characters.in_schedule(OnEnter(GameState::Battle)))
            .add_system(position_to_translation.in_set(OnUpdate(GameState::Battle)));
    }
}
