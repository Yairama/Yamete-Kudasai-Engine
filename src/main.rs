use std::borrow::Borrow;
use std::ops::{Add, Sub};
use bevy::{
    app::{AppExit, ScheduleRunnerPlugin, ScheduleRunnerSettings},
    ecs::schedule::ReportExecutionOrderAmbiguities,
    log::LogPlugin,
    prelude::*,
    utils::Duration,
};
use bevy::core::CorePlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::render::primitives::Frustum;
use bevy_inspector_egui::WorldInspectorPlugin;
mod utils;
use bevy_flycam::{FlyCam, MovementSettings, NoCameraPlayerPlugin, PlayerPlugin};
use utils::blocks_from_csv;
use crate::blocks_from_csv::{Block, BlockPlugin};

pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 720.0;

#[derive(Default)]
pub struct Rules {
    start_point: Transform
}

#[derive(Default)]
pub struct Status{
    blocks: usize,
}

fn main() {

    App::new()
        .init_resource::<Status>()
        .insert_resource(ClearColor(Color::GRAY))
        .insert_resource(WindowDescriptor{
            width: WIDTH,
            height: HEIGHT,
            title: "yamete kudasai sempai Engine".to_string(),
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(BlockPlugin)
        .add_startup_system(first_start)
        .add_plugin(NoCameraPlayerPlugin)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.2,
        })
        .insert_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 120.0, // default: 12.0
        })
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())

        .run();

}



fn first_start(mut commands: Commands, mut status: ResMut<Status>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>){
    let path = "G://My Drive//MinSur//Transformacion Digital//Pucamarca//Finos//parsed_data.csv".into();
    let blocks: Vec<(Block,Transform)> = blocks_from_csv::blocks_from_csv(path);
    // Create our game rules resource
    let camera_pos = blocks.get(0).clone().unwrap().1;

    let camera = Camera3dBundle {
        transform: camera_pos.looking_at(camera_pos.translation.add(Vec3::new(-5.0,2.0,0.0)), Vec3::Y),
        frustum: Frustum::default(),
        ..Default::default()
    };

    commands.insert_resource(Rules {
        start_point: camera_pos
    });
    status.blocks = blocks.len();


    let handle_mesh = meshes.add(Mesh::from(shape::Cube { size: 5.0 })).id;
    let morr_material = materials.add(Color::OLIVE.into()).id;
    let yellow_material = materials.add(Color::YELLOW.into()).id;
    let green_material = materials.add(Color::DARK_GREEN.into()).id;
    let blue_material = materials.add(Color::NAVY.into()).id;
    let orange_material = materials.add(Color::ORANGE.into()).id;
    let red_material = materials.add(Color::RED.into()).id;
    let purple_material = materials.add(Color::PURPLE.into()).id;

    for (block, transform) in blocks {

        match block.color.as_str() {
            "\"MORR\"" => commands.spawn_bundle(PbrBundle {
                mesh: meshes.get_handle(handle_mesh),
                material: materials.get_handle(morr_material),
                transform,
                ..Default::default()
            }).insert(block),
            "\"AMARILLO\"" => commands.spawn_bundle(PbrBundle {
                mesh: meshes.get_handle(handle_mesh),
                material: materials.get_handle(yellow_material),
                transform,
                ..Default::default()
            }).insert(block),
            "\"VERDE\"" => commands.spawn_bundle(PbrBundle {
                mesh: meshes.get_handle(handle_mesh),
                material: materials.get_handle(green_material),
                transform,
                ..Default::default()
            }).insert(block),
            "\"AZUL\"" => commands.spawn_bundle(PbrBundle {
                mesh: meshes.get_handle(handle_mesh),
                material: materials.get_handle(blue_material),
                transform,
                ..Default::default()
            }).insert(block),
            "\"NARANJA\"" => commands.spawn_bundle(PbrBundle {
                mesh: meshes.get_handle(handle_mesh),
                material: materials.get_handle(orange_material),
                transform,
                ..Default::default()
            }).insert(block),
            "\"ROJO\"" => commands.spawn_bundle(PbrBundle {
                mesh: meshes.get_handle(handle_mesh),
                material: materials.get_handle(red_material),
                transform,
                ..Default::default()
            }).insert(block),
            "\"ROSADO\"" => commands.spawn_bundle(PbrBundle {
                mesh: meshes.get_handle(handle_mesh),
                material: materials.get_handle(purple_material),
                transform,
                ..Default::default()
            }).insert(block),
            _ => commands.spawn_bundle(PbrBundle {
                mesh: meshes.get_handle(handle_mesh),
                material: materials.get_handle(green_material),
                transform,
                ..Default::default()
            }).insert(block),
        };

    }


    // add plugin
    commands.spawn_bundle(camera).insert(FlyCam);
    //commands.spawn_batch(blocks);

}

