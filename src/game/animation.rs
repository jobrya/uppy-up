use bevy::prelude::*;
use std::time::Duration;

// from Bevy sprite animation example
// https://bevyengine.org/examples/2d-rendering/sprite-animation/

#[derive(Component)]
pub struct AnimationConfig {
    pub first_sprite_index: usize,
    pub last_sprite_index: usize,
    pub fps: u8,
    pub frame_timer: Timer,
}

impl AnimationConfig {
    fn new(first: usize, last: usize, fps: u8, timer_mode: TimerMode) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps, timer_mode),
        }
    }

    fn timer_from_fps(fps: u8, timer_mode: TimerMode) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), timer_mode)
    }
}

pub fn execute_animations(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut TextureAtlas)>,
) {

    for (mut config, mut atlas) in &mut query {
        // we track how long the current sprite has been displayed for
        config.frame_timer.tick(time.delta());
        // If it has been displayed for the user-defined amount of time (fps)...
        if config.frame_timer.just_finished() {
            if atlas.index >= config.last_sprite_index {
                atlas.index = config.first_sprite_index;
            } else {
                atlas.index += 1;
            }
        }
    }
}


pub fn get_rest_animation_config() -> AnimationConfig {
    AnimationConfig::new(0, 7, 12, TimerMode::Repeating)
}

// pub fn get_jump_animation_config() -> AnimationConfig {
//     AnimationConfig::new(0, 7, 5, TimerMode::Once)
// }

pub fn get_fall_animation_config() -> AnimationConfig {
    AnimationConfig::new(0, 2, 12, TimerMode::Repeating)
}

pub fn get_checkpoint_animation_config() -> AnimationConfig {
    AnimationConfig::new(0, 5, 6, TimerMode::Repeating)
}