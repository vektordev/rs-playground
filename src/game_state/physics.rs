use nphysics3d::object::BodyHandle;
use nphysics3d;
use specs::prelude::*;
use nphysics3d::object;
use nalgebra;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Physics{
    pub handle : BodyHandle,
}

impl Physics{
    pub fn position(&self, phys_world: & nphysics3d::world::World<f32>)-> nalgebra::Isometry3<f32>{
        match self.get_body(phys_world){
            nphysics3d::object::Body::RigidBody(r) => return r.position(),
            nphysics3d::object::Body::Ground(r) => return r.position(),
            other => nalgebra::Isometry3::identity()
        }
    }

    pub fn get_body<'a>(&self, phys_world: &'a nphysics3d::world::World<f32>) -> nphysics3d::object::Body<'a, f32> {
        //really shouldn't panic....
        //match phys_world.body(self.handle){
        //    Some(body) => return body,
        //    None => panic!("Dangling Body Handle found in Physics Component.")
        //}
        return phys_world.body(self.handle)
    }
}
