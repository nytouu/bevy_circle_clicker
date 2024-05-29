use bevy::prelude::*;

use crate::utils::*;

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        MainCamera,
    ));
}
