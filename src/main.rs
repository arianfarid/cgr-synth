use bevy::{
    app::{App}, 
    prelude::*,
};
use rand::Rng;


fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .init_state::<MainState>()
    .add_systems(Startup, (setup_camera, setup))
    .add_systems(Update, add_cgr)
    .run();
}


#[derive(Component)]
pub struct Nodes {
    node_vec: Vec<Note>
}

#[derive(Component)]
pub struct NoteHistory {
    node_vec: Vec<Note>
}
pub struct Note {
    xy: Vec2, freq: f32, //note: String
}
#[derive(Component)]
pub struct Point(Vec2);
#[derive(Component)]
pub struct Node;
pub const NODE_COLOR: Vec3 = Vec3::new(255., 255., 0.);
pub const NODE_COUNT: u32 = 3;
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
    let node_count = NODE_COUNT;
    let node_color = Color::srgba(NODE_COLOR.x, NODE_COLOR.y, NODE_COLOR.z, 0.5);

    for num in 0..node_count {
        // Calculate
        let coords = n_to_coords(num, node_count);
        let freq = 440.0 * (2.0f32).powf(num as f32 / 12.0);
        println!("{:?}, {:?}", coords, freq);
        nodes.node_vec.push(Note {xy: coords, freq});

        let m = meshes.add(Circle::new(2.0));
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

    let notes_history = NoteHistory {
        node_vec: vec![]
    };
    commands.spawn(notes_history);

    //Init state to play


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

/**
 * Logic to run cgr.
 * This works by waiting set intervals, and adding a note to the history.
 * Adding a note will spawn an event, which will be used to play the sequence.
 */
pub const R_VALUE: f32 = 0.5;
pub fn add_cgr(
    mut commands: Commands, 
    mut nodes_query: Query<&Nodes>,
    mut notes_history_query: Query<&mut NoteHistory>,
    mut main_state: Res<State<MainState>>,
    mut main_state_next: ResMut<NextState<MainState>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    match main_state.get() {
        MainState::ReadyToAddNote => {
            let nodes: &Nodes = nodes_query.single_mut();
            let mut notes_history = notes_history_query.single_mut();
            let next_node_i = rand::thread_rng().gen_range(0..NODE_COUNT) as usize;
            println!("{:?}", next_node_i);
            let next_node_pointer = &nodes.node_vec[next_node_i];
            // Spawn initial note
            let mut new_note: Note = Note { xy: Vec2::new(0.,0.), freq: 0. };
            if let Some(last) = notes_history.node_vec.last() {
                new_note.xy = last.xy;
            }
            // move xy to the fraction R_VALUE towards next_node_pointer
            new_note.xy = new_note.xy + R_VALUE * (next_node_pointer.xy - new_note.xy);

            let m = meshes.add(Circle::new(2.0));
            // Render
            commands.spawn(
                (
                    Mesh2d(m),
                        MeshMaterial2d(materials.add(Color::srgba(255., 0.,0., 0.5))),
                    Point(new_note.xy), 
                    PointTimer(Timer::from_seconds(10., TimerMode::Once)),
                    Transform::from_xyz(
                        new_note.xy.x,
                        new_note.xy.y,
                        0.0,
                    )
                )
            );
            notes_history.node_vec.push(new_note);
            // main_state_next.set(MainState::PlayingNote);
        },
        MainState::PlayingNote => {
            // check if timer done then move on to ready to add
        }
    }
}
#[derive(Component, Resource)]
pub struct SequenceTimer(Timer);
#[derive(Component, Resource)]
pub struct PointTimer(Timer);
#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum MainState {
    #[default]
    ReadyToAddNote,
    PlayingNote,
}

/**
 * Camera set up
 */
#[derive(Component)]
pub struct MainCamera;
fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, MainCamera));
}
