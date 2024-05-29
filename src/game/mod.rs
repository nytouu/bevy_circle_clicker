use bevy::prelude::*;

mod camera;
mod click;
mod config;
mod cursor;
mod hitcircle;
pub mod score;

use click::*;
use config::*;
use cursor::*;
use score::*;

use crate::utils::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HitSound>()
            .init_resource::<CursorPosition>()
            .init_resource::<CircleConfig>()
            .init_resource::<Score>()
            .add_event::<ScoreUpdate>()
            .insert_resource(Time::<Fixed>::from_hz(REFRESH_RATE))
            .insert_resource(CursorTrailTimer(Timer::from_seconds(
                TRAIL_FREQUENCY,
                TimerMode::Repeating,
            )))
            .add_systems(
                Startup,
                (click::load_hitsound, camera::setup, cursor::setup_cursor),
            )
            .add_systems(
                Update,
                (
                    cursor::move_cursor,
                    cursor::update_cursor_coords,
                    hitcircle::spawn_hitcircle,
                    hitcircle::check_hitcircle_life,
                ),
            )
            .add_systems(PostUpdate, (
                click::detect_click,
            ))
            .add_systems(
                FixedUpdate,
                (
                    hitcircle::color_hitcircle,
                    hitcircle::shrink_approach_circle,
                    score::update_score,
                    cursor::spawn_trail,
                    cursor::remove_trail,
                    click::spawn_hit,
                    click::remove_hit
                )
                    .chain(),
            );
    }
}
