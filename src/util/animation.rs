use benimator::{Animation, FrameRate};

/// Creates a new animation.
///
/// # Parameters
/// - `indices`: An iterable of frame indices for the animation.
/// - `fps`: The frame rate of the animation in frames per second.
///
/// # Returns
/// A new `Animation` object.
///
/// This function constructs an animation from a sequence of frame indices
/// and a specified frame rate.
pub fn new_animation(indices: impl IntoIterator<Item = usize>, fps: f64) -> Animation {
    Animation::from_indices(indices, FrameRate::from_fps(fps))
}
