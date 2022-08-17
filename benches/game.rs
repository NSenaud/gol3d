#[macro_use]
extern crate bencher;
extern crate gol3d;

use bencher::Bencher;
use gol3d::{Game, Life};

fn bench_next_dim30_1st_sec(bench: &mut Bencher) {
    let mut game = Game::with_dimension(30).unwrap();
    bench.iter(|| {
        for _ in 0..2 {
            game.next();
        }
    });
}

fn bench_next_dim30_5th_sec(bench: &mut Bencher) {
    // Init game.
    let mut game = Game::with_dimension(30).unwrap();
    // Play the first 4 seconds.
    for _ in 0..8 {
        game.next();
    }

    // Play the fifth second.
    bench.iter(|| {
        for _ in 0..2 {
            game.next();
        }
    });
}

fn bench_next_dim30_30th_sec(bench: &mut Bencher) {
    // Init game.
    let mut game = Game::with_dimension(30).unwrap();
    // Play the first 29 seconds.
    for _ in 0..28 {
        game.next();
    }

    // Play the 30th second.
    bench.iter(|| {
        for _ in 0..2 {
            game.next();
        }
    });
}

benchmark_group!(
    benches,
    bench_next_dim30_1st_sec,
    bench_next_dim30_5th_sec,
    bench_next_dim30_30th_sec
);
benchmark_main!(benches);
