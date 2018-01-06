use ndarray::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Game {
    size: usize,
    pub world: Array<usize, Ix4>,
}

pub type Position = (usize, usize, usize);

pub trait Life {
    fn new(size: usize) -> Option<Game>;
    fn init(&mut self);
    fn next(&mut self);
    fn get_future_state(&self, x: usize, y: usize, z: usize) -> usize;
    fn neighbours_of(&self, pos: Position) -> Vec<Position>;
    fn get_range(&self, v: usize) -> Vec<usize>;
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

        let game = Game {
            size: size,
            world: Array::<usize, Ix4>::zeros(Ix4(size, size, size, 2)),
        };

        Some(game)
    }

    /// Initialize the cube with eight cells from (0, 0, 0) to (1, 1, 1)
    fn init(&mut self) {
        for x in 0..2 {
            for y in 0..2 {
                for z in 0..2 {
                    let mut cell = self.world.get_mut((x, y, z, 0)).unwrap();
                    *cell = 1;
                }
            }
        }

        let mut cell = self.world.get_mut((2, 2, 2, 0)).unwrap();
        *cell = 1;
    }

    fn next(&mut self) {
        for x in 0..self.size {
            for y in 0..self.size {
                for z in 0..self.size {
                    let mut state = self.get_future_state(x, y, z);
                    let mut future_cell = self.world.get_mut((x, y, z, 1)).unwrap();
                    *future_cell = state;

                    debug!("({},{},{}) future state is {}", x, y, z, future_cell);
                }
            }
        }

        // Update current from future
        for x in 0..self.size {
            for y in 0..self.size {
                for z in 0..self.size {
                    let mut state = self.world.get((x, y, z, 1)).unwrap().clone();
                    let mut past = self.world.get_mut((x, y, z, 0)).unwrap();
                    *past = state;

                    debug!("({},{},{}) new state is {}", x, y, z, past);
                }
            }
        }
    }

    fn get_future_state(&self, x: usize, y: usize, z: usize) -> usize {
        let mut count = 0;

        for n in self.neighbours_of((x, y, z)) {
            if *self.world.get((n.0, n.1, n.2, 0)).unwrap() == 1 {
                debug!("{:?} is alive", n);
                count += 1;
            }
        }

        if count < 2 || count > 4 {
            debug!("({},{},{}) has {} neighbours -> 0", x, y, z, count);
            return 0
        } else {
            debug!("({},{},{}) has {} neighbours -> 1", x, y, z, count);
            return 1
        }
    }

    fn neighbours_of(&self, pos: Position) -> Vec<Position> {
        let mut positions = Vec::<Position>::new();

        for x in self.get_range(pos.0) {
            for y in self.get_range(pos.1) {
                for z in self.get_range(pos.2) {
                    if (x, y, z) != pos && !positions.contains(&(x, y, z)) {
                        positions.push((x, y, z));
                    }
                }
            }
        }

        debug!("{:?} neighbours are: {:?}", pos, positions);
        positions
    }

    fn get_range(&self, v: usize) -> Vec<usize> {
        let mut range = Vec::new();

        if v > 0 { range.push(v - 1) };
        range.push(v);
        if v + 1 < self.size { range.push(v + 1) };

        range
    }
}

#[cfg(test)]
mod tests {
    use super::{Game, Life};

    #[test]
    fn test_it_can_create_game() {
        let game = Game::new(3).unwrap();
        assert!(!game.world.is_empty());

        assert_eq!(None, Game::new(2));
    }

    #[test]
    fn test_it_can_init_game() {
        let mut game = Game::new(3).unwrap();
        game.init();

        assert_eq!(1, *game.world.get((0, 0, 0, 0)).unwrap());
        assert_eq!(1, *game.world.get((0, 0, 1, 0)).unwrap());
        assert_eq!(1, *game.world.get((0, 1, 0, 0)).unwrap());
        assert_eq!(1, *game.world.get((0, 1, 1, 0)).unwrap());
        assert_eq!(1, *game.world.get((1, 0, 0, 0)).unwrap());
        assert_eq!(1, *game.world.get((1, 0, 1, 0)).unwrap());
        assert_eq!(1, *game.world.get((1, 1, 0, 0)).unwrap());
        assert_eq!(1, *game.world.get((1, 1, 1, 0)).unwrap());
        assert_eq!(0, *game.world.get((1, 1, 2, 0)).unwrap());
        assert_eq!(0, *game.world.get((1, 2, 1, 0)).unwrap());
        assert_eq!(0, *game.world.get((1, 2, 2, 0)).unwrap());
        assert_eq!(0, *game.world.get((2, 1, 1, 0)).unwrap());
        assert_eq!(0, *game.world.get((2, 1, 2, 0)).unwrap());
        assert_eq!(0, *game.world.get((2, 2, 1, 0)).unwrap());
        assert_eq!(1, *game.world.get((2, 2, 2, 0)).unwrap());
    }

    #[test]
    fn test_neighbours_of() {
        let mut game = Game::new(3).unwrap();
        game.init();

        assert_eq!(
            vec![           (0, 0, 1),
                 (0, 1, 0), (0, 1, 1),
                 (1, 0, 0), (1, 0, 1),
                 (1, 1, 0), (1, 1, 1)],
            game.neighbours_of((0, 0, 0))
        );
        assert_eq!(
            vec![(0, 0, 0),            (0, 0, 2),
                 (0, 1, 0), (0, 1, 1), (0, 1, 2),
                 (1, 0, 0), (1, 0, 1), (1, 0, 2),
                 (1, 1, 0), (1, 1, 1), (1, 1, 2)],
            game.neighbours_of((0, 0, 1))
        );
        assert_eq!(
            vec![(0, 0, 0), (0, 0, 1), (0, 0, 2),
                 (0, 1, 0), (0, 1, 1), (0, 1, 2),
                 (0, 2, 0), (0, 2, 1), (0, 2, 2),
                 (1, 0, 0), (1, 0, 1), (1, 0, 2),
                 (1, 1, 0),            (1, 1, 2),
                 (1, 2, 0), (1, 2, 1), (1, 2, 2),
                 (2, 0, 0), (2, 0, 1), (2, 0, 2),
                 (2, 1, 0), (2, 1, 1), (2, 1, 2),
                 (2, 2, 0), (2, 2, 1), (2, 2, 2)],
            game.neighbours_of((1, 1, 1))
        );
    }

    #[test]
    fn test_get_range() {
        let mut game = Game::new(3).unwrap();
        game.init();

        assert_eq!(vec![0, 1], game.get_range(0));
        assert_eq!(vec![0, 1, 2], game.get_range(1));
        assert_eq!(vec![1, 2], game.get_range(2));
    }

    #[test]
    fn test_get_future_state() {
        let mut game = Game::new(3).unwrap();
        game.init();

        assert_eq!(0, game.get_future_state(0, 0, 0));
        assert_eq!(0, game.get_future_state(0, 0, 1));
        assert_eq!(0, game.get_future_state(0, 1, 0));
        assert_eq!(0, game.get_future_state(0, 1, 1));
        assert_eq!(0, game.get_future_state(1, 0, 0));
        assert_eq!(0, game.get_future_state(1, 0, 1));
        assert_eq!(0, game.get_future_state(1, 1, 0));
        assert_eq!(0, game.get_future_state(1, 1, 1));
        assert_eq!(0, game.get_future_state(1, 1, 2));
        assert_eq!(0, game.get_future_state(1, 2, 1));
        assert_eq!(1, game.get_future_state(1, 2, 2));
        assert_eq!(0, game.get_future_state(2, 1, 1));
        assert_eq!(1, game.get_future_state(2, 1, 2));
        assert_eq!(1, game.get_future_state(2, 2, 1));
        assert_eq!(0, game.get_future_state(2, 2, 2));
    }

    #[test]
    fn test_it_compute_next_state() {
        let mut game = Game::new(3).unwrap();
        game.init();
        game.next();

        assert_eq!(0, *game.world.get((0, 0, 0, 0)).unwrap());
        assert_eq!(0, *game.world.get((0, 0, 1, 0)).unwrap());
        assert_eq!(0, *game.world.get((0, 1, 0, 0)).unwrap());
        assert_eq!(0, *game.world.get((0, 1, 1, 0)).unwrap());
        assert_eq!(0, *game.world.get((1, 0, 0, 0)).unwrap());
        assert_eq!(0, *game.world.get((1, 0, 1, 0)).unwrap());
        assert_eq!(0, *game.world.get((1, 1, 0, 0)).unwrap());
        assert_eq!(0, *game.world.get((1, 1, 1, 0)).unwrap());
        assert_eq!(0, *game.world.get((1, 1, 2, 0)).unwrap());
        assert_eq!(0, *game.world.get((1, 2, 1, 0)).unwrap());
        assert_eq!(1, *game.world.get((1, 2, 2, 0)).unwrap());
        assert_eq!(0, *game.world.get((2, 1, 1, 0)).unwrap());
        assert_eq!(1, *game.world.get((2, 1, 2, 0)).unwrap());
        assert_eq!(1, *game.world.get((2, 2, 1, 0)).unwrap());
        assert_eq!(0, *game.world.get((2, 2, 2, 0)).unwrap());
    }
}
