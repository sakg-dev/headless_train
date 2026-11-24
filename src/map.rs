// We are supporting only base zooms not fractional zooms, hence at following places, we doing:
//      - 

use crate::config::*;
use crate::utils;
use crossterm::terminal::window_size;

pub struct VectorTile {
    x: u32,
    y: u32
}

// const INITIAL_LON:f32 = -51.254156;
// const INITIAL_LAT:f32 = -76.220362;
// const INITIAL_ZOOM:u32 = 18;

pub fn init() {
    // let center_coord = (INITIAL_LON, INITIAL_LAT);
    // println!("{:?}", center_coord);
    let center_tile_pos = utils::ll_to_tile_pos(INITIAL_LON, INITIAL_LAT, INITIAL_ZOOM);
    let visible_tiles = visible_vector_tiles(center_tile_pos, PIXEL_SCALE);
}

// fn a(a: f32, b: f32, c: f32) -> (u32, f32) {
//     let x = a - b;
//     if x > c {
//         a()
//     }
// }

fn visible_vector_tiles(center_tile_pos: utils::TilePos, pixel_scale: u32) -> Vec<VectorTile> {
    // At z=0, there is only 1 tile and we will show it in original size, for rest, we have neighbours

    let window_size = window_size().unwrap();
 
    // pixel_scale / (1 / padding_in_fraction) = Left/Top padding in pixel
    let dx = pixel_scale as f32 / (1.0 / (center_tile_pos.x - (center_tile_pos.x as i32) as f32)); 
    let dy = pixel_scale as f32 / (1.0 / (center_tile_pos.y - (center_tile_pos.y as i32) as f32)); 

    // leftover space in pixel when removed the main tile space
    let top_l_space_in_pixel = (window_size.height/2) as f32 - dy;
    let bottom_l_space_in_pixel = (window_size.height/2) as f32 - (pixel_scale as f32 - dy);
    let left_l_space_in_pixel = (window_size.width/2) as f32 - dx;
    let right_l_space_in_pixel = (window_size.width/2) as f32 - (pixel_scale as f32 - dx);

    // find how many tiles can fit in the leftover space
    let tiles_in_l_space_top =  ((top_l_space_in_pixel / pixel_scale as f32)+1.0) as u32;
    let tiles_in_l_space_bottom =  ((bottom_l_space_in_pixel / pixel_scale as f32)+1.0) as u32;
    let tiles_in_l_space_left =  ((left_l_space_in_pixel / pixel_scale as f32)+1.0) as u32;
    let tiles_in_l_space_right =  ((right_l_space_in_pixel / pixel_scale as f32)+1.0) as u32;

    let visible_tiles: Vec<VectorTile> = vec!();

    // println!("{:?}", window_size);
    // println!("{:?}", center_tile_pos);

    visible_tiles
}
