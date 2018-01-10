#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate clap;
extern crate kiss3d;
extern crate nalgebra as na;
extern crate ndarray;
extern crate ndarray_parallel;
extern crate gol3d;

use std::{thread, time};
use std::sync::mpsc;

use na::{Vector3, Translation};
use kiss3d::window::Window;
use kiss3d::light::Light;

use gol3d::{Game, Life, Position};


const COLORS: [(f32, f32, f32); 6] = [
    (0., 0., 0.), (1., 0., 0.), (1., 0.5, 0.), (1., 0.7, 0.), (1., 0.9, 0.), (1., 0.95, 0.)
];


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

    let matches = clap_app!(gol3d =>
        // TODO: Get version from Cargo.toml
        (version: "0.1")
        (author: "Nicolas Senaud <nicolas@senaud.fr>")
        (about: "Game of Life 3D")
        (@arg INTERVAL: -i --interval +takes_value "Interval between each turn in ms")
        (@arg SIZE: -s --size +takes_value "Game board size")
    ).get_matches();

    let size = match usize::from_str_radix(matches.value_of("SIZE")
                                                  .unwrap_or("25"), 10) {
        Ok(s) => s,
        Err(e) => panic!("{}\nSize parameter is not valid!", e),
    };
    let interval = match u64::from_str_radix(matches.value_of("INTERVAL")
                                                    .unwrap_or("500"), 10) {
        Ok(s) => s,
        Err(e) => panic!("{}\nInterval parameter is not valid!", e),
    };

    info!("Board size: {}\nInterval: {}", size, interval);

    let mut game = Game::with_dimension(size).unwrap();
    game.init();

    let mut window = Window::new("Game of Life 3D");
    window.set_light(Light::StickToCamera);

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        loop {
            game.next();
            thread::sleep(time::Duration::from_millis(interval));
            tx.send(game.clone()).unwrap();
        }
    });

    let mut game = Game::with_dimension(size).unwrap();
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
                let age = game.world.get((x, y, z, 0)).unwrap();

                // Cell is alive
                if age != &0 {
                    let mut already_alive = false;
                    for cube in &mut living.cells {
                        let position = &cube.0;
                        let mut cell = &mut cube.1;
                        if *position == (Position { x:x, y:y, z:z }) {
                            debug!("Cell already alive");
                            cell.set_color(color_of(age).0, color_of(age).1, color_of(age).2);
                            already_alive = true;
                            break;
                        }
                    }

                    if !already_alive {
                        debug!("Draw cell at {}, {}, {}", x, y, z);
                        let mut c = window.add_cube(0.7, 0.7, 0.7);
                        c.set_color(color_of(age).0, color_of(age).1, color_of(age).2);
                        let cmove = Vector3::new(x as f32, y as f32, z as f32);
                        c.append_translation(&Translation {vector: cmove} );

                        living.save(Position { x:x, y:y, z:z }, c);
                    }
                } else {
                    let mut index = None;
                    for i in 0..living.len() {
                        if living.cells[i].0 == (Position { x:x, y:y, z:z }) {
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

fn color_of(age: &usize) -> (f32, f32, f32) {
    if *age >= COLORS.len() {
        COLORS[COLORS.len() - 1]
    } else {
        COLORS[*age]
    }
}
