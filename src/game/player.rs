use std::thread::sleep;
use std::time::Duration;

use bevy::prelude::*;
use crate::game::{Direction, Game, Location};
use crate::game::animation::AnimationConfig;
use crate::game::{Y_INC, X_INC, START_X, START_Y};

use super::animation::{get_jump_animation_config, get_rest_animation_config, get_fall_animation_config};

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

    // pub fn print_postiion(&mut self) {
    //     println!("({},{})", self.location.x, self.location.y);
    // }

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
            // texture: asset_server.load("player_sheet.png"),
            texture: asset_server.load("Ball Guy.png"),
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

pub fn do_rest_animation(mut game: ResMut<Game>,
    mut query: Query<&mut AnimationConfig>,
) {
    *query.get_mut(game.player.entity.unwrap()).unwrap() = get_rest_animation_config();
}

pub fn do_jump_animation(mut game: ResMut<Game>,
    mut query: Query<&mut AnimationConfig>,
) {
    *query.get_mut(game.player.entity.unwrap()).unwrap() = get_jump_animation_config();
}

pub fn set_fall_animation(mut game: ResMut<Game>,
    mut query: Query<&mut AnimationConfig>,
) {
    println!("set fall animation");
    *query.get_mut(game.player.entity.unwrap()).unwrap() = get_fall_animation_config();
}


pub fn fall(game: &mut ResMut<Game>,
    transforms: &mut Query<&mut Transform>,
    time: &Res<Time>,
) {
    let gravity: f32 = -2.;
    let ground_y = START_Y + PLAYER_OFFSET;
    while game.player.location.y > ground_y {
        println!("{}", game.player.location.y);
        game.player.location.y = game.player.location.y * gravity * time.delta_seconds();
        *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform::from_xyz(
            game.player.location.x,
            game.player.location.y,
            PLAYER_Z,
        );        
    }



}