use std::collections::HashMap;

use bevy_ecs::{
    entity::Entity,
    prelude::{FromWorld, World},
};
use bevy_rapier3d::na::Quaternion;

pub struct AuthidI {
    pub i: u16,
}

impl FromWorld for AuthidI {
    fn from_world(_world: &mut World) -> Self {
        AuthidI { i: 0 }
    }
}

#[derive(Default)]
pub struct UsedNames {
    pub names: HashMap<String, Entity>,
    pub user_names: HashMap<String, Entity>,
    pub player_i: u32,
    pub dummy_i: u32,
}

pub struct PawnYAxisRotations;

impl PawnYAxisRotations {
    pub fn new() -> Vec<Quaternion<f32>> {
        vec![
            //0deg
            Quaternion::new(1., 0., 0., 0.),
            //45deg
            Quaternion::new(0.9238795, 0., 0.3826834, 0.),
            //90deg
            Quaternion::new(0.7071068, 0., 0.7071068, 0.),
            //135deg
            Quaternion::new(0.3826834, 0., 0.9238795, 0.),
            //180deg
            Quaternion::new(0., 0., 1., 0.),
            //225deg
            Quaternion::new(-0.3826834, 0., 0.9238795, 0.),
            //270deg
            Quaternion::new(-0.7071068, 0., 0.7071068, 0.),
            //315deg
            Quaternion::new(-0.9238795, 0., 0.3826834, 0.),
        ]
    }
}
