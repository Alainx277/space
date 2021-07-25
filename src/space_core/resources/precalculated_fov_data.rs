use std::collections::HashMap;

use bevy::prelude::{FromWorld, World};

use crate::space_core::functions::converters::string_to_type_converters::string_vec2_to_vec2_int;


pub struct PrecalculatedFOVData {
    pub data: HashMap<Vec2Int, Vec<Vec2Int>>
}

impl PrecalculatedFOVData {
    pub fn new(raw_data : HashMap<String,Vec<String>>) -> HashMap<Vec2Int, Vec<Vec2Int>> {

        let mut pure_data = HashMap::new();

        for (key_value, value_vector) in raw_data {

            let key_vector = string_vec2_to_vec2_int(&key_value);

            let mut vector_values = vec![];

            for value in value_vector {

                vector_values.push(string_vec2_to_vec2_int(&value));

            }

            pure_data.insert(key_vector, vector_values);

        }

        pure_data

    }
}

impl FromWorld for PrecalculatedFOVData {
    fn from_world(_world: &mut World) -> Self {
        PrecalculatedFOVData {
           data : HashMap::new(), 
        }
    }
}


#[derive(PartialEq,Eq, Hash, Copy, Clone, Debug)]
pub struct Vec2Int {
    pub x : i16,
    pub y : i16,   
}
#[derive(PartialEq,Eq, Hash, Copy, Clone, Debug)]
pub struct Vec3Int {
    pub x : i16,
    pub y : i16,  
    pub z : i16,  
}
