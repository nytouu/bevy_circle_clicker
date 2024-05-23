use bevy::prelude::*;

use renet::{transport::NetcodeTransportError, ClientId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod client;
pub mod host;

pub const PROTOCOL_ID: u64 = 7;

/// Each player has a unique ID.
#[derive(Debug, Component, Serialize, Deserialize)]
pub struct Player {
    pub id: ClientId,
}

/// Lobby contains the list of players currently connected.
#[derive(Debug, Default, Resource)]
pub struct Lobby {
    pub players: HashMap<ClientId, Entity>,
}

/// Enum of all the possible server events.
#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
    PlayerConnected { id: ClientId },
    PlayerDisconnected { id: ClientId },
    SpawnCircle { position: Vec2 },
}

// If any error is found we just panic
#[allow(clippy::never_loop)]
pub fn panic_on_error(mut renet_error: EventReader<NetcodeTransportError>) {
    for e in renet_error.read() {
        panic!("{}", e);
    }
}
