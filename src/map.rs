use crate::config::*;
use crate::utils;
use crossterm::terminal::window_size;
use std::env;
use downloader::Download;
use downloader::downloader::Builder;
use std::time::Duration;
use std::path::{Path, PathBuf};
use mvt_reader::{Reader, error::{ParserError}};


pub fn init() {
    let map = Map::new();
    let visible_tiles = map.visible_tiles();
    let dl_dir = map.fetch_visible_tiles(visible_tiles);
    map.read_tiles(dl_dir);
    // Now once we have visible tiles, render them, by firstly getting them in order then loop it
    // and draw in ascii somehow smh
}

pub struct Map {
    pub zoom: u32,
    pub lat: f32,
    pub lon: f32,
}

#[derive(Debug)]
struct VectorTile {
    x: u32,
    y: u32
}

impl Map {
    // We are supporting only base zooms not fractional zooms
    fn new() -> Self {
        Self {
            zoom: INITIAL_ZOOM,
            lat: INITIAL_LAT,
            lon: INITIAL_LON
        }
    }

    fn visible_tiles(&self) -> Vec<VectorTile> {
        // At z=0, there is only 1 tile and we will show it in original size, for rest, we have neighbours

        let center_tile_pos = utils::ll_to_tile_pos(self.lon, self.lat, self.zoom);

        let window_size = window_size().unwrap();
     
        // PIXEL_SCALE / (1 / padding_in_fraction) = Left/Top padding in pixel
        let dx = PIXEL_SCALE as f32 / (1.0 / (center_tile_pos.x - (center_tile_pos.x as u32) as f32)); 
        let dy = PIXEL_SCALE as f32 / (1.0 / (center_tile_pos.y - (center_tile_pos.y as u32) as f32)); 
    
        // leftover space in pixel when removed the main tile space
        let top_l_space_in_pixel = (window_size.height/2) as f32 - dy;
        let bottom_l_space_in_pixel = (window_size.height/2) as f32 - (PIXEL_SCALE as f32 - dy);
        let left_l_space_in_pixel = (window_size.width/2) as f32 - dx;
        let right_l_space_in_pixel = (window_size.width/2) as f32 - (PIXEL_SCALE as f32 - dx);
    
        // find how many tiles can fit in the leftover space
        let tiles_in_top =  ((top_l_space_in_pixel / PIXEL_SCALE as f32)+1.0) as u32;
        let tiles_in_bottom =  ((bottom_l_space_in_pixel / PIXEL_SCALE as f32)+1.0) as u32;
        let tiles_in_left =  ((left_l_space_in_pixel / PIXEL_SCALE as f32)+1.0) as u32;
        let tiles_in_right =  ((right_l_space_in_pixel / PIXEL_SCALE as f32)+1.0) as u32;
    
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

    fn fetch_visible_tiles(&self, visible_tiles: Vec<VectorTile>) -> PathBuf {
        let key = env::var("MAPTILER_API_KEY").unwrap();

        let mut tiles = vec!();
        for tile in visible_tiles {
            let VectorTile{x, y} = tile;
            let url = format!("{SOURCE}/tiles/countries/{}/{x}/{y}.pbf?key={key}", self.zoom);
            let file = Download::new(&url).file_name(Path::new(&format!("{x}-{y}.pbf")));
            tiles.push(file);
        }

        let dl_dir = utils::create_and_get_dl_dir(self.zoom);
        // TODO: check when dir exists, why is it faster, is it bcz the same file is already there
        // or what?
        let mut dl = Builder::default()
            .connect_timeout(Duration::from_secs(4))
            .download_folder(&dl_dir)
            .parallel_requests(PARALLEL_DOWNLOAD_LIMIT)
            .build().unwrap();

        // TODO: later check status of individual files and if fails, try doing again for n times
        let _ = dl.download(&tiles);
        dl_dir
    }

    fn read_tiles(&self, dl_dir: PathBuf) {
        // let files = utils::files_in_dir(dl_dir).unwrap();
        // println!("{:?}", files);

        // for file in files {
        // }

        let file = "/home/sakg/Downloads/1.pbf";
        let mut buf:Vec<u8> = vec!();
        utils::read_file(&mut buf, file);

        let reader = Reader::new(buf).unwrap();
        // println!("{:?}", reader.get_layer_names().unwrap());
        // println!("{:?}", reader.get_layer_metadata().unwrap());
        // println!("{:?}", reader.get_features(0).unwrap()[50].properties.as_ref().unwrap());
    }
}

