use crate::config::*;
use crate::utils;
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Stylize}
};
use downloader::{Download, downloader::Builder};
use std::{
    time::{self, Duration},
    io::{self, Write},
    path::{Path, PathBuf},
    env,
    thread,
};
use mvt_reader::{Reader, error::{ParserError}, feature::{Feature, Value}};

pub fn init() {
    let map = Map::new();
    let window_size = terminal::window_size().unwrap();
    let center_tile_pos = map.get_center_tile_pos();
    let visible_tile_positions = map.get_visible_tile_positions(&center_tile_pos, &window_size);
    let dl_dir = map.download_visible_tile_files(&visible_tile_positions);
    let visible_tiles = map.read_visible_tile_files(dl_dir, visible_tile_positions);
    map.render_visible_tiles(visible_tiles, &center_tile_pos, &window_size);
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

#[derive(Debug)]
pub struct Paddings {
    pub x: f32,
    pub y: f32
}

#[derive(Debug)]
pub struct TileGridRelativePos {
    pub x: i32,
    pub y: i32
}

#[derive(Debug, PartialEq)]
pub struct TileGridPos {
    pub x: u32,
    pub y: u32
}

#[derive(Debug)]
pub struct TileGridAccuratePos { // Tile Grid Pos accurately -- with fraction
    pub x: f32,
    pub y: f32
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

    fn get_center_tile_pos(&self) -> TileGridAccuratePos {
        utils::ll_to_tile_pos(self.lon, self.lat, self.zoom)
    }

    // based on center tile position and window size, finding out visible tiles
    fn get_visible_tile_positions(&self, center_tile_pos: &TileGridAccuratePos, window_size: &terminal::WindowSize) -> Vec<TileGridPos> {
        // TODO: Its panicking at lower zoom: 

        let pixel_scale = PIXEL_SCALE as f32;
        let half_window_width  = (window_size.width / 2) as f32;
        let half_window_height = (window_size.height / 2) as f32;

        // PIXEL_SCALE * padding_in_fraction = padding in pixels
        let l_padding = pixel_scale * (center_tile_pos.x - center_tile_pos.x.trunc());
        let t_padding = pixel_scale * (center_tile_pos.y - center_tile_pos.y.trunc());

        // leftover space in pixel when removed the center tile
        let top_l_space_in_pixel    = half_window_height - t_padding;
        let bottom_l_space_in_pixel = half_window_height - (pixel_scale - t_padding);
        let left_l_space_in_pixel   = half_window_width - l_padding;
        let right_l_space_in_pixel  = half_window_width - (pixel_scale - l_padding);

        // find how many tiles can fit in the leftover space
        let tiles_in_top    =  (top_l_space_in_pixel / pixel_scale).ceil();
        let tiles_in_bottom =  (bottom_l_space_in_pixel / pixel_scale).ceil();
        let tiles_in_left   =  (left_l_space_in_pixel / pixel_scale).ceil();
        let tiles_in_right  =  (right_l_space_in_pixel / pixel_scale).ceil();

        // find the {direction}-most tile aka min-{x|y} and max-{x|y}
        let min_y = (center_tile_pos.y - tiles_in_top) as u32;
        let max_y = (center_tile_pos.y + tiles_in_bottom) as u32;
        let min_x = (center_tile_pos.x - tiles_in_left) as u32;
        let max_x = (center_tile_pos.x + tiles_in_right) as u32;

        // tiles inside min-max are visible tiles
        let mut visible_tile_positions: Vec<TileGridPos> = vec!();
        for y in min_y..(max_y + 1) {
            for x in min_x..(max_x + 1) {
                let visible_tile_position = TileGridPos {
                    x: x,
                    y: y
                };
                visible_tile_positions.push(visible_tile_position);
            }
        }
        visible_tile_positions
    }

    // download the visible tiles and return download directory
    fn download_visible_tile_files(&self, visible_tile_positions: &Vec<TileGridPos>) -> PathBuf {
        let key = env::var("MAPTILER_API_KEY").unwrap();
        let z = self.zoom;

        let mut visible_tile_files = vec!();
        for visible_tile_position in visible_tile_positions {
            let TileGridPos{x, y} = visible_tile_position;
            let url = format!("{SOURCE}/tiles/countries/{z}/{x}/{y}.pbf?key={key}");
            let file = Download::new(&url).file_name(Path::new(&format!("{x}-{y}.pbf")));
            visible_tile_files.push(file);
        }

        let dl_dir = utils::create_and_get_dl_dir(self.zoom);
        // TODO: check when dir exists, why is it faster, is it bcz the same file is already there?
        let mut dl = Builder::default()
            .connect_timeout(Duration::from_secs(DOWNLOAD_TIMEOUT))
            .download_folder(&dl_dir)
            .parallel_requests(PARALLEL_DOWNLOAD_LIMIT)
            .build().unwrap();

        let _ = dl.download(&visible_tile_files);
        // TODO: later check status of individual files and if fails, try doing again for n times
        dl_dir
    }

    // reads all tile files in zoom dir, filter current tiles and output position and features of each
    fn read_visible_tile_files(&self, dl_dir: PathBuf, visible_tile_positions: Vec<TileGridPos>) -> Vec<Tile> {
        let all_tile_files = utils::files_in_dir(dl_dir).unwrap();
        // there might be tiles from prev download, bt we need current only
        let visible_tile_files = utils::filter_visible_tile_files(all_tile_files, visible_tile_positions); 

        let mut visible_tiles:Vec<Tile> = vec!();
        for visible_tile_file in visible_tile_files {
            let mut buf:Vec<u8> = vec!();
            utils::read_file(&mut buf, &visible_tile_file);

            let reader = Reader::new(buf).unwrap();
            let layer_names = reader.get_layer_names().unwrap();

            let administrative_idx = layer_names.iter().position(|l| l.as_str()=="administrative");

            if let Some(idx) = administrative_idx {
                // println!("{:?}", reader.get_layer_metadata().unwrap());
                let pos = utils::get_pos_from_file_path(&visible_tile_file);
                let features = reader.get_features(idx).unwrap();
                let visible_tile = Tile {
                    pos: pos,
                    features: features
                };
                visible_tiles.push(visible_tile);
            } else {
                // TODO: todo!("Administrative layer doesn't exist in file: {}", file.into_string().unwrap());
            }
        }
        visible_tiles
    }

    fn render_visible_tiles(&self, visible_tiles: Vec<Tile>, center_tile_pos: &TileGridAccuratePos, window_size: &terminal::WindowSize) {
//        println!("{:?}", tiles[6].features[8].geometry);
//        for (p_key, p_val) in tiles[6].features[8].properties.as_ref().unwrap() {
//            if !p_key.starts_with("name") {
//                println!("{}: {:?}", p_key, p_val);
//            }
//        }
        
//        let mut stdout = io::stdout();
//        terminal::enable_raw_mode();
//
//        stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();
        let non_acc_center_tile_pos = TileGridPos {
            x: center_tile_pos.x as u32,
            y: center_tile_pos.y as u32
        };
        let half_width = (window_size.width / 2) as f32;
        let half_height = (window_size.height / 2) as f32;

        for visible_tile in visible_tiles {
            let Tile {pos, features} = visible_tile;
            // # which part to show
            // top and left are negative | right and bottom are positive
            let relative_pos = TileGridRelativePos {
                x: (pos.x as i32) - (non_acc_center_tile_pos.x as i32),
                y: (pos.y as i32) - (non_acc_center_tile_pos.y as i32)
            };
            if relative_pos.x == 0 && relative_pos.y == 0 {
                // println!("center_tile which is full");

            } else {
                let center_paddings = utils::get_paddings(&relative_pos, center_tile_pos, PIXEL_SCALE);
                // total space between the center and that tile(with direction)
                let space_x = (relative_pos.x.unsigned_abs() * PIXEL_SCALE) as f32 + center_paddings.x;
                let space_y = (relative_pos.y.unsigned_abs() * PIXEL_SCALE) as f32 + center_paddings.y;

                // if negative, means fraction and if positive, means full
                let diff_x = half_width - space_x;
                let diff_y = half_height - space_y;

                if diff_x >= 0.0 {
                    // full x
                } else {
                    let fraction_x = diff_x + PIXEL_SCALE as f32;
                }

                if diff_y >= 0.0 {
                    // full y
                } else {
                    let fraction_y = diff_y + PIXEL_SCALE as f32;
                }
            }
            // # where in the screen
        }

//        for y in 0..40 {
//            for x in 0..150 {
//                if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
//                    stdout
//                      .queue(cursor::MoveTo(x,y)).unwrap()
//                      .queue(style::PrintStyledContent("#".blue())).unwrap();
//                }
//            }
//        }

//        stdout.flush().unwrap();
//        thread::sleep(time::Duration::from_secs(10));
//        terminal::disable_raw_mode();
    }
}

