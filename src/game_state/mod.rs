pub mod entity;
pub mod initializer;
pub mod physics;
pub mod graphic;
pub mod physics_system;

use nphysics3d;
use nalgebra::{Isometry3, Vector3};
use nalgebra as na;
use ncollide3d::shape::Cuboid;
use ncollide3d::shape::ShapeHandle;
use nphysics3d::object::BodyHandle;
use nphysics3d::object::Material;

use specs;
use specs::DispatcherBuilder;
use specs::Dispatcher;

pub struct GameState {
    pub world: specs::World,
    //pub phys_world: nphysics3d::world::World<f32>,
    //moved to world.res
}

impl GameState {
    pub fn step(&mut self){
        let mut logic_dispatcher = DispatcherBuilder::new().with(physics_system::PhysicsSystem, "physics_system", &[]).build();
        logic_dispatcher.dispatch(&self.world.res);
    }

    pub fn new() -> GameState {
        let mut phys_world = nphysics3d::world::World::new();
        println!("timestep: {}", phys_world.timestep());
        phys_world.set_gravity(Vector3::new(0f32, -9.81f32, 0f32));


        let mut world = specs::World::new();

        //List all Components here:
        world.register::<physics::Physics>();
        world.register::<graphic::Graphic>();

        //set up scene:
        entity::Entity::make_cube(&mut world, &mut phys_world);
        entity::Entity::make_player(&mut world, &mut phys_world);

        entity::Entity::make_floor(&mut world, &mut phys_world);

        world.add_resource(phys_world);



        let gs = GameState {world};
        //gs.add_floor();
        return gs
    }
}