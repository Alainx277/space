use bevy_app::CoreStage::PostUpdate;
use bevy_app::{App, Plugin};
use bevy_ecs::schedule::{ParallelSystemDescriptorCoercion, SystemSet};
use bevy_ecs::system::ResMut;
use bevy_transform::components::Transform;

use crate::core::entity::functions::initialize_entity_data::initialize_entity_data;
use crate::core::entity::resources::{EntityDataProperties, EntityDataResource, GridItemData};
use crate::core::tab_actions::TabActionsQueueLabels;
use crate::core::{PostUpdateLabels, StartupLabels};

use self::events::{air_locks_actions, AirLockUnlock, NetAirLock};
use self::spawn::AirlockBundle;
use self::systems::air_lock_added::air_lock_added;
use self::systems::air_lock_default_map_added::air_lock_default_map_added;
use self::systems::air_lock_events::air_lock_events;
use self::systems::air_lock_tick_timers::air_lock_tick_timers;
use self::systems::net_system::net_system;
use self::{
    entity_update::air_lock_update,
    events::{AirLockCollision, AirLockLockClosed, AirLockLockOpen, InputAirLockToggleOpen},
};

pub mod components;
pub mod entity_update;
pub mod events;
pub mod functions;
pub mod spawn;
pub mod systems;

pub struct AirLocksPlugin;

impl Plugin for AirLocksPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AirLockCollision>()
            .add_event::<InputAirLockToggleOpen>()
            .add_event::<AirLockLockOpen>()
            .add_event::<NetAirLock>()
            .add_system(air_lock_added)
            .add_system(air_lock_tick_timers)
            .add_system(air_lock_default_map_added)
            .add_event::<AirLockLockClosed>()
            .add_event::<AirLockUnlock>()
            .add_system(air_lock_events)
            .add_system_set_to_stage(
                PostUpdate,
                SystemSet::new()
                    .label(PostUpdateLabels::EntityUpdate)
                    .with_system(air_lock_update),
            )
            .add_startup_system(content_initialization.before(StartupLabels::BuildGridmap))
            .add_system(air_locks_actions.after(TabActionsQueueLabels::TabAction))
            .add_system_to_stage(
                PostUpdate,
                net_system.after(PostUpdateLabels::VisibleChecker),
            );
    }
}

pub fn content_initialization(mut entity_data: ResMut<EntityDataResource>) {
    let entity_properties = EntityDataProperties {
        name: "securityAirLock1".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(AirlockBundle::spawn),
        grid_item: Some(GridItemData {
            transform_offset: Transform::identity(),
            can_be_built_with_grid_item: vec![],
        }),
    };

    initialize_entity_data(&mut entity_data, entity_properties);

    let entity_properties = EntityDataProperties {
        name: "vacuumAirLock".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(AirlockBundle::spawn),
        grid_item: Some(GridItemData {
            transform_offset: Transform::identity(),
            can_be_built_with_grid_item: vec![],
        }),
    };

    initialize_entity_data(&mut entity_data, entity_properties);

    let entity_properties = EntityDataProperties {
        name: "governmentAirLock".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(AirlockBundle::spawn),
        grid_item: Some(GridItemData {
            transform_offset: Transform::identity(),
            can_be_built_with_grid_item: vec![],
        }),
    };

    initialize_entity_data(&mut entity_data, entity_properties);

    let entity_properties = EntityDataProperties {
        name: "bridgeAirLock".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(AirlockBundle::spawn),
        grid_item: Some(GridItemData {
            transform_offset: Transform::identity(),
            can_be_built_with_grid_item: vec![],
        }),
    };

    initialize_entity_data(&mut entity_data, entity_properties);
}
