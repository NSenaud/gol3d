use ndarray::prelude::*;

struct Game {
    size: usize,
    world: OwnedArray<bool, (Ix, Ix, Ix, Ix)>,
}

trait Life {
    fn new(size: usize) -> Option<Game>;
    fn init(&mut self);
    fn next(&mut self);
}

impl Life for Game {
    /// Create an empty game cube with a given edge `size` (minimum 3).
    ///
    /// The cube is initialized empty (i.e. `false` everywhere) and has a fourth
    /// dimension for the time (used to compute the next turn).
    ///
    /// If the `size` parameter is less than 3, `None` is returned.
    fn new(size: usize) -> Option<Game> {
        if size < 3 { return None; }

        let mut game = Game {
            size: size,
            world: OwnedArray::from_shape_vec(
                       (size, size, size, 2),
                       vec![false; size * size * size * 2]
                   ).unwrap()
        };

        Some(game)
    }

    /// Initialize the cube with nine cells from (0, 0, 0) to (1, 1, 1)
    fn init(&mut self) {
        for x in 0..1 {
            for y in 0..1 {
                for z in 0..1 {
                    let mut cell = self.world.get_mut((x, y, z, 0)).unwrap();
                    *cell = true;
                }
            }
        }
    }

    fn next(&mut self) {

    }
}

#[cfg(test)]
mod tests {
    use super::{ Game, Life };

    #[test]
    fn test_it_can_create_game() {
        let game = Game::new(3).unwrap();

        assert_eq!(&false, game.world.get((0, 0, 0, 0)).unwrap());
    }

    #[test]
    fn test_it_can_init_game() {
        let mut game = Game::new(3).unwrap();
        game.init();

        for x in 0..1 {
            for y in 0..1 {
                for z in 0..1 {
                    assert_eq!(&true, game.world.get((x, y, z, 0)).unwrap());
                    assert_eq!(&false, game.world.get((x, y, z, 1)).unwrap());
                }
            }
        }
    }
}
