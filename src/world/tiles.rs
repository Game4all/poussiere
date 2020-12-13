use super::{world::TILE_AIR, Position, Tile, TileType, World};
use rand::{thread_rng, Rng};

pub fn get_color(tile_type: TileType, variant: u8) -> &'static [u8] {
    match tile_type {
        TileType::Sand => {
            if variant % 2 == 0 {
                &[220, 204, 171, 255]
            } else {
                &[204, 180, 149, 255]
            }
        }
        TileType::Stone => {
            if variant % 2 == 0 {
                &[132, 132, 132, 255]
            } else {
                &[124, 124, 124, 255]
            }
        }
        TileType::Water => &[12, 84, 220, 255],
        TileType::Lava => &[255, 0, 0, 255],
        TileType::Wall => {
            if variant % 2 == 0 {
                &[212, 212, 212, 255]
            } else {
                &[220, 220, 220, 255]
            }
        }
        TileType::Fire => &[238, 88, 34, 255],
        TileType::Acid => &[0, 255, 126, 255],
        _ => &[0, 0, 0, 0],
    }
}

/// Returns a random direction which can be interpreted on the X or Y axis
fn random_direction() -> i64 {
    if thread_rng().gen_bool(0.5) {
        1
    } else {
        -1
    }
}

pub fn update_falling_tile(world: &mut World, position: Position, tile: Tile) {
    let dir = position + (random_direction(), 1).into();

    if let Some(down_tile) = world.get_tile(position + (0, 1).into()) {
        if down_tile.tile_type == TileType::Air || down_tile.tile_type == TileType::Water {
            world.set_tile(position + (0, 1).into(), tile);
            world.set_tile(position, down_tile);
            return;
        }
    }

    if let Some(next_tile) = world.get_tile(dir) {
        if next_tile.tile_type == TileType::Air || next_tile.tile_type == TileType::Water {
            world.set_tile(dir, tile);
            world.set_tile(position, TILE_AIR);
            return;
        }
    }
}

pub fn update_fluid(world: &mut World, position: Position, tile: Tile) {
    if let Some(down_tile) = world.get_tile(position + (0, 1).into()) {
        if down_tile.tile_type == TileType::Air {
            world.set_tile(position + (0, 1).into(), tile);
            world.set_tile(position, TILE_AIR);
            return;
        }
    }

    let direction = random_direction();
    let diag_pos = (direction, 1).into();

    if let Some(next_diag_tile) = world.get_tile(position + diag_pos) {
        if next_diag_tile.tile_type == TileType::Air {
            world.set_tile(position + diag_pos, tile);
            world.set_tile(position, TILE_AIR);
            return;
        }
    }

    let next_pos = (direction, 0).into();

    if let Some(next_tile) = world.get_tile(position + next_pos) {
        if next_tile.tile_type == TileType::Air {
            world.set_tile(position + next_pos, tile);
            world.set_tile(position, TILE_AIR);
        }
    }
}

pub fn update_acid(world: &mut World, position: Position, tile: Tile) {
    if let Some(right_tile) = world.get_tile(position + (1, 0).into()) {
        if right_tile.tile_type != TileType::Air
            && right_tile.tile_type != TileType::Acid
            && right_tile.tile_type != TileType::Wall
        {
            world.set_tile(position + (1, 0).into(), TILE_AIR);
            world.set_tile(position, TILE_AIR);
            return;
        }
    }

    if let Some(left_tile) = world.get_tile(position + (-1, 0).into()) {
        if left_tile.tile_type != TileType::Air
            && left_tile.tile_type != TileType::Acid
            && left_tile.tile_type != TileType::Wall
        {
            world.set_tile(position + (-1, 0).into(), TILE_AIR);
            world.set_tile(position, TILE_AIR);
            return;
        }
    }

    if let Some(up_tile) = world.get_tile(position + (0, 1).into()) {
        if up_tile.tile_type != TileType::Air
            && up_tile.tile_type != TileType::Acid
            && up_tile.tile_type != TileType::Wall
        {
            world.set_tile(position + (0, 1).into(), TILE_AIR);
            world.set_tile(position, TILE_AIR);
            return;
        }
    }

    if let Some(down_tile) = world.get_tile(position + (0, -1).into()) {
        if down_tile.tile_type != TileType::Air
            && down_tile.tile_type != TileType::Acid
            && down_tile.tile_type != TileType::Wall
        {
            world.set_tile(position + (0, -1).into(), TILE_AIR);
            world.set_tile(position, TILE_AIR);
            return;
        }
    }

    update_fluid(world, position, tile);
}

pub fn update_fire(world: &mut World, position: Position, tile: Tile) {
    let dir = position + (random_direction(), 1).into();

    if let Some(down_tile) = world.get_tile(dir) {
        if down_tile.tile_type == TileType::Air {
            world.set_tile(dir, tile);
            world.set_tile(position, down_tile);
            return;
        } else if down_tile.tile_type != TileType::Fire {
            world.set_tile(position, TILE_AIR);
        }
    } else {
        world.set_tile(position, TILE_AIR);
    }
}

pub fn update_water(world: &mut World, position: Position, tile: Tile) {
    if let Some(water_tile) = neigbour_of_type(world, position, TileType::Lava) {
        world.set_tile(
            position,
            Tile {
                variant: tile.variant,
                tile_type: TileType::Stone,
            },
        );
        world.set_tile(
            position + water_tile,
            Tile {
                variant: tile.variant,
                tile_type: TileType::Stone,
            },
        );
        return;
    }

    update_fluid(world, position, tile);
}

pub fn update_lava(world: &mut World, position: Position, tile: Tile) {
    if let Some(water_tile) = neigbour_of_type(world, position, TileType::Water) {
        world.set_tile(
            position,
            Tile {
                variant: tile.variant,
                tile_type: TileType::Stone,
            },
        );
        world.set_tile(
            position + water_tile,
            Tile {
                variant: tile.variant,
                tile_type: TileType::Stone,
            },
        );
        return;
    }

    update_fluid(world, position, tile);
}

fn neigbour_of_type(world: &mut World, pos: Position, tile_type: TileType) -> Option<Position> {
    if let Some(right_tile) = world.get_tile(pos + (1, 0).into()) {
        if right_tile.tile_type == tile_type {
            return Some((1, 0).into());
        }
    }

    if let Some(left_tile) = world.get_tile(pos + (-1, 0).into()) {
        if left_tile.tile_type == tile_type {
            return Some((-1, 0).into());
        }
    }

    if let Some(up_tile) = world.get_tile(pos + (0, 1).into()) {
        if up_tile.tile_type == tile_type {
            return Some((0, 1).into());
        }
    }

    if let Some(down_tile) = world.get_tile(pos + (0, -1).into()) {
        if down_tile.tile_type == tile_type {
            return Some((1, 0).into());
        }
    }

    None
}
