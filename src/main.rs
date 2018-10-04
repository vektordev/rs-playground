//*
extern crate kiss3d;
extern crate nalgebra;
extern crate ncollide3d;
extern crate nphysics3d;
extern crate rodio;
extern crate glfw;

//use na::{Vector3, UnitQuaternion};
//use kiss3d::window::Window;
//use kiss3d::light::Light;
pub mod renderer;

pub mod game_state;

pub mod core {
    use renderer;
    use game_state;

    use std::time as t_i;
    use std::time as t_d;

    pub struct Core{
        r: renderer::Renderer,
        gs: game_state::GameState,
    }

    impl Core{
        pub fn new() -> Core{
            Core{r : renderer::Renderer::new(), gs : game_state::GameState::new()}
        }

        pub fn run(&mut self){
            let start_t = t_i::Instant::now();
            let mut sim_t = t_i::Instant::now();
            while self.r.render(&self.gs) {
                if sim_t <  t_i::Instant::now() {
                    //simulation rate can be set here by setting the deltaT, must be set in world accordingly
                    let delta_t = t_d::Duration::from_millis(1000) / 60;
                    sim_t += delta_t;
                    self.gs.step();
                    self.r.get_all_input();
                }
            }
        }
    }
}


fn main() {
    //renderer::sound::test();
    let mut c = core::Core::new();
    c.run();

//    let mut window = Window::new("Kiss3d: cube");//
//    let mut c      = window.add_cube(1.0, 1.0, 1.0);//
//
//    c.set_color(1.0, 0.0, 0.0);//
//
//    window.set_light(Light::StickToCamera);//
//
//    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.0004);
//
//    while window.render() {
//        c.prepend_to_local_rotation(&rot);
//    }
}


/*/
extern crate nalgebra as na;
extern crate ncollide;
extern crate nphysics3d;
//extern crate nphysics_testbed3d;

use na::{Point3, Vector3, Translation3};
use ncollide::shape::{Plane, Cuboid, Compound, ShapeHandle};
use nphysics3d::volumetric::Volumetric;
use nphysics3d::world::World;
use nphysics3d::object::RigidBody;
//use nphysics_testbed3d::Testbed;

fn main() {
    let mut world = World::new();
    world.set_gravity(Vector3::new(0.0, -9.81, 0.0));

    let rb = RigidBody::new_static(Plane::new(Vector3::new(0.0, 1.0, 0.0)), 0.3, 0.6);

    world.add_rigid_body(rb);

    let mut cross_geoms = Vec::new();

    let edge_x = Cuboid::new(Vector3::new(4.96f32, 0.21, 0.21));
    let edge_y = Cuboid::new(Vector3::new(0.21f32, 4.96, 0.21));
    let edge_z = Cuboid::new(Vector3::new(0.21f32, 0.21, 4.96));

    cross_geoms.push((na::one(), ShapeHandle::new(edge_x)));
    cross_geoms.push((na::one(), ShapeHandle::new(edge_y)));
    cross_geoms.push((na::one(), ShapeHandle::new(edge_z)));

    let compound = Compound::new(cross_geoms);
    let mass     = compound.mass_properties(1.0);
    let cross    = ShapeHandle::new(compound);

    let num     = 6;
    let rad     = 5.0;
    let shift   = (rad + 0.08) * 2.0;
    let centerx = shift * (num as f32) / 2.0;
    let centery = 30.0 + shift / 2.0;
    let centerz = shift * (num as f32) / 2.0;

    for i in 0usize .. num {
        for j in 0usize .. num {
            for k in 0usize .. num {
                let x = i as f32 * shift - centerx;
                let y = j as f32 * shift + centery;
                let z = k as f32 * shift - centerz;

                let mut rb = RigidBody::new(cross.clone(), Some(mass), 0.3, 0.5);

                rb.append_translation(&Translation3::new(x, y, z));

                world.add_rigid_body(rb);
            }
        }
    }

    let mut testbed = Testbed::new(world);

    testbed.look_at(Point3::new(-30.0, 30.0, -30.0), Point3::new(0.0, 0.0, 0.0));
    testbed.run();
}
*/
