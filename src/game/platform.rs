use bevy::prelude::*;

use crate::game::{Direction, Game, Location};

const RIGHT_BOUND: f32 = crate::WINDOW_X / 2. - 50.;
const LEFT_BOUND: f32 = crate::WINDOW_X / 2. * -1. + 50.;
const X_INC: f32 = 125.;
const Y_INC: f32 = 75.;

#[derive(Component, Default)]
pub struct Platform;

pub fn init_platforms(mut commands: Commands,
     asset_server: Res<AssetServer>,
    mut loc: Location,
    game: &mut ResMut<Game>
) -> Vec<Direction> 
{
    let mut correct_path = Vec::new();

    for _i in 0..10 {

        let dir = gen_rand_dir(&loc);
        increment_loc(&mut loc, &dir);
        correct_path.push(dir);

        commands.spawn(SpriteBundle {
            texture: asset_server.load("platform.png"),
            transform: Transform::from_xyz(loc.x, loc.y, 0.0),
            ..default()
        });
        game.top_platform_loc = loc.clone();    
    }

    correct_path
}

pub fn increment_platform(commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    game: &mut ResMut<Game>,
)
{
    let dir = gen_rand_dir(&game.top_platform_loc);
    increment_loc(&mut game.top_platform_loc, &dir);
    game.correct_path.push(dir);
    commands.spawn(SpriteBundle {
        texture: asset_server.load("platform.png"),
        transform: Transform::from_xyz(game.top_platform_loc.x, game.top_platform_loc.y, 0.0),
        ..default()
    });
    println!("platform incremented");
}

fn increment_loc(loc: &mut Location, dir: &Direction) {
    match dir {
        Direction::Left => {loc.x -= X_INC;},
        Direction::Right => {loc.x += X_INC;},
    }
    println!("({},{})",loc.x, loc.y);
    loc.y += Y_INC;
}

fn gen_rand_dir(loc: &Location) -> Direction {
    let mut dir = Direction::Left;
    if loc.x + X_INC > RIGHT_BOUND || loc.x - X_INC < LEFT_BOUND 
    {
        if loc.x + X_INC > RIGHT_BOUND 
        { 
            dir = Direction::Left;
        }
        if loc.x - X_INC < LEFT_BOUND 
        {
            dir = Direction::Right;
        }    
    }
    else {
        if rand::random() 
        {
            dir = Direction::Right;
        }
        else 
        {
            dir = Direction::Left;
        }           
    }

    return dir;
}