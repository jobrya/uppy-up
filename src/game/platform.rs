use bevy::prelude::*;

use crate::game::{Direction, Game, Location};
use crate::game::{Y_INC, X_INC, PLATFORM_Z};

const RIGHT_BOUND: f32 = crate::WINDOW_X / 2. - 50.;
const LEFT_BOUND: f32 = crate::WINDOW_X / 2. * -1. + 50.;

#[derive(Component, Default)]
pub struct Platform {
    location: Location,
}

pub fn init_platforms(commands: &mut Commands,
    asset_server: &mut Res<AssetServer>,
    game: &mut ResMut<Game>
)
{
    for _i in 0..20 {
        increment_platform(commands, &asset_server, game);  
    }
}

pub fn increment_platform(commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    game: &mut ResMut<Game>,
)
{
    let mut dir = gen_rand_dir();
    out_of_bounds(&game.top_platform_loc, &mut dir);
    increment_loc(&mut game.top_platform_loc, &dir);
    game.correct_path.push(dir);
    commands.spawn(SpriteBundle {
        texture: asset_server.load("cloud.png"),
        transform: Transform::from_xyz(game.top_platform_loc.x, game.top_platform_loc.y, PLATFORM_Z),
        ..default()
    });
}

fn increment_loc(loc: &mut Location, dir: &Direction) {
    match dir {
        Direction::Left => {loc.x -= X_INC;},
        Direction::Right => {loc.x += X_INC;},
    }
    loc.y += Y_INC;
}

fn gen_rand_dir() -> Direction {
    if rand::random() {Direction::Right}
    else {Direction::Left}
}

fn out_of_bounds(loc: &Location, dir: &mut Direction) {
    if loc.x + X_INC > RIGHT_BOUND {
        *dir = Direction::Left;
    }
    if loc.x - X_INC < LEFT_BOUND {
        *dir = Direction::Right;
    }
}
