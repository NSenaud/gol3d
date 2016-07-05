#[macro_use]
extern crate log;
extern crate env_logger;
extern crate kiss3d;
extern crate nalgebra as na;

use log::LogLevel;

use na::Vector3;
use na::Point3;
use kiss3d::window::Window;
use kiss3d::light::Light;

fn main() {
    env_logger::init().unwrap();
    info!("Launching Game of Life 3D…");

    let mut window = Window::new("Game of Life 3D");
    let mut c1     = window.add_cube(1.0, 1.0, 1.0);
    let mut c2     = window.add_cube(1.0, 1.0, 1.0);
    let mut c3     = window.add_cube(1.0, 1.0, 1.0);
    let mut cr     = window.add_sphere(0.5);

    c1.set_color(1.0, 0.0, 0.0);
    c2.set_color(0.0, 1.0, 0.0);
    c3.set_color(0.0, 0.0, 1.0);
    cr.set_color(1.0, 0.0, 1.0);

    let c2move = Vector3 { x:2.0, y:0.0, z:0.0 };
    let c3move = Vector3 { x:0.0, y:2.0, z:0.0 };
    let mut crmove = Vector3 { x:0.0, y:0.0, z:2.0 };
    c2.append_translation(&c2move);
    c3.append_translation(&c3move);
    cr.append_translation(&crmove);

    window.set_light(Light::StickToCamera);

    while window.render() {
        c1.prepend_to_local_rotation(&Vector3::new(0.0f32, 0.014, 0.0));
        c2.prepend_to_local_rotation(&Vector3::new(0.0f32, -0.014, 0.0));
        c3.prepend_to_local_rotation(&Vector3::new(0.0f32, 0.0, 0.014));
    }
}
