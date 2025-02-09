use std::collections::HashMap;

use bevy_ecs::{
    entity::Entity,
    event::EventWriter,
    system::{Commands, ResMut},
};
use bevy_transform::components::Transform;

use crate::core::{
    entity::{
        events::NetShowcase,
        resources::{EntityDataResource, PawnDesignation, SpawnHeldData, SpawnPawnData},
    },
    networking::resources::ConsoleCommandVariantValues,
    pawn::{components::PersistentPlayerData, resources::UsedNames},
};

pub fn spawn_entity(
    entity_name: String,
    transform: Transform,
    commands: &mut Commands,
    correct_transform: bool,
    used_names_option: Option<&mut ResMut<UsedNames>>,
    entity_data: &ResMut<EntityDataResource>,
    held_data_option: Option<(
        Entity,
        bool,
        Option<u32>,
        &mut Option<&mut EventWriter<NetShowcase>>,
    )>,
    pawn_data_option: Option<(Vec<(String, String)>, PersistentPlayerData)>,
    mut properties: HashMap<String, ConsoleCommandVariantValues>,
) -> Option<Entity> {
    let return_entity;

    properties.insert(
        "entity_name".to_string(),
        ConsoleCommandVariantValues::String(entity_name.clone()),
    );

    match entity_data.name_to_id.get(&entity_name) {
        Some(entity_type_id) => {
            let entity_properties = entity_data.data.get(*entity_type_id).unwrap();

            let held;

            match held_data_option {
                Some(data) => {
                    held = Some(SpawnHeldData { data });
                }
                None => {
                    held = None;
                }
            }

            match pawn_data_option {
                Some(data) => {
                    let pawn = Some(SpawnPawnData {
                        data: (
                            &data.1,
                            None,
                            data.0,
                            PawnDesignation::Dummy,
                            Some(used_names_option.unwrap()),
                            None,
                            None,
                            &entity_data,
                        ),
                    });
                    return_entity = Some((*entity_properties.spawn_function)(
                        transform,
                        commands,
                        correct_transform,
                        pawn,
                        held,
                        false,
                        properties,
                    ));
                }
                None => {
                    return_entity = Some((*entity_properties.spawn_function)(
                        transform,
                        commands,
                        correct_transform,
                        None,
                        held,
                        false,
                        properties,
                    ));
                }
            }
        }
        None => {
            return_entity = None;
        }
    };

    match return_entity {
        Some(_entity) => {
            //info!("{:?}",entity);
        }
        None => {}
    }

    return_entity
}

pub fn spawn_held_entity(
    entity_name: String,
    commands: &mut Commands,
    holder_entity: Entity,
    showcase_instance: bool,
    showcase_handle_option: Option<u32>,
    net_showcase: &mut Option<&mut EventWriter<NetShowcase>>,
    entity_data: &ResMut<EntityDataResource>,
) -> Option<Entity> {
    let return_entity;

    match entity_data.name_to_id.get(&entity_name) {
        Some(entity_type_id) => {
            let entity_properties = entity_data.data.get(*entity_type_id).unwrap();

            let mut map = HashMap::new();
            map.insert(
                "entity_name".to_string(),
                ConsoleCommandVariantValues::String(entity_name),
            );

            return_entity = Some((*entity_properties.spawn_function)(
                Transform::identity(),
                commands,
                false,
                None,
                Some(SpawnHeldData {
                    data: (
                        holder_entity,
                        showcase_instance,
                        showcase_handle_option,
                        net_showcase,
                    ),
                }),
                false,
                map,
            ));
        }
        None => {
            return_entity = None;
        }
    }

    match return_entity {
        Some(_entity) => {
            //info!("(0) {:?}",entity);
        }
        None => {}
    }

    return_entity
}
