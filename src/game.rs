use ndarray::prelude::*;

struct Game {
    world: OwnedArray<bool, (Ix, Ix, Ix)>,
}

trait Life {
    fn new(size: usize) -> Option<Game>;
    fn init(&mut self);
}

impl Life for Game {
    /// Create an empty game cube with a given edge `size` (minimum 3).
    ///
    /// The cube is initialized empty (i.e. `false` everywhere).
    ///
    /// If the `size` parameter is less than 3, `None` is returned.
    fn new(size: usize) -> Option<Game> {
        if size < 3 { return None; }

        let mut game = Game {
            world: OwnedArray::from_shape_vec(
                       (size, size, size),
                       vec![false; size * size * size]
                   ).unwrap()
        };

        Some(game)
    }

    /// Initialize the cube with nine cells from (0, 0, 0) to (1, 1, 1)
    fn init(&mut self) {
        for x in 0..1 {
            for y in 0..1 {
                for z in 0..1 {
                    let mut cell = self.world.get_mut((x, y, z)).unwrap();
                    *cell = true;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ Game, Life };

    #[test]
    fn test_it_can_create_game() {
        let game = Game::new(3).unwrap();

        assert_eq!(&false, game.world.get((0, 0, 0)).unwrap());
    }

    #[test]
    fn test_it_can_init_game() {
        let mut game = Game::new(3).unwrap();
        game.init();

        for x in 0..1 {
            for y in 0..1 {
                for z in 0..1 {
                    assert_eq!(&true, game.world.get((x, y, z)).unwrap());
                }
            }
        }
    }
}
