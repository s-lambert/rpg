use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer, utils::HashMap};

use crate::{GameState, Position};

pub struct BattlePlugin;

#[derive(Resource, Deref, DerefMut)]
struct Enemies(HashMap<Position, Entity>);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

fn place_characters(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.transform.scale = Vec3::new(1.0 / 3.0, 1.0 / 3.0, 1.0);
    commands.spawn(camera);

    commands.spawn((
        Player,
        Position {
            x: -2,
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

    let enemy_position = Position {
        x: 2,
        y: 0,
        layer: 1,
    };
    let enemy_id = commands
        .spawn((
            Enemy,
            enemy_position,
            SpriteBundle {
                texture: asset_server.load("goblin.png"),
                ..default()
            },
        ))
        .id();

    let mut enemies = Enemies(HashMap::default());
    enemies.insert(enemy_position, enemy_id);
    commands.insert_resource(enemies);
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut enemies: ResMut<Enemies>,
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
    let move_position = Position {
        x: player.x + move_x,
        y: player.y + move_y,
        layer: player.layer,
    };

    if let Some(_enemy_id) = enemies.get(&move_position) {
        println!("attack enemy");
    } else {
        *player = move_position;
    }

    for mut enemy in enemy_query.iter_mut() {
        let mut enemy_move_position = enemy.clone();
        let horr_distance = (player.x - enemy.x).abs();
        let ver_distance = (player.y - enemy.y).abs();

        if ver_distance >= horr_distance {
            enemy_move_position.y += (player.y - enemy.y).cmp(&0) as i32;
        } else {
            enemy_move_position.x += (player.x - enemy.x).cmp(&0) as i32;
        }

        if enemy_move_position == *player {
            println!("attack player");
        } else {
            let Some((_, enemy_id)) = enemies.remove_entry(&enemy) else {
                dbg!("enemy not in resource");
                return;
            };
            *enemy = enemy_move_position;
            enemies.insert(*enemy, enemy_id);
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
        app.insert_resource(Enemies(HashMap::default()))
            .add_system(place_characters.in_schedule(OnEnter(GameState::Battle)))
            .add_systems(
                (
                    move_player.run_if(on_timer(Duration::from_millis(200))),
                    position_to_translation.after(move_player),
                )
                    .in_set(OnUpdate(GameState::Battle)),
            );
    }
}
