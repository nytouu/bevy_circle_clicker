use bevy::prelude::*;
use rand::prelude::*;

use crate::game::config::CircleConfig;

use crate::utils::*;

/// Marker to identify HitCirlce sprites which should be coloured.
/// Holds data to know its correct timing and lifetime.
#[derive(Component, Default)]
pub struct HitCircle {
    /// Correct click timing of the circle.
    pub clicktime: f32,
    /// Used to despawn the hitcircle if it is not clicked.
    pub lifetime: f32,
}

/// Enum that represents the accuracy of a click.
#[allow(dead_code)]
#[derive(Component)]
pub enum Hit {
    /// Equivalent to 300 in osu!
    Good,
    /// Equivalent to 100 in osu!
    Mid,
    /// Equivalent to 50 in osu!
    Bad,
    /// Missed note
    Miss,
}

/// Marker to identify HitCircleOverlay sprites.
#[derive(Component)]
pub struct HitCircleOverlay;

/// Marker to identify ApproachCircle sprites.
#[derive(Component)]
pub struct ApproachCircle;

pub fn spawn_hitcircle(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<CircleConfig>,
    query: Query<&HitCircle>,
    time: Res<Time>,
) {
    if query.is_empty() {
        let mut rng = rand::thread_rng();

        let clicktime = time.elapsed_seconds() + 150.0 * config.approach_rate;
        info!("circle should be click at :{}", clicktime);

        commands
            .spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(
                            rng.gen_range(-200.0..200.0),
                            rng.gen_range(-300.0..300.0),
                            0.0,
                        ),
                        scale: Vec3::new(
                            config.circle_size,
                            config.circle_size,
                            config.circle_size,
                        ),
                        ..Default::default()
                    },
                    texture: asset_server.load("imgs/gameplay/hitcircle@2x.png"),
                    ..Default::default()
                },
                HitCircle {
                    clicktime,
                    ..Default::default()
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    SpriteBundle {
                        texture: asset_server.load("imgs/gameplay/hitcircleoverlay@2x.png"),
                        transform: Transform {
                            translation: Vec3::new(0.0, 0.0, OVERLAY_Z),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    HitCircleOverlay,
                ));
            })
            .with_children(|parent| {
                parent.spawn((
                    SpriteBundle {
                        texture: asset_server.load("imgs/gameplay/approachcircle@2x.png"),
                        transform: Transform {
                            translation: Vec3::new(0.0, 0.0, OVERLAY_Z),
                            scale: Vec3::new(
                                config.circle_size * MAX_APPROACH_SIZE,
                                config.circle_size * MAX_APPROACH_SIZE,
                                config.circle_size * MAX_APPROACH_SIZE,
                            ),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    ApproachCircle,
                ));
            });
    }
}

pub fn color_hitcircle(mut query: Query<&mut Sprite, With<HitCircle>>) {
    for mut sprite in &mut query {
        if sprite.color != Color::LIME_GREEN {
            sprite.color = Color::LIME_GREEN;
        }
    }
}

pub fn shrink_approach_circle(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<ApproachCircle>>,
    config: Res<CircleConfig>,
) {
    for (entity, mut transform) in &mut query {
        let scale = transform.scale;

        transform.scale = Vec3::new(
            scale.x - config.approach_rate,
            scale.y - config.approach_rate,
            scale.z - config.approach_rate,
        );

        if transform.scale.x <= MIN_APPROACH_SIZE {
            commands.entity(entity).despawn();
        }
    }
}

pub fn check_hitcircle_life(
    mut query: Query<(Entity, &mut HitCircle, &Transform)>,
    config: Res<CircleConfig>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut hitcircle, transform) in &mut query {
        hitcircle.lifetime += time.delta_seconds();
        if hitcircle.lifetime >= hitcircle.clicktime + config.overall_difficulty / 2.0 {
            commands.entity(entity).despawn_recursive();

            commands.spawn((
                Hit::Miss,
                Transform {
                    translation: transform.translation,
                    ..Default::default()
                },
            ));
        }
    }
}
