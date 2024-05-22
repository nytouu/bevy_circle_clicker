use bevy::prelude::*;
use rand::prelude::*;

use crate::game::config::CircleConfig;

use crate::utils::*;

/// Marker to identify HitCirlce sprites which should be coloured.
/// This component contains data to know when to despawn itself and detect the player's click
/// precision.
#[derive(Component, Default)]
pub struct HitCircle {
    pub time: f32,
    pub clicked_time: Option<f32>,
    pub missed: bool,
}

/// Marker to identify HitCircleOverlay sprites
#[derive(Component)]
pub struct HitCircleOverlay;

/// Marker to identify ApproachCircle sprites
#[derive(Component)]
pub struct ApproachCircle;

pub fn spawn_hitcircle(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<CircleConfig>,
    query: Query<&HitCircle>,
) {
    if query.is_empty() {
        let mut rng = rand::thread_rng();

        commands
            .spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(
                            rng.gen_range(-500.0..500.0),
                            rng.gen_range(-500.0..500.0),
                            0.0,
                        ),
                        scale: Vec3::new(
                            config.circle_size,
                            config.circle_size,
                            config.circle_size,
                        ),
                        ..Default::default()
                    },
                    texture: asset_server.load("hitcircle@2x.png"),
                    ..Default::default()
                },
                HitCircle::default(),
            ))
            .with_children(|parent| {
                parent.spawn((
                    SpriteBundle {
                        texture: asset_server.load("hitcircleoverlay@2x.png"),
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
                        texture: asset_server.load("approachcircle@2x.png"),
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
