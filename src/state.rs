use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
pub enum ClockState {
    ClockScreen,
    AlarmScreen,
    #[default]
    AlarmMenu,
}
