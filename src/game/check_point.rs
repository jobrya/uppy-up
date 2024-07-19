use bevy::prelude::*;
use crate::game::{Location, AnimationIndices, AnimationTimer};
use crate::game::PLATFORM_Z;

const CHECK_POINT_SIZE: UVec2 = UVec2::splat(32);

#[derive(Component, Default)]
pub struct CheckPoint {
    timer: Timer,
}


pub fn add_checkpoint (
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    loc: &Location,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let atlas_layout = texture_atlases.add(TextureAtlasLayout::from_grid(CHECK_POINT_SIZE, 2, 3, None, None));
    let animation_indices = AnimationIndices { first: 0, last: 5 };

    commands.spawn((
        SpriteBundle {
            // texture: asset_server.load("player_sheet.png"),
            texture: asset_server.load("hour_glass_v2.png"),
            transform: Transform::from_xyz(
                loc.x,
                loc.y,
                1.5,
            ),
            ..default()
        },
        TextureAtlas {
            layout: atlas_layout,
            index: animation_indices.first,
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
    ));
}
