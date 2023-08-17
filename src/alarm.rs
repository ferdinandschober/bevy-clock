use bevy::prelude::*;
use chrono::NaiveTime;

use crate::{state::ClockState, style::*};
use super::clear::clear;

pub struct AlarmMenuPlugin;

impl Plugin for AlarmMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ClockState::AlarmMenu), setup_alarm_menu)
            .add_systems(Update, update_alarm_menu.run_if(in_state(ClockState::AlarmMenu)))
            .add_systems(OnExit(ClockState::AlarmMenu), clear::<AlarmMenuItem>);
    }
}

#[derive(Resource)]
pub struct AlarmTime {
    pub time: NaiveTime,
}

// marker component to mark entities as menu items
#[derive(Component)]
struct AlarmMenuItem;

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone)]
pub struct Alarms(pub Vec<chrono::NaiveDateTime>);

fn setup_alarm_menu(mut commands: Commands) {

    let text_heading = TextBundle::from_section("Enter Alarm Time", text_style_heading());
    let text_button_exit = TextBundle::from_section("Exit", text_style_button());

    let heading_container = NodeBundle {
        style: container_top_center(),
        ..Default::default()
    };

    let button_container = NodeBundle {
        style: container_top_left(),
        ..Default::default()
    };

    let button = ButtonBundle {
        style: style_button(),
        ..Default::default()
    };


    commands.spawn((heading_container, AlarmMenuItem)).with_children(|p| {
        p.spawn((text_heading, AlarmMenuItem));
    });

    commands.spawn((button_container, AlarmMenuItem)).with_children(|p| {
        p.spawn(button).with_children(|p| {
            p.spawn(text_button_exit);
        });
    });
}

fn update_alarm_menu(
    mut next_state: ResMut<NextState<ClockState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = (Color::GOLD * 0.8).into();
                next_state.set(ClockState::ClockScreen);
            }
            Interaction::Hovered => {
                *color = (Color::GOLD * 0.7).into();
            }
            Interaction::None => {
                *color = Color::GOLD.into();
            }
        }
    }
}
