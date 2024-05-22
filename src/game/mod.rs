use bevy::prelude::*;

mod camera;
mod click;
mod cursor;
mod hitcircle;
mod config;

use cursor::*;
use click::*;
use config::*;

use crate::utils::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HitSound>()
            .init_resource::<CursorPosition>()
            .init_resource::<CircleConfig>()
            .insert_resource(CursorTrailTimer(Timer::from_seconds(
                TRAIL_FREQUENCY,
                TimerMode::Repeating,
            )))
            .add_systems(
                Startup,
                (
                    click::load_hitsound,
                    camera::setup,
                    cursor::setup_cursor,
                ),
            )
            .add_systems(
                Update,
                (
                    cursor::update_cursor_coords,
                    hitcircle::spawn_hitcircle,
                    cursor::move_cursor,
                    hitcircle::color_hitcircle,
                    click::detect_click,
                    hitcircle::shrink_approach_circle,
                ),
            )
            .add_systems(FixedUpdate, (cursor::spawn_trail, cursor::remove_trail));
    }
}
