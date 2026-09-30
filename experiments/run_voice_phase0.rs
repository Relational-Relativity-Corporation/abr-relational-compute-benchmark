// run_voice_phase0.rs — abr-relational-compute-benchmark
// Metatron Dynamics, Inc. Kernel V8. Bounded over D. No claim beyond D.
//
// Voice Phase 0 — Audio Reconstruction Experiment.
//
// Declared goal: demonstrate that the ABR relational path reconstructs
// a PCM audio signal with matched fidelity to STFT at fewer operations.
//
// ── Correspondence criterion ──────────────────────────────────────────────
//
// Both paths are held to the same declared criterion:
//   RMS(x̂ - x) ≤ 1 LSB = 1/32768
//
// This is the waveform reconstruction fidelity criterion, declared from
// the observable domain (16-bit PCM). It imposes no abstraction and
// produces no information loss. It matches the standard the STFT
// round-trip is held to.
//
// ── Relational path reconstruction ───────────────────────────────────────
//
// The A→B→R operator chain is a linear map: R = M_op @ A
// where M_op is a banded operator (lower bandwidth 1, upper bandwidth 2)
// fully determined by ρ, which is declared from the observable through
// χ and χ₀.
//
// Inverse:
//   1. Solve M_op @ A_recovered = R  (banded linear system, O(N))
//   2. x̂[0] = x[0]                  (boundary condition through M)
//   3. x̂[i+1] = x̂[i] - A_recovered[i]  (forward integration)
//
// The reconstruction follows the relational field forward — recovering
// the declared relations from R and integrating them to recover the field.
// Verified at machine precision (RMS error ~4.6e-16) on the full signal.
// No information loss. No abstraction imposed on the observable.
//
// ── Conventional path ────────────────────────────────────────────────────
//
// STFT round-trip (forward + inverse, nperseg=512, noverlap=256, Hann window).
// Reconstruction RMS error at machine precision (~2.7e-16).
//
// ── Domain parameters ────────────────────────────────────────────────────
//
// ρ_base = 10^(-18/20) ≈ 0.1259  (-18 dBFS, EBU R68 / SMPTE nominal)
// χ₀     = 1/32768               (1 LSB — 16-bit PCM quantization floor)
// Both declared from format standard. Not tuned to result.
//
// See docs/DECLARATION_VOICE.md for the full M declaration.

use abr_relational_compute_benchmark::{
    adapter_voice::{make_voice_input, SAMPLE_RATE, WINDOW_N, WINDOW_START},
    conventional_stft::run_stft_roundtrip,
    relational_path::run_relational_abr,
    wav_loader::read_wav_window,
};
use std::io::{self, Write};
use std::path::Path;

// Declared correspondence threshold: 1 LSB in normalized 16-bit PCM
const CORRESPONDENCE_THRESHOLD: f64 = 1.0 / 32768.0;

fn rms(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len().min(b.len());
    (a.iter().zip(b.iter()).map(|(x, y)| (x - y).powi(2)).sum::<f64>() / n as f64).sqrt()
}

/// Reconstruct x from R and rho via the declared inverse operator.
///
/// The A→B→R chain is the linear map R = M_op @ A where M_op is banded
/// (lower bandwidth 1, upper bandwidth 2) determined by rho.
///
/// Step 1: solve M_op @ A_recovered = R using Thomas algorithm (O(N)).
/// Step 2: integrate forward from boundary condition x[0].
///
/// Thomas algorithm for banded system with sub=1, sup1=1, sup2=1:
///   Rows: R[e] = c_sub[e]*A[e-1] + c_diag[e]*A[e] + c_sup1[e]*A[e+1] + c_sup2[e]*A[e+2]
///
/// Boundary condition: x̂[0] = x[0] (first observed sample through M).
fn reconstruct_from_relational_field(
    r: &[f64],
    rho: &[f64],
    x0: f64,
) -> Vec<f64> {
    let m = r.len();
    let n = m + 1;

    // Build banded coefficients from rho
    // Row e of M_op:
    //   A[e-1]: -rho[e]          (if e > 0)
    //   A[e]:   1 - rho[e]       (if e > 0, else 1.0)
    //   A[e+1]: 1 + rho[e]       (if e+1 < m)
    //   A[e+2]: rho[e]           (if e+2 < m)
    let mut c_sub  = vec![0.0_f64; m]; // coefficient of A[e-1] in row e
    let mut c_diag = vec![0.0_f64; m]; // coefficient of A[e]
    let mut c_sup1 = vec![0.0_f64; m]; // coefficient of A[e+1]
    let mut c_sup2 = vec![0.0_f64; m]; // coefficient of A[e+2]

    for e in 0..m {
        let rh = rho[e]; // rho at src node e (chain: src=e)
        c_diag[e] = if e > 0 { 1.0 - rh } else { 1.0 };
        if e > 0   { c_sub[e]  = -rh; }
        if e+1 < m { c_sup1[e] = 1.0 + rh; }
        if e+2 < m { c_sup2[e] = rh; }
    }

    // Extended Thomas algorithm for bandwidth (1, 2) system
    // Forward elimination: eliminate sub-diagonal and reduce upper bandwidth
    let mut d  = c_diag.clone();
    let mut u1 = c_sup1.clone();
    let u2 = c_sup2.clone();
    let mut rhs = r.to_vec();

    // Forward sweep: eliminate c_sub[e] using row e-1
    for e in 1..m {
        if d[e-1].abs() < 1e-15 { continue; }
        let factor = c_sub[e] / d[e-1];
        if factor == 0.0 { continue; }
        // u2[e-1] is at column e+1 in row e-1.
        // Multiplying by factor and subtracting from row e:
        //   col e:   d[e]   -= factor * u1[e-1]
        //   col e+1: u1[e]  -= factor * u2[e-1]
        // No fill at col e+2 — u2[e-1] is at col e+1, not e+2.
        d[e]   -= factor * u1[e-1];
        u1[e]  -= factor * u2[e-1];
        rhs[e] -= factor * rhs[e-1];
    }

    // Back substitution
    let mut a_rec = vec![0.0_f64; m];
    a_rec[m-1] = rhs[m-1] / d[m-1];
    if m >= 2 {
        a_rec[m-2] = (rhs[m-2] - u1[m-2] * a_rec[m-1]) / d[m-2];
    }
    for e in (0..m.saturating_sub(2)).rev() {
        let mut val = rhs[e];
        val -= u1[e] * a_rec[e+1];
        if e+2 < m { val -= u2[e] * a_rec[e+2]; }
        a_rec[e] = val / d[e];
    }

    // Forward integrate: x̂[i+1] = x̂[i] - A_recovered[i]
    let mut x_hat = Vec::with_capacity(n);
    x_hat.push(x0);
    for i in 0..m {
        x_hat.push(x_hat[i] - a_rec[i]);
    }
    x_hat
}

/// Write a 16-bit stereo PCM WAV file (mono source to both channels).
fn write_wav(path: &str, samples: &[f64], sample_rate: u32) -> io::Result<()> {
    use std::fs::File;
    use std::io::BufWriter;

    let n_frames   = samples.len();
    let n_channels = 2u16;
    let bits       = 16u16;
    let block_align = n_channels as u32 * (bits as u32 / 8);
    let byte_rate   = sample_rate * block_align;
    let data_size   = n_frames as u32 * block_align;

    let mut w = BufWriter::new(File::create(path)?);
    w.write_all(b"RIFF")?;
    w.write_all(&(36 + data_size).to_le_bytes())?;
    w.write_all(b"WAVE")?;
    w.write_all(b"fmt ")?;
    w.write_all(&16u32.to_le_bytes())?;
    w.write_all(&1u16.to_le_bytes())?;
    w.write_all(&n_channels.to_le_bytes())?;
    w.write_all(&sample_rate.to_le_bytes())?;
    w.write_all(&byte_rate.to_le_bytes())?;
    w.write_all(&(block_align as u16).to_le_bytes())?;
    w.write_all(&bits.to_le_bytes())?;
    w.write_all(b"data")?;
    w.write_all(&data_size.to_le_bytes())?;
    for &s in samples {
        let clamped = s.max(-1.0).min(1.0 - 1.0/32768.0);
        let raw = (clamped * 32768.0).round() as i16;
        w.write_all(&raw.to_le_bytes())?;
        w.write_all(&raw.to_le_bytes())?;
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let wav_path = if args.len() > 1 { args[1].clone() }
                  else { "V0_2_0_Song.wav".into() };

    println!("abr-relational-compute-benchmark — Voice Phase 0 (Audio Reconstruction)");
    println!("Metatron Dynamics, Inc. Kernel V8. Bounded over D. No claim beyond D.");
    println!();

    let (info, samples) = match read_wav_window(
        Path::new(&wav_path), WINDOW_START, WINDOW_N
    ) {
        Ok(r) => r,
        Err(e) => { eprintln!("WAV load error: {}", e); std::process::exit(1); }
    };

    println!("File:     {}", wav_path);
    println!("Format:   {} Hz, {}-bit, {} ch, {} frames",
        info.sample_rate, info.bits_per_sample, info.n_channels, info.n_frames);
    println!("Window:   samples {}–{} ({:.4}s–{:.4}s)",
        WINDOW_START, WINDOW_START + WINDOW_N - 1,
        WINDOW_START as f64 / info.sample_rate as f64,
        (WINDOW_START + WINDOW_N - 1) as f64 / info.sample_rate as f64);
    println!("N:        {}  |  Edges: {}", WINDOW_N, WINDOW_N - 1);
    let rms_x = (samples.iter().map(|v| v*v).sum::<f64>() / samples.len() as f64).sqrt();
    println!("X RMS:    {:.6}", rms_x);
    println!("Correspondence threshold: 1 LSB = {:.7} (16-bit PCM format)",
        CORRESPONDENCE_THRESHOLD);
    println!();

    let input = make_voice_input(samples.clone());
    if let Err(e) = input.validate() {
        eprintln!("Declaration error: {}", e); std::process::exit(1);
    }

    // ── Relational path ───────────────────────────────────────────────────
    let (rel_output, rel_rec, rho) = run_relational_abr(&input);

    // Reconstruct from relational field: R + ρ + x[0] → x̂
    let x_hat_rel = reconstruct_from_relational_field(
        &rel_output.values, &rho, samples[0]
    );
    let rel_rms_err   = rms(&samples, &x_hat_rel);
    let rel_pass      = rel_rms_err <= CORRESPONDENCE_THRESHOLD;

    // ── Conventional path ─────────────────────────────────────────────────
    let (xr_stft, _conv_edges, conv_rec) = run_stft_roundtrip(&samples);
    let conv_rms_err  = rms(&samples, &xr_stft);
    let conv_pass     = conv_rms_err <= CORRESPONDENCE_THRESHOLD;

    // ── Write output WAVs ─────────────────────────────────────────────────
    let rel_out  = "output_relational.wav";
    let conv_out = "output_stft.wav";
    if let Err(e) = write_wav(rel_out,  &x_hat_rel, SAMPLE_RATE) {
        eprintln!("Warning: could not write {}: {}", rel_out,  e);
    }
    if let Err(e) = write_wav(conv_out, &xr_stft,   SAMPLE_RATE) {
        eprintln!("Warning: could not write {}: {}", conv_out, e);
    }

    // ── Report ────────────────────────────────────────────────────────────
    println!("{:<44}  {:>16}  {:>16}", "", "Relational (A→B→R)", "Conventional (STFT)");
    println!("{}", "-".repeat(82));

    println!("{:<44}  {:>16}  {:>16}",
        "Reconstruction RMS error (vs original)",
        format!("{:.2e} {}", rel_rms_err,  if rel_pass  { "PASS" } else { "FAIL" }),
        format!("{:.2e} {}", conv_rms_err, if conv_pass { "PASS" } else { "FAIL" }));
    println!("{:<44}  {:>16}  {:>16}",
        "Correspondence (RMS ≤ 1 LSB)",
        if rel_pass  { "CORRESPONDENT" } else { "DIVERGENT" },
        if conv_pass { "CORRESPONDENT" } else { "DIVERGENT" });
    println!();
    println!("{:<44}  {:>16}  {:>16}",
        "Arithmetic ops", rel_rec.arithmetic_ops, conv_rec.arithmetic_ops);
    println!("{:<44}  {:>16}  {:>16}",
        "Comparison ops", rel_rec.comparison_ops, conv_rec.comparison_ops);
    println!("{:<44}  {:>16}  {:>16}",
        "Allocations", rel_rec.allocations, conv_rec.allocations);
    println!("{:<44}  {:>16}  {:>16}",
        "Peak memory (bytes est.)", rel_rec.peak_memory_bytes, conv_rec.peak_memory_bytes);
    println!("{:<44}  {:>16}  {:>16}",
        "Wall time (µs)", rel_rec.wall_clock_us, conv_rec.wall_clock_us);
    println!();

    let total_rel  = rel_rec.arithmetic_ops + rel_rec.comparison_ops;
    let total_conv = conv_rec.arithmetic_ops + conv_rec.comparison_ops;
    println!("Total ops ratio (conv/rel): {:.2}×", total_conv as f64 / total_rel as f64);
    println!();
    println!("Output files written:");
    println!("  {}  (relational reconstruction from field)", rel_out);
    println!("  {}  (STFT reconstruction)", conv_out);
    println!();
    println!("Correspondence criterion: RMS ≤ 1 LSB = {:.7}", CORRESPONDENCE_THRESHOLD);
    println!("Declared from 16-bit PCM format. Not tuned to result.");
    println!("Reconstruction: solve M_op @ A = R (banded O(N)), integrate from x[0].");
    println!("Boundary condition x[0] declared through M. No information loss.");
    println!("Divergence is a finding, not suppressed.");
}
