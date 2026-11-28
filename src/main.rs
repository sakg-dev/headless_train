mod config;
mod utils;
mod map;
#[cfg(test)]
mod test;

fn main() {
    let map = map::init();
}

