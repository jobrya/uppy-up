use bevy::prelude::*;
use crate::game::Direction;
//use crate::game::Game;
use crate::game::{Y_INC, X_INC, Location};

#[derive(Component, Default)]
pub struct Player {
    pub entity: Option<Entity>,
    pub location: Location,
    pub direction: Direction,
}


impl Player {

    // pub fn print_postiion(&mut self) {
    //     println!("({},{})", self.location.x, self.location.y);
    // }

    // pub fn flip_player(&mut self,
    //     mut sprite: Query<&mut Sprite>,
    //     game:&mut ResMut<Game>,
    // ) 
    // {
    //     // move the background
    //     *sprite.get_mut(game.player.entity.unwrap()).unwrap() = Sprite {
    //         flip_x: true,
    //         ..default()
    //     };
    // }

    pub fn advance(&mut self, dir: Direction) {
        match dir {
            Direction::Left => 
            {
                self.direction = Direction::Left;
                self.location.x -= X_INC;
                self.location.y += Y_INC;
            },
            Direction::Right =>
            {
                self.direction = Direction::Right;
                self.location.x += X_INC;
                self.location.y += Y_INC;
            },
        }
    }
}