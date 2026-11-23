use crate::config::*;
use crate::utils;

pub struct VectorTile {
    x: u32,
    y: u32
}

// const INITIAL_LON:f32 = -51.254156;
// const INITIAL_LAT:f32 = -76.220362;
// const INITIAL_ZOOM:u32 = 7;
pub fn init() {
    let tile_pos = utils::ll_to_tile_pos(INITIAL_LON, INITIAL_LAT, INITIAL_ZOOM);
    // println!("{:?}", tile_pos);
}

// pub fn visible_vector_tiles() -> Vec<VectorTile>{
//     
// }
