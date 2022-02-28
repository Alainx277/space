use bevy_internal::prelude::Transform;

use crate::space::core::{
    entity::components::{EntityData, EntityUpdates, Sensable},
    rigid_body::components::{CachedBroadcastTransform, UpdateTransform},
    sfx::components::{get_random_pitch_scale, FootstepsWalking, RepeatingSfx},
    static_body::components::StaticTransform,
};

pub struct FootstepsWalkingSfxBundle;

impl FootstepsWalkingSfxBundle {
    pub fn new(
        passed_transform: Transform,
    ) -> (
        StaticTransform,
        EntityData,
        Sensable,
        RepeatingSfx,
        EntityUpdates,
        FootstepsWalking,
        UpdateTransform,
        CachedBroadcastTransform,
    ) {
        (
            StaticTransform {
                transform: passed_transform,
            },
            EntityData {
                entity_class: "RepeatingSFX".to_string(),
                ..Default::default()
            },
            Sensable {
                is_audible: true,
                ..Default::default()
            },
            RepeatingSfx {
                unit_db: 12.0,
                stream_id: "concrete_walking_footsteps".to_string(),
                auto_destroy: true,
                repeat_time: 0.5,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },
            EntityUpdates::default(),
            FootstepsWalking,
            UpdateTransform,
            CachedBroadcastTransform::default(),
        )
    }
}
