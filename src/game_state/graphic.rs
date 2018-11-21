use nphysics3d::object::BodyHandle;
use specs::prelude::*;

#[derive(Component, Hash, Eq, PartialEq)]
#[storage(VecStorage)]
pub struct Graphic{
    pub data: u32,
}
