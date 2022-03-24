extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::window::{State, Window};
use na::{UnitQuaternion, Vector3};
use std::env;
use std::path::Path;

struct AppState {
    c: SceneNode,
    rot: UnitQuaternion<f32>,
}

impl State for AppState {
    fn step(&mut self, _: &mut Window) {
        self.c.prepend_to_local_rotation(&self.rot)
    }
}

fn main() {
    let path: Vec<_> = env::args().collect();
    if path.len() > 2 || path.len() == 1 {
        println!("usage: renderizer3d /path/to/.obj");
        std::process::exit(0)
    } else {
    }
    match &Path::new(&path[1]).exists() {
        false => {
            println!("No such file directory.");
            std::process::exit(1)
        }
        _ => (),
    }
    let mut window = Window::new("renderizer3d");
    let c = window.add_obj(
        &Path::new(&path[1]),
        &Path::new(&path[1]),
        Vector3::new(1.0, 1.0, 1.0),
    );

    window.set_light(Light::StickToCamera);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);
    let state = AppState { c, rot };

    window.render_loop(state)
}
