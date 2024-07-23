use bevy::prelude::*;

mod player;
mod platform;
mod score;
mod check_point;
mod animation;
use player::Player;
use player::PlayerAction;
use score::Score;
use check_point::CheckPoint;
use super::GameState;

const GROUND_OFFSET: f32 = 200.;
const START_X: f32 = 0.0;
const START_Y: f32 = -200.;
const X_INC: f32 = 70.;
const Y_INC: f32 = 40.;
const PLAYER_Z: f32 = 2.0;
const PLATFORM_Z: f32 = 1.0;
const CAMERA_Z: f32 = 10.0;

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
    pub camera: Option<Entity>,
    pub background: Option<Entity>,
    pub score: Score,
    pub high_score: Score,
    top_platform_loc: Location,
    correct_path: Vec<Direction>,
    platforms: Vec<Location>,
    check_point: CheckPoint,
}

impl Game {
    pub fn reset(&mut self) {
        self.player = Player::default();
        self.score = Score::default();
        self.top_platform_loc = Location::default();
        self.correct_path = Vec::new();
        self.platforms = Vec::new();
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
                score::update_score,
                check_point::update_display_checkpoint,
                animation::execute_animations,
                timer_check,
            )
                .run_if(in_state(GameState::Playing)))
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
    // spawn the ground
    commands.spawn(SpriteBundle {
        texture: asset_server.load("ground_cloud.png"),
        transform: Transform::from_xyz(START_X, START_Y - GROUND_OFFSET, PLATFORM_Z),
        ..default()
    });

    player::spawn_player(&mut texture_atlases, &mut game, &mut commands, &mut asset_server);
    game.high_score.init_high_score();
    score::load_scores(&mut commands, &mut asset_server, &mut game);
    platform::init_platforms(&mut commands, &mut asset_server, &mut game);
    check_point::spawn_checkpoint(&mut commands, &asset_server, &mut game, texture_atlases); 
    check_point::display_checkpoint_timer(&mut game, &mut asset_server, &mut commands);

    // get the camera
    for entity in camera_query.iter() {
        game.camera = Some(entity);
    }
}

fn handle_rest(mut player_action: ResMut<NextState<PlayerAction>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game: ResMut<Game>,
) 
{

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
) 
{
    let correct_dir = game.correct_path.remove(0);
    let correct_loc = game.platforms.remove(0);
    // fail early
    if correct_dir != game.player.direction { // game over starts
        // game over sound
        commands.spawn(AudioBundle {
            source: asset_server.load("woops.ogg"),
            ..default()
        });
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

        // check point
        if correct_loc.y == game.check_point.location.y {
            game.check_point.timer.reset();
            // check point sound
            commands.spawn(AudioBundle {
                source: asset_server.load("impactGlass_heavy_002.ogg"),
                ..default()
            });
            check_point::move_checkpoint(&mut game, &mut transforms);
        }

        game.player.increment();
        game.score.increment();
        game.set_high_score();
        platform::increment_platform(&mut commands, &asset_server, &mut game);

        // jump sound
        commands.spawn(AudioBundle {
            source: asset_server.load("footstep_wood_004.ogg"),
            ..default()
        });

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

fn timer_check(game: ResMut<Game>,
    mut player_action: ResMut<NextState<PlayerAction>>
)
{
    if game.check_point.timer.finished() {
        player_action.set(PlayerAction::Fall);
    }
}
