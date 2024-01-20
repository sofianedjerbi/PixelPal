// Virtual pixel for 1 real pixel
#[cfg(target_arch = "wasm32")]
pub const ZOOM: i32 = 2;

#[cfg(not(target_arch = "wasm32"))]
pub const ZOOM: i32 = 3;
