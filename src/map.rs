use crate::config::*;
use crate::utils;
use crossterm::terminal::window_size;
use downloader::{Download, downloader::Builder};
use std::env;
use std::time::Duration;
use std::path::{Path, PathBuf};
use mvt_reader::{Reader, error::{ParserError}, feature::Feature};

pub fn init() {
    let map = Map::new();
    let visible_tiles = map.visible_tiles();
    let dl_dir = map.download_visible_tiles(&visible_tiles);
    let tiles_to_render = map.read_tiles(dl_dir, visible_tiles); // getting features and pos of each visible tile
    // println!("{:?}", tiles_to_render);
}

pub struct Map {
    pub zoom: u32,
    pub lat: f32,
    pub lon: f32,
}

#[derive(Debug)]
struct Tile {
    pos: TileGridPos,
    features: Vec<Feature>
}

#[derive(Debug, PartialEq)]
pub struct TileGridPos {
    pub x: u32,
    pub y: u32
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

    fn visible_tiles(&self) -> Vec<TileGridPos> {
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
        let mut visible_tiles: Vec<TileGridPos> = vec!();
        for y in top_tile..(bottom_tile + 1) {
            for x in left_tile..(right_tile + 1) {
                // println!("{x}, {y}");
                let visible_tile = TileGridPos {
                    x: x,
                    y: y
                };
                visible_tiles.push(visible_tile);
            }
        }
        visible_tiles
    }

    fn download_visible_tiles(&self, visible_tiles: &Vec<TileGridPos>) -> PathBuf {
        let key = env::var("MAPTILER_API_KEY").unwrap();

        let mut tiles = vec!();
        for tile in visible_tiles {
            let TileGridPos{x, y} = tile;
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


    fn read_tiles(&self, dl_dir: PathBuf, visible_tiles_pos: Vec<TileGridPos>) -> Vec<Tile>{
        let files = utils::files_in_dir(dl_dir).unwrap();
        // there might be tiles from prev download, bt we need current only
        let files = utils::filter_visible_tiles_file(files, visible_tiles_pos); 

        let mut tiles:Vec<Tile> = vec!();

        for file in files {
            let mut buf:Vec<u8> = vec!();
            utils::read_file(&mut buf, &file);

            let reader = Reader::new(buf).unwrap();
            let layer_names = reader.get_layer_names().unwrap();

            let administrative_idx = layer_names.iter().position(|l| l.as_str()=="administrative");

            if let Some(idx) = administrative_idx {
                // println!("{idx}");
                // println!("{:?}", reader.get_layer_metadata().unwrap());
                let pos = utils::get_pos_from_file_path(&file);
                let pos = TileGridPos {
                    x: pos.x,
                    y: pos.y
                };
                let features = reader.get_features(idx).unwrap();
                let tile = Tile {
                    pos: pos,
                    features: features
                };
                tiles.push(tile);
            } else {
                // TODO: todo!("Administrative layer doesn't exist in file: {}", file.into_string().unwrap());
            }
        }
        tiles
    }
}

