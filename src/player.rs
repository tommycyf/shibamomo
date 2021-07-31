use bevy::{core::FixedTimestep, ecs::system::Command, prelude::*, transform};

use crate::{
    Block, FromPlayer, Laser, Materials, Player, PlayerReadyFire, PlayerState, Speed, WindowSize,
    GRAVITY_ACC, GROUND_HEIGHT, PLAYER_RESPAWM_DELAY, TIME_STEP,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(PlayerState::default())
            .add_startup_stage(
                "game_setup_stage",
                SystemStage::single(player_spawn.system()),
            )
            .add_system(player_movement.system())
            // .add_system(player_fire.system())
            // .add_system(laser_movement.system())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.5))
                    .with_system(player_spawn.system()),
            );
    }
}

fn player_spawn(
    mut commands: Commands,
    materials: Res<Materials>,
    windowSize: Res<WindowSize>,
    time: Res<Time>,
    mut player_state: ResMut<PlayerState>,
) {
    let now = time.seconds_since_startup();
    if !player_state.on
        && (player_state.last_shot == 0. || now > player_state.last_shot + PLAYER_RESPAWM_DELAY)
    {
        let bottom = windowSize.height / 2. - 100.;
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.player.clone(),
                transform: Transform {
                    translation: Vec3::new(0., bottom + 100., 10.),
                    scale: Vec3::new(0.15, 0.15, 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Player)
            .insert(PlayerReadyFire(true))
            .insert(Speed::default());

        player_state.spawned();
    }
}

fn player_movement(
    keyboardInput: Res<Input<KeyCode>>,
    win_size: Res<WindowSize>,
    mut query: Query<(&mut Speed, &mut Transform, With<Player>)>,
) {
    let ground_y = -win_size.height + GROUND_HEIGHT;
    if let Ok((mut speed, mut transform, _)) = query.single_mut() {
        // x-dir
        let x_direction = match &keyboardInput {
            x if x.pressed(KeyCode::Left) => -1.,
            x if x.pressed(KeyCode::Right) => 1.,
            _ => 0.,
        };

        if keyboardInput.just_pressed(KeyCode::Space) && speed.1 == 0. {
            speed.accelarate(Vec2::new(0., 1000.));
        }

        if transform.translation.y > ground_y {
            speed.accelarate(Vec2::new(0., -GRAVITY_ACC));
        }

        if (transform.translation.y + speed.1 * TIME_STEP) < ground_y {
            transform.translation.y = ground_y;
            speed.reset_y();
        } else {
            transform.translation.y += speed.1 * TIME_STEP;
        }

        if (x_direction != 0.) {
            transform.translation.x += x_direction * speed.0 * TIME_STEP;
        }
    }
    // if let Ok((speed, mut transform, _)) = query.single_mut() {
    //     let x_direction = if keyboardInput.pressed(KeyCode::Left) {
    //         -1.
    //     } else if keyboardInput.pressed(KeyCode::Right) {
    //         1.
    //     } else {
    //         0.
    //     };
    // let y_direction = if keyboardInput.pressed(KeyCode::Down) {
    //     -1.
    // } else if keyboardInput.pressed(KeyCode::Up) {
    //     1.
    // } else {
    //     0.
    // };
    // transform.translation.x += x_direction * speed.0 * TIME_STEP;
    // transform.translation.y += y_direction * speed.0 * TIME_STEP;
    // }
}

fn player_fire(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    materials: Res<Materials>,
    mut query: Query<(&Transform, &mut PlayerReadyFire, With<Player>)>,
) {
    if let Ok((player_transform, mut player_ready_fire, _)) = query.single_mut() {
        if player_ready_fire.0 && keyboard_input.pressed(KeyCode::Space) {
            let x = player_transform.translation.x;
            let y = player_transform.translation.y;

            let mut spawn_lasers = |x_offset: f32| {
                commands
                    .spawn_bundle(SpriteBundle {
                        material: materials.player_laser.clone(),
                        transform: Transform {
                            translation: Vec3::new(x + x_offset, y, 0.),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Laser)
                    .insert(FromPlayer)
                    .insert(Speed::default());
                player_ready_fire.0 = false;
            };
            let x_offset = 20.;
            spawn_lasers(x_offset);
            spawn_lasers(-x_offset);
        }
        if keyboard_input.just_released(KeyCode::Space) {
            player_ready_fire.0 = true;
        }
    }
}

fn laser_movement(
    mut commands: Commands,
    win_size: Res<WindowSize>,
    mut query: Query<(Entity, &Speed, &mut Transform), (With<Laser>, With<FromPlayer>)>,
) {
    for (laser_entity, speed, mut laser_transform) in query.iter_mut() {
        let translation = &mut laser_transform.translation;
        translation.y += speed.0 * TIME_STEP;
        if (translation.y > win_size.height) {
            commands.entity(laser_entity).despawn();
        }
    }
}
