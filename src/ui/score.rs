use bevy::prelude::*;

use crate::game::score::*;
use crate::ui::*;

use crate::utils::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        // Here we are able to call the `From` method instead of creating a new `TextSection`.
        // This will use the default font (a minimal subset of FiraMono) and apply the default styling.
        TextBundle::from_section(
            "0",
            TextStyle {
                font: asset_server.load("fonts/azuki.ttf"),
                font_size: SCORE_TEXT_SIZE,
                ..Default::default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(5.0),
            ..Default::default()
        }),
        ScoreText,
    ));
}

pub fn update_score_text(
    mut ev_score: EventReader<ScoreUpdate>,
    score: Res<Score>,
    mut query: Query<&mut Text, With<ScoreText>>,
) {
    for _ in ev_score.read() {
        let mut text = query.single_mut();
        text.sections[0].value = score.0.to_string();
    }
}
