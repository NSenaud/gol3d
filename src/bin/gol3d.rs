use std::sync::mpsc;
use std::{thread, time};

use clap::clap_app;
use kiss3d::light::Light;
use kiss3d::prelude::*;

use gol3d::{Game, Life, Position};

const COLORS: [Color; 6] = [
    Color::new(0., 0., 0., 1.),
    Color::new(1., 0., 0., 1.),
    Color::new(1., 0.5, 0., 1.),
    Color::new(1., 0.7, 0., 1.),
    Color::new(1., 0.9, 0., 1.),
    Color::new(1., 0.95, 0., 1.),
];

struct LivingCells {
    cells: Vec<(Position, SceneNode3d)>,
}

impl LivingCells {
    fn new() -> LivingCells {
        LivingCells { cells: Vec::new() }
    }

    fn save(&mut self, pos: Position, cube: SceneNode3d) {
        self.cells.push((pos, cube))
    }

    fn remove(&mut self, index: usize) {
        self.cells.remove(index);
    }

    fn len(&self) -> usize {
        self.cells.len()
    }
}

#[kiss3d::main]
async fn main() {
    env_logger::init();
    log::info!("Launching Game of Life 3D…");

    let matches = clap_app!(gol3d =>
        (version: "0.1")
        (author: "Nicolas Senaud <nicolas@senaud.fr>")
        (about: "Game of Life 3D")
        (@arg INTERVAL: -i --interval +takes_value "Interval between each turn in ms")
        (@arg SIZE: -s --size +takes_value "Game board size")
        (@arg WIDTH: -w --width +takes_value "Set window width")
        (@arg HEIGHT: -h --height +takes_value "Set window height")
    )
    .get_matches();

    let size = match matches.value_of("SIZE").unwrap_or("25").parse::<usize>() {
        Ok(s) => s,
        Err(e) => panic!("{}\nSize parameter is not valid!", e),
    };
    let interval = match matches.value_of("INTERVAL").unwrap_or("500").parse::<u64>() {
        Ok(i) => i,
        Err(e) => panic!("{}\nInterval parameter is not valid!", e),
    };
    let width = match matches.value_of("WIDTH").unwrap_or("1000").parse::<u32>() {
        Ok(w) => w,
        Err(e) => panic!("{}\nInvalid width parameter", e),
    };
    let height = match matches.value_of("HEIGHT").unwrap_or("800").parse::<u32>() {
        Ok(h) => h,
        Err(e) => panic!("{}\nInvalid height parameter", e),
    };

    log::info!("Board size: {}\nInterval: {}", size, interval);

    // Init game.
    let mut game = Game::with_dimension(size).unwrap();
    game.init();

    // Init 3d engine.
    let mut window = Window::new_with_size("Game of Life 3D", width, height).await;

    // Init scene with a light.
    let mut scene = SceneNode3d::empty();
    let from = (size * 2) as f32;
    let center = (size / 2) as f32;
    let mut light_node = SceneNode3d::new_light(Light::point(from * 2.0).with_intensity(5.0));
    light_node.set_position(Vec3::new(from, from, from));
    scene.add_child(light_node);

    // Init a custom camera.
    let eye = Vec3::new(from, from, from);
    let at = Vec3::new(center, center, center);
    let mut camera = OrbitCamera3d::new(eye, at);

    // Init threads.
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

    while window.render_3d(&mut scene, &mut camera).await {
        if let Ok(g) = rx.try_recv() {
            game = g
        };
        living = render(&mut scene, &game, living);
    }
}

fn render(scene: &mut SceneNode3d, game: &Game, mut living: LivingCells) -> LivingCells {
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
                            log::debug!("Cell already alive");
                            cell.set_color(color_of(age));
                            already_alive = true;
                            break;
                        }
                    }

                    if !already_alive {
                        log::debug!("Draw cell at {}, {}, {}", x, y, z);
                        let mut c = scene.add_cube(0.7, 0.7, 0.7);
                        c.set_color(color_of(age));
                        c.set_position(Vec3::new(x as f32, y as f32, z as f32));

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

                    if let Some(index) = index {
                        living.cells[index].1.remove();
                        living.remove(index);
                    }
                }
            }
        }
    }

    living
}

fn color_of(age: &usize) -> Color {
    if *age >= COLORS.len() {
        COLORS[COLORS.len() - 1]
    } else {
        COLORS[*age]
    }
}
