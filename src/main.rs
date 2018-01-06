#[macro_use]
extern crate log;
extern crate env_logger;
extern crate kiss3d;
extern crate nalgebra as na;
extern crate ndarray;

mod game;

use std::{thread, time};
use std::sync::mpsc;

use na::{Vector3, Translation};
use kiss3d::window::Window;
use kiss3d::light::Light;

use game::{Game, Life, Position};


struct LivingCells {
    cells: Vec<(Position, kiss3d::scene::SceneNode)>,
}

impl LivingCells {
    fn new() -> LivingCells {
        LivingCells { cells: Vec::new() }
    }

    fn save(&mut self, pos: Position, cube: kiss3d::scene::SceneNode) {
        self.cells.push((pos, cube))
    }

    fn remove(&mut self, index: usize) {
        self.cells.remove(index);
    }

    fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    fn len(&self) -> usize {
        self.cells.len()
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

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        loop {
            game.next();
            thread::sleep(time::Duration::from_millis(500));
            tx.send(game.clone()).unwrap();
        }
    });

    let mut game = Game::new(size).unwrap();
    game.init();

    let mut living = LivingCells::new();

    // TODO Exit when all cells are dead
    while window.render() {
        match rx.try_recv() {
            Ok(g) => game = g,
            Err(_) => (),
        };
        living = render(&mut window, &game, living);
    }
}

fn render<'a>(window: &'a mut Window, game: &Game, mut living: LivingCells) -> LivingCells {
    for x in 0..game.size {
        for y in 0..game.size {
            for z in 0..game.size {
                let alive = game.world.get((x, y, z, 0)).unwrap();

                // Cell is alive
                if alive != &0 {
                    let mut already_alive = false;
                    for cube in &living.cells {
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

                        living.save((x, y, z), c);
                    }
                } else {
                    let mut index = None;
                    for i in 0..living.len() {
                        if living.cells[i].0 == (x, y, z) {
                            index = Some(i);
                            break;
                        }

                    }

                    match index {
                        Some(index) => {
                            window.remove(&mut living.cells[index].1);
                            living.remove(index);
                        },
                        None => (),
                    }
                }
            }
        }
    }

    living
}
