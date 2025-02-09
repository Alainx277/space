use bevy_ecs::{
    event::{EventReader, EventWriter},
    system::Query,
};

use crate::core::{
    map::{
        components::Map,
        events::{InputMapRequestDisplayModes, NetRequestDisplayModes},
    },
    networking::resources::ReliableServerMessage,
};

pub fn request_display_modes(
    mut events: EventReader<InputMapRequestDisplayModes>,
    map_holders: Query<&Map>,
    mut net: EventWriter<NetRequestDisplayModes>,
) {
    for event in events.iter() {
        let map_component;

        match map_holders.get(event.entity) {
            Ok(m) => {
                map_component = m;
            }
            Err(_) => {
                continue;
            }
        }

        net.send(NetRequestDisplayModes {
            handle: event.handle,
            message: ReliableServerMessage::MapSendDisplayModes(
                map_component.available_display_modes.clone(),
            ),
        });
    }
}
