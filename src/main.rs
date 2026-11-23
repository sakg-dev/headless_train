mod config;

fn main() {
    map::init();
}

mod utils {
    use std::f32::consts::PI;

    #[derive(Debug)]
    pub struct TilePos {
        pub x: u32,
        pub y: u32,
        pub z: u32
    }

    pub fn ll_to_tile_pos(lon: f32, lat: f32, zoom: u32) -> TilePos {
        // https://wiki.openstreetmap.org/wiki/Slippy_map_tilenames
        let n = 2_i32.pow(zoom) as f32;
        let x_tile = n * ((lon + 180.0) / 360.0);
        let y_tile = n * (1.0 - ((lat.tan() + (1.0/lat.cos())).ln() / PI)) / 2.0;
        TilePos {
            x: x_tile as u32,
            y: y_tile as u32,
            z: zoom
        }
    }
}

mod map {
    use crate::config::*;   

    use crate::utils;

    pub fn init() {
        let tile_pos = utils::ll_to_tile_pos(INITIAL_LON, INITIAL_LAT, INITIAL_ZOOM);
        println!("{:?}", tile_pos);
    }
}
