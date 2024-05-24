use bevy::audio::Volume;
use bevy::prelude::*;

use crate::game::cursor::CursorPosition;
use crate::game::hitcircle::HitCircle;
use crate::game::config::CircleConfig;
use crate::game::score::*;

use crate::utils::*;

#[derive(Resource, Default)]
pub struct HitSound(pub Handle<AudioSource>);

pub fn load_hitsound(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let handle: Handle<AudioSource> = asset_server.load("normal-hitnormal.ogg");
    commands.insert_resource(HitSound(handle));
}

pub fn detect_click(
    query: Query<(Entity, &Transform), With<HitCircle>>,
    keys: Res<ButtonInput<KeyCode>>,
    cursor_pos: Res<CursorPosition>,
    mut commands: Commands,
    hitsound: Res<HitSound>,
    config: Res<CircleConfig>,
    mut ev_score: EventWriter<ScoreUpdate>,
    score: Res<Score>
) {
    for (entity, hitcircle) in &query {
        if keys.any_just_pressed([KeyCode::KeyA, KeyCode::KeyW, KeyCode::KeyX, KeyCode::KeyZ]) {
            let distance = cursor_pos.0.distance(hitcircle.translation.xy());

            // successfully clicked the hitcircle
            if distance < config.circle_size * CIRCLE_RADIUS {
                commands.spawn(AudioBundle {
                    source: hitsound.0.clone_weak(),
                    settings: PlaybackSettings {
                        volume: Volume::new(0.1),
                        ..Default::default()
                    }
                });

                commands.entity(entity).despawn_recursive();

                ev_score.send(ScoreUpdate(score.0 + HIT_SCORE));
            }
        }
    }
}
