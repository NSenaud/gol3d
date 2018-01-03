#[macro_use]
extern crate log;
extern crate env_logger;
extern crate kiss3d;
extern crate nalgebra as na;
extern crate ndarray;

mod game;

use std::{thread, time};

use na::{Vector3, Translation};
use kiss3d::window::Window;
use kiss3d::light::Light;

use game::{Game, Life, Position};


struct Drew {
    cubes: Vec<(Position, kiss3d::scene::SceneNode)>,
}

impl Drew {
    fn new() -> Drew {
        Drew { cubes: Vec::new() }
    }

    fn save(&mut self, pos: Position, cube: kiss3d::scene::SceneNode) {
        self.cubes.push((pos, cube))
    }

    fn remove(&mut self, index: usize) {
        self.cubes.remove(index);
    }

    fn is_empty(&self) -> bool {
        self.cubes.is_empty()
    }

    fn len(&self) -> usize {
        self.cubes.len()
    }
}


fn main() {
    env_logger::init().unwrap();
    info!("Launching Game of Life 3D…");

    let size = 30;

    let mut game = Game::new(size).unwrap();
    game.init();

    let mut window = Window::new("Game of Life 3D");
    window.set_light(Light::StickToCamera);

    render(&mut window, game, size);
}

// TODO: Parallelize game and rendering
fn render<'a>(window: &'a mut Window, mut game: Game, size: usize) {
    let mut cubes = Drew::new();

    while window.render() {
        for x in 0..size {
            for y in 0..size {
                for z in 0..size {
                    let alive = game.world.get((x, y, z, 0)).unwrap();

                    // Cell is alive
                    if alive != &0 {
                        let mut already_alive = false;
                        for cube in &cubes.cubes {
                            if cube.0 == (x, y, z) {
                                debug!("Cell already alive");
                                already_alive = true;
                                break;
                            }
                        }

                        if !already_alive {
                            debug!("Draw cell at {}, {}, {}", x, y, z);
                            let mut c = window.add_cube(0.7, 0.7, 0.7);
                            c.set_color(1., 0., 0.);
                            let cmove = Vector3::new(x as f32, y as f32, z as f32);
                            c.append_translation(&Translation {vector: cmove} );

                            cubes.save((x, y, z), c);
                        }
                    } else {
                        let mut index = None;
                        for i in 0..cubes.len() {
                            if cubes.cubes[i].0 == (x, y, z) {
                                index = Some(i);
                                break;
                            }

                        }
                        
                        match index {
                            Some(index) => {
                                window.remove(&mut cubes.cubes[index].1);
                                cubes.remove(index);
                            },
                            None => (),
                        }
                    }
                }
            }
        }

        if cubes.is_empty() {
            info!("This is the end!");
            break;
        }

        game.next();
        thread::sleep(time::Duration::from_millis(16));
    }
}
