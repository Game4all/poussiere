use super::Tile;

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
