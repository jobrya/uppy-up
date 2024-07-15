use bevy::prelude::*;
use crate::game::Direction;
use crate::game::{Y_INC, X_INC, Location};

#[derive(Component, Default)]
pub struct Player {
    pub entity: Option<Entity>,
    pub location: Location,
}


impl Player {

    // pub fn init_location(&self, mut transforms: Query<&mut Transform>,) {
    //     *transforms.get_mut(self.entity.unwrap()).unwrap() = Transform::from_xyz(
    //         self.location.x,
    //         self.location.y,
    //         0.0,
    //     );
    // }

    pub fn print_postiion(&mut self) {
        println!("({},{})", self.location.x, self.location.y);
    }

    pub fn advance(&mut self, dir: Direction) {
        match dir {
            Direction::Left => 
            {
                self.location.x -= X_INC;
                self.location.y += Y_INC;
            },
            Direction::Right =>
            {
                self.location.x += X_INC;
                self.location.y += Y_INC;
            },
        }
    }
}