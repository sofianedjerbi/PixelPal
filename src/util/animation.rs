use benimator::{Animation, FrameRate};


pub fn new_animation(
    indices: impl IntoIterator<Item = usize>,
    fps: f64
) -> Animation {
    Animation::from_indices(indices, FrameRate::from_fps(fps))
}
