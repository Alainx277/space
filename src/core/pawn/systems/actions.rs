use bevy_ecs::{event::EventWriter, system::Res};

use crate::core::{
    connected_player::events::{InputExamineEntity, InputExamineMap},
    gridmap::resources::Vec3Int,
    tab_actions::resources::QueuedTabActions,
};

pub fn actions(
    queue: Res<QueuedTabActions>,

    mut event_examine_entity: EventWriter<InputExamineEntity>,
    mut event_examine_map: EventWriter<InputExamineMap>,
) {
    for queued in queue.queue.iter() {
        if queued.tab_id == "actions::pawn/examine" && queued.handle_option.is_some() {
            match queued.target_entity_option {
                Some(entity_bits) => {
                    event_examine_entity.send(InputExamineEntity {
                        handle: queued.handle_option.unwrap(),
                        examine_entity_bits: entity_bits,
                        entity: queued.player_entity,
                    });
                }
                None => match &queued.target_cell_option {
                    Some((gridmap_type, idx, idy, idz)) => {
                        event_examine_map.send(InputExamineMap {
                            handle: queued.handle_option.unwrap(),
                            entity: queued.player_entity,
                            gridmap_type: gridmap_type.clone(),
                            gridmap_cell_id: Vec3Int {
                                x: *idx,
                                y: *idy,
                                z: *idz,
                            },
                        });
                    }
                    None => {}
                },
            }
        }
    }
}
