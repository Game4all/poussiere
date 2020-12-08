use super::{Position, Tile, World};
use rand::{thread_rng, Rng};

pub fn get_color(tile: Tile) -> &'static [u8] {
    match tile {
        Tile::Sand => &[194, 178, 128, 255],
        Tile::Stone => &[12, 12, 12, 255],
        Tile::Water => &[3, 78, 162, 255],
        Tile::Lava => &[255, 0, 0, 255],
        Tile::Wall => &[100, 100, 100, 255],
        Tile::Fire => &[238, 88, 34, 255],
        Tile::Acid => &[0, 255, 126, 255],
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
        if down_tile == Tile::Air || down_tile == Tile::Water {
            world.set_tile(position + (0, 1).into(), tile);
            world.set_tile(position, down_tile);
            return;
        }
    }

    if let Some(next_tile) = world.get_tile(dir) {
        if next_tile == Tile::Air || next_tile == Tile::Water {
            world.set_tile(dir, tile);
            world.set_tile(position, Tile::Air);
            return;
        }
    }
}

pub fn update_fluid(world: &mut World, position: Position, tile: Tile) {
    if let Some(down_tile) = world.get_tile(position + (0, 1).into()) {
        if down_tile == Tile::Air {
            world.set_tile(position + (0, 1).into(), tile);
            world.set_tile(position, Tile::Air);
            return;
        }
    }

    let direction = random_direction();
    let diag_pos = (direction, 1).into();

    if let Some(next_diag_tile) = world.get_tile(position + diag_pos) {
        if next_diag_tile == Tile::Air {
            world.set_tile(position + diag_pos, tile);
            world.set_tile(position, Tile::Air);
            return;
        }
    }

    let next_pos = (direction, 0).into();

    if let Some(next_tile) = world.get_tile(position + next_pos) {
        if next_tile == Tile::Air {
            world.set_tile(position + next_pos, tile);
            world.set_tile(position, Tile::Air);
        }
    }
}

pub fn update_acid(world: &mut World, position: Position) {
    if let Some(right_tile) = world.get_tile(position + (1, 0).into()) {
        if right_tile != Tile::Air
            && right_tile != Tile::Acid
            && right_tile != Tile::Wall
        {
            world.set_tile(position + (1, 0).into(), Tile::Air);
            world.set_tile(position, Tile::Air);
            return;
        }
    }

    if let Some(left_tile) = world.get_tile(position + (-1, 0).into()) {
        if left_tile != Tile::Air && left_tile != Tile::Acid && left_tile != Tile::Wall
        {
            world.set_tile(position + (-1, 0).into(), Tile::Air);
            world.set_tile(position, Tile::Air);
            return;
        }
    }

    if let Some(up_tile) = world.get_tile(position + (0, 1).into()) {
        if up_tile != Tile::Air && up_tile != Tile::Acid && up_tile != Tile::Wall {
            world.set_tile(position + (0, 1).into(), Tile::Air);
            world.set_tile(position, Tile::Air);
            return;
        }
    }

    if let Some(down_tile) = world.get_tile(position + (0, -1).into()) {
        if down_tile != Tile::Air && down_tile != Tile::Acid && down_tile != Tile::Wall
        {
            world.set_tile(position + (0, -1).into(), Tile::Air);
            world.set_tile(position, Tile::Air);
            return;
        }
    }

    update_fluid(world, position, Tile::Acid);
}

pub fn update_fire(world: &mut World, position: Position) {
    let dir = position + (random_direction(), 1).into();

    if let Some(down_tile) = world.get_tile(dir) {
        if down_tile == Tile::Air {
            world.set_tile(dir, Tile::Fire);
            world.set_tile(position, down_tile);
            return;
        } else if down_tile != Tile::Fire {
            world.set_tile(position, Tile::Air);
        }
    } else {
        world.set_tile(position, Tile::Air);
    }
}

pub fn update_water(world: &mut World, position: Position) {
    if let Some(water_tile) = neigbour_of_type(world, position, Tile::Lava) {
        world.set_tile(position, Tile::Stone);
        world.set_tile(position + water_tile, Tile::Stone);
        return;
    }

    update_fluid(world, position, Tile::Water);
}

pub fn update_lava(world: &mut World, position: Position) {
    if let Some(water_tile) = neigbour_of_type(world, position, Tile::Water) {
        world.set_tile(position, Tile::Stone);
        world.set_tile(position + water_tile, Tile::Stone);
        return;
    }

    update_fluid(world, position, Tile::Lava);
}

fn neigbour_of_type(world: &mut World, pos: Position, tile: Tile) -> Option<Position> {
    if let Some(right_tile) = world.get_tile(pos + (1, 0).into()) {
        if right_tile == tile {
            return Some((1, 0).into());
        }
    }

    if let Some(left_tile) = world.get_tile(pos + (-1, 0).into()) {
        if left_tile == tile {
            return Some((-1, 0).into());
        }
    }

    if let Some(up_tile) = world.get_tile(pos + (0, 1).into()) {
        if up_tile == tile {
            return Some((0, 1).into());
        }
    }

    if let Some(down_tile) = world.get_tile(pos + (0, -1).into()) {
        if down_tile == tile {
            return Some((1, 0).into());
        }
    }

    None
}

