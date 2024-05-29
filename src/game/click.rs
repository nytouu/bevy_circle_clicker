use bevy::audio::Volume;
use bevy::prelude::*;

use crate::game::config::*;
use crate::game::cursor::*;
use crate::game::hitcircle::*;
use crate::game::score::*;

use crate::utils::*;

#[derive(Resource, Default)]
pub struct HitSound(pub Handle<AudioSource>);

pub fn load_hitsound(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle: Handle<AudioSource> = asset_server.load("audio/normal-hitnormal.ogg");
    commands.insert_resource(HitSound(handle));
}

pub fn detect_click(
    query: Query<(Entity, &Transform, &HitCircle)>,
    keys: Res<ButtonInput<KeyCode>>,
    cursor_pos: Res<CursorPosition>,
    mut commands: Commands,
    hitsound: Res<HitSound>,
    config: Res<CircleConfig>,
    mut ev_score: EventWriter<ScoreUpdate>,
    score: Res<Score>,
    time: Res<Time>,
) {
    for (entity, transform, hitcircle) in &query {
        if keys.any_just_pressed([KeyCode::KeyA, KeyCode::KeyW, KeyCode::KeyX, KeyCode::KeyZ]) {
            let distance = cursor_pos.0.distance(transform.translation.xy());

            // successfully clicked the hitcircle
            if distance < config.circle_size * CIRCLE_RADIUS {
                commands.spawn(AudioBundle {
                    source: hitsound.0.clone_weak(),
                    settings: PlaybackSettings {
                        volume: Volume::new(0.1),
                        ..Default::default()
                    },
                });

                let current_time = time.elapsed_seconds();
                let timing = current_time - hitcircle.clicktime;

                info!(
                    "current time : {}, circle hit time : {}, diff: {}",
                    current_time, hitcircle.clicktime, timing * 10.0
                );

                ev_score.send(ScoreUpdate(score.0 + HIT_SCORE));
                commands.entity(entity).despawn_recursive();

                commands.spawn((
                    Hit::Good,
                    Transform {
                        translation: transform.translation,
                        ..Default::default()
                    },
                ));
            }
        }
    }
}

pub fn spawn_hit(
    query: Query<(Entity, &Transform, &Hit), Without<Sprite>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (entity, transform, click) in &query {
        commands.entity(entity).insert(SpriteBundle {
            transform: Transform {
                translation: transform.translation,
                scale: Vec3::new(0.5, 0.5, 0.5),
                ..Default::default()
            },
            texture: match click {
                Hit::Good => asset_server.load("imgs/gameplay/hit300-0.png"),
                Hit::Mid => asset_server.load("imgs/gameplay/hit100k-0@2x.png"),
                Hit::Bad => asset_server.load("imgs/gameplay/hit50-0@2x.png"),
                Hit::Miss => asset_server.load("imgs/gameplay/hit0-0@2x.png"),
            },
            ..Default::default()
        });
    }
}

pub fn remove_hit(
    mut query: Query<(Entity, &mut Sprite, &mut Transform, &Hit)>,
    mut commands: Commands,
) {
    for (entity, mut sprite, mut transform, hit) in &mut query {
        let mut alpha = sprite.color.a();
        alpha -= ALPHA_DECREMENT / 4.0;

        match hit {
            Hit::Miss => {
                transform.translation.y -= MISS_SLIDE;
                transform.rotate_local_z(-MISS_SLIDE / 80.0);
            }
            _ => {}
        }

        if alpha <= 0.0 {
            commands.entity(entity).despawn();
        } else {
            sprite.color = Color::rgba(1.0, 1.0, 1.0, alpha);
        }
    }
}
