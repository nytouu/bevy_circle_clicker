use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

pub const CURSOR_Z: f32 = 5.0;
pub const TRAIL_Z: f32 = 4.0;
pub const OVERLAY_Z: f32 = 1.0;
// pub const NUMBER_Z: f32 = 2.0;

/// Defines how fast the cursor trail textures fade out.
pub const ALPHA_DECREMENT: f32 = 0.095;

/// Defines how frequently the cursor trail should draw.
pub const TRAIL_FREQUENCY: f32 = 0.0025;

/// Constant value for the circle radius, dependant on @2x osu! textures.
/// Circles in @2x are usually 256 x 256px which make the raidus of the circle 256 / 2 = 126.
/// There's usually a blank space around the circle in the texture so I removed 3px arbitrarily.
///
/// Should be ok for most osu! skins that support HD (@2x texture)
pub const CIRCLE_RADIUS: f32 = 123.0;

/// Maximum approach circle size, should be the spawn size of the approach circle.
pub const MAX_APPROACH_SIZE: f32 = 8.0;

/// Minimum approach circle size at which it should despawn.
pub const MIN_APPROACH_SIZE: f32 = 0.9;

pub const HIT_SCORE: u64 = 300;

pub const SCORE_TEXT_SIZE: f32 = 60.0;
