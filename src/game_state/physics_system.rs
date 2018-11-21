use specs;
use specs::Join;

use nphysics3d;

pub struct PhysicsSystem;

impl<'a> specs::System<'a> for PhysicsSystem {

    type SystemData = specs::Write<'a, nphysics3d::world::World<f32>>;

    fn run(&mut self, mut world: Self::SystemData){
        world.step();
    }
}