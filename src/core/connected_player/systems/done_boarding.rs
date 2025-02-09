use bevy_core::Timer;
use bevy_ecs::{
    event::{EventReader, EventWriter},
    system::{Commands, ResMut},
};
use bevy_log::info;

use crate::{
    core::{
        chat::functions::get_talk_spaces,
        connected_player::{
            components::{OnBoard, SetupPhase, SoftPlayer, Spawning},
            events::{BoardingPlayer, NetDoneBoarding},
        },
        gridmap::resources::SpawnPoints,
        networking::resources::{ReliableServerMessage, ServerConfigMessage},
    },
    entities::asana::resources::AsanaBoardingAnnouncements,
};

pub fn done_boarding(
    mut spawn_points: ResMut<SpawnPoints>,
    mut net_done_boarding: EventWriter<NetDoneBoarding>,
    mut boarding_player_event: EventReader<BoardingPlayer>,
    mut commands: Commands,

    mut asana_boarding_announcements: ResMut<AsanaBoardingAnnouncements>,
) {
    for boarding_player in boarding_player_event.iter() {
        let player_character_name = boarding_player.player_character_name.clone();
        let player_handle = boarding_player.player_handle;
        let entity_id = boarding_player.entity;

        info!(
            "{} [{}] has boarded the spaceship.",
            player_character_name, player_handle
        );

        let assigned_spawn_transform = spawn_points.list[spawn_points.i].transform;

        commands
            .entity(entity_id)
            .insert_bundle((
                OnBoard,
                Spawning {
                    transform: assigned_spawn_transform,
                },
            ))
            .remove_bundle::<(SetupPhase, SoftPlayer)>();

        spawn_points.i += 1;

        if spawn_points.i >= spawn_points.list.len() {
            spawn_points.i = 0;
        }

        // Queue net_code message for client so he goes back to the main scene and ditches setupUI.
        net_done_boarding.send(NetDoneBoarding {
            handle: player_handle,
            message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::ChangeScene(
                true,
                "main".to_string(),
            )),
        });

        let talk_spaces = get_talk_spaces();

        net_done_boarding.send(NetDoneBoarding {
            handle: player_handle,
            message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::TalkSpaces(
                talk_spaces,
            )),
        });

        asana_boarding_announcements.announcements.push((
            ";Security Officer ".to_owned() + &player_character_name + " is now on board.",
            Timer::from_seconds(2., false),
        ));
    }
}
