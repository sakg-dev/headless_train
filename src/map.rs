// We are supporting only base zooms not fractional zooms, hence at following places, we doing:
//      - 

use crate::config::*;
use crate::utils;
use crossterm::terminal::window_size;

#[derive(Debug)]
pub struct VectorTile {
    x: u32,
    y: u32
}

const INITIAL_LON:f32 = -51.254156;
const INITIAL_LAT:f32 = -76.220362;
const INITIAL_ZOOM:u32 = 18;

pub fn init() {
    // let center_coord = (INITIAL_LON, INITIAL_LAT);
    // println!("{:?}", center_coord);
    let center_tile_pos = utils::ll_to_tile_pos(INITIAL_LON, INITIAL_LAT, INITIAL_ZOOM);
    let visible_tiles = visible_vector_tiles(center_tile_pos, PIXEL_SCALE);
    println!("{:?}", visible_tiles);
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
    let dx = pixel_scale as f32 / (1.0 / (center_tile_pos.x - (center_tile_pos.x as u32) as f32)); 
    let dy = pixel_scale as f32 / (1.0 / (center_tile_pos.y - (center_tile_pos.y as u32) as f32)); 

    // leftover space in pixel when removed the main tile space
    let top_l_space_in_pixel = (window_size.height/2) as f32 - dy;
    let bottom_l_space_in_pixel = (window_size.height/2) as f32 - (pixel_scale as f32 - dy);
    let left_l_space_in_pixel = (window_size.width/2) as f32 - dx;
    let right_l_space_in_pixel = (window_size.width/2) as f32 - (pixel_scale as f32 - dx);

    // find how many tiles can fit in the leftover space
    let tiles_in_top =  ((top_l_space_in_pixel / pixel_scale as f32)+1.0) as u32;
    let tiles_in_bottom =  ((bottom_l_space_in_pixel / pixel_scale as f32)+1.0) as u32;
    let tiles_in_left =  ((left_l_space_in_pixel / pixel_scale as f32)+1.0) as u32;
    let tiles_in_right =  ((right_l_space_in_pixel / pixel_scale as f32)+1.0) as u32;

    let top_tile = (center_tile_pos.y as u32) - tiles_in_top;
    let bottom_tile = (center_tile_pos.y as u32) + tiles_in_bottom;
    let left_tile = (center_tile_pos.x as u32) - tiles_in_left;
    let right_tile = (center_tile_pos.x as u32) + tiles_in_right;

    // Now tiles inside these tiles are visible tiles?!?!!

    let mut visible_tiles: Vec<VectorTile> = vec!();
    for y in top_tile..(bottom_tile + 1) {
        for x in left_tile..(right_tile + 1) {
            // println!("{x}, {y}");
            let visible_tile = VectorTile {
                x: x,
                y: y
            };
            visible_tiles.push(visible_tile);
        }
    }
    visible_tiles
}
