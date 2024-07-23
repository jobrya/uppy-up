// use std::thread::sleep;
// use std::time::Duration;

use bevy::prelude::*;
use crate::game::{Direction, Game, Location};
use crate::game::animation::AnimationConfig;
use crate::game::{Y_INC, X_INC, START_X, START_Y};

use super::animation::{get_rest_animation_config, get_fall_animation_config};

const PLAYER_SIZE: UVec2 = UVec2::splat(64);
pub const PLAYER_OFFSET: f32 = 40.;
const PLAYER_Z: f32 = 2.0;

#[derive(Component, Default)]
pub struct Player {
    pub entity: Option<Entity>,
    pub location: Location,
    pub direction: Direction,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum PlayerAction {
    #[default]
    Rest,
    Jump,
    Fall,
}


impl Player {

    pub fn increment(&mut self) {
        match self.direction {
            Direction::Left => 
            {
                self.location.x -= X_INC;
            },
            Direction::Right =>
            {
                self.location.x += X_INC;
            },
        }
        self.location.y += Y_INC;
    }
}

pub fn spawn_player(texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
    game: &mut ResMut<Game>,
    commands: &mut Commands,
    asset_server: &mut Res<AssetServer>,
) {

    let atlas_layout = texture_atlases.add(TextureAtlasLayout::from_grid(PLAYER_SIZE, 4, 2, None, None));
    let animation_config = get_rest_animation_config();

    game.player.location = Location {
        x: START_X,
        y: START_Y + PLAYER_OFFSET,
    };

    game.player.entity = Some(commands.spawn((
        SpriteBundle {
            texture: asset_server.load("ball_guy.png"),
            transform: Transform::from_xyz(
                game.player.location.x,
                game.player.location.y,
                PLAYER_Z,
            ),
            ..default()
        },
        TextureAtlas {
            layout: atlas_layout,
            index: animation_config.first_sprite_index,
        },
        animation_config,
    )).id());
}

pub fn set_fall_animation(game: Res<Game>,
    mut query: Query<&mut AnimationConfig>,
) {
    *query.get_mut(game.player.entity.unwrap()).unwrap() = get_fall_animation_config();
}
