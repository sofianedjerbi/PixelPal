pub const NOISE_ZOOM: f64 = 200.;
pub const NOISE_OCTAVES: usize = 5;
pub const NOISE_FREQUENCY: f64 = 0.5;
pub const CACHE_SIZE: usize = 10000;
pub const SAMPLE_NUMBER: usize = 1;
/**
 * 0: water
 * 1: soil
 * 2: grass
 * 3: grass
 * 4: grass
 * 5: dark_grass
 * 6: dark_grass
 * 7: dark_grass
 */
pub const MAX_VALUE: f64 = 7.; // Relief
                               // From 0 to 2 (performance reasons)
pub const LAYER_RANGE: [f64; MAX_VALUE as usize + 2] = [0., 0.8, 0.9, 1.1, 1.25, 1.4, 1.6, 1.8, 2.];
