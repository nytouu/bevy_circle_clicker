use bevy::prelude::*;

/// Current score
#[derive(Resource, Default)]
pub struct Score(pub u64);

/// Sent whenever there's a score change
#[derive(Event)]
pub struct ScoreUpdate(pub u64);

pub fn update_score(
    mut ev_score: EventReader<ScoreUpdate>,
    mut score: ResMut<Score>
) {
    for ev in ev_score.read() {
        score.0 = ev.0;
    }
}
