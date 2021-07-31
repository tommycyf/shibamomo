#[allow(unused)]
mod block;
mod enemy;
mod player;

use std::collections::HashSet;

use bevy::{prelude::*, sprite::collide_aabb::collide, transform};

use crate::{block::BlockPlugin, enemy::EnemyPlugin, player::PlayerPlugin};

const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";
const EXPLOSION_SHEET: &str = "explo_a_sheet.png";
const ENEMY_SPRITE: &str = "enemy_a_02.png";
const ENEMY_LASER_SPRITE: &str = "laser_b_01.png";
const BLOCK_SPRITE: &str = "block.png";

const TIME_STEP: f32 = 1. / 60.;
const SCALE: f32 = 0.5;
const PLAYER_RESPAWM_DELAY: f64 = 2.;
const GRAVITY_ACC: f32 = 30.;
const GROUND_HEIGHT: f32 = 500.;
const BLOCK_NUMBER: u32 = 10;
// region : Resources
pub struct Materials {
    player: Handle<ColorMaterial>,
    player_laser: Handle<ColorMaterial>,
    enemy: Handle<ColorMaterial>,
    enemy_laser: Handle<ColorMaterial>,
    explosion: Handle<TextureAtlas>,
    block: Handle<ColorMaterial>,
}
struct WindowSize {
    width: f32,
    height: f32,
}
struct PlayerState {
    on: bool,
    last_shot: f64,
}

struct BlockNumber(u32);

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            on: false,
            last_shot: 0.,
        }
    }
}

impl PlayerState {
    fn shot(&mut self, time: f64) {
        self.on = false;
        self.last_shot = time;
    }
    fn spawned(&mut self) {
        self.on = true;
        self.last_shot = 0.
    }
}

struct ActiveEnemies(u32);
//end region : Resources

//region : Components
struct Player;
struct PlayerReadyFire(bool);

struct Laser;
struct FromPlayer;

struct FromEnemy;
struct Enemy;

struct Explosion;
struct ExplosionToSpawn(Vec3);

struct Block;
struct Speed(f32, f32);
impl Default for Speed {
    fn default() -> Self {
        Self(500., 0.)
    }
}
impl Speed {
    fn from_speed(speed: Vec2) -> Self {
        Self(speed.x, speed.y)
    }
    fn reset_x(&mut self) {
        self.0 = 0.;
    }
    fn reset_y(&mut self) {
        self.1 = 0.;
    }
    fn accelarate(&mut self, acc: Vec2) {
        self.0 += acc.x;
        self.1 += acc.y;
    }
}
//end region : Components∏
fn main() {
    println!("Hello, world!");
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(WindowDescriptor {
            title: "test game".to_string(),
            width: 1000.0,
            height: 1000.0,
            ..Default::default()
        })
        .insert_resource(ActiveEnemies(0))
        .insert_resource(BlockNumber(0))
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(BlockPlugin)
        .add_startup_system(setup.system())
        .add_system(player_hit_enemy.system())
        .add_system(player_block_collide.system())
        // .add_system(player_laser_hit_enemy.system())
        // .add_system(enemy_laser_hit_player.system())
        .add_system(explosion_to_spawn.system())
        .add_system(animate_explosion.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut windows: ResMut<Windows>,
) {
    let window = windows.get_primary_mut().unwrap();

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let texture_handle = asset_server.load(EXPLOSION_SHEET);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64., 64.), 4, 4);

    commands.insert_resource(Materials {
        player: materials.add(asset_server.load(PLAYER_SPRITE).into()),
        player_laser: materials.add(asset_server.load(PLAYER_LASER_SPRITE).into()),
        enemy: materials.add(asset_server.load(ENEMY_SPRITE).into()),
        enemy_laser: materials.add(asset_server.load(ENEMY_LASER_SPRITE).into()),
        explosion: texture_atlases.add(texture_atlas),
        block: materials.add(asset_server.load(BLOCK_SPRITE).into()),
    });
    commands.insert_resource(WindowSize {
        width: window.width(),
        height: window.height(),
    });
    window.set_position(IVec2::new(0, 0));
}

fn player_hit_enemy(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform, &Sprite, With<Player>)>,
    mut enemy_query: Query<(Entity, &Transform, &Sprite, With<Enemy>)>,
    mut active_enemies: ResMut<ActiveEnemies>,
) {
    let mut enemies_blasted: HashSet<Entity> = HashSet::new();
    if let Ok((player_entity, player_tf, player_sprite, _)) = player_query.single_mut() {
        for (enemy_entity, enemy_tf, enemy_sprite, _) in enemy_query.iter_mut() {
            let player_scale = Vec2::from(player_tf.scale);
            let enemy_scale = Vec2::from(enemy_tf.scale);
            let collision = collide(
                player_tf.translation,
                player_sprite.size * player_scale,
                enemy_tf.translation,
                enemy_sprite.size * enemy_scale,
            );

            if let Some(_) = collision {
                if enemies_blasted.get(&enemy_entity).is_none() {
                    commands.entity(enemy_entity).despawn();
                    active_enemies.0 -= 1;
                    // commands.entity(player_entity).despawn();
                    commands
                        .spawn()
                        .insert(ExplosionToSpawn(enemy_tf.translation.clone()));
                    enemies_blasted.insert(enemy_entity);
                }
            }
        }
    }
}

// fn player_laser_hit_enemy(
//     mut commands: Commands,
//     mut laser_query: Query<(Entity, &Transform, &Sprite, (With<Laser>, With<FromPlayer>))>,
//     mut enemy_query: Query<(Entity, &Transform, &Sprite, With<Enemy>)>,
//     mut active_enemies: ResMut<ActiveEnemies>,
// ) {
//     let mut enemies_blasted: HashSet<Entity> = HashSet::new();
//     for (laser_entity, laser_tf, laser_sprite, _) in laser_query.iter_mut() {
//         for (enemy_entity, enemy_tf, enemy_sprite, _) in enemy_query.iter_mut() {
//             let laser_scale = Vec2::from(laser_tf.scale);
//             let enemy_scale = Vec2::from(enemy_tf.scale);
//             let collision = collide(
//                 laser_tf.translation,
//                 laser_sprite.size * laser_scale,
//                 enemy_tf.translation,
//                 enemy_sprite.size * enemy_scale,
//             );

//             if let Some(_) = collision {
//                 if enemies_blasted.get(&enemy_entity).is_none() {
//                     commands.entity(enemy_entity).despawn();
//                     active_enemies.0 -= 1;
//                     commands.entity(laser_entity).despawn();

//                     commands
//                         .spawn()
//                         .insert(ExplosionToSpawn(enemy_tf.translation.clone()));
//                     enemies_blasted.insert(enemy_entity);
//                 }
//             }
//         }
//     }
// }

fn explosion_to_spawn(
    mut commands: Commands,
    query: Query<(Entity, &ExplosionToSpawn)>,
    materials: Res<Materials>,
) {
    for (explosion_spawn_entity, explosion_to_spawn) in query.iter() {
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: materials.explosion.clone(),
                transform: Transform {
                    translation: explosion_to_spawn.0,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Explosion)
            .insert(Timer::from_seconds(0.05, true));

        commands.entity(explosion_spawn_entity).despawn();
    }
}

fn animate_explosion(
    mut commands: Commands,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<
        (
            Entity,
            &mut Timer,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
        ),
        With<Explosion>,
    >,
) {
    for (entity, mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index += 1;
            if sprite.index == texture_atlas.textures.len() as u32 {
                commands.entity(entity).despawn()
            }
        }
    }
}

fn enemy_laser_hit_player(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    time: Res<Time>,
    laser_query: Query<(Entity, &Transform, &Sprite), (With<Laser>, With<FromEnemy>)>,
    player_query: Query<(Entity, &Transform, &Sprite), With<Player>>,
) {
    if let Ok((player_entity, player_tf, player_sprite)) = player_query.single() {
        let player_size = player_sprite.size * Vec2::from(player_tf.scale.abs());
        for (laser_entity, laser_tf, laser_sprite) in laser_query.iter() {
            let laser_size = laser_sprite.size * Vec2::from(laser_tf.scale.abs());
            let collision = collide(
                laser_tf.translation,
                laser_size,
                player_tf.translation,
                player_size,
            );

            if let Some(_) = collision {
                commands.entity(player_entity).despawn();
                player_state.shot(time.seconds_since_startup());
                commands.entity(laser_entity).despawn();
                commands
                    .spawn()
                    .insert(ExplosionToSpawn(player_tf.translation.clone()));
            }
        }
    }
}

fn player_block_collide(
    mut commands: Commands,
    mut player_query: Query<(
        &mut Speed,
        &mut Transform,
        &Sprite,
        With<Player>,
        Without<Block>,
    )>,
    mut player_state: ResMut<PlayerState>,
    time: Res<Time>,
    mut block_query: Query<(Entity, &Transform, &Sprite, With<Block>, Without<Player>)>,
) {
    if let Ok((mut player_speed, mut player_tf, player_sprite, _, _)) = player_query.single_mut() {
        let player_size = player_sprite.size * Vec2::from(player_tf.scale.abs());
        for (block_entity, block_tf, block_sprite, _, _) in block_query.iter_mut() {
            let block_size = block_sprite.size * Vec2::from(block_tf.scale.abs());
            let collision = collide(
                block_tf.translation,
                block_size,
                player_tf.translation,
                player_size,
            );

            if let Some(_) = collision {
                if (player_tf.translation.y > block_tf.translation.y) {
                    player_tf.translation.y =
                        block_tf.translation.y + block_size.y / 2. + player_size.y / 2.;
                    player_speed.reset_y();
                }

                // commands.entity(block_entity).despawn();
                // commands
                //     .spawn()
                //     .insert(ExplosionToSpawn(player_tf.translation.clone()));
            }
        }
    }
}
