use assets_manager::Handle;
use dirs;
// use crate::chunk::{Chunk, CubeArray};
use crate::cube::Cube;
// use cube::Cube;
// use macroquad::prelude::*;
//
// use rapier3d::crossbeam;
// use rapier3d::{geometry::ColliderBuilder, na::{Vector3}};
// use rapier3d::dynamics::{CCDSolver, JointSet, BodyStatus, RigidBodyBuilder, RigidBodySet, IntegrationParameters};
// use rapier3d::geometry::{BroadPhase, NarrowPhase, ColliderSet};
// use rapier3d::pipeline::{PhysicsPipeline, ChannelEventCollector};
// use world::World;
//
// mod tmp;
// mod cube;
// mod world;
// mod savefile;
//
// fn conf() -> Conf {
//     Conf {
//         window_title: String::from("Macroquad"),
//         fullscreen: true,
//         ..Default::default()
//     }
// }
//
// #[macroquad::main(conf)]
// async fn main() {
//     let mut p = tmp::player::default();
//     p.player_init();
//
//     let mut world = World::default();
//     let mut cube = cube::Cube {
//         pos_x: 0.,
//         pos_y: 0.,
//         pos_z: 0.,
//         pipeline: world.pipeline,
//         bodyset: world.body_set,
//         colliderset: world.collider_set,
//     };
//
//     let c = cube.new();
//
//     world.objects.push(c);
//
//     loop {
//
//         world.step();
//
//         if is_key_pressed(KeyCode::Escape) {
//             break;
//         }
//
//
//         p.player_update();
//         clear_background(BLACK);
//         next_frame().await
//     }
// }
mod chunk;
mod cube;
mod error;
mod savefile;

fn main() {
    let cfg_dir = dirs::config_dir().unwrap();
    std::fs::create_dir(format!("{}/minerust", cfg_dir.to_str().unwrap())).expect("Couldnt create directory");

    // let cubes = Cube::load_all_defs().unwrap();
    // let grass: Handle<Cube> = cubes.load("grass").expect("Something went fucky wucky");
    // dbg!(grass);

}
