use bevy::prelude::*;

#[derive(Component)]
pub struct Score {
    pub text: String,
    pub value: i32,
}

#[derive(Component)]
pub struct ScoreEntity;

#[derive(Component)]
pub struct HighScoreEntity;

impl Score {
    pub fn to_string(&self) -> String {
        self.text.clone() + &self.value.to_string()
    }

    pub fn increment(&mut self) {
        self.value += 1;
    }

    pub fn init_high_score(&mut self) {
        self.text = String::from("High Score: ");
    }
}

impl Default for Score {
    fn default() -> Self {
        Score {
            text: String::from("Score: "),
            value: 0,
        }
    }
}
