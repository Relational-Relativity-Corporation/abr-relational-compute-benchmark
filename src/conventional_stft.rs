// conventional_stft.rs — abr-relational-compute-benchmark
// Metatron Dynamics, Inc. Kernel V8. Bounded over D. No claim beyond D.
//
// Conventional path — STFT round-trip.
//
// Declaration (docs/DECLARATION_VOICE.md):
//   Forward STFT + inverse STFT. nperseg=512, noverlap=256, Hann window.
//   Matches scipy.signal.stft / istft default behavior exactly:
//     - Pads signal by nperseg//2 on each side before analysis
//     - Hann window (periodic form: w[i] = 0.5*(1 - cos(2π·i/N)))
//     - Overlap-add with window² normalization
//     - Reconstruction RMS error: 3.41e-17 (machine precision)
//
// Operations counted explicitly (operator execution only — correspondence
// check is not included in the count):
//   - Windowing: nperseg multiplications per frame
//   - FFT: nperseg · log₂(nperseg) butterfly operations per frame (forward)
//   - IFFT: same cost as FFT (inverse)
//   - Overlap-add and normalization: proportional to output length
//
// F-1 fix (2026-09-29): added nperseg//2 padding to match scipy default.
// The manual DFT is replaced by a Cooley-Tukey radix-2 FFT to avoid
// accumulated floating-point error across 18 frames × 512 × 257 terms.

use crate::types::{ComputeRecord, PathOutput};
use std::f64::consts::PI;
use std::time::Instant;

pub const NPERSEG:  usize = 512;
pub const NOVERLAP: usize = 256;

// ── Cooley-Tukey radix-2 FFT (in-place, complex) ─────────────────────────
// Input/output: interleaved (re, im) pairs, length must be a power of 2.
// inverse=true applies the IFFT scaling (divide by N at the end).
fn fft_inplace(buf: &mut Vec<(f64, f64)>, inverse: bool) {
    let n = buf.len();
    // Bit-reversal permutation
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 { j ^= bit; bit >>= 1; }
        j ^= bit;
        if i < j { buf.swap(i, j); }
    }
    // Cooley-Tukey butterfly
    let mut len = 2usize;
    while len <= n {
        let ang = if inverse { 2.0 * PI / len as f64 } else { -2.0 * PI / len as f64 };
        let (wre, wim) = (ang.cos(), ang.sin());
        let mut i = 0;
        while i < n {
            let (mut ure, mut uim) = (1.0_f64, 0.0_f64);
            for k in 0..len/2 {
                let (tre, tim) = buf[i + k + len/2];
                let (vr, vi) = (tre * ure - tim * uim, tre * uim + tim * ure);
                buf[i + k + len/2] = (buf[i + k].0 - vr, buf[i + k].1 - vi);
                buf[i + k] = (buf[i + k].0 + vr, buf[i + k].1 + vi);
                let new_ure = ure * wre - uim * wim;
                uim = ure * wim + uim * wre;
                ure = new_ure;
            }
            i += len;
        }
        len <<= 1;
    }
    if inverse {
        let scale = 1.0 / n as f64;
        for v in buf.iter_mut() { v.0 *= scale; v.1 *= scale; }
    }
}

/// Execute STFT round-trip over the declared sample window.
/// Matches scipy.signal.stft/istft default behavior (center padding).
///
/// Returns (reconstructed_samples, PathOutput over declared edges, ComputeRecord).
pub fn run_stft_roundtrip(x: &[f64]) -> (Vec<f64>, PathOutput, ComputeRecord) {
    let mut rec = ComputeRecord::new();
    let start = Instant::now();

    let n = x.len();
    let hop = NPERSEG - NOVERLAP;
    let pad = NPERSEG / 2;  // scipy default: pad by nperseg//2 on each side

    // ── Hann window (periodic form, matches scipy 'hann') ────────────────
    rec.allocations += 1;
    let hann: Vec<f64> = (0..NPERSEG)
        .map(|i| {
            rec.arithmetic_ops += 2;
            0.5 * (1.0 - (2.0 * PI * i as f64 / NPERSEG as f64).cos())
        })
        .collect();

    // ── Pad signal ────────────────────────────────────────────────────────
    rec.allocations += 1;
    let mut x_padded = vec![0.0_f64; n + 2 * pad];
    x_padded[pad..pad + n].copy_from_slice(x);
    let n_padded = x_padded.len();
    let n_frames = 1 + (n_padded.saturating_sub(NPERSEG)) / hop;
    let n_bins = NPERSEG / 2 + 1;

    let fft_ops_per_frame = (NPERSEG as f64 * (NPERSEG as f64).log2()) as u64;

    // ── Forward STFT ──────────────────────────────────────────────────────
    rec.allocations += 1;
    let mut spectra: Vec<Vec<(f64, f64)>> = Vec::with_capacity(n_frames);

    for f in 0..n_frames {
        let frame_start = f * hop;
        let end = (frame_start + NPERSEG).min(n_padded);
        let len = end - frame_start;

        rec.allocations += 1;
        let mut buf: Vec<(f64, f64)> = (0..NPERSEG)
            .map(|i| {
                let s = if i < len { x_padded[frame_start + i] * hann[i] } else { 0.0 };
                rec.arithmetic_ops += 1; // windowing multiply
                (s, 0.0)
            })
            .collect();

        fft_inplace(&mut buf, false);
        rec.arithmetic_ops += fft_ops_per_frame;

        // Retain only positive-frequency bins (0..n_bins)
        spectra.push(buf[..n_bins].to_vec());
    }
    rec.peak_memory_bytes += (n_frames * n_bins * 16) as u64;

    // ── Inverse STFT — overlap-add ────────────────────────────────────────
    let out_len = n_padded + NPERSEG;
    rec.allocations += 1;
    let mut xr_padded = vec![0.0_f64; out_len];
    rec.allocations += 1;
    let mut win_sum = vec![0.0_f64; out_len];

    for f in 0..n_frames {
        let frame_start = f * hop;

        // Reconstruct full spectrum from one-sided bins
        rec.allocations += 1;
        let mut buf: Vec<(f64, f64)> = vec![(0.0, 0.0); NPERSEG];
        buf[0] = spectra[f][0];
        for k in 1..n_bins - 1 {
            buf[k] = spectra[f][k];
            buf[NPERSEG - k] = (spectra[f][k].0, -spectra[f][k].1); // conjugate
        }
        buf[n_bins - 1] = spectra[f][n_bins - 1];

        fft_inplace(&mut buf, true); // IFFT
        rec.arithmetic_ops += fft_ops_per_frame;

        // Apply window and overlap-add
        for i in 0..NPERSEG {
            let idx = frame_start + i;
            if idx < out_len {
                xr_padded[idx] += buf[i].0 * hann[i];
                win_sum[idx]   += hann[i] * hann[i];
                rec.arithmetic_ops += 3; // 2×mul, 2×add
            }
        }
    }

    // Normalize by window² sum
    rec.allocations += 1;
    let mut xr = vec![0.0_f64; n];
    for i in 0..n {
        let idx = i + pad;
        if idx < out_len && win_sum[idx] > 1e-10 {
            xr[i] = xr_padded[idx] / win_sum[idx];
            rec.arithmetic_ops += 1;
        }
    }
    rec.peak_memory_bytes += (n * 8) as u64;

    rec.wall_clock_us = start.elapsed().as_micros() as u64;

    // Project to edge domain: xr[i] − xr[i+1]
    let m = n - 1;
    rec.allocations += 1;
    let edge_values: Vec<f64> = (0..m).map(|e| xr[e] - xr[e + 1]).collect();
    rec.arithmetic_ops += m as u64;

    (xr, PathOutput { values: edge_values, n_edges: m }, rec)
}
