
use nphysics3d::object::{Material, BodyHandle, RigidBody};
use nalgebra::{Isometry3, Vector3};
use ncollide3d::shape::{Cuboid, ShapeHandle};
use nphysics3d::world::World;
use nphysics3d::volumetric::Volumetric;
use nalgebra as na;

const COLLIDER_MARGIN: f32 = 0.01;

pub struct Entity {
    pub i : u32,
    pub handle : BodyHandle,
}

impl Entity {
    pub fn step(& mut self, world: &mut World<f32>){
        let result = world.rigid_body_mut(self.handle);
        match result {
            Some(body) => println!("{}", body.position().translation),
            None => println!("no such luck")
        }
    }

    pub fn make_cube(world: &mut World<f32>) -> Entity{
        let geom = ShapeHandle::new(Cuboid::new(Vector3::repeat(1.0f32 - COLLIDER_MARGIN)));
        let inertia = geom.inertia(1.0);
        let center_of_mass = geom.center_of_mass();

        let pos = Isometry3::new(Vector3::new(0.0, 0.0, 0.0), na::zero());
        let handle = world.add_rigid_body(pos, inertia, center_of_mass);
        let e_h = world.add_collider(
            COLLIDER_MARGIN,
            geom.clone(),
            handle,
            Isometry3::identity(),
            Material::default(),
        );
        return Entity{i:0, handle: handle}
    }
}