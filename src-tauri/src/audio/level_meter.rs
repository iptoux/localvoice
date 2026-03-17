/// Calculates the root-mean-square (RMS) amplitude of a block of f32 samples.
/// Returns a value in [0.0, 1.0] representing the signal level.
pub fn calculate_rms(data: &[f32]) -> f32 {
    if data.is_empty() {
        return 0.0;
    }
    let sum_sq: f32 = data.iter().map(|x| x * x).sum();
    (sum_sq / data.len() as f32).sqrt()
}
