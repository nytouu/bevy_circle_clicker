use bevy::{window::WindowMode, winit::WinitWindows};
use bevy::prelude::*;

use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};
use winit::window::Icon;

use bevy::render::pipelined_rendering::PipelinedRenderingPlugin;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "click the circles!".to_string(),
                        present_mode: bevy::window::PresentMode::Immediate,
                        window_theme: Some(bevy::window::WindowTheme::Dark),
                        mode: WindowMode::Fullscreen,
                        resizable: false,
                        ..default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    watch_for_changes_override: Some(true),
                    ..Default::default()
                })
                .build()
                .disable::<PipelinedRenderingPlugin>(),
            FramepacePlugin,
        ))
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, (set_window_icon, set_framerate_limit));
    }
}

fn set_framerate_limit(mut framespace_settings: ResMut<FramepaceSettings>) {
    framespace_settings.limiter = Limiter::from_framerate(400.0);
}

fn set_window_icon(windows: NonSend<WinitWindows>) {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("./assets/imgs/icon.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}
