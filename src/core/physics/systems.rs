use bevy_ecs::{
    entity::Entity,
    event::{EventReader, EventWriter},
    prelude::With,
    system::Query,
};
use bevy_hierarchy::Parent;
use bevy_rapier3d::{pipeline::CollisionEvent, prelude::Collider};
use bevy_transform::prelude::Transform;

use crate::{
    core::entity::components::{EntityData, EntityGroup},
    entities::{
        air_locks::events::AirLockCollision, counter_windows::events::CounterWindowSensorCollision,
    },
};

pub fn physics_events(
    mut collision_events: EventReader<CollisionEvent>,
    interesting_entities_query: Query<(Entity, &EntityData, &Transform)>,
    parents: Query<&Parent, With<Collider>>,
    mut air_lock_collision_event: EventWriter<AirLockCollision>,
    mut counter_window_collision_event: EventWriter<CounterWindowSensorCollision>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(collider1_handle, collider2_handle, _flags) => {
                let collider1_parent;
                match parents.get(*collider1_handle) {
                    Ok(parent_component) => {
                        collider1_parent = parent_component.0;
                    }
                    Err(_rr) => {
                        collider1_parent = *collider1_handle;
                    }
                }

                let collider2_parent;

                match parents.get(*collider2_handle) {
                    Ok(parent_component) => {
                        collider2_parent = parent_component.0;
                    }
                    Err(_rr) => {
                        collider2_parent = *collider2_handle;
                    }
                }

                process_physics_event(
                    collider1_parent,
                    collider2_parent,
                    true,
                    &interesting_entities_query,
                    &mut air_lock_collision_event,
                    &mut counter_window_collision_event,
                );
            }
            CollisionEvent::Stopped(collider1_handle, collider2_handle, _flags) => {
                process_physics_event(
                    *collider1_handle,
                    *collider2_handle,
                    false,
                    &interesting_entities_query,
                    &mut air_lock_collision_event,
                    &mut counter_window_collision_event,
                );
            }
        }
    }
}

fn process_physics_event(
    collider1_entity: Entity,
    collider2_entity: Entity,
    collision_started: bool,
    interesting_entities_query: &Query<(Entity, &EntityData, &Transform)>,
    air_lock_collision_event: &mut EventWriter<AirLockCollision>,
    counter_window_collision_event: &mut EventWriter<CounterWindowSensorCollision>,
) {
    let mut first_collider_group = EntityGroup::None;
    let mut second_collider_group = EntityGroup::None;

    let collider1_components;

    match interesting_entities_query.get(collider1_entity) {
        Ok(t) => {
            collider1_components = t;
        }
        Err(_) => {
            return;
        }
    }

    let collider2_components;

    match interesting_entities_query.get(collider2_entity) {
        Ok(t) => {
            collider2_components = t;
        }
        Err(_) => {
            return;
        }
    }

    if matches!(collider1_components.1.entity_group, EntityGroup::None) == false {
        first_collider_group = collider1_components.1.entity_group;
    }

    if matches!(collider2_components.1.entity_group, EntityGroup::None) == false {
        second_collider_group = collider2_components.1.entity_group;
    }

    if matches!(first_collider_group, EntityGroup::AirLock)
        || matches!(second_collider_group, EntityGroup::AirLock)
    {
        air_lock_collision_event.send(AirLockCollision {
            collider1_entity: collider1_entity,
            collider2_entity: collider2_entity,

            collider1_group: first_collider_group,
            collider2_group: second_collider_group,

            started: collision_started,
        });
    } else if (matches!(first_collider_group, EntityGroup::CounterWindowSensor)
        || matches!(second_collider_group, EntityGroup::CounterWindowSensor))
    {
        counter_window_collision_event.send(CounterWindowSensorCollision {
            collider1_entity: collider1_entity,
            collider2_entity: collider2_entity,

            collider1_group: first_collider_group,
            collider2_group: second_collider_group,

            started: collision_started,
        });
    }
}
