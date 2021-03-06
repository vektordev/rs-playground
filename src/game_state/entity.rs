
use nphysics3d::object::{Material, BodyHandle, RigidBody};
use nalgebra::{Isometry3, Vector3};
use ncollide3d::shape::{Cuboid, ShapeHandle};
use nphysics3d::world::World;
use nphysics3d::volumetric::Volumetric;
use nalgebra as na;
use specs;
use specs::Builder;
use game_state::physics;
use game_state::graphic;

const COLLIDER_MARGIN: f32 = 0.01;

#[derive(PartialEq, Eq, Hash)]
pub struct Entity {
    pub i : u32,
    pub handle : BodyHandle,
}

impl Entity {
    pub fn step(& mut self, world: &mut World<f32>){
        //let result = world.rigid_body_mut(self.handle);
        //match result {
        //    Some(body) => println!("{}", body.position().translation),
        //    None => println!("no such luck")
        //}
    }

    pub fn make_cube(world: &mut specs::World, phys_world: &mut World<f32>){
        let geom = ShapeHandle::new(Cuboid::new(Vector3::repeat(1.0f32 - COLLIDER_MARGIN)));
        let inertia = geom.inertia(1.0);
        let center_of_mass = geom.center_of_mass();

        let pos = Isometry3::new(Vector3::new(0.0, 0.0, 0.0), na::zero());
        let handle = phys_world.add_rigid_body(pos, inertia, center_of_mass);
        let e_h = phys_world.add_collider(
            COLLIDER_MARGIN,
            geom.clone(),
            handle,
            Isometry3::identity(),
            Material::default(),
        );
        world.create_entity().with(physics::Physics{handle}).with(graphic::Graphic{data:0}).build();
    }

    pub fn make_player(world: &mut specs::World, phys_world: &mut World<f32>){
        let geom = ShapeHandle::new(Cuboid::new(Vector3::repeat(1.0f32 - COLLIDER_MARGIN)));
        let inertia = geom.inertia(1.0);
        let center_of_mass = geom.center_of_mass();

        let pos = Isometry3::new(Vector3::new(1.0, 0.0, 0.0), na::zero());
        let handle = phys_world.add_rigid_body(pos, inertia, center_of_mass);
        let e_h = phys_world.add_collider(
            COLLIDER_MARGIN,
            geom.clone(),
            handle,
            Isometry3::identity(),
            Material::default(),
        );
        world.create_entity().with(physics::Physics{handle}).with(graphic::Graphic{data:0}).build();
    }

    pub fn make_floor(world: &mut specs::World, phys_world: &mut World<f32>){
        let ground_size = 50.0;
        let ground_shape =
            ShapeHandle::new(Cuboid::new(Vector3::repeat(ground_size - 0.01)));
        let ground_pos = Isometry3::new(Vector3::y() * (-1.5f32 * ground_size), na::zero());
        let handle = BodyHandle::ground();
        phys_world.add_collider(
            0.01,
            ground_shape,
            handle,
            ground_pos,
            Material::default(),
        );
        world.create_entity().with(physics::Physics{handle}).with(graphic::Graphic{data:0}).build();
    }
}