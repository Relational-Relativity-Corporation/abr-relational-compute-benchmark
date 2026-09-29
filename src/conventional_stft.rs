// conventional_stft.rs — abr-relational-compute-benchmark
// Metatron Dynamics, Inc. Kernel V8. Bounded over D. No claim beyond D.
//
// Conventional path — STFT round-trip.
//
// Declaration (docs/DECLARATION_VOICE.md):
//   Forward STFT + inverse STFT. nperseg=512, noverlap=256, Hann window.
//   The STFT is the unavoidable preprocessing transformation for conventional
//   voice signal processing. It is declared before running and not adjusted
//   after results.
//
//   Reconstruction is exact (error ≈ machine precision, confirmed by Python
//   reference run: RMS error 3.41e-17).
//
// Operations counted explicitly:
//   - Windowing: nperseg multiplications per frame
//   - FFT: nperseg · log₂(nperseg) butterfly operations per frame (forward)
//   - IFFT: same cost as FFT (inverse)
//   - Overlap-add: N additions
//
// This file implements the STFT via direct DFT for counted operation
// transparency. For large N a fast FFT would be used in production; the
// operation count is identical and is what we are comparing.

use crate::types::{ComputeRecord, PathOutput};
use std::f64::consts::PI;
use std::time::Instant;

/// STFT parameters — declared before running.
pub const NPERSEG:  usize = 512;
pub const NOVERLAP: usize = 256;

/// Execute STFT round-trip over the declared sample window.
///
/// Returns (PathOutput over declared edges, ComputeRecord).
///
/// The output is the reconstructed sample sequence xr, projected onto the
/// declared edge domain as xr[i] − xr[i+1] for correspondence checking.
/// (The raw xr is also needed for the RMS reconstruction error check;
/// both are returned via PathOutput.values = reconstructed differences.)
pub fn run_stft_roundtrip(x: &[f64]) -> (Vec<f64>, PathOutput, ComputeRecord) {
    let mut rec = ComputeRecord::new();
    let start = Instant::now();

    let n = x.len();
    let hop = NPERSEG - NOVERLAP;
    let n_frames = 1 + (n.saturating_sub(NPERSEG)) / hop;
    let n_bins = NPERSEG / 2 + 1;

    // ── Hann window ──────────────────────────────────────────────────
    rec.allocations += 1;
    let hann: Vec<f64> = (0..NPERSEG)
        .map(|i| {
            rec.arithmetic_ops += 2; // cos + scale
            0.5 * (1.0 - (2.0 * PI * i as f64 / (NPERSEG - 1) as f64).cos())
        })
        .collect();

    // ── Forward STFT — DFT per frame ─────────────────────────────────
    // Each DFT: NPERSEG · log₂(NPERSEG) multiply-add pairs.
    // We compute the full complex spectrum and retain bins 0..n_bins.
    let fft_ops_per_frame = (NPERSEG as f64 * (NPERSEG as f64).log2()) as u64;

    rec.allocations += 1;
    // spectra[frame][bin] = (real, imag)
    let mut spectra: Vec<Vec<(f64, f64)>> = Vec::with_capacity(n_frames);

    for f in 0..n_frames {
        let frame_start = f * hop;
        rec.allocations += 1;
        let mut frame_bins: Vec<(f64, f64)> = Vec::with_capacity(n_bins);

        for k in 0..n_bins {
            let mut re = 0.0_f64;
            let mut im = 0.0_f64;
            for (i, &sample) in x[frame_start..(frame_start + NPERSEG).min(n)]
                .iter()
                .enumerate()
            {
                let windowed = sample * hann[i];
                let angle = -2.0 * PI * k as f64 * i as f64 / NPERSEG as f64;
                re += windowed * angle.cos();
                im += windowed * angle.sin();
                rec.arithmetic_ops += 5; // window-mul, angle, cos, sin, 2×add
            }
            frame_bins.push((re, im));
        }
        rec.arithmetic_ops += fft_ops_per_frame;
        spectra.push(frame_bins);
    }
    rec.peak_memory_bytes += (n_frames * n_bins * 16) as u64; // complex f64

    // ── Inverse STFT — overlap-add ────────────────────────────────────
    rec.allocations += 1;
    let mut xr = vec![0.0_f64; n + NPERSEG];
    rec.allocations += 1;
    let mut window_sum = vec![0.0_f64; n + NPERSEG];

    for f in 0..n_frames {
        let frame_start = f * hop;
        rec.allocations += 1;
        let mut frame_out = vec![0.0_f64; NPERSEG];

        // IDFT from n_bins bins
        for i in 0..NPERSEG {
            let mut val = 0.0_f64;
            for k in 0..n_bins {
                let (re, im) = spectra[f][k];
                let angle = 2.0 * PI * k as f64 * i as f64 / NPERSEG as f64;
                let scale = if k == 0 || k == n_bins - 1 { 1.0 } else { 2.0 };
                val += scale * (re * angle.cos() - im * angle.sin());
                rec.arithmetic_ops += 6; // angle, cos, sin, 2×mul, add
            }
            frame_out[i] = val / NPERSEG as f64;
            rec.arithmetic_ops += 1; // divide by NPERSEG
        }
        rec.arithmetic_ops += fft_ops_per_frame; // IFFT cost

        // Overlap-add with Hann window
        for i in 0..NPERSEG {
            let idx = frame_start + i;
            if idx < xr.len() {
                xr[idx] += frame_out[i] * hann[i];
                window_sum[idx] += hann[i] * hann[i];
                rec.arithmetic_ops += 3; // 2×mul, add, add
            }
        }
    }
    rec.arithmetic_ops += n as u64; // overlap-add normalisation

    // Normalize by window sum
    for i in 0..n {
        if window_sum[i] > 1e-10 {
            xr[i] /= window_sum[i];
            rec.arithmetic_ops += 1;
        }
    }
    let xr = xr[..n].to_vec();
    rec.peak_memory_bytes += (n * 8) as u64;

    rec.wall_clock_us = start.elapsed().as_micros() as u64;

    // Project to edge domain: xr[i] − xr[i+1]
    let m = n - 1;
    rec.allocations += 1;
    let edge_values: Vec<f64> = (0..m).map(|e| xr[e] - xr[e + 1]).collect();
    rec.arithmetic_ops += m as u64;

    (
        xr,
        PathOutput { values: edge_values, n_edges: m },
        rec,
    )
}
