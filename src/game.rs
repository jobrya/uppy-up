use bevy::prelude::*;

use super::GameState;

mod player;
use player::Player;
use player::PlayerAction;

mod platform;
use platform::Platform;
mod score;
use score::{Score, ScoreEntity, HighScoreEntity};

mod check_point;
use check_point::CheckPoint;

mod animation;
use animation::AnimationConfig;

const GROUND_OFFSET: f32 = 200.;
// const OFFSET: f32 = 40.;
const START_X: f32 = 0.0;
const START_Y: f32 = -200.;
const X_INC: f32 = 70.;
const Y_INC: f32 = 40.;
const PLAYER_Z: f32 = 2.0;
const PLATFORM_Z: f32 = 1.0;
const CAMERA_Z: f32 = 10.0;

// const PLAYER_SIZE: UVec2 = UVec2 {
//     x: 64,
//     y: 64,
// };

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

// #[derive(Component)]
// struct ScoreEntity;


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

#[derive(PartialEq, Default)]
enum Direction {
    #[default]
    Left,
    Right,
}

#[derive(Resource, Default)]

pub struct Game {
    player: Player,
    camera: Option<Entity>,
    background: Option<Entity>,
    pub score: Score,
    pub high_score: Score,
    top_platform_loc: Location,
    correct_path: Vec<Direction>,
    platforms: Vec<Platform>,
    check_point: CheckPoint,
}

impl Game {
    pub fn reset(&mut self) {
        self.player = Player::default();
        self.score = Score::default();
        self.top_platform_loc = Location::default();
        self.correct_path = Vec::new();
    }

    pub fn set_high_score(&mut self) {
        if self.score.value > self.high_score.value
        {
            self.high_score.value = self.score.value;
        }
    }
}


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App){
        app
            .init_resource::<Game>()
            .init_state::<PlayerAction>()
            .add_systems(OnEnter(GameState::Playing), start_game)
            .add_systems(Update, (
                update_camera,
                update_background,
                update_score,
                //animate_sprites,
                animation::execute_animations,
            )
                .run_if(in_state(GameState::Playing)))
            // .add_systems(OnEnter(PlayerAction::Rest), player::do_rest_animation
            //     .run_if(in_state(GameState::Playing)))
            // .add_systems(OnEnter(PlayerAction::Jump), player::do_jump_animation
            //     .run_if(in_state(GameState::Playing)))
            .add_systems(OnEnter(PlayerAction::Fall), player::set_fall_animation
                    .run_if(in_state(GameState::Playing)))
            .add_systems(Update, handle_rest
                    .run_if(in_state(PlayerAction::Rest))
                    .run_if(in_state(GameState::Playing)))
            .add_systems(Update, handle_jump
                    .run_if(in_state(PlayerAction::Jump))
                    .run_if(in_state(GameState::Playing)))
            .add_systems(Update, handle_fall
                    .run_if(in_state(PlayerAction::Fall))
                    .run_if(in_state(GameState::Playing)));
    }
}

fn start_game(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    mut game: ResMut<Game>,
    camera_query: Query<Entity, With<Camera2d>>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) 
{
    // spawn the background
    game.background = Some(commands.spawn(SpriteBundle {
        texture: asset_server.load("background_small.png"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    }).id());

    // spawn the ground
    commands.spawn(SpriteBundle {
        texture: asset_server.load("ground_cloud.png"),
        transform: Transform::from_xyz(START_X, START_Y - GROUND_OFFSET, PLATFORM_Z),
        ..default()
    });

    player::spawn_player(&mut texture_atlases, &mut game, &mut commands, &mut asset_server);

    // set high score to zero
    game.high_score.init_high_score();

    // spawn the scores
    load_scores(&mut commands, &mut asset_server, &mut game);

    // init platforms
    platform::init_platforms(&mut commands, &mut asset_server, &mut game);

    // spawn the first checkpoint
    check_point::add_checkpoint(&mut commands, &asset_server, &game.top_platform_loc, texture_atlases);

    // get the camera
    for entity in camera_query.iter() {
        game.camera = Some(entity);
    }
}

fn handle_rest(mut player_action: ResMut<NextState<PlayerAction>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game: ResMut<Game>,
    mut query: Query<&mut AnimationConfig>,
) {
    // do rest animation?
    //player::do_rest_animation(&mut game, &mut query);

    if keyboard_input.just_pressed(KeyCode::KeyA) ||
    keyboard_input.just_pressed(KeyCode::ArrowLeft)
    {
        player_action.set(PlayerAction::Jump);
        game.player.direction = Direction::Left;
    }

    if keyboard_input.just_pressed(KeyCode::KeyD) ||
    keyboard_input.just_pressed(KeyCode::ArrowRight)
    {
        player_action.set(PlayerAction::Jump);
        game.player.direction = Direction::Right;
    }
}

fn handle_jump(mut player_action: ResMut<NextState<PlayerAction>>,
    mut game: ResMut<Game>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut transforms: Query<&mut Transform>,
    mut sprite: Query<&mut Sprite>,
) {
    let correct_dir = game.correct_path.remove(0);
    // fail early
    if correct_dir != game.player.direction { // game over starts
        player_action.set(PlayerAction::Fall);
    }
    else { // do jump
        // sprite flip
        *sprite.get_mut(game.player.entity.unwrap()).unwrap() = Sprite {
            flip_x: match correct_dir {
                Direction::Left => false,
                Direction::Right => true,
            },
            ..default()
        };
        game.player.increment();
        game.score.increment();
        game.set_high_score();
        platform::increment_platform(&mut commands, &asset_server, &mut game);

        // move the player
        *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform::from_xyz(
            game.player.location.x,
            game.player.location.y,
            PLAYER_Z,
        );

        player_action.set(PlayerAction::Rest);       
    }
}

fn handle_fall(mut game_state: ResMut<NextState<GameState>>,
    mut player_action: ResMut<NextState<PlayerAction>>,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
    //mut animation: Query<&mut AnimationConfig>,
    time: Res<Time>,
) {
    let gravity: f32 = 1000.;
    let ground_y = START_Y + player::PLAYER_OFFSET;

    if game.player.location.y > ground_y {
        game.player.location.y -= gravity * time.delta_seconds();
        *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform::from_xyz(
            game.player.location.x,
            game.player.location.y,
            PLAYER_Z,
        );
        //*animation.get_mut(game.player.entity.unwrap()).unwrap() = animation::get_fall_animation_config();
    }
    else {
        player_action.set(PlayerAction::Rest);
        do_game_over(&mut game_state);
    }
}

fn update_background(mut transforms: Query<&mut Transform>,
    game: ResMut<Game>,) {
    *transforms.get_mut(game.background.unwrap()).unwrap() = Transform::from_xyz(
        0.0,
        game.player.location.y - START_Y,
        0.0,
    );
}

fn update_camera(mut transforms: Query<&mut Transform>,
    game: ResMut<Game>,
) {
    *transforms.get_mut(game.camera.unwrap()).unwrap() = Transform::from_xyz(
        0.0,
        game.player.location.y + 100.,
        CAMERA_Z,
    );
}

fn do_game_over(game_state: &mut ResMut<NextState<GameState>>) {
    game_state.set(GameState::GameOver);
}

fn load_scores(commands: &mut Commands,
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

fn update_score(mut score_query: Query<&mut Text, (With<ScoreEntity>, Without<HighScoreEntity>)>,
    mut high_score_query: Query<&mut Text, (With<HighScoreEntity>, Without<ScoreEntity>)>,
    game: Res<Game>, 
) {
    for mut score in &mut score_query {
        score.sections[0].value =  game.score.to_string();
    }
    for mut high_score in &mut high_score_query {
        high_score.sections[0].value =  game.high_score.to_string();
    }
}

// fn animate_sprites(
//     time: Res<Time>,
//     mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
// ) {
//     for (indices, mut timer, mut atlas) in &mut query {
//         timer.tick(time.delta());
//         if timer.just_finished() {
//             atlas.index = if atlas.index == indices.last {
//                 indices.first
//             } else {
//                 atlas.index + 1
//             };
//         }
//     }
// }