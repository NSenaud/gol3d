#[macro_use]
extern crate log;
extern crate env_logger;
extern crate kiss3d;
extern crate nalgebra as na;
extern crate ndarray;

use ndarray::prelude::*;


/// Main game structure.
#[derive(Debug, PartialEq, Clone)]
pub struct Game {
    pub size: usize,
    pub world: Array<usize, Ix4>,
    state: usize,
}

/// A Cell position in the game space.
#[derive(Debug, PartialEq, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

/// A Cell structure.
struct Cell {
    position: Position,
    age: usize,
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
        if size < 3 { return Err("Too small board! Should be 3 or more."); }

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
                    let mut state = self.get_future_state(&Position { x:x, y:y, z:z });
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

    fn get_future_state(&self, pos: &Position) -> usize {
        let mut count = 0;

        for n in self.neighbours_of(pos) {
            if *self.world.get((n.x, n.y, n.z, self.current_state())).unwrap() == 1 {
                debug!("{:?} is alive", n);
                count += 1;
            }
        }

        if count < 2 || count > 4 {
            debug!("({},{},{}) has {} neighbours -> 0", pos.x, pos.y, pos.z, count);
            return 0
        } else {
            debug!("({},{},{}) has {} neighbours -> 1", pos.x, pos.y, pos.z, count);
            return 1
        }
    }

    fn neighbours_of(&self, pos: &Position) -> Vec<Position> {
        let mut positions = Vec::<Position>::new();

        for x in self.get_range(pos.x) {
            for y in self.get_range(pos.y) {
                for z in self.get_range(pos.z) {
                    let new_pos = Position { x:x, y:y, z:z };
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

        if v > 0 { range.push(v - 1) };
        range.push(v);
        if v + 1 < self.size { range.push(v + 1) };

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

    // #[test]
    // fn test_neighbours_of() {
    //     let mut game = Game::with_dimension(3).unwrap();
    //     game.init();
    //
    //     assert_eq!(
    //         vec![           (0, 0, 1),
    //              (0, 1, 0), (0, 1, 1),
    //              (1, 0, 0), (1, 0, 1),
    //              (1, 1, 0), (1, 1, 1)],
    //         game.neighbours_of(&Position { x:0, y:0, z:0 })
    //     );
    //     assert_eq!(
    //         vec![(0, 0, 0),            (0, 0, 2),
    //              (0, 1, 0), (0, 1, 1), (0, 1, 2),
    //              (1, 0, 0), (1, 0, 1), (1, 0, 2),
    //              (1, 1, 0), (1, 1, 1), (1, 1, 2)],
    //         game.neighbours_of(&Position { x:0, y:0, z:1 })
    //     );
    //     assert_eq!(
    //         vec![(0, 0, 0), (0, 0, 1), (0, 0, 2),
    //              (0, 1, 0), (0, 1, 1), (0, 1, 2),
    //              (0, 2, 0), (0, 2, 1), (0, 2, 2),
    //              (1, 0, 0), (1, 0, 1), (1, 0, 2),
    //              (1, 1, 0),            (1, 1, 2),
    //              (1, 2, 0), (1, 2, 1), (1, 2, 2),
    //              (2, 0, 0), (2, 0, 1), (2, 0, 2),
    //              (2, 1, 0), (2, 1, 1), (2, 1, 2),
    //              (2, 2, 0), (2, 2, 1), (2, 2, 2)],
    //         game.neighbours_of(&Position { x:1, y:1, z:1 })
    //     );
    // }

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

        assert_eq!(0, game.get_future_state(&Position { x:0, y:0, z:0 }));
        assert_eq!(0, game.get_future_state(&Position { x:0, y:0, z:1 }));
        assert_eq!(0, game.get_future_state(&Position { x:0, y:1, z:0 }));
        assert_eq!(0, game.get_future_state(&Position { x:0, y:1, z:1 }));
        assert_eq!(0, game.get_future_state(&Position { x:1, y:0, z:0 }));
        assert_eq!(0, game.get_future_state(&Position { x:1, y:0, z:1 }));
        assert_eq!(0, game.get_future_state(&Position { x:1, y:1, z:0 }));
        assert_eq!(0, game.get_future_state(&Position { x:1, y:1, z:1 }));
        assert_eq!(0, game.get_future_state(&Position { x:1, y:1, z:2 }));
        assert_eq!(0, game.get_future_state(&Position { x:1, y:2, z:1 }));
        assert_eq!(1, game.get_future_state(&Position { x:1, y:2, z:2 }));
        assert_eq!(0, game.get_future_state(&Position { x:2, y:1, z:1 }));
        assert_eq!(1, game.get_future_state(&Position { x:2, y:1, z:2 }));
        assert_eq!(1, game.get_future_state(&Position { x:2, y:2, z:1 }));
        assert_eq!(0, game.get_future_state(&Position { x:2, y:2, z:2 }));
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
