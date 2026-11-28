use super::*;
use crossterm::terminal;

#[test]
fn ll_to_tile_pos() {
    struct Args {
        lon: f32,
        lat: f32,
        zoom: u32,
        x: f32,
        y: f32
    }

    let args = vec!(
        Args { lon: 0.0, lat: 0.0, zoom: 0, x: 0.5, y: 0.5 },
        Args { lon: 67.676767, lat: -67.676767, zoom: 11, x: 1409.0056078222221, y: 1553.0102207059954 }
    );

    for arg in args {
        let Args {lon, lat, zoom, x, y} = arg;
        let test_tile_pos = utils::ll_to_tile_pos(lon, lat, zoom);
        assert!(x == test_tile_pos.x && y == test_tile_pos.y);
    }
}

#[test]
fn get_visible_tile_positions() {
    use map::{TileGridPos, TileGridAccuratePos};

    struct Args {
        center_tile_pos: TileGridAccuratePos,
        window_size: terminal::WindowSize,
        visible_tile_positions: Vec<TileGridPos>
    }
    
    let args = vec!(
        Args {
            center_tile_pos: TileGridAccuratePos { x: 0.5, y: 0.5 },
            window_size: terminal::WindowSize { rows: 20, columns: 20, width: 200, height: 200 },
            visible_tile_positions: vec!(
                TileGridPos { x: 0, y: 0 }
            )
        },
        Args {
            center_tile_pos: TileGridAccuratePos { x: 0.5, y: 0.5 },
            window_size: terminal::WindowSize { rows: 24, columns: 24, width: 256, height: 256 },
            visible_tile_positions: vec!(
                TileGridPos { x: 0, y: 0 }
            )
        },
        Args {
            center_tile_pos: TileGridAccuratePos { x: 0.5, y: 0.5 },
            window_size: terminal::WindowSize { rows: 50, columns: 50, width: 550, height: 1075 },
            visible_tile_positions: vec!(
                TileGridPos { x: 0, y: 0 },
                TileGridPos { x: 1, y: 0 },
                TileGridPos { x: 0, y: 1 },
                TileGridPos { x: 1, y: 1 },
                TileGridPos { x: 0, y: 2 },
                TileGridPos { x: 1, y: 2 }
            )
        },
        Args {
            center_tile_pos: TileGridAccuratePos { x: 2.5, y: 2.5 },
            window_size: terminal::WindowSize { rows: 100, columns: 100, width: 1024, height: 1024 },
            visible_tile_positions: vec!(
                TileGridPos { x: 0, y: 0 },
                TileGridPos { x: 1, y: 0 },
                TileGridPos { x: 2, y: 0 },
                TileGridPos { x: 3, y: 0 },
                TileGridPos { x: 4, y: 0 },
        
                TileGridPos { x: 0, y: 1 },
                TileGridPos { x: 1, y: 1 },
                TileGridPos { x: 2, y: 1 },
                TileGridPos { x: 3, y: 1 },
                TileGridPos { x: 4, y: 1 },
        
                TileGridPos { x: 0, y: 2 },
                TileGridPos { x: 1, y: 2 },
                TileGridPos { x: 2, y: 2 },
                TileGridPos { x: 3, y: 2 },
                TileGridPos { x: 4, y: 2 },
        
                TileGridPos { x: 0, y: 3 },
                TileGridPos { x: 1, y: 3 },
                TileGridPos { x: 2, y: 3 },
                TileGridPos { x: 3, y: 3 },
                TileGridPos { x: 4, y: 3 },
        
                TileGridPos { x: 0, y: 4 },
                TileGridPos { x: 1, y: 4 },
                TileGridPos { x: 2, y: 4 },
                TileGridPos { x: 3, y: 4 },
                TileGridPos { x: 4, y: 4 }
            )
        },
        Args {
            center_tile_pos: TileGridAccuratePos { x: 3.9, y: 3.9 },
            window_size: terminal::WindowSize { rows: 30, columns: 30, width: 300, height: 300 },
            visible_tile_positions: vec!(
                TileGridPos { x: 3, y: 3 },
                TileGridPos { x: 4, y: 3 },
                TileGridPos { x: 3, y: 4 },
                TileGridPos { x: 4, y: 4 }
            )
        },
        Args {
            center_tile_pos: TileGridAccuratePos { x: 0.0, y: 0.0 },
            window_size: terminal::WindowSize { rows: 50, columns: 50, width: 512, height: 512 },
            visible_tile_positions: vec!(
                TileGridPos { x: 0, y: 0 }
            )
        }
    );

    let map = map::Map::new();
    for arg in args {
        let Args {center_tile_pos, window_size, visible_tile_positions} = arg;
        let test_visible_tile_positions = map.get_visible_tile_positions(&center_tile_pos, &window_size);
        assert!(test_visible_tile_positions==visible_tile_positions);
    }
}
