use super::*;
use std::vec::Vec;

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
            size: size,
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
        let result = match self.index_of(pos) {
            Some(idx) => Some(self.tiles[idx]),
            None => None,
        };

        result
    }

    pub fn get_tiles(&self) -> &[Tile] {
        &self.tiles
    }

    pub fn get_tiles_mut(&mut self) -> &mut [Tile] {
        &mut self.tiles
    }

    pub fn step(&mut self) {
        let mut next_gen = self.clone();

        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                let position = (x, y).into();
                let current_tile = self.get_tile(position).unwrap();
                match current_tile.tile_type {
                    TileType::Sand => update_falling_tile(&mut next_gen, position, current_tile),
                    TileType::Water => update_water(&mut next_gen, position, current_tile),
                    TileType::Lava => update_lava(&mut next_gen, position, current_tile),
                    TileType::Stone => update_falling_tile(&mut next_gen, position, current_tile),
                    TileType::Fire => update_fire(&mut next_gen, position, current_tile),
                    TileType::Acid => update_acid(&mut next_gen, position, current_tile),
                    _ => {}
                }
            }
        }

        self.tiles.copy_from_slice(next_gen.get_tiles());
    }
}
