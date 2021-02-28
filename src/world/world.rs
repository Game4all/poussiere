use super::*;
use std::{iter::Enumerate, slice::Iter, vec::Vec};

#[derive(Clone)]
pub struct World {
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

impl World {
    pub fn new(size: (u64, u64)) -> World {
        World {
            tiles: vec![TILE_AIR; (size.0 * size.1) as usize],
            size,
        }
    }

    pub fn size(&self) -> (u64, u64) {
        self.size
    }

    pub fn resize(&mut self, new_size: (u64, u64)) {
        let old_world = self.clone();

        self.size = new_size;

        self.tiles
            .iter_mut()
            .for_each(|tile_type| *tile_type = TILE_AIR);
        self.tiles
            .resize((new_size.0 * new_size.1) as usize, TILE_AIR);

        let max_x = u64::min(old_world.size.0, self.size.0);
        let max_y = u64::min(old_world.size.1, self.size.1);

        for x in 0..max_x {
            for y in 0..max_y {
                self.set_tile((x, y).into(), old_world.get_tile((x, y).into()).unwrap())
            }
        }
    }

    fn index_of(&self, position: Position) -> Option<usize> {
        if position.x >= self.size.0 || position.y >= self.size.1 {
            return None;
        }

        Some((position.y * self.size.0 + position.x) as usize)
    }

    pub fn set_tile(&mut self, pos: Position, tile: Tile) {
        if let Some(idx) = self.index_of(pos) {
            self.tiles[idx] = tile;
        }
    }

    pub fn get_tile(&self, pos: Position) -> Option<Tile> {
        self.index_of(pos).map(|idx| self.tiles[idx])
    }
    
    pub fn iter_tiles(&self) -> WorldIter<'_> {
        WorldIter::from_world(self)
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
    world: &'a World,
    iter: Enumerate<Iter<'a, Tile>>,
}

impl<'a> WorldIter<'a> {
    fn from_world(world: &'a World) -> WorldIter<'a> {
        WorldIter {
            world,
            iter: world.tiles.iter().enumerate(),
        }
    }
}

impl<'a> Iterator for WorldIter<'a> {
    type Item = (Position, &'a Tile);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => None,
            Some((idx, tile)) => {
                let pos = (
                    (idx % self.world.size.0 as usize) as u64,
                    (idx / self.world.size.0 as usize) as u64,
                )
                    .into();
                Some((pos, tile))
            }
        }
    }
}
