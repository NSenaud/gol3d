#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate clap;
extern crate gol3d;
extern crate kiss3d;
extern crate ndarray;

use std::sync::mpsc;
use std::{thread, time};

use kiss3d::camera::ArcBall;
use kiss3d::light::Light;
use kiss3d::nalgebra::{Point3, Translation, Vector3};
use kiss3d::window::Window;

use gol3d::{Game, Life, Position};

const COLORS: [(f32, f32, f32); 6] = [
    (0., 0., 0.),
    (1., 0., 0.),
    (1., 0.5, 0.),
    (1., 0.7, 0.),
    (1., 0.9, 0.),
    (1., 0.95, 0.),
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

    fn len(&self) -> usize {
        self.cells.len()
    }
}

fn main() {
    env_logger::init();
    info!("Launching Game of Life 3D…");

    let matches = clap_app!(gol3d =>
        // TODO: Get version from Cargo.toml
        (version: "0.1")
        (author: "Nicolas Senaud <nicolas@senaud.fr>")
        (about: "Game of Life 3D")
        (@arg INTERVAL: -i --interval +takes_value "Interval between each turn in ms")
        (@arg SIZE: -s --size +takes_value "Game board size")
        (@arg WIDTH: -w --width +takes_value "Set window width")
        (@arg HEIGHT: -h --height +takes_value "Set window height")
    )
    .get_matches();

    let size = match usize::from_str_radix(matches.value_of("SIZE").unwrap_or("25"), 10) {
        Ok(s) => s,
        Err(e) => panic!("{}\nSize parameter is not valid!", e),
    };
    let interval = match u64::from_str_radix(matches.value_of("INTERVAL").unwrap_or("500"), 10) {
        Ok(i) => i,
        Err(e) => panic!("{}\nInterval parameter is not valid!", e),
    };
    let width = match u32::from_str_radix(matches.value_of("WIDTH").unwrap_or("1000"), 10) {
        Ok(w) => w,
        Err(e) => panic!("{}\nInvalid width parameter", e),
    };
    let height = match u32::from_str_radix(matches.value_of("HEIGHT").unwrap_or("800"), 10) {
        Ok(h) => h,
        Err(e) => panic!("{}\nInvalid height parameter", e),
    };

    info!("Board size: {}\nInterval: {}", size, interval);

    // Init game.
    let mut game = Game::with_dimension(size).unwrap();
    game.init();

    // Init 3d engine.
    let mut window = Window::new_with_size("Game of Life 3D", width, height);
    window.set_light(Light::StickToCamera);

    // Init a custom camera.
    let from = (size * 2) as f32;
    let center = (size / 2) as f32;
    let eye = Point3::new(from, from, from);
    let at = Point3::new(center, center, center);
    let mut camera = ArcBall::new(eye, at);

    // Init threads.
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || loop {
        game.next();
        thread::sleep(time::Duration::from_millis(interval));
        tx.send(game.clone()).unwrap();
    });

    let mut game = Game::with_dimension(size).unwrap();
    game.init();

    let mut living = LivingCells::new();

    // TODO Exit when all cells are dead
    while window.render_with_camera(&mut camera) {
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
                        let cell = &mut cube.1;
                        if *position == (Position { x, y, z }) {
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
                        c.append_translation(&Translation { vector: cmove });

                        living.save(Position { x, y, z }, c);
                    }
                } else {
                    let mut index = None;
                    for i in 0..living.len() {
                        if living.cells[i].0 == (Position { x, y, z }) {
                            index = Some(i);
                            break;
                        }
                    }

                    match index {
                        Some(index) => {
                            window.remove_node(&mut living.cells[index].1);
                            living.remove(index);
                        }
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
