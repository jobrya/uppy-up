use bevy::prelude::*;
//use crate::Game;

use super::GameState;

pub struct MenuPlugin;

#[derive(Component)]
struct MenuEntity;

const BLUE: Color = Color::srgb(0.0,0.67,1.0);
const PINK: Color = Color::srgb(1.0,0.67,1.0);
const PURPLE: Color = Color::srgb(0.69, 0.67, 1.0);

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App){
        app
            .add_systems(OnEnter(GameState::Menu), load_menu)
            .add_systems(OnExit(GameState::Menu), clear_menu)
            .add_systems(Update, button_system.run_if(in_state(GameState::Menu)));
    }
}

fn load_menu(mut commands: Commands, asset_server: Res<AssetServer>) {

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
            background_color: BackgroundColor(PINK),
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
            Interaction::None => *background_color = BackgroundColor(PINK),
        }
    }
}