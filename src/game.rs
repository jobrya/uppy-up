use bevy::prelude::*;

use super::GameState;

mod player;
use player::Player;

mod platform;
mod score;
use score::Score;


#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

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

const GROUND_OFFSET: f32 = 200.;
const OFFSET: f32 = 40.;
const START_X: f32 = 0.0;
const START_Y: f32 = -200.;
const X_INC: f32 = 70.;
const Y_INC: f32 = 40.;
const PLAYER_Z: f32 = 2.0;
const PLATFORM_Z: f32 = 1.0;
const CAMERA_Z: f32 = 10.0;


const PLAYER_SIZE: UVec2 = UVec2 {
    x: 64,
    y: 64,
};

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App){
        app
            .init_resource::<Game>()
            .add_systems(OnEnter(GameState::Playing), start_game)
            .add_systems(Update, movement.run_if(in_state(GameState::Playing)))
            .add_systems(Update, (update_score, animate_sprite));
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

    let atlas_layout = texture_atlases.add(TextureAtlasLayout::from_grid(PLAYER_SIZE, 4, 2, None, None));
    let animation_indices = AnimationIndices { first: 0, last: 7 };

    // spawn the player
    game.player.location = Location {
        x: START_X,
        y: START_Y + OFFSET,
    };

    game.player.entity = Some(commands.spawn((
        SpriteBundle {
            // texture: asset_server.load("player_sheet.png"),
            texture: asset_server.load("Ball Guy.png"),
            transform: Transform::from_xyz(
                game.player.location.x,
                game.player.location.y,
                CAMERA_Z,
            ),
            ..default()
        },
        TextureAtlas {
            layout: atlas_layout,
            index: animation_indices.first,
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    )).id());

    // set high score to zero
    game.high_score.init_high_score();

    // spawn the scores
    load_scores(&mut commands, &mut asset_server, &mut game);

    // init platforms
    // game.correct_path = platform::init_platforms(commands, asset_server, Location::default(), &mut game);
    platform::init_platforms(commands, asset_server, &mut game);

    // get the camera
    for entity in camera_query.iter() {
        game.camera = Some(entity);
    }
}


fn movement(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game: ResMut<Game>,
    mut game_state: ResMut<NextState<GameState>>,
    mut transforms: Query<&mut Transform>,
    mut sprite: Query<&mut Sprite>,
)
{

    if keyboard_input.just_pressed(KeyCode::KeyA) ||
    keyboard_input.just_pressed(KeyCode::ArrowLeft)
    {
        if !correct_movement(&mut game, Direction::Left, &mut sprite)
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
        if !correct_movement(&mut game, Direction::Right, &mut sprite)
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
        PLAYER_Z,
    );

    // move the camera
    *transforms.get_mut(game.camera.unwrap()).unwrap() = Transform::from_xyz(
        0.0,
        //game.player.location.y - START_Y,
        game.player.location.y + 100.,
        CAMERA_Z,
    );

    // move the background
    *transforms.get_mut(game.background.unwrap()).unwrap() = Transform::from_xyz(
        0.0,
        game.player.location.y - START_Y,
        0.0,
    );

}

fn correct_movement(game: &mut ResMut<Game>,
    user_dir: Direction,
    sprite: &mut Query<&mut Sprite>,
) -> bool 
{
    let correct_dir = game.correct_path.remove(0);
    // sprite flip
    if correct_dir != game.player.direction
    {
        *sprite.get_mut(game.player.entity.unwrap()).unwrap() = Sprite {
            flip_x: match correct_dir {
                Direction::Left => false,
                Direction::Right => true,
            },
            ..default()
        };
    }
    correct_dir == user_dir
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
        parent.spawn(TextBundle::from_section(
            game.high_score.to_string()
            , TextStyle { 
                font: asset_server.load("FiraSans-Regular.ttf"),
                font_size: 40.,
                color: Color::WHITE,
            }
        ));
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

fn update_score(mut score_query: Query<&mut Text, With<ScoreEntity>>, game: ResMut<Game>) {
    for mut score in &mut score_query {
        score.sections[0].value =  game.score.to_string();
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}