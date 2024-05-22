use bevy::core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping};
use bevy::prelude::*;

use crate::utils::*;

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..Default::default()
            },
            tonemapping: Tonemapping::AcesFitted,
            ..Default::default()
        },
        MainCamera,
        BloomSettings::default(),
    ));
}
