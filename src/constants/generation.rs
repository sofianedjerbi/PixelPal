pub const NOISE_ZOOM: f64 = 120.;
pub const SAMPLE_NUMBER: usize = 2;
/**
 * 0: grass 
 * 1: soil
 * 2: water
 * 3: soil
 * 4: grass
 * 5: grass
 * 6: grass
 * 7: dark_grass
 * 8: dark_grass
 * 9: dark_grass
 */
pub const MAX_VALUE: f64 = 9.; // Relief
// From 0 to 2 (performance reasons)
pub const LAYER_RANGE: [f64; MAX_VALUE as usize + 2] = [
    0.,
    0.05,
    0.1,
    0.6,
    0.8,
    1.,
    1.2,
    1.4,
    1.6,
    1.8,
    2.
];
