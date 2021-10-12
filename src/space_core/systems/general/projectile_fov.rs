use std::collections::HashMap;

use bevy::{math::Vec3, prelude::{EventReader, EventWriter, Query}};

use crate::space_core::{components::{connected_player::ConnectedPlayer, senser::Senser}, events::{general::projectile_fov::ProjectileFOV, net::net_projectile_fov::NetProjectileFOV}, functions::gridmap::gridmap_functions::world_to_cell_id, resources::{doryen_fov::{Vec3Int, to_doryen_coordinates}, network_messages::{NetProjectileType, ReliableServerMessage}}};

pub fn projectile_fov(
    mut projectile_fov_events : EventReader<ProjectileFOV>,
    sensers : Query<(&Senser, &ConnectedPlayer)>,
    mut net_projectile_fov : EventWriter<NetProjectileFOV>,
) {

    let mut cell_ids_with_projectiles : HashMap<Vec3Int, Vec<(usize, Vec3)>> = HashMap::new();
    let mut projectiles = vec![];
    let mut projectiles_i : usize = 0;

    for event in projectile_fov_events.iter() {

        match event.laser_projectile {
            crate::space_core::resources::network_messages::NetProjectileType::Laser(laser_color, laser_height, laser_radius, start_pos, end_pos) => {

                let direction = (end_pos - start_pos).normalize();
                let distance = start_pos.distance(end_pos);

                let mut iterated_distance : f32 = 0.;

                projectiles.insert(projectiles_i, (laser_color, laser_height, laser_radius, direction));
                

                while iterated_distance < distance {

                    let point = start_pos + (direction*iterated_distance);

                    let cell_id = world_to_cell_id(point);

                    match cell_ids_with_projectiles.get_mut(&cell_id) {
                        Some(list) => {
                            list.push((projectiles_i, point));
                        },
                        None => {
                            cell_ids_with_projectiles.insert(
                                cell_id,
                                vec![(projectiles_i, point)]
                            );
                        },
                    }

                    iterated_distance+=2.;

                }

                projectiles_i+=1;

            },
            crate::space_core::resources::network_messages::NetProjectileType::Ballistic => {},
        }

    }



    for (senser_component, connected_player_component) in sensers.iter() {

        let mut used_projectiles_i = vec![];

        for (cell_id, projectiles_i_list) in cell_ids_with_projectiles.iter() {

            let coords = to_doryen_coordinates(cell_id.x, cell_id.z);

            match senser_component.fov.is_in_fov(coords.0, coords.1) {
                true => {

                    for (projectile_i, point) in projectiles_i_list.iter() {

                        if used_projectiles_i.contains(projectile_i) {
                            continue;
                        }

                        used_projectiles_i.push(*projectile_i);

                        let (
                            laser_color,
                            laser_height,
                            laser_radius,
                            direction,
                        ) = projectiles.get(*projectile_i).unwrap();



                        let mut iterated_distance : f32 = 0.;

                        let mut negative_distance = false;

                        let mut cells_left_to_try = true;

                        let mut adjusted_start_pos = *point;
                        let mut adjusted_end_pos = *point;

                        while cells_left_to_try {

                            if negative_distance {
                                iterated_distance-=2.;
                            } else {
                                iterated_distance+=2.;
                            }

                            let new_point = *point + (iterated_distance * *direction);
                            let cell_id = world_to_cell_id(new_point);
                            let coords = to_doryen_coordinates(cell_id.x, cell_id.z);

                            match senser_component.fov.is_in_fov(coords.0, coords.1) {
                                true => {
                                    if negative_distance {
                                        adjusted_end_pos = new_point;
                                    } else {
                                        adjusted_start_pos = new_point;
                                    }
                                    
                                },
                                false => {
                                    if negative_distance {
                                        negative_distance=true;
                                    } else {
                                        cells_left_to_try=false;
                                    }
                                },
                            }

                        }

                        if adjusted_start_pos == adjusted_end_pos {

                            adjusted_start_pos+= *direction;
                            adjusted_end_pos-= *direction;

                        }

                        net_projectile_fov.send( NetProjectileFOV {
                            handle: connected_player_component.handle,
                            message: ReliableServerMessage::FireProjectile(NetProjectileType::Laser(
                                *laser_color,
                                *laser_height,
                                *laser_radius,
                                adjusted_start_pos,
                                adjusted_end_pos,
                            )),
                        });

                    }

                },
                false => {},
            }

        }

    }




}
