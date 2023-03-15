use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::{GameState, Position};

pub struct BattlePlugin;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

fn place_characters(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.transform.scale = Vec3::new(0.25, 0.25, 1.0);
    commands.spawn(camera);

    commands.spawn((
        Player,
        Position {
            x: -1,
            y: 0,
            layer: 1,
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
        Enemy,
        Position {
            x: 1,
            y: 0,
            layer: 1,
        },
        SpriteBundle {
            texture: asset_server.load("goblin.png"),
            ..default()
        },
    ));
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Position, With<Player>>,
    mut enemy_query: Query<&mut Position, (Without<Player>, With<Enemy>)>,
) {
    let mut movement: Option<(i32, i32)> = None;
    if keyboard_input.pressed(KeyCode::Up) {
        movement = Some((0, -1));
    } else if keyboard_input.pressed(KeyCode::Down) {
        movement = Some((0, 1));
    } else if keyboard_input.pressed(KeyCode::Left) {
        movement = Some((-1, 0));
    } else if keyboard_input.pressed(KeyCode::Right) {
        movement = Some((1, 0));
    }

    let Some((move_x, move_y)) = movement else { return };
    let mut player = player_query.single_mut();
    player.x += move_x;
    player.y += move_y;

    for mut enemy in enemy_query.iter_mut() {
        let horr_distance = (player.x - enemy.x).abs();
        let ver_distance = (player.y - enemy.y).abs();

        if horr_distance > 1 && horr_distance >= ver_distance {
            enemy.x += (player.x - enemy.x).cmp(&0) as i32;
        } else if ver_distance > 1 {
            enemy.y += (player.y - enemy.y).cmp(&0) as i32;
        }
    }
}

fn position_to_translation(
    mut changed_positions_query: Query<(&mut Transform, &Position), Changed<Position>>,
) {
    for (mut transform, position) in changed_positions_query.iter_mut() {
        transform.translation = position.to_translation();
    }
}

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(place_characters.in_schedule(OnEnter(GameState::Battle)))
            .add_systems(
                (
                    move_player.run_if(on_timer(Duration::from_millis(200))),
                    position_to_translation.after(move_player),
                )
                    .in_set(OnUpdate(GameState::Battle)),
            );
    }
}
