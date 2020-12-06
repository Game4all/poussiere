mod world;

use core::ops::Add;
pub use world::World;

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(i64)]
pub enum Tile {
    Air = 0,
    Sand = 1,
    Stone = 2,
    Water = 3,
    Lava = 4,
    Wall = 5,
    Fire = 6,
    Acid = 7,

    //hack to get the total number of entries in the enum
    _NumTotalTiles,
}

#[derive(Clone, Copy)]
pub struct Position {
    pub x: u64,
    pub y: u64,
}

impl Into<Position> for (u64, u64) {
    fn into(self) -> Position {
        Position {
            x: self.0,
            y: self.1,
        }
    }
}

impl Into<Position> for (i64, i64) {
    fn into(self) -> Position {
        Position {
            x: self.0 as u64,
            y: self.1 as u64,
        }
    }
}

impl Into<Position> for (i32, i32) {
    fn into(self) -> Position {
        Position {
            x: self.0 as u64,
            y: self.1 as u64,
        }
    }
}

impl Add<Position> for Position {
    type Output = Position;
    fn add(self, rhs: Position) -> Self::Output {
        let x: i64 = self.x as i64 + rhs.x as i64;
        let y: i64 = self.y as i64 + rhs.y as i64;
        Position {
            x: x as u64,
            y: y as u64,
        }
    }
}
