use bevy::prelude::*;

mod config;
use config::ConfigPlugin;

mod net;
use net::host::NetHostPlugin;
use net::client::NetClientPlugin;

mod game;
use game::GamePlugin;

use crate::net::{panic_on_error, Lobby};

mod utils;

#[bevy_main]
fn main() {
    println!("Usage: run with \"server\" or \"client\" argument");
    let args: Vec<String> = std::env::args().collect();

    let exec_type = &args[1];
    let is_host = match exec_type.as_str() {
        "client" => false,
        "server" => true,
        _ => panic!("Invalid argument, must be \"client\" or \"server\"."),
    };

    let mut app = App::new();
    app.add_plugins((
        ConfigPlugin,
        GamePlugin,
    ));
    app.init_resource::<Lobby>();

    if is_host {
        app.add_plugins(NetHostPlugin);
    } else {
        app.add_plugins(NetClientPlugin);
    }

    app.add_systems(Update, panic_on_error);

    app.run();
}
