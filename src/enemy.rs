use bevy::{core::FixedTimestep, ecs::system::Command, math::const_m128, prelude::*};
use rand::{thread_rng, Rng};

use crate::{
    ActiveEnemies, Enemy, FromEnemy, Laser, Materials, Speed, WindowSize, SCALE, TIME_STEP,
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(enemy_spawn.system()),
        )
        // .add_system_set(
        //     SystemSet::new()
        //         .with_run_criteria(FixedTimestep::step(1.0))
        //         .with_system(enemy_fire.system()),
        // )
        .add_system(enemy_laser_movement.system())
        .add_system(enemy_movement.system());
    }
}

fn enemy_spawn(
    mut commands: Commands,
    mut active_enemies: ResMut<ActiveEnemies>,
    win_size: Res<WindowSize>,
    mateirals: Res<Materials>,
) {
    if active_enemies.0 < 5 {
        let mut rng = thread_rng();
        let w_span = win_size.width / 2. - 100.;
        let h_span = win_size.height / 2. - 100.;
        let x = rng.gen_range(-w_span..w_span) as f32;
        let y = rng.gen_range(-h_span..h_span) as f32;

        commands
            .spawn_bundle(SpriteBundle {
                material: mateirals.enemy.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 10.),
                    scale: Vec3::new(0.075, 0.075, 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Enemy)
            .insert(Speed::from_speed(Vec2::new(0., -100.)));

        active_enemies.0 += 1;
    }
}
fn enemy_fire(
    mut commands: Commands,
    materials: Res<Materials>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    for &tf in enemy_query.iter() {
        let x = tf.translation.x;
        let y = tf.translation.y;

        commands
            .spawn_bundle(SpriteBundle {
                material: materials.enemy_laser.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y - 15., 0.),
                    scale: Vec3::new(SCALE, -SCALE, 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Laser)
            .insert(FromEnemy)
            .insert(Speed::default());
    }
}

fn enemy_laser_movement(
    mut commands: Commands,
    win_size: Res<WindowSize>,
    mut laser_query: Query<(Entity, &Speed, &mut Transform), (With<Laser>, With<FromEnemy>)>,
) {
    for (entity, speed, mut tf) in laser_query.iter_mut() {
        tf.translation.y -= speed.0 * TIME_STEP;
        if tf.translation.y < -win_size.height / 2. - 50. {
            commands.entity(entity).despawn();
        }
    }
}

fn enemy_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &Speed), With<Enemy>>,
    mut active_enemies: ResMut<ActiveEnemies>,
    win_size: Res<WindowSize>,
) {
    let now = time.seconds_since_startup() as f32;

    for (entity, mut tf, speed) in query.iter_mut() {
        tf.translation.y += speed.1 * TIME_STEP;

        if (tf.translation.y < -win_size.height / 2.) {
            commands.entity(entity).despawn();
            active_enemies.0 -= 1;
        }
    }
}
