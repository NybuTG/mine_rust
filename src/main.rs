use macroquad::prelude::*;

use rapier3d::crossbeam;
use rapier3d::{geometry::ColliderBuilder, na::{Vector3}};
use rapier3d::dynamics::{CCDSolver, JointSet, BodyStatus, RigidBodyBuilder, RigidBodySet, IntegrationParameters};
use rapier3d::geometry::{BroadPhase, NarrowPhase, ColliderSet};
use rapier3d::pipeline::{PhysicsPipeline, ChannelEventCollector};

mod tmp;
mod cube;
fn conf() -> Conf {
    Conf {
        window_title: String::from("Macroquad"),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut p = tmp::player::default();
    p.player_init();

    let mut pipeline = PhysicsPipeline::new();
    let gravity = Vector3::new(0.0, -9.81, 0.0);
    let integration_parameters = IntegrationParameters::default();
    let mut broad_phase = BroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();

    let mut bodies = RigidBodySet::new();
    let mut colliders = ColliderSet::new();

    let mut joints = JointSet::new();
    let mut ccd_solver = CCDSolver::new();

    let (contact_send, contact_recv) = crossbeam::channel::unbounded();
    let (intersection_send, intersection_recv) = crossbeam::channel::unbounded();
    let event_handler = ChannelEventCollector::new(intersection_send, contact_send);
    let physics_hooks = ();

    let rigid_body = RigidBodyBuilder::new(BodyStatus::Dynamic)
        .translation(0., 10., 0.)
        .build();
    
    let collider = ColliderBuilder::cuboid(0.5, 0.5, 0.5)
        // Relative to Rigidbody..?
        .translation(0., 0., 0.)
        .build();

    let parent_handle = bodies.insert(rigid_body);
    colliders.insert(collider, parent_handle, &mut bodies);

    let floor_body = RigidBodyBuilder::new(BodyStatus::Static)
        .translation(0., 0., 0.)
        .build();

    let floor_collider = ColliderBuilder::cuboid(0.5, 0.5, 0.5)
        .build();

    let floor_handle = bodies.insert(floor_body);
    colliders.insert(floor_collider, floor_handle, &mut bodies);


    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        pipeline.step(
            &gravity,
            &integration_parameters,
            &mut broad_phase,
            &mut narrow_phase,
            &mut bodies,
            &mut colliders,
            &mut joints,
            &mut ccd_solver,
            &physics_hooks,
            &event_handler
        );

        p.player_update();

        clear_background(BLACK);
        draw_grid(20, 1.);

        let body = bodies
            .get_mut(parent_handle)
            .unwrap();
        
        let pos = body
            .position()
            .translation
            .vector;

        let floor = bodies
            .get(floor_handle)
            .unwrap();

        let floor_pos = floor
            .position()
            .translation
            .vector;

        draw_cube(vec3(pos.x, pos.y, pos.z), vec3(1., 1., 1.), None, WHITE);

        draw_cube(vec3(floor_pos.x, floor_pos.y, floor_pos.z), vec3(1., 1., 1.), None, WHITE);

        next_frame().await
    }
}
