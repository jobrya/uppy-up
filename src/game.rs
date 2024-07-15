use bevy::prelude::*;

use super::GameState;

mod player;
use player::Player;

mod platform;
mod score;
use score::Score;

#[derive(Component)]
struct ScoreEntity;


#[derive(Clone)]
pub struct Location {
    pub x: f32,
    pub y: f32,
}

impl Default for Location {
    fn default() -> Self {
        Location {
            x: START_X,
            y: START_Y,
        }
    }
}

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Resource, Default)]

pub struct Game {
    player: Player,
    camera: Option<Entity>,
    pub score: Score,
    top_platform_loc: Location,
    correct_path: Vec<Direction>,
}

impl Game {
    pub fn reset(&mut self) {
        self.player = Player::default();
        self.score = Score::default();
        self.top_platform_loc = Location::default();
        self.correct_path = Vec::new();
    }
}

pub struct GamePlugin;

const START_X: f32 = 0.0;
const START_Y: f32 = -200.;
const X_INC: f32 = 125.;
const Y_INC: f32 = 75.;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App){
        app
            .init_resource::<Game>()
            .add_systems(OnEnter(GameState::Playing), start_game)
            //.add_systems(OnExit(GameState::Playing), reset_game)
            .add_systems(Update, movement.run_if(in_state(GameState::Playing)))
            .add_systems(Update, update_score);
    }
}

fn start_game(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    mut game: ResMut<Game>,
    camera_query: Query<Entity, With<Camera2d>>
) 
{
    // spawn the player
    game.player.entity = Some(commands.spawn(SpriteBundle {
        texture: asset_server.load("dot.png"),
        transform: Transform::from_xyz(START_X, START_Y, 0.0),
        ..default()
    }).id());

    // set score to zero
    //game.score = 0;

    // spawn the score
    load_score(&mut commands, &mut asset_server, &mut game);

    // init platforms
    game.correct_path = platform::init_platforms(commands, asset_server, Location::default(), &mut game);

    // get the camera
    for entity in camera_query.iter() {
        game.camera = Some(entity);
    }
}

// fn reset_game(mut game: ResMut<Game>) {
//     game.reset();
// }

fn movement(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game: ResMut<Game>,
    mut game_state: ResMut<NextState<GameState>>,
    mut transforms: Query<&mut Transform>,
)
{

    if keyboard_input.just_pressed(KeyCode::KeyA) ||
    keyboard_input.just_pressed(KeyCode::ArrowLeft)
    {
        game.player.print_postiion();
        if !correct_movement(&mut game, Direction::Left)
        {
            do_game_over(&mut game_state)
        }
        game.player.advance(Direction::Left);
        platform::increment_platform(&mut commands, &asset_server, &mut game);
        game.score.increment_score();
    }

    if keyboard_input.just_pressed(KeyCode::KeyD) ||
    keyboard_input.just_pressed(KeyCode::ArrowRight)
    {
        game.player.print_postiion();
        game.player.print_postiion();
        if !correct_movement(&mut game, Direction::Right)
        {
            do_game_over(&mut game_state)
        }
        game.player.advance(Direction::Right);
        platform::increment_platform(&mut commands, &asset_server, &mut game);
        game.score.increment_score();
    }

    // move the player
    *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform::from_xyz(
        game.player.location.x,
        game.player.location.y,
        0.0,
    );

    // move the camera
    *transforms.get_mut(game.camera.unwrap()).unwrap() = Transform::from_xyz(
        0.0,
        game.player.location.y - START_Y,
        0.0,
    );

}

fn correct_movement(game: &mut ResMut<Game>, user_dir: Direction) -> bool {
    let correct_dir = game.correct_path.remove(0);
    correct_dir == user_dir
}


fn do_game_over(game_state: &mut ResMut<NextState<GameState>>) {
    game_state.set(GameState::GameOver);
}

fn load_score(commands: &mut Commands,
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
            ..default()
        },
        //background_color: BackgroundColor(PURPLE),
        ..default()
        }
    )
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

fn update_score(mut score_query: Query<&mut Text, With<ScoreEntity>>, game: ResMut<Game>) {
    for mut score in &mut score_query {
        score.sections[0].value =  game.score.to_string();
    }
}