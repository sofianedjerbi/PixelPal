use noise::{NoiseFn, Perlin, Seedable, Fbm};
use quick_cache::sync::Cache;
use crate::constants::generation::*;


const _TOTAL_SAMPLE: f64 = SAMPLE_NUMBER as f64 * SAMPLE_NUMBER as f64;


pub struct TiledNoise{
    noise: Fbm<Perlin>,
    layer_range: Vec<f64>,
    zoom: f64,
    sample_number: usize,
    total_sample: f64,
    cache: Cache<(i32, i32), u32>,
}

impl TiledNoise {
    pub fn new(
        seed: u32,
        layer_range: Vec<f64>,
        zoom: f64,
        sample_number: usize,
        cache_size: usize
    ) -> Self {
        let mut noise = Fbm::<Perlin>::default();
        noise = noise.set_seed(seed);
        noise.octaves = NOISE_OCTAVES;
        noise.frequency = NOISE_FREQUENCY;
        TiledNoise {
            noise, 
            layer_range,
            zoom,
            sample_number,
            total_sample: sample_number as f64 * sample_number as f64,
            cache: Cache::new(cache_size)
        }
    }

    pub fn get_value(&self, x: i32, y: i32) -> u32 {
        if let Some(cached_result) = self.cache.get(&(x,y)) {
            return cached_result;
        }

        let mut total_noise = 0.0;
        let start_x = x as f64 / self.zoom;
        let start_y = y as f64 / self.zoom;
        let reduction_factor = 1. / (self.sample_number as f64 * self.zoom);

        // Take samples inside the noise section
        for i in 0..self.sample_number {
            let sample_x = start_x + (i as f64 * reduction_factor);
            for j in 0..self.sample_number {
                let sample_y = start_y + (j as f64 * reduction_factor);
                total_noise += self.noise.get([sample_x, sample_y, 0.0]);
            }
        }

        // Calculate and return the mean noise value
        let mean_noise = (total_noise / self.total_sample + 1.).clamp(0., 2.);
        for (i, &value) in self.layer_range.iter().enumerate() {
            if i + 1 == self.layer_range.len()
              || (mean_noise >= value 
              && mean_noise <= self.layer_range[i + 1]) {
                self.cache.insert((x, y), i as u32);
                return i as u32;
            }
        }
        self.cache.insert((x, y), 0);
        return 0;
    }
}
