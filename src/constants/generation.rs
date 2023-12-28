pub const NOISE_ZOOM: f64 = 50.;
pub const SAMPLE_NUMBER: usize = 4;
pub const MAX_VALUE: f64 = 6. - 1.; // Relief
// From 0 to 2 (performance reasons)
pub const LAYER_RANGE: [f64; MAX_VALUE as usize + 2] = [0., 0.3, 0.6, 0.65, 1.2, 1.6, 2.];
