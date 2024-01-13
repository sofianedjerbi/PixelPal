use crate::constants::generation::*;
use noise::{Fbm, NoiseFn, Perlin, Seedable};
use quick_cache::sync::Cache;

const _TOTAL_SAMPLE: f64 = SAMPLE_NUMBER as f64 * SAMPLE_NUMBER as f64;

/// A struct representing a tiled noise generator.
///
/// This struct uses Fbm (Fractal Brownian Motion) noise algorithm for generating noise values.
/// It supports caching for efficient noise value retrieval.
pub struct TiledNoise<'a> {
    noise: Fbm<Perlin>,
    layer_range: &'a [f64],
    zoom: f64,
    sample_number: usize,
    total_sample: f64,
    cache: Cache<(i32, i32), u32>,
}

impl<'a> TiledNoise<'a> {
    /// Constructs a new `TiledNoise`.
    ///
    /// # Parameters
    /// - `seed`: Seed value for the noise generator.
    /// - `layer_range`: A range of values representing different layers.
    /// - `zoom`: Zoom level for noise generation.
    /// - `sample_number`: Number of samples to take within each tile.
    /// - `cache_size`: Size of the cache for storing noise values.
    ///
    /// # Returns
    /// A new instance of `TiledNoise`.
    pub fn new(
        seed: u32,
        layer_range: &'a [f64],
        zoom: f64,
        sample_number: usize,
        cache_size: usize,
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
            cache: Cache::new(cache_size),
        }
    }

    /// Retrieves a noise value for the specified coordinates.
    ///
    /// # Parameters
    /// - `x`: The x-coordinate.
    /// - `y`: The y-coordinate.
    ///
    /// # Returns
    /// The noise value as `u32`.
    ///
    /// If the value is cached, it's retrieved from the cache; otherwise, it's computed.
    pub fn get_value(&self, x: i32, y: i32) -> u32 {
        if let Some(cached_result) = self.cache.get(&(x, y)) {
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
                || (mean_noise >= value && mean_noise <= self.layer_range[i + 1])
            {
                self.cache.insert((x, y), i as u32);
                return i as u32;
            }
        }
        self.cache.insert((x, y), 0);
        0
    }

    /// Computes a mask value for a given tile based on its surrounding tiles.
    ///
    /// # Parameters
    /// - `value`: The value of the current tile.
    /// - `x`: The x-coordinate of the tile.
    /// - `y`: The y-coordinate of the tile.
    ///
    /// # Returns
    /// The mask value as `u32`.
    ///
    /// The mask value helps determine the tile's surrounding environment.
    #[allow(clippy::unusual_byte_groupings)]
    pub fn get_mask(&self, value: u32, x: i32, y: i32) -> u32 {
        let got_n = self.get_value(x, y + 1) < value;
        let got_s = self.get_value(x, y - 1) < value;
        let got_e = self.get_value(x + 1, y) < value;
        let got_w = self.get_value(x - 1, y) < value;
        let got_nw = self.get_value(x - 1, y + 1) < value;
        let got_ne = self.get_value(x + 1, y + 1) < value;
        let got_sw = self.get_value(x - 1, y - 1) < value;
        let got_se = self.get_value(x + 1, y - 1) < value;

        (if got_n { 0b010_0_0_000 } else { 0 }
            + if got_s { 0b000_0_0_010 } else { 0 }
            + if got_w { 0b000_1_0_000 } else { 0 }
            + if got_e { 0b000_0_1_000 } else { 0 }
            + if got_nw { 0b100_0_0_000 } else { 0 }
            + if got_ne { 0b001_0_0_000 } else { 0 }
            + if got_sw { 0b000_0_0_100 } else { 0 }
            + if got_se { 0b000_0_0_001 } else { 0 })
    }
}
