use bevy_transform::components::Transform;

use crate::core::{
    entity::components::{EntityData, EntityUpdates},
    sensable::components::Sensable,
    sfx::components::{get_random_pitch_scale, Sfx},
};

pub struct LaserLightHit4Bundle;

pub const LASER_LIGHT_HIT4_PLAY_BACK_DURATION: f32 = 2.1 + 1.;

impl LaserLightHit4Bundle {
    pub fn new(
        passed_transform: Transform,
    ) -> (Transform, EntityData, Sensable, Sfx, EntityUpdates) {
        (
            passed_transform,
            EntityData {
                entity_class: "SFX".to_string(),
                ..Default::default()
            },
            Sensable {
                is_audible: true,
                ..Default::default()
            },
            Sfx {
                unit_db: 25.,
                unit_size: 1.,
                stream_id: "/content/audio/combat/laser_light_hit4.sample".to_string(),
                play_back_duration: LASER_LIGHT_HIT4_PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },
            EntityUpdates::default(),
        )
    }
}
