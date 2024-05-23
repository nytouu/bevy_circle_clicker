use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use serde::{Deserialize, Serialize};

use crate::utils::*;

#[derive(Debug, Default, Serialize, Deserialize, Component, Resource)]
pub struct CursorPosition(pub Vec2);

#[derive(Component, Default)]
pub struct Cursor;

#[derive(Component)]
pub struct CursorTrail;

#[derive(Resource)]
pub struct CursorTrailTimer(pub Timer);

pub fn spawn_trail(
    mut timer: ResMut<CursorTrailTimer>,
    time: Res<Time>,
    query: Query<&Transform, With<Cursor>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    timer.0.tick(time.delta());

    for transform in &query {
        if timer.0.just_finished() {
            commands.spawn((
                CursorTrail,
                SpriteBundle {
                    texture: asset_server.load("cursortrail.png"),
                    transform: Transform {
                        translation: Vec3::new(transform.translation.x, transform.translation.y, TRAIL_Z),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ));
        }
    }

}

pub fn remove_trail(
    mut query: Query<(Entity, &mut Sprite), With<CursorTrail>>,
    mut commands: Commands,
) {
    for (entity, mut sprite) in &mut query {
        let mut alpha = sprite.color.a();
        alpha -= ALPHA_DECREMENT;

        if alpha <= 0.0 {
            commands.entity(entity).despawn();
        } else {
            sprite.color = Color::rgba(0.0, 0.0, 1.0, alpha);
        }
    }
}

#[allow(dead_code)]
pub fn update_cursor_coords(
    mut cursor_pos: ResMut<CursorPosition>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        cursor_pos.0 = world_position;
    }
}

#[allow(dead_code)]
pub fn spawn_cursor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut q_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = q_window.single_mut();

    commands.spawn((
        Cursor,
        SpriteBundle {
            texture: asset_server.load("cursor.png"),
            ..Default::default()
        },
    ));

    primary_window.cursor.visible = false;
}

#[allow(dead_code)]
pub fn move_cursor(input: Res<CursorPosition>, mut query: Query<&mut Transform, With<Cursor>>) {
    let mut cursor = query.single_mut();
    let x = input.0.x;
    let y = input.0.y;

    cursor.translation = Vec3::new(x, y, CURSOR_Z);
}
