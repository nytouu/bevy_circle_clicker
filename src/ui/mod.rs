use bevy::prelude::*;

pub struct UiPlugin;

mod score;

#[derive(Component)]
pub struct ScoreText;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, score::setup)
            .add_systems(PostUpdate, score::update_score_text);
    }
}
