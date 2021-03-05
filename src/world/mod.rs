mod grid;
mod tiles;

use core::ops::Add;
use strum::{EnumIter, IntoStaticStr};

pub use grid::*;
pub use tiles::*;

#[derive(Clone, Copy, PartialEq, Debug, EnumIter, IntoStaticStr)]
#[repr(u8)]
pub enum TileType {
    Air,
    Sand,
    Dirt,
    Stone,
    Water,
    Lava,
    Wall,
    Acid,
}

impl Default for TileType {
    fn default() -> Self {
        TileType::Sand
    }
}

/// A struct representing coordinates of a tile in the simulation grid.
#[derive(Clone, Copy)]
pub struct Pos2i {
    pub x: i64,
    pub y: i64,
}

impl From<(i64, i64)> for Pos2i {
    fn from(pos: (i64, i64)) -> Self {
        Pos2i { x: pos.0, y: pos.1 }
    }
}

impl From<(i32, i32)> for Pos2i {
    fn from(pos: (i32, i32)) -> Self {
        Pos2i {
            x: pos.0 as i64,
            y: pos.1 as i64,
        }
    }
}

impl Add<Pos2i> for Pos2i {
    type Output = Pos2i;
    fn add(self, rhs: Pos2i) -> Self::Output {
        Pos2i {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
