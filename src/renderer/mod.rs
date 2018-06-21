extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use game_state::GameState;

pub struct Renderer {
    window: Window,
    node: SceneNode,
}

impl Renderer {
    pub fn render(& mut self, gs: & GameState) -> bool{
        let e = &gs.entities[0];
        match gs.phys_world.rigid_body(e.handle) {
            Some(body) => self.node.set_local_transformation(body.position()),
            None => println!("no such luck")
        }

        return self.window.render();
    }

    pub fn new() -> Renderer {
        let mut wdw = Window::new("rs-playground");
        let mut c      = wdw.add_cube(1.0, 1.0, 1.0);
        c.set_color(1.0, 0.0, 0.0);

        wdw.set_light(Light::StickToCamera);
        Renderer {window : wdw, node:c}
    }
}