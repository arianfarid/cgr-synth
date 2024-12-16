use bevy::{
    app::{App}, 
    prelude::*,
};
use std::{env, f64::consts::PI};
use structopt::StructOpt;



fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, (setup_camera, setup))
    // .add_systems(Update, add_cgr)
    .run();
}


fn add_cgr(mut commands: Commands) {

}


//TODO cli args 
// #[derive(Debug, StructOpt)]
// #[structopt(name = "Args")]
// struct Opt {
//     nodes: u8
// }

#[derive(Component)]
pub struct Nodes {
    node_vec: Vec<Note>
}
pub struct Note {
    xy: Vec2, freq: f32, //note: String
}
#[derive(Component)]
pub struct Node;
pub const NODE_COLOR: Vec3 = Vec3::new(255., 255., 0.);

fn setup
    (
        mut commands: Commands, 
        mut materials: ResMut<Assets<ColorMaterial>>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) {
    
    //set 7 nodes. TODO: set this from CLI
    let mut nodes = Nodes {
        node_vec: vec![]
    };
    let node_count = 7;
    let node_color = Color::rgb(NODE_COLOR.x, NODE_COLOR.y, NODE_COLOR.z);

    for num in 0..node_count {
        // Calculate
        let coords = n_to_coords(num, node_count);
        let freq = 440.0 * (2.0f32).powf(num as f32 / 12.0);
        println!("{:?}, {:?}", coords, freq);
        nodes.node_vec.push(Note {xy: coords, freq});

        let m = meshes.add(Circle::new(5.0));
        // Render
        commands.spawn(
            (
                Mesh2d(m),
                    MeshMaterial2d(materials.add(node_color)),
                Transform::from_xyz(
                    coords.x,
                    coords.y,
                    0.0,
                )
        )
        );

    }
    commands.spawn(nodes);

}

const DISTANCE_FROM_ORIGIN: f64 = 300.;
fn n_to_coords(n: u32, total: u32) -> Vec2 {
    let angle = 2.0 * (n as f64 /total as f64)* std::f64::consts::PI ;
    Vec2::new(
        round_to_3_decimal_places(DISTANCE_FROM_ORIGIN * angle.cos()),
        round_to_3_decimal_places(DISTANCE_FROM_ORIGIN * angle.sin())
    )
}
fn round_to_3_decimal_places(value: f64) -> f32 {
    ((value * 1000.0).round() / 1000.0) as f32
}

#[derive(Component)]
pub struct MainCamera;
fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, MainCamera));
}
