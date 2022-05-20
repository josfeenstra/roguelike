/// Fade function as defined by Ken Perlin.  This eases coordinate values
/// so that they will ease towards integral values.  This ends up smoothing
/// the final output.
pub fn smooth(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}