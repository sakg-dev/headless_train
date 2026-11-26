use std::f32::consts::PI;
use std::path::PathBuf;
use std::{fs, env};
use std::io::Read;

use crate::map::{TileGridPos, TileGridAccuratePos, TileGridRelativePos, Paddings};

pub fn get_paddings(relative_pos: &TileGridRelativePos, center_tile_pos: &TileGridAccuratePos, pixel_scale:u32) -> Paddings {
    let pixel_scale = pixel_scale as f32;

    let mut paddings = Paddings {
        x: if relative_pos.x < 0 {
            // left
            pixel_scale * (center_tile_pos.x - center_tile_pos.x.trunc())
        } else {
            // right
            pixel_scale - (pixel_scale * (center_tile_pos.x - center_tile_pos.x.trunc()))
        },

        y: if relative_pos.y < 0 {
            // top
            pixel_scale * (center_tile_pos.y - center_tile_pos.y.trunc())
        } else {
            // bottom
            pixel_scale - (pixel_scale * (center_tile_pos.y - center_tile_pos.y.trunc()))
        }
    };

    paddings
}

pub fn create_and_get_dl_dir(zoom:u32) -> PathBuf {
    // rn, we create a folder everytime and will delete once programs terminates TODO: <-
    // NO Cache or other thing is being done atm
    let dl_dir = env::home_dir().unwrap()
        .join(".headless_train")
        .join(format!("{zoom}"));

    // lwk didn't needed ts thing
    // if(fs::exists(&dl_dir).unwrap()) {
    //     fs::remove_dir_all(&dl_dir);
    // }

    if (!fs::exists(&dl_dir).unwrap()) {
        fs::create_dir_all(&dl_dir);
    }

    dl_dir
}

pub fn get_pos_from_file_path(file_path: &PathBuf) -> TileGridPos {
    let pos_vec = file_path.file_prefix()
        .unwrap().to_str()
        .unwrap().split("-")
        .map(|s| s.parse::<u32>()
        .unwrap()).collect::<Vec<u32>>();
    TileGridPos {
        x: pos_vec[0],
        y: pos_vec[1]
    }
}

pub fn filter_visible_tile_files(files: Vec<PathBuf>, visible_tiles_pos: Vec<TileGridPos>) -> Vec<PathBuf> {
    let visible_tiles_file = files.into_iter().filter(|f| {
        let pos = get_pos_from_file_path(&f);
        visible_tiles_pos.contains(&pos)
    }).collect::<Vec<PathBuf>>();
    visible_tiles_file
}

pub fn files_in_dir(dir: PathBuf) -> Option<Vec<PathBuf>> {
    if dir.is_dir() {
        let files = fs::read_dir(dir).unwrap().into_iter().map(|f| f.unwrap().path()).collect::<Vec<_>>();
        Some(files)
    } else {
        None
    }
}

pub fn read_file(mut buf: &mut Vec<u8>, file_path: &PathBuf) {
    let mut file = fs::File::open(file_path).unwrap();
    file.read_to_end(&mut buf);
} 

pub fn deg_to_rad(deg: f32) -> f32 {
    (deg * PI)/180.0
}

pub fn ll_to_tile_pos(lon: f32, lat: f32, zoom: u32) -> TileGridAccuratePos {
    // https://wiki.openstreetmap.org/wiki/Slippy_map_tilenames
    let lat_rad = deg_to_rad(lat);
    let n = 2_i32.pow(zoom) as f32; // n*n is number of tiles
    let x_tile = n * ((lon + 180.0) / 360.0);
    let y_tile = n * (1.0 - ((lat_rad.tan() + (1.0/lat_rad.cos())).ln() / PI)) / 2.0;
    TileGridAccuratePos {
        x: x_tile,
        y: y_tile
    }
}
