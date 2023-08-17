mod clock;
mod background;
mod alarm;
mod state;
mod clear;
mod style;

use bevy::prelude::*;
use bevy::window::WindowMode;
use clock::ClockPlugin;
use alarm::{AlarmMenuPlugin, AlarmTime};
use background::BackgroundPlugin;
use state::ClockState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window {
                mode: WindowMode::Fullscreen,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(AlarmTime {
            time: chrono::Local::now().time()
        })
        .add_systems(Startup, spawn_camera)
        .add_plugins(BackgroundPlugin)
        .add_state::<ClockState>()
        .add_plugins(AlarmMenuPlugin)
        .add_plugins(ClockPlugin)
        .run();
}


fn spawn_camera(mut commands: Commands) {
    // Add a default camera
    commands.spawn(Camera2dBundle::default());
}

