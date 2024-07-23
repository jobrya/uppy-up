use bevy::prelude::*;
use crate::game::Game;

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

pub fn load_scores(commands: &mut Commands,
    asset_server: &mut Res<AssetServer>,
    game: &mut ResMut<Game>,
)
{
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            align_content: AlignContent::Start,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Start,
            padding: UiRect::all(Val::Percent(1.)),
            ..default()
        },
        ..default()
        }
    )
    .with_children(|parent|{
        parent.spawn((TextBundle::from_section(
            game.high_score.to_string()
            , TextStyle { 
                font: asset_server.load("FiraSans-Regular.ttf"),
                font_size: 40.,
                color: Color::WHITE,
            }
        ), HighScoreEntity));
    })
    .with_children(|parent|{
        parent.spawn((TextBundle::from_section(
            game.score.to_string()
            , TextStyle { 
                font: asset_server.load("FiraSans-Regular.ttf"),
                font_size: 40.,
                color: Color::WHITE,
            }
        ), ScoreEntity));
    });
}


pub fn update_score(mut score_query: Query<&mut Text, (With<ScoreEntity>, Without<HighScoreEntity>)>,
    mut high_score_query: Query<&mut Text, (With<HighScoreEntity>, Without<ScoreEntity>)>,
    game: Res<Game>, 
) 
{
    for mut score in &mut score_query {
        score.sections[0].value =  game.score.to_string();
    }
    for mut high_score in &mut high_score_query {
        high_score.sections[0].value =  game.high_score.to_string();
    }
}