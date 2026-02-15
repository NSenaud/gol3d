use std::collections::HashMap;
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

fn parse_arg<T: std::str::FromStr>(matches: &clap::ArgMatches, name: &str, default: &str) -> T
where
    T::Err: std::fmt::Display,
{
    matches
        .value_of(name)
        .unwrap_or(default)
        .parse::<T>()
        .unwrap_or_else(|e| panic!("{}\nInvalid {} parameter", e, name.to_lowercase()))
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

    let size: usize = parse_arg(&matches, "SIZE", "25");
    let interval: u64 = parse_arg(&matches, "INTERVAL", "500");
    let width: u32 = parse_arg(&matches, "WIDTH", "1000");
    let height: u32 = parse_arg(&matches, "HEIGHT", "800");

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
    scene
        .add_light(Light::point(from * 2.0).with_intensity(5.0))
        .set_position(Vec3::new(from, from, from));

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
            if tx.send(game.clone()).is_err() {
                break;
            }
        }
    });

    let mut game = Game::with_dimension(size).unwrap();
    game.init();

    let mut living: HashMap<Position, SceneNode3d> = HashMap::new();

    while window.render_3d(&mut scene, &mut camera).await {
        match rx.try_recv() {
            Ok(g) => game = g,
            Err(mpsc::TryRecvError::Empty) => {}
            Err(mpsc::TryRecvError::Disconnected) => {
                log::error!("Game thread disconnected");
                break;
            }
        }
        render(&mut scene, &game, &mut living);
    }
}

fn render(scene: &mut SceneNode3d, game: &Game, living: &mut HashMap<Position, SceneNode3d>) {
    let state = game.current_state();

    for x in 0..game.size {
        for y in 0..game.size {
            for z in 0..game.size {
                let age = *game.world.get((x, y, z, state)).unwrap();
                let pos = Position { x, y, z };

                if age != 0 {
                    if let Some(cell) = living.get_mut(&pos) {
                        log::debug!("Cell already alive");
                        cell.set_color(color_of(age));
                    } else {
                        log::debug!("Draw cell at {}, {}, {}", x, y, z);
                        let mut c = scene.add_cube(0.7, 0.7, 0.7);
                        c.set_color(color_of(age));
                        c.set_position(Vec3::new(x as f32, y as f32, z as f32));
                        living.insert(pos, c);
                    }
                } else if let Some(mut node) = living.remove(&pos) {
                    node.remove();
                }
            }
        }
    }
}

fn color_of(age: usize) -> Color {
    COLORS[age.min(COLORS.len() - 1)]
}
