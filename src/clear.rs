use bevy::prelude::*;

pub fn clear<T: Component>(to_clear: Query<Entity, With<T>>, mut commands: Commands) {
    for e in &to_clear {
        commands.entity(e).despawn_recursive();
    }
}
