pub mod entity;
pub mod initializer;


use nphysics3d::world::World;
use nalgebra::core as nac;
use self::nac::{Vector3};

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
        world.set_gravity(Vector3::new(0f32, -0.06f32, 0f32));
        let e = entity::Entity::make_cube(&mut world);
        GameState {entities: vec![e], phys_world: world}
    }
}