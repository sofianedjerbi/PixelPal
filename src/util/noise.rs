use noise::{NoiseFn, Perlin, Seedable, Fbm};
use quick_cache::sync::Cache;
use crate::constants::generation::*;


const _TOTAL_SAMPLE: f64 = SAMPLE_NUMBER as f64 * SAMPLE_NUMBER as f64;


pub struct TiledNoise(Fbm<Perlin>, Cache<(i32, i32), u32>);

impl TiledNoise {
    // Initialize with a given seed
    pub fn new(seed: u32) -> Self {
        let mut noise = Fbm::<Perlin>::default();
        noise = noise.set_seed(seed);
        TiledNoise(noise, Cache::new(CACHE_SIZE))
    }

    // Get the mean noise value for the block containing the specific coordinate
    pub fn get_value(&self, x: i32, y: i32) -> u32 {
        if let Some(cached_result) = self.1.get(&(x,y)) {
            return cached_result;
        }
        let mut total_noise = 0.0;
        let start_x = x as f64 / NOISE_ZOOM;
        let start_y = y as f64 / NOISE_ZOOM;
        let reduction_factor = SAMPLE_NUMBER as f64 * NOISE_ZOOM;

        // Take samples inside the noise section
        for i in 0..SAMPLE_NUMBER {
            let sample_x = start_x + (i as f64 / reduction_factor);
            for j in 0..SAMPLE_NUMBER {
                let sample_y = start_y + (j as f64 / reduction_factor);
                // Accumulate the noise value
                total_noise += self.0.get([sample_x, sample_y, 0.0]);
            }
        }

        // Calculate and return the mean noise value
        let mean_noise = (total_noise / _TOTAL_SAMPLE + 1.).clamp(0., 2.);
        for (i, &value) in LAYER_RANGE.iter().enumerate() {
            if i + 1 == LAYER_RANGE.len()
              || (mean_noise >= value 
              && mean_noise <= LAYER_RANGE[i + 1]) {
                self.1.insert((x, y), i as u32);
                return i as u32;
            }
        }
        self.1.insert((x, y), 0);
        return 0;
    }
}
