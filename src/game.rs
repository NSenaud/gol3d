use ndarray::prelude::*;

struct Game {
    world: OwnedArray<bool, (Ix, Ix, Ix)>,
}

trait Life {
    fn new(size: usize) -> Game;
}

impl Life for Game {
    fn new(size: usize) -> Game {
        let game = Game {
            world: OwnedArray::from_shape_vec(
                       (size, size, size),
                       vec![false; size * size * size]
                   ).unwrap()
        };

        game
    }
}

#[cfg(test)]
mod tests {
    use super::{ Game, Life };

    #[test]
    fn test_it_can_create_game() {
        let game = Game::new(3);

        assert_eq!(&false, game.world.get((0, 0, 0)).unwrap());
    }
}
