use bevy::prelude::*;

pub fn container_top_left() -> Style {
    Style {
        position_type: PositionType::Absolute,
        justify_content: JustifyContent::Start,
        align_items: AlignItems::Start,
        flex_direction: FlexDirection::Row,
        padding: UiRect {
            left: Val::Px(30.0),
            top: Val::Px(30.0),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn container_top_center() -> Style {
    Style {
        position_type: PositionType::Absolute,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Start,
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Row,
        padding: UiRect {
            top: Val::Px(30.0),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn style_button() -> Style {
    Style {
        width: Val::Px(150.),
        height: Val::Px(65.),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
    }
}

// text styles
pub fn text_style_heading() -> TextStyle {
    TextStyle {
        font_size: 30.,
        color: Color::WHITE,
        ..Default::default()
    }
}
pub fn text_style_button() -> TextStyle {
    TextStyle {
        font_size: 30.,
        color: Color::BLACK,
        ..Default::default()
    }
}

pub fn style_clock_face(font: Handle<Font>) -> TextStyle {
    TextStyle {
        font: font.clone(),
        font_size: 150.0,
        color: Color::GOLD,
    }
}

pub fn style_alarm_face(font: Handle<Font>) -> TextStyle {
    TextStyle {
        font: font.clone(),
        font_size: 50.0,
        color: Color::GOLD,
    }
}
