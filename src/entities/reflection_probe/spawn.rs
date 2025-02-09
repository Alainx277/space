use bevy_ecs::system::Commands;
use bevy_transform::components::Transform;

use crate::core::entity::components::{EntityData, EntityUpdates};

use super::components::ReflectionProbe;

pub struct ReflectionProbeBundle;

impl ReflectionProbeBundle {
    pub fn spawn(
        entity_transform: Transform,
        commands: &mut Commands,
        _correct_transform: bool,
        reflection_probe_component: ReflectionProbe,
    ) {
        let static_transform_component = entity_transform;

        commands.spawn_bundle((
            reflection_probe_component,
            static_transform_component,
            EntityData {
                entity_class: "reflection_probe".to_string(),
                ..Default::default()
            },
            EntityUpdates::default(),
        ));
    }
}
