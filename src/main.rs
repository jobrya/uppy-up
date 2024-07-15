use bevy::prelude::*;
mod menu;
mod game;
mod game_over;

const WINDOW_Y: f32 = 600.;
const WINDOW_X: f32 = 800.;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum GameState {
    #[default]
    Menu,
    Playing,
    GameOver,
}


fn main() {
   App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Uppy Up"),
                resolution: (WINDOW_X, WINDOW_Y).into(),
                resizable:false,
                ..default()
                }),
                ..default()
        }))
        .init_state::<GameState>()
        .add_plugins((
            menu::MenuPlugin,
            game::GamePlugin,
            game_over::GameOverPlugin,
        ))
        .add_systems(Startup, start)
        .run();
}

fn start(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
