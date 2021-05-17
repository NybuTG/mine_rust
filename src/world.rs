use macroquad::prelude::*;

use rapier3d::{dynamics::{CCDSolver, JointSet, RigidBodySet, IntegrationParameters}, na::Vector3};
use rapier3d::geometry::{BroadPhase, NarrowPhase, ColliderSet};
use rapier3d::pipeline::PhysicsPipeline;
use crate::cube::CubeConstructor;

pub struct World {
    pub pipeline: PhysicsPipeline,
    pub gravity: Vector3<f32>,
    integration_parameters: IntegrationParameters,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    pub body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    joints: JointSet,
    ccd_solver: CCDSolver,
    event_handler: (),
    physics_hooks: (),
    pub objects: Vec<CubeConstructor>
}

impl World {
    pub fn step(&mut self) {
        self.pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.body_set,
            &mut self.collider_set,
            &mut self.joints,
            &mut self.ccd_solver,
            &self.physics_hooks,
            &self.event_handler
        );

        for cube in self.objects.iter() {
            draw_cube(vec3(cube.x, cube.y, cube.z), vec3(cube.width, cube.thickness, cube.height), None, WHITE);
        }
    }
}

impl Default for World {
    fn default() -> Self {
        World {
            pipeline: PhysicsPipeline::new(),
            gravity: Vector3::new(0.0, -9.81, 0.0),
            integration_parameters: IntegrationParameters::default(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            joints: JointSet::new(),
            ccd_solver: CCDSolver::new(),
            event_handler: (),
            physics_hooks: (),
            objects: Vec::new()
        }
    }
}
