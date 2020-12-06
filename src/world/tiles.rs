use super::{Position, Tile, World};
use rand::{thread_rng, Rng};

pub fn get_color(tile: &Tile) -> &'static [u8] {
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
