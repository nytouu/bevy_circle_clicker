use bevy::prelude::*;

/// Holds data to determine :
/// - **Circle size**: how big the hitcircles should be
/// - **Approach rate**: how fast should the appraoch circle shrink and dissapear
/// - **Overall difficulty**: how lenient the rhythm detection is when clicking circles
/// - **Health**: how fast the HP Bar should deplete
#[derive(Resource)]
pub struct CircleConfig {
    /// **Circle size**: how big the hitcircles should be
    pub circle_size: f32,
    /// **Approach rate**: how fast should the appraoch circle shrink and dissapear
    pub approach_rate: f32,
    /// **Overall difficulty**: how lenient the rhythm detection is when clicking circles
    pub overall_difficulty: f32,
    /// **Health**: how fast the HP Bar should deplete
    pub health: f32,
}

impl Default for CircleConfig {
    fn default() -> Self {
        CircleConfig {
            circle_size: 0.5,
            approach_rate: 0.023,
            overall_difficulty: 50.0,
            health: 10.0,
        }
    }
}
