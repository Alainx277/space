use std::collections::HashMap;

use super::precalculated_fov_data::Vec2Int;


pub struct WorldFOV {
    pub data: HashMap<Vec2Int, Vec<Vec2Int>>,
    pub to_be_recalculated : Vec<Vec2Int>,
    pub to_be_recalculated_priority : Vec<Vec2Int>,
    pub init : bool,
    pub blocking_load_at_init : bool,
}
