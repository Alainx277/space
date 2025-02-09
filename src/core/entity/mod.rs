use bevy_app::CoreStage::PostUpdate;
use bevy_app::{App, Plugin};
use bevy_core::FixedTimestep;
use bevy_ecs::schedule::{ParallelSystemDescriptorCoercion, SystemSet};
use bevy_ecs::system::{Res, ResMut};
use bevy_log::info;

use self::systems::entity_console_commands::entity_console_commands;
use self::systems::net_system::net_system;
use self::{
    events::{NetLoadEntity, NetSendEntityUpdates, NetShowcase, NetUnloadEntity},
    resources::EntityDataResource,
    systems::{
        broadcast_position_updates::{broadcast_position_updates, INTERPOLATION_LABEL1},
        send_entity_updates::send_entity_updates,
    },
};

use super::console_commands::resources::AllConsoleCommands;
use super::console_commands::ConsoleCommandsLabels;
use super::networking::resources::ConsoleCommandVariant;
use super::{PostUpdateLabels, StartupLabels};

pub mod components;
pub mod events;
pub mod functions;
pub mod resources;
pub mod systems;

pub fn startup_entities(entity_data: Res<EntityDataResource>) {
    info!("Loaded {} different entity types.", entity_data.data.len());
}

pub struct EntityPlugin;
impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EntityDataResource>()
            .add_system_to_stage(
                PostUpdate,
                send_entity_updates
                    .after(PostUpdateLabels::EntityUpdate)
                    .label(PostUpdateLabels::SendEntityUpdates),
            )
            .add_event::<NetShowcase>()
            .add_event::<NetSendEntityUpdates>()
            .add_event::<NetUnloadEntity>()
            .add_event::<NetLoadEntity>()
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(
                        FixedTimestep::step(1. / 2.).with_label(INTERPOLATION_LABEL1),
                    )
                    .with_system(broadcast_position_updates),
            )
            .add_startup_system(
                startup_entities
                    .before(StartupLabels::BuildGridmap)
                    .label(StartupLabels::InitEntities),
            )
            .add_system(entity_console_commands)
            .add_startup_system(initialize_console_commands.before(ConsoleCommandsLabels::Finalize))
            .add_system_to_stage(
                PostUpdate,
                net_system.after(PostUpdateLabels::VisibleChecker),
            );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

pub fn initialize_console_commands(mut commands: ResMut<AllConsoleCommands>) {
    commands.list.push((
        "spawn".to_string(),
        "For server administrators only. Spawn in entities in proximity.".to_string(),
        vec![
            ("entity_name".to_string(), ConsoleCommandVariant::String),
            ("amount".to_string(), ConsoleCommandVariant::Int),
            ("player_selector".to_string(), ConsoleCommandVariant::String),
        ],
    ));
}
