use bevy::prelude::*;
use crate::game::Game;

use super::GameState;

pub struct MenuPlugin;

#[derive(Component)]
struct MenuEntity;

const BLUE: Color = Color::srgb(0.0,0.67,1.0);
//const PINK: Color = Color::srgb(1.0,0.67,1.0);
const PURPLE: Color = Color::srgb(0.69, 0.67, 1.0);
const INSTRUCTIONS: &str = 
"Use (←, →) or (a, d) to go up. \
Move in the wrong direction or run out of time and it's game over. \
Reach an hour glass to earn more time.";

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App){
        app
            .add_systems(OnEnter(GameState::Menu), 
    (
                load_button,
                load_logo,
                load_background,
                load_instructions,
            ))
            .add_systems(OnExit(GameState::Menu), clear_menu)
            .add_systems(Update, button_system.run_if(in_state(GameState::Menu)));
    }
}

fn load_button(mut commands: Commands, asset_server: Res<AssetServer>) {

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
        ..default()
    }, MenuEntity))
    .with_children(|parent|{
        parent.spawn((ButtonBundle {
            style: Style {
                width: Val::Px(150.0),
                height: Val::Px(65.0),
                border: UiRect::all(Val::Px(5.0)),
                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(PURPLE),
            border_color: BorderColor(Color::BLACK),
            border_radius: BorderRadius::MAX,
            ..default()
        }, MenuEntity))
        .with_children(|parent|{
            parent.spawn((TextBundle::from_section(
                "PLAY"
                , TextStyle { 
                    font: asset_server.load("FiraSans-Regular.ttf"),
                    font_size: 40.,
                    color: Color::WHITE,
                }
            ), MenuEntity));
        });
    });
}

fn load_logo(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((SpriteBundle {
        texture: asset_server.load("logo.png"),
        transform: Transform::from_xyz(0., 180.0, 5.),
        ..default()
    }, MenuEntity));
}

fn load_background(mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game: ResMut<Game>,
) {
    game.background = Some(commands.spawn(SpriteBundle {
        texture: asset_server.load("background_small.png"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    }).id());
}

fn load_instructions(mut commands: Commands, asset_server: Res<AssetServer>) {

    commands.spawn((NodeBundle { 
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            align_content: AlignContent::Center,
            justify_content: JustifyContent::End,
            align_items: AlignItems::Center,
            padding: UiRect {
                bottom: Val::Percent(20.),
                ..default()
            },
            ..default()
        },
        ..default()
    }, MenuEntity))
    .with_children(|parent|{
        parent.spawn((TextBundle::from_section(
            INSTRUCTIONS,
            TextStyle {
                font: asset_server.load("FiraSans-Regular.ttf"),
                font_size: 25.,
                color: Color::WHITE,
            })
        , MenuEntity));
    });
}


fn clear_menu(mut commands: Commands, entity_query: Query<Entity, With<MenuEntity>>) {
    for entity in entity_query.iter() {
        commands.entity(entity).despawn();
    }
}

fn button_system(mut interaction_query: Query<(&Interaction, &mut BackgroundColor)>
    , mut game_state: ResMut<NextState<GameState>>) 
{
    for (interaction, mut background_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background_color = BackgroundColor(BLUE);
                game_state.set(GameState::Playing);
            },
            Interaction::Hovered => *background_color = BackgroundColor(BLUE),
            Interaction::None => *background_color = BackgroundColor(PURPLE),
        }
    }
}