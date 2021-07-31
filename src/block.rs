use bevy::{core::FixedTimestep, ecs::system::Command, math::const_m128, prelude::*};
use rand::{thread_rng, Rng};

use crate::{Block, BlockNumber, Materials, WindowSize, BLOCK_NUMBER};

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system(block_spawn.system());
    }
}

fn block_spawn(
    mut commands: Commands,
    win_size: Res<WindowSize>,
    mateirals: Res<Materials>,
    mut block_number: ResMut<BlockNumber>,
) {
    if (block_number.0 < BLOCK_NUMBER) {
        let mut rng = thread_rng();
        let w_span = win_size.width / 2. - 100.;
        let h_span = win_size.height / 2. - 100.;
        let x = rng.gen_range(-w_span..w_span) as f32;
        let y = rng.gen_range(-h_span..h_span) as f32;

        commands
            .spawn_bundle(SpriteBundle {
                material: mateirals.block.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 10.),
                    scale: Vec3::new(0.075, 0.075, 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Block);
        block_number.0 += 1;
    }
}
