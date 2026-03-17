use std::io::Read;
use std::path::Path;

use sha2::{Digest, Sha256};

use crate::errors::AppError;

/// Computes the SHA-256 digest of a file and compares it to `expected_hex`.
///
/// Returns `Ok(())` if:
/// - `expected_hex` is `None` — verification is skipped (model has no registered checksum yet).
/// - The digest matches `expected_hex` (case-insensitive).
///
/// Returns `Err(ChecksumMismatch)` if the digest does not match.
pub fn verify_checksum(path: &Path, expected_hex: Option<&str>) -> Result<(), AppError> {
    let Some(expected) = expected_hex else {
        // No checksum registered — skip verification.
        return Ok(());
    };

    let mut file = std::fs::File::open(path)
        .map_err(|e| AppError(format!("Cannot open model file for verification: {e}")))?;

    let mut hasher = Sha256::new();
    let mut buf = [0u8; 65536];

    loop {
        let n = file
            .read(&mut buf)
            .map_err(|e| AppError(format!("Read error during checksum: {e}")))?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }

    let digest = format!("{:x}", hasher.finalize());

    if digest.eq_ignore_ascii_case(expected) {
        Ok(())
    } else {
        Err(AppError(format!(
            "Checksum mismatch for {}: expected {expected}, got {digest}",
            path.display()
        )))
    }
}
