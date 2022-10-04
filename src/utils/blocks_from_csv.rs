use std::any::Any;
use bevy::ecs::schedule::IntoRunCriteria;
use bevy::prelude::*;
use polars::prelude::*;
use polars::prelude::DataType::Float32;
use std::cmp;
use std::ops::{Add, Sub};

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Block {
    pub name: String,
    pub size: f32,
    pub thin: f32,
    pub color: String
}

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Block>();
    }
}


impl Block {
    fn new(name:String , size: f32, thin:f32, color: String) -> Self {
        Block{
            name,
            size,
            thin,
            color
        }
    }
}


pub fn blocks_from_csv(filepath: String) -> Vec<(Block,Transform)> {



    let df = LazyCsvReader::new(filepath)
        .has_header(true).with_ignore_parser_errors(true)
        .finish()
        .unwrap()
        .select([col("X").cast(DataType::Float32),
            col("Y").cast(DataType::Float32),
            col("Z").cast(DataType::Float32),
            col("Index").cast(DataType::Utf8),
            col("COLOR").cast(DataType::Utf8),
            col("FINO").cast(DataType::Float32),
        ])
        .collect()
        .unwrap();

    let mut blocks:Vec<(Block,Transform)> = Vec::new();

    let mut temp_x = f32::MAX;
    let mut temp_y = f32::MAX;
    let mut temp_z = f32::MAX;

    for index in 0..df.height(){
        let x:f32 = df.get(index).unwrap().get(0).unwrap().to_string().parse().unwrap();
        let y:f32 = df.get(index).unwrap().get(1).unwrap().to_string().parse().unwrap();
        let z:f32 = df.get(index).unwrap().get(2).unwrap().to_string().parse().unwrap();
        let id:String = df.get(index).unwrap().get(3).unwrap().to_string();
        let color:String = df.get(index).unwrap().get(4).unwrap().to_string();
        let thin:f32 = df.get(index).unwrap().get(5).unwrap().to_string().parse().unwrap();
        let position = Vec3::new(x,z,y);
        temp_x = f32::min(temp_x,x);
        temp_y = f32::min(temp_y,y);
        temp_z = f32::min(temp_z,z);
        blocks.push((Block::new(id.to_string(),5.,thin, color),
                     Transform::from_xyz(x,z,y)));
    }

    let min_vec = Vec3::new(temp_x,temp_z,temp_y);
    for (block, transform) in &mut blocks {
        *transform = Transform::from_translation(transform.translation.sub(min_vec));
    }

    blocks
}