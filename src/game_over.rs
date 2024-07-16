use bevy::prelude::*;
use crate::GameState;
use crate::game::Game;

const BLUE: Color = Color::srgb(0.0,0.67,1.0);
const PINK: Color = Color::srgb(1.0,0.67,1.0);
const PURPLE: Color = Color::srgb(0.69, 0.67, 1.0);

#[derive(Component)]
struct GameOverEntity;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App){
        app
            .add_systems(OnEnter(GameState::GameOver), (load_game_over, set_high_score))
            .add_systems(OnExit(GameState::GameOver),  (clear_game_over, reset_game))
            .add_systems(Update, button_system.run_if(in_state(GameState::GameOver)));
    }
}

fn load_game_over(mut commands: Commands,
    entities: Query<Entity, (Without<Camera>, Without<Window>)>,
    //mut game_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
    game: Res<Game>,
)
{
    clear_ui(&mut commands, entities);
    game_over_ui(&mut commands, asset_server, game);
}

fn clear_game_over(mut commands: Commands,
    entities: Query<Entity, (Without<Camera>, Without<Window>)>,
)
{
    clear_ui(&mut commands, entities);
}

fn clear_ui(commands: &mut Commands,
    entities: Query<Entity, (Without<Camera>, Without<Window>)>,
)
{
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}

fn reset_game(mut game: ResMut<Game>) {
    game.reset();
}

fn set_high_score(mut game: ResMut<Game>) {
    if game.score.value > game.high_score.value
    {
        game.high_score.value = game.score.value;
    }
}

fn game_over_ui(commands: &mut Commands, asset_server: Res<AssetServer>, game: Res<Game>) {

    commands.spawn((NodeBundle { 
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: BackgroundColor(PURPLE),
        ..default()
    }, GameOverEntity))
    .with_children(|parent|{
        parent.spawn((ButtonBundle {
            style: Style {
                width: Val::Px(300.0),
                height: Val::Px(65.0),
                border: UiRect::all(Val::Px(5.0)),
                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(PINK),
            border_color: BorderColor(Color::BLACK),
            border_radius: BorderRadius::MAX,
            ..default()
        }, GameOverEntity))
        .with_children(|parent|{
            parent.spawn((TextBundle::from_section(
                "GAME OVER"
                , TextStyle { 
                    font: asset_server.load("FiraSans-Regular.ttf"),
                    font_size: 40.,
                    color: Color::WHITE,
                }
            ), GameOverEntity));
        });
    })
    .with_children(|parent|{
        parent.spawn((TextBundle::from_section(
            game.score.to_string()
            , TextStyle { 
                font: asset_server.load("FiraSans-Regular.ttf"),
                font_size: 40.,
                color: Color::WHITE,
            }
        ), GameOverEntity));
    });
}


fn button_system(mut interaction_query: Query<(&Interaction, &mut BackgroundColor)>
    , mut game_state: ResMut<NextState<GameState>>) 
{
    for (interaction, mut background_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background_color = BackgroundColor(BLUE);
                game_state.set(GameState::Menu);
            },
            Interaction::Hovered => *background_color = BackgroundColor(BLUE),
            Interaction::None => *background_color = BackgroundColor(PINK),
        }
    }
}
