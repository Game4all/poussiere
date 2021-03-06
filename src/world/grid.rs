use super::*;
use std::{iter::Enumerate, slice::Iter, vec::Vec};

/// The falling sand simulation grid.
/// (X, Y) coordinates in the grid extend respectively to the right and to the bottom
#[derive(Clone)]
pub struct Grid {
    tiles: Vec<Tile>,
    size: (u64, u64),
}

pub const TILE_AIR: Tile = Tile {
    variant: 0,
    tile_type: TileType::Air,
};

#[derive(Clone, Copy)]
pub struct Tile {
    pub variant: u8,
    pub tile_type: TileType,
}

impl Grid {
    pub fn new(size: (u64, u64)) -> Grid {
        Grid {
            tiles: vec![TILE_AIR; (size.0 * size.1) as usize],
            size,
        }
    }

    pub fn size(&self) -> (u64, u64) {
        self.size
    }

    fn index_of(&self, position: Pos2i) -> Option<usize> {
        if (0..self.size.0 as i64).contains(&position.x)
            && (0..self.size.1 as i64).contains(&position.y)
        {
            Some((position.y * self.size.0 as i64 + position.x) as usize)
        } else {
            None
        }
    }

    pub fn set_tile(&mut self, pos: Pos2i, tile: Tile) {
        if let Some(idx) = self.index_of(pos) {
            self.tiles[idx] = tile;
        }
    }

    pub fn get_tile(&self, pos: Pos2i) -> Option<Tile> {
        self.index_of(pos).map(|idx| self.tiles[idx])
    }

    pub fn iter_tiles(&self) -> WorldIter<'_> {
        WorldIter::from_world(self)
    }

    pub fn snapshot(&self) -> Vec<Tile> {
        self.tiles.clone()
    }

    pub fn restore(&mut self, tiles: Vec<Tile>) {
        self.tiles.copy_from_slice(&tiles);
    }

    pub fn step(&mut self) {
        let mut next_gen = self.clone();

        for (position, tile) in self.iter_tiles() {
            match tile.tile_type {
                TileType::Sand => update_falling_tile(&mut next_gen, position, tile),
                TileType::Dirt => update_falling_tile(&mut next_gen, position, tile),
                TileType::Water => update_water(&mut next_gen, position, tile),
                TileType::Lava => update_lava(&mut next_gen, position, tile),
                TileType::Stone => update_falling_tile(&mut next_gen, position, tile),
                TileType::Acid => update_acid(&mut next_gen, position, tile),
                _ => {}
            }
        }

        self.tiles.copy_from_slice(&next_gen.tiles);
    }

    pub fn clear(&mut self) {
        self.tiles.iter_mut().for_each(|tile| *tile = TILE_AIR);
    }
}

pub struct WorldIter<'a> {
    world: &'a Grid,
    iter: Enumerate<Iter<'a, Tile>>,
}

impl<'a> WorldIter<'a> {
    fn from_world(world: &'a Grid) -> WorldIter<'a> {
        WorldIter {
            world,
            iter: world.tiles.iter().enumerate(),
        }
    }
}

impl<'a> Iterator for WorldIter<'a> {
    type Item = (Pos2i, &'a Tile);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => None,
            Some((idx, tile)) => {
                let pos = (
                    (idx % self.world.size.0 as usize) as i64,
                    (idx / self.world.size.0 as usize) as i64,
                )
                    .into();
                Some((pos, tile))
            }
        }
    }
}
