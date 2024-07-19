use bevy::prelude::*;
use std::time::Duration;
use crate::game::{Location, Game};
use crate::game::animation;


const CHECK_POINT_SIZE: UVec2 = UVec2::splat(32);
pub const CHECK_POINT_OFFSET: f32 = 20.;

#[derive(Component)]
pub struct CheckPointTimerEntity;

#[derive(Component, Default)]
pub struct CheckPoint {
    timer: Timer,
    location: Location,
}

impl CheckPoint {
    pub fn to_string(&self) -> String{
        String::from("Time: ") + &self.timer.elapsed_secs().round().to_string()
    }
}


pub fn spawn_checkpoint (
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    game: &mut ResMut<Game>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let atlas_layout = texture_atlases.add(TextureAtlasLayout::from_grid(CHECK_POINT_SIZE, 2, 3, None, None));
    let animation_config = animation::get_checkpoint_animation_config();
    let dur = Duration::from_secs_f32(30.);
    game.check_point = CheckPoint {
        timer: Timer::new(dur, TimerMode::Once),
        location: game.top_platform_loc.clone(),
    };

    commands.spawn((
        SpriteBundle {
            // texture: asset_server.load("player_sheet.png"),
            texture: asset_server.load("hour_glass_v2.png"),
            transform: Transform::from_xyz(
                game.top_platform_loc.x,
                game.top_platform_loc.y + CHECK_POINT_OFFSET,
                1.5,
            ),
            ..default()
        },
        TextureAtlas {
            layout: atlas_layout,
            index: animation_config.first_sprite_index,
        },
        animation_config,
    ));
}

pub fn display_check_point_timer(game: &mut ResMut<Game>,
    asset_server: &mut Res<AssetServer>,
    commands: &mut Commands) 
{
        commands.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_content: AlignContent::Start,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::End,
                ..default()
            },
            ..default()
            }
        )
        .with_children(|parent|{
            parent.spawn((TextBundle::from_section(
                game.check_point.to_string()
                , TextStyle { 
                    font: asset_server.load("FiraSans-Regular.ttf"),
                    font_size: 40.,
                    color: Color::WHITE,
                }
            ), CheckPointTimerEntity));
        });

}

pub fn update_display_checkpoint(mut query: Query<&mut Text, With<CheckPointTimerEntity>>,
    mut game: ResMut<Game>,
    time: Res<Time>,
) 
{
    println!("updating check point");
    game.check_point.timer.tick(time.delta());
    for mut check_point in &mut query {
        check_point.sections[0].value =  game.check_point.to_string();
    }
}