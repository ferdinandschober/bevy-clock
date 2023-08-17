use bevy::prelude::*;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        let clear_color = ClearColor(Color::BLACK);
        app.insert_resource(clear_color);
    }
}
