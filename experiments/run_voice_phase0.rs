// run_voice_phase0.rs — abr-relational-compute-benchmark
// Metatron Dynamics, Inc. Kernel V8. Bounded over D. No claim beyond D.
//
// Voice Phase 0 experiment.
//
// Experimental control: X in, X' out. Both paths checked against X.
//
// Relational path:    Full ABR spatial kernel A→B→R.
// Conventional path:  STFT round-trip (forward + inverse, nperseg=512,
//                     noverlap=256, Hann window). Exact reconstruction.
//
// Correspondence criterion: amplitude-weighted.
//   Weight of each edge = |A[e]| = |x[s] - x[t]|, the observed directed
//   difference magnitude declared through M.
//   Weighted correspondence = Σ|A[e]| over correspondent edges
//                           / Σ|A[e]| over all edges.
//   Binary edge count also reported.
//
// See docs/DECLARATION_VOICE.md for the full M declaration.

use abr_relational_compute_benchmark::{
    adapter_voice::{make_voice_input, WINDOW_N, WINDOW_START},
    conventional_stft::run_stft_roundtrip,
    correspondence::check_correspondence_weighted,
    relational_path::run_relational_abr,
    wav_loader::read_wav_window,
};
use std::path::Path;

const ZERO_TOL: f64 = 1e-10;

fn weighted_correspondence_stft(x: &[f64], xr: &[f64]) -> (f64, usize, usize, Vec<usize>) {
    let m = x.len() - 1;
    let mut total_w = 0.0_f64;
    let mut w_corr  = 0.0_f64;
    let mut b_corr  = 0usize;
    let mut div     = Vec::new();
    for e in 0..m {
        let a_e = x[e] - x[e+1];
        let r_e = xr[e] - xr[e+1];
        let w   = a_e.abs();
        total_w += w;
        let ok = (a_e.abs() <= ZERO_TOL && r_e.abs() <= ZERO_TOL)
               || (a_e * r_e > 0.0);
        if ok { w_corr += w; b_corr += 1; }
        else  { div.push(e); }
    }
    let pct = if total_w > ZERO_TOL { 100.0 * w_corr / total_w } else { 100.0 };
    (pct, b_corr, m, div)
}

fn rms(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len().min(b.len());
    (a.iter().zip(b.iter()).map(|(x,y)|(x-y).powi(2)).sum::<f64>() / n as f64).sqrt()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let wav_path = if args.len() > 1 { args[1].clone() }
                  else { "Voice_44100_16bit.wav".into() };

    println!("abr-relational-compute-benchmark — Voice Phase 0");
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
    println!("Window:   samples {}–{} (1.3000s–1.4000s)", WINDOW_START, WINDOW_START+WINDOW_N-1);
    println!("N:        {}  |  Edges: {}", WINDOW_N, WINDOW_N-1);

    let rms_x = (samples.iter().map(|v|v*v).sum::<f64>() / samples.len() as f64).sqrt();
    println!("X RMS:    {:.6}", rms_x);
    println!();

    let input = make_voice_input(samples.clone());
    if let Err(e) = input.validate() {
        eprintln!("Declaration completeness error: {}", e); std::process::exit(1);
    }

    // ── Relational path ───────────────────────────────────────────────────
    let (rel_output, rel_rec) = run_relational_abr(&input);
    let (rel_wpct, rel_bcorr, m, rel_div) =
        check_correspondence_weighted(&input, &rel_output);

    // ── Conventional path ─────────────────────────────────────────────────
    let (xr, _conv_edges, conv_rec) = run_stft_roundtrip(&samples);
    let recon_err = rms(&samples, &xr);
    let (conv_wpct, conv_bcorr, _, conv_div) =
        weighted_correspondence_stft(&samples, &xr);

    // ── Report ────────────────────────────────────────────────────────────
    println!("{:<38}  {:>20}  {:>20}", "", "Relational (A→B→R)", "Conventional (STFT)");
    println!("{}", "-".repeat(82));
    println!("{:<38}  {:>20.1}%  {:>19.1}%",
        "Weighted correspondence (primary)",  rel_wpct, conv_wpct);
    println!("{:<38}  {:>20}  {:>20}",
        "Binary correspondence",
        format!("{}/{}", rel_bcorr, m), format!("{}/{}", conv_bcorr, m));
    println!("{:<38}  {:>20}  {:>20}",
        "Reconstruction RMS error", "(edge domain)", format!("{:.2e}", recon_err));
    println!();
    println!("{:<38}  {:>20}  {:>20}",
        "Arithmetic ops", rel_rec.arithmetic_ops, conv_rec.arithmetic_ops);
    println!("{:<38}  {:>20}  {:>20}",
        "Comparison ops", rel_rec.comparison_ops, conv_rec.comparison_ops);
    println!("{:<38}  {:>20}  {:>20}",
        "Allocations", rel_rec.allocations, conv_rec.allocations);
    println!("{:<38}  {:>20}  {:>20}",
        "Peak memory (bytes est.)", rel_rec.peak_memory_bytes, conv_rec.peak_memory_bytes);
    println!("{:<38}  {:>20}  {:>20}",
        "Wall time (µs)", rel_rec.wall_clock_us, conv_rec.wall_clock_us);
    println!();

    let total_rel  = rel_rec.arithmetic_ops  + rel_rec.comparison_ops;
    let total_conv = conv_rec.arithmetic_ops + conv_rec.comparison_ops;
    println!("Total ops ratio (conv/rel): {:.2}×", total_conv as f64 / total_rel as f64);
    println!();

    // ── Divergent edge summary ─────────────────────────────────────────────
    println!("Divergent edges (binary): relational={}, conventional={}",
        rel_div.len(), conv_div.len());

    if !rel_div.is_empty() {
        let x = &input.x;
        let edges = &input.edges;
        let div_weights: Vec<f64> = rel_div.iter()
            .map(|&e| { let (s,t) = edges[e]; (x[s]-x[t]).abs() })
            .collect();
        let all_weight: f64 = (0..m)
            .map(|e| { let (s,t) = edges[e]; (x[s]-x[t]).abs() })
            .sum();
        let div_weight_sum: f64 = div_weights.iter().sum();
        let mean_div  = div_weights.iter().sum::<f64>() / div_weights.len() as f64;
        let corr_weights: Vec<f64> = (0..m)
            .filter(|e| !rel_div.contains(e))
            .map(|e| { let (s,t) = edges[e]; (x[s]-x[t]).abs() })
            .collect();
        let mean_corr = corr_weights.iter().sum::<f64>() / corr_weights.len() as f64;

        println!("  Divergent edge amplitude share: {:.1}% of total signal weight",
            100.0 * div_weight_sum / all_weight);
        println!("  Mean |A[e]| divergent:    {:.6}", mean_div);
        println!("  Mean |A[e]| correspondent:{:.6}", mean_corr);
        println!("  Ratio: {:.4}×", mean_div / mean_corr);
    }

    println!();
    println!("Amplitude-weighted correspondence is the primary declared criterion.");
    println!("Weight = |A[e]| = observed directed difference magnitude through M.");
    println!("Correspondence checked against X independently for each path.");
    println!("Divergence is a finding, not suppressed.");
}
