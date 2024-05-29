use std::thread::available_parallelism;

use bevy::core::TaskPoolThreadAssignmentPolicy;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy::winit::WinitWindows;
// use bevy::window::WindowMode;

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
                        present_mode: bevy::window::PresentMode::AutoNoVsync,
                        // mode: WindowMode::Fullscreen,
                        resolution: WindowResolution::new(1280.0, 720.0),
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
                .set(TaskPoolPlugin {
                    task_pool_options: TaskPoolOptions {
                        compute: TaskPoolThreadAssignmentPolicy {
                            min_threads: available_parallelism().unwrap().get(),
                            max_threads: std::usize::MAX,
                            percent: 1.0,
                        },
                        ..Default::default()
                    },
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
    framespace_settings.limiter = Limiter::from_framerate(600.0);
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
