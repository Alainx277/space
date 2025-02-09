use bevy_ecs::{prelude::Changed, system::Query};

use crate::{
    core::{
        entity::{
            components::EntityUpdates,
            functions::get_entity_update_difference::get_entity_update_difference,
        },
        networking::resources::EntityUpdateData,
    },
    entities::gi_probe::components::GIProbe,
};

pub fn gi_probe_update(
    mut updated_gi_probes: Query<(&GIProbe, &mut EntityUpdates), Changed<GIProbe>>,
) {
    for (gi_probe_component, mut entity_updates_component) in updated_gi_probes.iter_mut() {
        let old_entity_updates = entity_updates_component.updates.clone();

        let entity_updates = entity_updates_component
            .updates
            .get_mut(&".".to_string())
            .unwrap();

        entity_updates.insert(
            "bias".to_string(),
            EntityUpdateData::Float(gi_probe_component.bias),
        );
        entity_updates.insert(
            "compressed".to_string(),
            EntityUpdateData::Bool(gi_probe_component.compressed),
        );
        entity_updates.insert(
            "dynamic_range".to_string(),
            EntityUpdateData::Int(gi_probe_component.dynamic_range as i64),
        );
        entity_updates.insert(
            "energy".to_string(),
            EntityUpdateData::Float(gi_probe_component.energy),
        );
        entity_updates.insert(
            "interior".to_string(),
            EntityUpdateData::Bool(gi_probe_component.interior),
        );
        entity_updates.insert(
            "normal_bias".to_string(),
            EntityUpdateData::Float(gi_probe_component.normal_bias),
        );
        entity_updates.insert(
            "propagation".to_string(),
            EntityUpdateData::Float(gi_probe_component.propagation),
        );
        entity_updates.insert(
            "subdiv".to_string(),
            EntityUpdateData::Int(gi_probe_component.subdiv as i64),
        );
        entity_updates.insert(
            "extents".to_string(),
            EntityUpdateData::Vec3(gi_probe_component.extents),
        );

        let difference_updates =
            get_entity_update_difference(old_entity_updates, &entity_updates_component.updates);

        entity_updates_component
            .updates_difference
            .push(difference_updates);
    }
}
