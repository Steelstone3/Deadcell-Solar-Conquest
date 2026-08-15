use bevy::{
    ecs::component::Component,
    time::{Timer, TimerMode},
};

#[derive(Component)]
pub struct AnimationTimer {
    pub timer: Timer,
    pub frame_count: usize,
}

impl AnimationTimer {
    pub fn new(timer: f32, frame_count: usize) -> AnimationTimer {
        Self {
            timer: Timer::from_seconds(timer, TimerMode::Repeating),
            frame_count,
        }
    }
}
