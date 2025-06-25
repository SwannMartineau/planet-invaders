mod map;

fn main() {
    let width = 40;
    let height = 20;
    let seed = 1337;

    let map = map::generate_map(width, height, seed);
    map::display_map(&map);
}