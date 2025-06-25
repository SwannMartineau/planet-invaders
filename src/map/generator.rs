use noise::{NoiseFn, Perlin};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

use super::tile::Tile;

pub fn generate_map(width: usize, height: usize, seed: u32) -> Vec<Vec<Tile>> {
    let perlin = Perlin::new(seed);  // Correction ici - passer le seed directement
    let mut rng = StdRng::seed_from_u64(seed as u64);

    let scale = 0.1;
    let mut map = vec![vec![Tile::Empty; width]; height];

    for y in 0..height {
        for x in 0..width {
            let nx = x as f64 * scale;
            let ny = y as f64 * scale;
            let noise_val = perlin.get([nx, ny]);

            map[y][x] = match noise_val {
                v if v > 0.45 => Tile::Obstacle,
                v if v > 0.2 => {
                    // Petite chance de mettre une ressource
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