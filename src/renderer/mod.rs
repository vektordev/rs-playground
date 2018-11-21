pub mod sound;

use kiss3d::camera::FirstPerson;
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::event::{Action, WindowEvent};
use nalgebra::Point3;

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::hash::Hash;

use specs;
use specs::Join;

use game_state::GameState;
use game_state::physics;
use game_state::graphic;
use game_state::entity::Entity;
use game_state::graphic::Graphic;

use nphysics3d::object::BodyHandle;
use nphysics3d;

#[derive(Debug)]
#[derive(Default)]
pub struct RenderInfo{
    pub keep_open: bool,
}

#[derive(Hash, Eq, PartialEq)]
pub struct GfxPtr<'a>(&'a Graphic);

pub struct Renderer{
    window: Window,
    cam: FirstPerson,
    node_map: HashMap<u64, SceneNode>,
}

impl<'a> specs::System<'a> for Renderer {
    type SystemData = (specs::ReadStorage<'a, graphic::Graphic>,
                       specs::ReadStorage<'a, physics::Physics>,
                       specs::Read<'a, nphysics3d::world::World<f32>>,
                       specs::Write<'a, RenderInfo>);

    fn run(&mut self, (gfx, phys, phys_world, mut r_info): Self::SystemData) {
        for (gfx, phys) in (&gfx, &phys).join() {
            println!("elem");
            let transform = phys.position(&phys_world);
            self.find_scene_node(gfx).set_local_transformation(transform);
        }
        r_info.keep_open = self.window.render_with_camera(&mut self.cam);
        println!("{:?}", self.node_map.keys());
    }
}

impl Renderer {
    pub fn find_scene_node(& mut self, gfx: &graphic::Graphic) -> &mut SceneNode{
        let wref = &mut self.window;
        let noderef = self.node_map
            .entry(Renderer::get_hash(gfx))
            .or_insert_with(|| {Renderer::make_scene_node(wref, gfx)});

        return noderef;
    }

    pub fn make_scene_node(wdw: &mut Window, gfx: &graphic::Graphic) -> SceneNode{
        let mut c = wdw.add_cube(1.0, 1.0, 1.0);
        c.set_color(1.0, 0.0, 0.0);
        return c;
    }

    pub fn get_hash(gfx: &graphic::Graphic) -> u64{
        let mut hasher = DefaultHasher::new();
        let gfx_ptr: *const graphic::Graphic = gfx;
        gfx_ptr.hash(&mut hasher);
        return hasher.finish()
    }
    //pub fn render(& mut self, gs: & GameState) -> bool{
//        for e in &gs.entities{
//            if !self.nodeMap.contains_key(e){
//                self.nodeMap.insert(e, self.window.add_cube(1.0, 1.0, 1.0));
//            }
//            match self.nodeMap.get(e){
//                Some(node) => {
//                    match gs.phys_world.rigid_body(e.handle) {
//                        Some(body) => self.node.set_local_transformation(body.position()),
//                        None => println!("Error in Renderer. No transformation found.")
//                    }
//                }
//                None => println!("Error in Renderer. No Node found.")
//            }
//        }

    //}

    pub fn get_all_input(& mut self){
        for mut event in self.window.events().iter() {
            match event.value {
                WindowEvent::Key(code, Action::Press, _) => {
                    println!("You pressed the key with code: {:?}", code);
                    println!("Do not try to press escape: the event is inhibited!");
                    //event.inhibited = true // override the default keyboard handler
                },
                WindowEvent::Key(code, Action::Release, _) => {
                    println!("You released the key with code: {:?}", code);
                    println!("Do not try to press escape: the event is inhibited!");
                    //event.inhibited = true // override the default keyboard handler
                },
                WindowEvent::MouseButton(button, Action::Press, mods) => {
                    println!("You pressed the mouse button with code: {:?}", button);
                    println!("You pressed the mouse button with modifiers: {:?}", mods);
                    //event.inhibited = true // dont override the default mouse handler
                },
                WindowEvent::MouseButton(button, Action::Release, mods) => {
                    println!("You released the mouse button with code: {:?}", button);
                    println!("You released the mouse button with modifiers: {:?}", mods);
                    //event.inhibited = true // dont override the default mouse handler
                },
                WindowEvent::CursorPos(x, y, _) => {
                    println!("Cursor pos: ({} , {})", x, y);
                    //event.inhibited = true // dont override the default mouse handler
                },
                WindowEvent::Scroll(xshift, yshift, _) => {
                    println!("Cursor pos: ({} , {})", xshift, yshift);
                    //event.inhibited = true // dont override the default mouse handler
                },
                _ => { }
            }
        }
    }

    pub fn new() -> Renderer {
        let mut wdw = Window::new("rs-playground");

        let mut camera = FirstPerson::new(Point3::new(10.0f32, 10.0, 10.0), Point3::origin());

        //wdw.set_light(Light::StickToCamera);
        Renderer {window : wdw, cam: camera, node_map: HashMap::new()}
    }
}