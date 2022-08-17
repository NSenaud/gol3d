#[macro_use]
extern crate log;
extern crate env_logger;
extern crate kiss3d;
extern crate nalgebra as na;
extern crate ndarray;

use ndarray::prelude::*;

macro_rules! pos {
    ($tuple:expr) => {
        Position {
            x: $tuple.0,
            y: $tuple.1,
            z: $tuple.2,
        }
    };
}

/// Main game structure.
///
/// size: game board size
/// world: cell ages
/// state: the board is a 4D array, the last dimension is time. The state is
///        weither the index 0 or 1 of this dimension is the current state of
///        the game. The other dimension is used to store the next state.
#[derive(Debug, PartialEq, Clone)]
pub struct Game {
    pub size: usize,
    pub world: Array<usize, Ix4>,
    state: usize,
}

// TODO Use nalgebra::geometry::Point3
/// A Cell position in the game space.
#[derive(Debug, PartialEq, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

/// Public game interface.
pub trait Life {
    fn with_dimension<'a>(size: usize) -> Result<Game, &'a str>;
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
    fn with_dimension<'a>(size: usize) -> Result<Game, &'a str> {
        if size < 3 {
            return Err("Too small board! Should be 3 or more.");
        }

        let game = Game {
            size: size,
            world: Array::<usize, Ix4>::zeros(Ix4(size, size, size, 2)),
            state: 0,
        };

        Ok(game)
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
        let future_state = self.next_state();

        for x in 0..self.size {
            for y in 0..self.size {
                for z in 0..self.size {
                    let mut state = self.get_next_state(&pos!((x, y, z)));
                    let mut future_cell = self.world.get_mut((x, y, z, future_state)).unwrap();
                    *future_cell = state;

                    debug!("({},{},{}) future state is {}", x, y, z, future_cell);
                }
            }
        }

        // Update current_state from future
        self.swap_state()
    }
}

/// Private game interface.
impl Game {
    fn swap_state(&mut self) {
        self.state = if self.state == 0 { 1 } else { 0 }
    }

    fn next_state(&self) -> usize {
        match self.state {
            0 => 1,
            1 => 0,
            _ => panic!("Unknown state!"),
        }
    }

    fn current_state(&self) -> usize {
        self.state
    }

    fn get_next_state(&self, pos: &Position) -> usize {
        if self.will_live(pos) && !self.is_alive(pos) {
            1
        } else if self.will_live(pos) && self.is_alive(pos) {
            self.world
                .get((pos.x, pos.y, pos.z, self.current_state()))
                .unwrap()
                + 1
        } else {
            0
        }
    }

    fn is_alive(&self, pos: &Position) -> bool {
        *self
            .world
            .get((pos.x, pos.y, pos.z, self.current_state()))
            .unwrap()
            > 0
    }

    fn will_live(&self, pos: &Position) -> bool {
        let mut count = 0;

        for n in self.neighbours_of(pos) {
            if *self
                .world
                .get((n.x, n.y, n.z, self.current_state()))
                .unwrap()
                > 0
            {
                debug!("{:?} is alive", n);
                count += 1;
            }
        }

        if count < 2 || count > 4 {
            debug!(
                "({},{},{}) has {} neighbours -> 0",
                pos.x, pos.y, pos.z, count
            );
            return false;
        } else {
            debug!(
                "({},{},{}) has {} neighbours -> 1",
                pos.x, pos.y, pos.z, count
            );
            return true;
        }
    }

    fn neighbours_of(&self, pos: &Position) -> Vec<Position> {
        let mut positions = Vec::<Position>::new();

        for x in self.get_range(pos.x) {
            for y in self.get_range(pos.y) {
                for z in self.get_range(pos.z) {
                    let new_pos = pos!((x, y, z));
                    if new_pos != *pos && !positions.contains(&new_pos) {
                        positions.push(new_pos);
                    }
                }
            }
        }

        debug!("{:?} neighbours are: {:?}", pos, positions);
        positions
    }

    fn get_range(&self, v: usize) -> Vec<usize> {
        let mut range = Vec::new();

        if v > 0 {
            range.push(v - 1)
        };
        range.push(v);
        if v + 1 < self.size {
            range.push(v + 1)
        };

        range
    }
}

#[cfg(test)]
mod tests {
    use super::{Game, Life, Position};

    #[test]
    fn test_it_can_create_game() {
        let game = Game::with_dimension(3).unwrap();
        assert!(!game.world.is_empty());

        assert!(Game::with_dimension(2).is_err());
    }

    #[test]
    fn test_it_can_init_game() {
        let mut game = Game::with_dimension(3).unwrap();
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
        let mut game = Game::with_dimension(3).unwrap();
        game.init();

        assert_eq!(
            vec![
                pos!((0, 0, 1)),
                pos!((0, 1, 0)),
                pos!((0, 1, 1)),
                pos!((1, 0, 0)),
                pos!((1, 0, 1)),
                pos!((1, 1, 0)),
                pos!((1, 1, 1))
            ],
            game.neighbours_of(&pos!((0, 0, 0)))
        );
        assert_eq!(
            vec![
                pos!((0, 0, 0)),
                pos!((0, 0, 2)),
                pos!((0, 1, 0)),
                pos!((0, 1, 1)),
                pos!((0, 1, 2)),
                pos!((1, 0, 0)),
                pos!((1, 0, 1)),
                pos!((1, 0, 2)),
                pos!((1, 1, 0)),
                pos!((1, 1, 1)),
                pos!((1, 1, 2))
            ],
            game.neighbours_of(&pos!((0, 0, 1)))
        );
        assert_eq!(
            vec![
                pos!((0, 0, 0)),
                pos!((0, 0, 1)),
                pos!((0, 0, 2)),
                pos!((0, 1, 0)),
                pos!((0, 1, 1)),
                pos!((0, 1, 2)),
                pos!((0, 2, 0)),
                pos!((0, 2, 1)),
                pos!((0, 2, 2)),
                pos!((1, 0, 0)),
                pos!((1, 0, 1)),
                pos!((1, 0, 2)),
                pos!((1, 1, 0)),
                pos!((1, 1, 2)),
                pos!((1, 2, 0)),
                pos!((1, 2, 1)),
                pos!((1, 2, 2)),
                pos!((2, 0, 0)),
                pos!((2, 0, 1)),
                pos!((2, 0, 2)),
                pos!((2, 1, 0)),
                pos!((2, 1, 1)),
                pos!((2, 1, 2)),
                pos!((2, 2, 0)),
                pos!((2, 2, 1)),
                pos!((2, 2, 2))
            ],
            game.neighbours_of(&pos!((1, 1, 1)))
        );
    }

    #[test]
    fn test_get_range() {
        let mut game = Game::with_dimension(3).unwrap();
        game.init();

        assert_eq!(vec![0, 1], game.get_range(0));
        assert_eq!(vec![0, 1, 2], game.get_range(1));
        assert_eq!(vec![1, 2], game.get_range(2));
    }

    #[test]
    fn test_get_future_state() {
        let mut game = Game::with_dimension(3).unwrap();
        game.init();

        assert_eq!(0, game.get_next_state(&pos!((0, 0, 0))));
        assert_eq!(0, game.get_next_state(&pos!((0, 0, 1))));
        assert_eq!(0, game.get_next_state(&pos!((0, 1, 0))));
        assert_eq!(0, game.get_next_state(&pos!((0, 1, 1))));
        assert_eq!(0, game.get_next_state(&pos!((1, 0, 0))));
        assert_eq!(0, game.get_next_state(&pos!((1, 0, 1))));
        assert_eq!(0, game.get_next_state(&pos!((1, 1, 0))));
        assert_eq!(0, game.get_next_state(&pos!((1, 1, 1))));
        assert_eq!(0, game.get_next_state(&pos!((1, 1, 2))));
        assert_eq!(0, game.get_next_state(&pos!((1, 2, 1))));
        assert_eq!(1, game.get_next_state(&pos!((1, 2, 2))));
        assert_eq!(0, game.get_next_state(&pos!((2, 1, 1))));
        assert_eq!(1, game.get_next_state(&pos!((2, 1, 2))));
        assert_eq!(1, game.get_next_state(&pos!((2, 2, 1))));
        assert_eq!(0, game.get_next_state(&pos!((2, 2, 2))));

        game.next();

        assert_eq!(0, game.get_next_state(&pos!((0, 0, 0))));
        assert_eq!(1, game.get_next_state(&pos!((0, 0, 1))));
        assert_eq!(1, game.get_next_state(&pos!((0, 1, 0))));
        assert_eq!(0, game.get_next_state(&pos!((0, 1, 1))));
        assert_eq!(1, game.get_next_state(&pos!((1, 0, 0))));
        assert_eq!(0, game.get_next_state(&pos!((1, 0, 1))));
        assert_eq!(0, game.get_next_state(&pos!((1, 1, 0))));
        assert_eq!(0, game.get_next_state(&pos!((1, 1, 1))));
        assert_eq!(0, game.get_next_state(&pos!((1, 1, 2))));
        assert_eq!(0, game.get_next_state(&pos!((1, 2, 1))));
        assert_eq!(0, game.get_next_state(&pos!((1, 2, 2))));
        assert_eq!(0, game.get_next_state(&pos!((2, 1, 1))));
        assert_eq!(0, game.get_next_state(&pos!((2, 1, 2))));
        assert_eq!(0, game.get_next_state(&pos!((2, 2, 1))));
        assert_eq!(1, game.get_next_state(&pos!((2, 2, 2))));

        game.next();

        assert_eq!(1, game.get_next_state(&pos!((0, 0, 0))));
        assert_eq!(2, game.get_next_state(&pos!((0, 0, 1))));
        assert_eq!(2, game.get_next_state(&pos!((0, 1, 0))));
        assert_eq!(0, game.get_next_state(&pos!((0, 1, 1))));
        assert_eq!(2, game.get_next_state(&pos!((1, 0, 0))));
        assert_eq!(0, game.get_next_state(&pos!((1, 0, 1))));
        assert_eq!(0, game.get_next_state(&pos!((1, 1, 0))));
        assert_eq!(0, game.get_next_state(&pos!((1, 1, 1))));
        assert_eq!(0, game.get_next_state(&pos!((1, 1, 2))));
        assert_eq!(0, game.get_next_state(&pos!((1, 2, 1))));
        assert_eq!(1, game.get_next_state(&pos!((1, 2, 2))));
        assert_eq!(0, game.get_next_state(&pos!((2, 1, 1))));
        assert_eq!(1, game.get_next_state(&pos!((2, 1, 2))));
        assert_eq!(1, game.get_next_state(&pos!((2, 2, 1))));
        assert_eq!(0, game.get_next_state(&pos!((2, 2, 2))));
    }

    #[test]
    fn test_it_compute_next_state() {
        let mut game = Game::with_dimension(3).unwrap();
        game.init();
        game.next();

        assert_eq!(0, *game.world.get((0, 0, 0, 1)).unwrap());
        assert_eq!(0, *game.world.get((0, 0, 1, 1)).unwrap());
        assert_eq!(0, *game.world.get((0, 1, 0, 1)).unwrap());
        assert_eq!(0, *game.world.get((0, 1, 1, 1)).unwrap());
        assert_eq!(0, *game.world.get((1, 0, 0, 1)).unwrap());
        assert_eq!(0, *game.world.get((1, 0, 1, 1)).unwrap());
        assert_eq!(0, *game.world.get((1, 1, 0, 1)).unwrap());
        assert_eq!(0, *game.world.get((1, 1, 1, 1)).unwrap());
        assert_eq!(0, *game.world.get((1, 1, 2, 1)).unwrap());
        assert_eq!(0, *game.world.get((1, 2, 1, 1)).unwrap());
        assert_eq!(1, *game.world.get((1, 2, 2, 1)).unwrap());
        assert_eq!(0, *game.world.get((2, 1, 1, 1)).unwrap());
        assert_eq!(1, *game.world.get((2, 1, 2, 1)).unwrap());
        assert_eq!(1, *game.world.get((2, 2, 1, 1)).unwrap());
        assert_eq!(0, *game.world.get((2, 2, 2, 1)).unwrap());
    }
}
