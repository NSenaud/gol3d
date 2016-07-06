use ndarray::prelude::*;

struct Game {
    world: OwnedArray<bool, (Ix, Ix, Ix)>,
}

trait Life {
    fn new(size: usize) -> Option<Game>;
    fn init(&mut self);
}

impl Life for Game {
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

    fn init(&mut self) {
        let mut start = self.world.get_mut((0, 0, 0)).unwrap();

        *start = true;
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

        assert_eq!(&true, game.world.get((0, 0, 0)).unwrap());
    }
}
