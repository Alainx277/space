use std::collections::{BTreeMap, HashMap};

use bevy_ecs::{entity::Entity, event::EventWriter, system::Commands};
use bevy_hierarchy::BuildChildren;
use bevy_log::warn;
use bevy_math::{Mat4, Quat, Vec3};
use bevy_rapier3d::prelude::{
    CoefficientCombineRule, Collider, CollisionGroups, ExternalForce, ExternalImpulse, Friction,
    GravityScale, RigidBody, Sleeping, Velocity,
};
use bevy_transform::components::Transform;

use crate::core::{
    entity::{
        components::{EntityData, EntityUpdates, Showcase},
        events::NetShowcase,
        resources::{SpawnHeldData, SpawnPawnData},
    },
    examinable::components::{Examinable, RichName},
    health::components::{DamageFlag, DamageModel, Health},
    inventory::components::SlotType,
    inventory_item::components::{
        CombatAttackAnimation, CombatSoundSet, CombatStandardAnimation, CombatType, InventoryItem,
    },
    networking::resources::{ConsoleCommandVariantValues, ReliableServerMessage},
    physics::{
        components::{WorldMode, WorldModes},
        functions::{get_bit_masks, ColliderGroup},
    },
    rigid_body::components::{
        CachedBroadcastTransform, DefaultTransform, RigidBodyData, RigidBodyDisabled,
        RigidBodyLinkTransform,
    },
    sensable::components::Sensable,
};

use super::components::Helmet;

pub const STANDARD_BODY_FRICTION: f32 = 0.125;

pub struct HelmetSecurityBundle;

impl HelmetSecurityBundle {
    pub fn spawn(
        passed_transform: Transform,
        commands: &mut Commands,
        correct_transform: bool,
        _pawn_data_option: Option<SpawnPawnData>,
        held_data_option: Option<SpawnHeldData>,
        _default_map_spawn: bool,
        _properties: HashMap<String, ConsoleCommandVariantValues>,
    ) -> Entity {
        match held_data_option {
            Some(held_data) => {
                let (holder_entity, showcase_instance, showcase_handle_option, net_showcase) =
                    held_data.data;
                spawn_entity(
                    commands,
                    None,
                    true,
                    Some(holder_entity),
                    showcase_instance,
                    showcase_handle_option,
                    net_showcase,
                    false,
                )
            }
            None => spawn_entity(
                commands,
                Some(passed_transform),
                false,
                None,
                false,
                None,
                &mut None,
                correct_transform,
            ),
        }
    }
}

fn spawn_entity(
    commands: &mut Commands,

    passed_transform_option: Option<Transform>,

    held: bool,
    holder_entity_option: Option<Entity>,

    showcase_instance: bool,
    showcase_handle_option: Option<u32>,

    net_showcase: &mut Option<&mut EventWriter<NetShowcase>>,

    correct_transform: bool,
) -> Entity {
    let mut this_transform;
    let default_transform = Transform::from_matrix(Mat4::from_scale_rotation_translation(
        Vec3::new(1., 1., 1.),
        Quat::from_axis_angle(Vec3::new(-0.0394818427, 0.00003351599, 1.), 3.124470974),
        Vec3::new(0., 0.355, 0.),
    ));

    match passed_transform_option {
        Some(transform) => {
            this_transform = transform;
        }
        None => {
            this_transform = default_transform;
        }
    }

    if correct_transform {
        this_transform.rotation = default_transform.rotation;
    }

    let shape = Collider::cuboid(0.208, 0.277, 0.213);

    let collider_position = Vec3::new(0., 0.011, -0.004).into();
    let friction_val = STANDARD_BODY_FRICTION;
    let friction_combine_rule = CoefficientCombineRule::Multiply;

    let mut t = Transform::from_translation(this_transform.translation);
    t.rotation = this_transform.rotation;
    let mut friction = Friction::coefficient(friction_val);
    friction.combine_rule = friction_combine_rule;

    let mut builder = commands.spawn();
    builder
        .insert(RigidBody::Dynamic)
        .insert(t)
        .insert(Velocity::default())
        .insert(ExternalForce::default())
        .insert(ExternalImpulse::default());

    if held == false {
        let masks = get_bit_masks(ColliderGroup::Standard);

        builder
            .insert(Sleeping::default())
            .insert(GravityScale::default())
            .with_children(|children| {
                children
                    .spawn()
                    .insert(shape)
                    .insert(Transform::from_translation(collider_position))
                    .insert(friction)
                    .insert(CollisionGroups::new(masks.0, masks.1));
            });
    } else {
        let masks = get_bit_masks(ColliderGroup::NoCollision);

        let sleeping = Sleeping {
            sleeping: true,
            ..Default::default()
        };

        builder
            .insert(GravityScale(0.))
            .insert(sleeping)
            .with_children(|children| {
                children
                    .spawn()
                    .insert(shape)
                    .insert(Transform::from_translation(collider_position))
                    .insert(friction)
                    .insert(CollisionGroups::new(masks.0, masks.1));
            });
    }

    let template_examine_text = "A standard issue helmet used by Security Officers.".to_string();
    let mut examine_map = BTreeMap::new();
    examine_map.insert(0, template_examine_text);

    let mut attachment_transforms = HashMap::new();

    attachment_transforms.insert(
        "left_hand".to_string(),
        Transform::from_matrix(Mat4::from_scale_rotation_translation(
            Vec3::new(0.5, 0.5, 0.5),
            Quat::from_axis_angle(Vec3::new(1., 0., 0.), 3.111607897),
            Vec3::new(0., -0.003, -0.108),
        )),
    );

    let right_hand_rotation = Vec3::new(0.11473795, 0.775676679, 0.);
    let right_hand_rotation_length = right_hand_rotation.length();

    attachment_transforms.insert(
        "right_hand".to_string(),
        Transform::from_matrix(Mat4::from_scale_rotation_translation(
            Vec3::new(0.5, 0.5, 0.5),
            Quat::from_axis_angle(
                Vec3::new(0.11473795, 0.775676679, 0.).normalize(),
                right_hand_rotation_length,
            ),
            Vec3::new(0.064, -0.019, 0.065),
        )),
    );

    attachment_transforms.insert(
        "helmet".to_string(),
        Transform::from_matrix(Mat4::from_scale_rotation_translation(
            Vec3::new(0.5, 0.5, 0.5),
            Quat::from_axis_angle(Vec3::new(1., 0., 0.), -1.41617761),
            Vec3::new(0., 0.132, 0.05),
        )),
    );

    let entity_id = builder.id();

    let mut melee_damage_flags = HashMap::new();
    melee_damage_flags.insert(0, DamageFlag::SoftDamage);

    builder.insert_bundle((
        EntityData {
            entity_class: "entity".to_string(),
            entity_name: "helmetSecurity".to_string(),
            ..Default::default()
        },
        EntityUpdates::default(),
        WorldMode {
            mode: WorldModes::Physics,
        },
        CachedBroadcastTransform::default(),
        Examinable {
            assigned_texts: examine_map,
            name: RichName {
                name: "security helmet".to_string(),
                n: false,
                ..Default::default()
            },
            ..Default::default()
        },
        Helmet,
        InventoryItem {
            in_inventory_of_entity: holder_entity_option,
            attachment_transforms: attachment_transforms,
            drop_transform: default_transform,
            slot_type: SlotType::Helmet,
            is_attached_when_worn: true,
            combat_attack_animation: CombatAttackAnimation::OneHandedMeleePunch,
            combat_type: CombatType::MeleeDirect,
            combat_melee_damage_model: DamageModel {
                brute: 9.,
                damage_flags: melee_damage_flags,
                ..Default::default()
            },
            combat_projectile_damage_model: None,
            combat_melee_sound_set: CombatSoundSet::default(),
            combat_standard_animation: CombatStandardAnimation::StandardStance,
            combat_projectile_sound_set: None,
            combat_melee_text_set: InventoryItem::get_default_strike_words(),
            combat_projectile_text_set: None,
            trigger_melee_text_set: InventoryItem::get_default_trigger_melee_words(),
            trigger_projectile_text_set: None,
            active_slot_tab_actions: vec![],
            throw_force_factor: 2.,
        },
        DefaultTransform {
            transform: default_transform,
        },
        RigidBodyData {
            friction: friction.coefficient,
            friction_combine_rule: friction.combine_rule,
        },
    ));

    if showcase_instance {
        let handle = showcase_handle_option.unwrap();
        builder.insert(Showcase { handle: handle });
        let entity_updates = HashMap::new();
        net_showcase.as_deref_mut().unwrap().send(NetShowcase {
            handle: handle,
            message: ReliableServerMessage::LoadEntity(
                "entity".to_string(),
                "helmetSecurity".to_string(),
                entity_updates,
                entity_id.to_bits(),
                true,
                "main".to_string(),
                "".to_string(),
                false,
            ),
        });
    } else {
        builder.insert_bundle((Sensable::default(), Health::default()));
    }

    match held {
        true => {
            builder.insert_bundle((
                RigidBodyDisabled,
                WorldMode {
                    mode: WorldModes::Worn,
                },
            ));
        }
        false => {
            builder.insert(WorldMode {
                mode: WorldModes::Physics,
            });
        }
    }

    match holder_entity_option {
        Some(holder_entity) => {
            builder.insert(RigidBodyLinkTransform {
                follow_entity: holder_entity,
                ..Default::default()
            });
        }
        None => {
            if held == true {
                warn!("Spawned entity in held mode but holder_entity_option is none.");
            }
        }
    }

    entity_id
}
