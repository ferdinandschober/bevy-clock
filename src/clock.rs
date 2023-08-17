use chrono;

use bevy::prelude::*;
use crate::{state::ClockState, style::*, alarm::AlarmTime};

use super::clear::clear;

pub struct ClockPlugin;

impl Plugin for ClockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ClockState::ClockScreen), setup_clock_screen);
        app.add_systems(OnExit(ClockState::ClockScreen), clear::<ClockScreenItem>);
        app.add_systems(Update, text_clock_update_system.run_if(in_state(ClockState::ClockScreen)));
        app.add_systems(Update, update_alarm_button.run_if(in_state(ClockState::ClockScreen)));
    }
}

// marker component to identify clock
#[derive(Component)]
struct ClockText;

// marker component to identify clock screen items
#[derive(Component)]
struct ClockScreenItem;

fn setup_clock_screen(mut commands: Commands, asset_server: Res<AssetServer>, alarm_time: Option<Res<AlarmTime>>) {

    let font = asset_server.load("/usr/share/fonts/OTF/FiraMono-Bold.otf");

    let bell_icon = ImageBundle {
        image: asset_server.load(match alarm_time {
            Some(_) => "bell-solid.png",
            None => "bell-slash-solid.png",
        }).into(),
        style: Style {
            width: Val::Px(40.0),
            margin: UiRect {
                right: Val::Px(30.),
                ..Default::default()
            },
            ..Default::default()
        },
        ..default()
    };


    let text_clock_face = Text::from_section("88:88:88", style_clock_face(font.clone()));
    let text_alarm = match alarm_time {
        None => "88:88".to_owned(),
        Some(time) => format!("{}", time.time.format("%H:%M")),
    };
    let text_alarm_face = Text::from_section(text_alarm, style_alarm_face(font.clone()));

    let transform_clock_face = Transform::default();

    let clock_face = Text2dBundle {
        text: text_clock_face,
        transform: transform_clock_face,
        ..Default::default()
    };

    let alarm_container = NodeBundle {
        style: container_top_left(),
        ..Default::default()
    };

    let alarm_button = ButtonBundle {
        style: style_button(),
        background_color: Color::NONE.into(),
        ..Default::default()
    };

    let alarm_face = TextBundle {
        text: text_alarm_face,
        ..Default::default()
    };

    commands.spawn((clock_face, ClockText, ClockScreenItem));
    commands.spawn((alarm_container, ClockScreenItem)).with_children(|p| {
        p.spawn(alarm_button).with_children(|p| {
            p.spawn(bell_icon);
            p.spawn(alarm_face);
        });
    });
}

fn text_clock_update_system(
    mut query: Query<&mut Text, With<ClockText>>,
) {
    let time = chrono::offset::Local::now();
    let time = time.format("%H:%M:%S");

    for mut text in &mut query {
        text.sections[0].value = format!("{}", time);
    }
}

fn update_alarm_button(
    mut next_state: ResMut<NextState<ClockState>>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
) {
    for interaction in &mut interaction_query {
        if let Interaction::Pressed = interaction {
            next_state.set(ClockState::AlarmMenu);
        }
    }
}
