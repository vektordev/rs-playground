pub mod entity;
pub mod initializer;


use nphysics3d::world::World;
use nalgebra::{Isometry3, Vector3};
use nalgebra as na;
use ncollide3d::shape::Cuboid;
use ncollide3d::shape::ShapeHandle;
use nphysics3d::object::BodyHandle;
use nphysics3d::object::Material;

pub struct GameState {
    pub entities: Vec<entity::Entity>,
    pub phys_world: World<f32>,
}

impl GameState {
    pub fn step(&mut self){
        self.phys_world.step();
        for ent in self.entities.iter_mut(){
            ent.step(&mut self.phys_world);
        }
    }

    pub fn new() -> GameState {
        let mut world = World::new();
        println!("timestep: {}", world.timestep());
        world.set_gravity(Vector3::new(0f32, -9.81f32, 0f32));
        let e = entity::Entity::make_cube(&mut world);
        let mut gs = GameState {entities: vec![e], phys_world: world};
        gs.add_floor();
        return gs
    }

    pub fn add_floor(&mut self) {
        let ground_size = 50.0;
        let ground_shape =
            ShapeHandle::new(Cuboid::new(Vector3::repeat(ground_size - 0.01)));
        let ground_pos = Isometry3::new(Vector3::y() * (-1.5f32 * ground_size), na::zero());

        self.phys_world.add_collider(
            0.01,
            ground_shape,
            BodyHandle::ground(),
            ground_pos,
            Material::default(),
        );
    }
}