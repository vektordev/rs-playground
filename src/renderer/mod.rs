pub mod sound;

use kiss3d::camera::FirstPerson;
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use game_state::GameState;
use glfw::{Action, WindowEvent};
use nalgebra::Point3;

pub struct Renderer {
    window: Window,
    node: SceneNode,
    cam: FirstPerson,
}

impl Renderer {
    pub fn render(& mut self, gs: & GameState) -> bool{
        let e = &gs.entities[0];
        match gs.phys_world.rigid_body(e.handle) {
            Some(body) => self.node.set_local_transformation(body.position()),
            None => println!("no such luck")
        }

        return self.window.render_with_camera(&mut self.cam);
    }

    pub fn get_all_input(& mut self){
        for mut event in self.window.events().iter() {
            match event.value {
                WindowEvent::Key(code, _, Action::Press, _) => {
                    println!("You pressed the key with code: {:?}", code);
                    println!("Do not try to press escape: the event is inhibited!");
                    //event.inhibited = true // override the default keyboard handler
                },
                WindowEvent::Key(code, _, Action::Release, _) => {
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
                WindowEvent::CursorPos(x, y) => {
                    println!("Cursor pos: ({} , {})", x, y);
                    //event.inhibited = true // dont override the default mouse handler
                },
                WindowEvent::Scroll(xshift, yshift) => {
                    println!("Cursor pos: ({} , {})", xshift, yshift);
                    //event.inhibited = true // dont override the default mouse handler
                },
                _ => { }
            }
        }
    }

    pub fn new() -> Renderer {
        let mut wdw = Window::new("rs-playground");
        let mut c      = wdw.add_cube(1.0, 1.0, 1.0);

        let mut camera = FirstPerson::new(Point3::new(10.0f32, 10.0, 10.0), Point3::origin());
        c.set_color(1.0, 0.0, 0.0);

        //wdw.set_light(Light::StickToCamera);
        Renderer {window : wdw, node:c, cam: camera}
    }
}