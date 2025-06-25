use noise::{NoiseFn, Perlin};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

use super::tile::Tile;

pub fn generate_map(width: usize, height: usize, seed: u32) -> Vec<Vec<Tile>> {
    let perlin = Perlin::new(seed);
    let mut rng = StdRng::seed_from_u64(seed as u64);

    let scale = 0.1;
    let mut map = vec![vec![Tile::Empty; width]; height];

    let base_size = usize::min(10, usize::min(width, height) / 5);
    let base_x0 = (width - base_size) / 2;
    let base_y0 = (height - base_size) / 2;

    for y in 0..height {
        for x in 0..width {
            if x >= base_x0 && x < base_x0 + base_size && y >= base_y0 && y < base_y0 + base_size {
                map[y][x] = Tile::Base;
                continue;
            }

            let nx = x as f64 * scale;
            let ny = y as f64 * scale;
            let noise_val = perlin.get([nx, ny]);

            map[y][x] = match noise_val {
                v if v > 0.45 => Tile::Obstacle,
                v if v > 0.2 => {
                    match rng.gen_range(0..100) {
                        0..=4 => Tile::Energy,
                        5..=9 => Tile::Mineral,
                        10..=12 => Tile::Science,
                        _ => Tile::Empty,
                    }
                }
                _ => Tile::Empty,
            };
        }
    }

    map
}