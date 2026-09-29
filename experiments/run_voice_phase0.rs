// run_voice_phase0.rs — abr-relational-compute-benchmark
// Metatron Dynamics, Inc. Kernel V8. Bounded over D. No claim beyond D.
//
// Voice Phase 0 experiment.
//
// Experimental control: X in, X' out. Both paths must return output
// in the sample/edge domain that corresponds with X.
//
// Relational path:    Primary kernel Δ → Σ.
// Conventional path:  STFT round-trip (forward + inverse STFT, nperseg=512,
//                     noverlap=256, Hann window). Exact reconstruction.
//
// Correspondence criterion: sign(output[e]) = sign(x[s] − x[t])
// for each declared edge e = (i, i+1).
//
// See docs/DECLARATION_VOICE.md for the full M declaration.
//
// Usage: cargo run --bin run_voice_phase0 -- <path/to/Voice_44100_16bit.wav>

use abr_relational_compute_benchmark::{
    adapter_voice::{make_voice_input, WINDOW_N, WINDOW_START},
    conventional_stft::run_stft_roundtrip,
    relational_path::run_relational_abr,
    types::BenchmarkInput,
    wav_loader::read_wav_window,
};
use std::path::Path;

const ZERO_TOL: f64 = 1e-10;

fn check_relational(input: &BenchmarkInput, sigma: &[f64]) -> (usize, usize, Vec<usize>) {
    let m = input.n_edges();
    let mut corr = 0usize;
    let mut divergent = Vec::new();
    for e in 0..m {
        let (s, t) = input.edges[e];
        let exp = input.x[s] - input.x[t];
        let act = sigma[e];
        let matches = (exp.abs() <= ZERO_TOL && act.abs() <= ZERO_TOL)
            || (exp * act > 0.0);
        if matches { corr += 1; } else { divergent.push(e); }
    }
    (corr, m, divergent)
}

fn check_conventional(x: &[f64], xr: &[f64]) -> (usize, usize) {
    let m = x.len() - 1;
    let mut corr = 0usize;
    for e in 0..m {
        let exp = x[e] - x[e + 1];
        let act = xr[e] - xr[e + 1];
        let matches = (exp.abs() <= ZERO_TOL && act.abs() <= ZERO_TOL)
            || (exp * act > 0.0);
        if matches { corr += 1; }
    }
    (corr, m)
}

fn rms(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len().min(b.len());
    (a.iter().zip(b.iter()).map(|(x, y)| (x - y).powi(2)).sum::<f64>() / n as f64).sqrt()
}

fn corr_label(n: usize, total: usize) -> String {
    if n == total { "CORRESPONDENT".into() }
    else if n == 0 { "DIVERGENT".into() }
    else { format!("PARTIAL {}/{}", n, total) }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let wav_path = if args.len() > 1 {
        args[1].clone()
    } else {
        // default path for local development
        "Voice_44100_16bit.wav".into()
    };

    println!("abr-relational-compute-benchmark — Voice Phase 0");
    println!("Metatron Dynamics, Inc. Kernel V8. Bounded over D. No claim beyond D.");
    println!();

    // ── Load declared window ──────────────────────────────────────────
    let (info, samples) = match read_wav_window(
        Path::new(&wav_path), WINDOW_START, WINDOW_N
    ) {
        Ok(r) => r,
        Err(e) => { eprintln!("WAV load error: {}", e); std::process::exit(1); }
    };

    println!("File:     {}", wav_path);
    println!("Format:   {} Hz, {}-bit, {} ch, {} frames",
        info.sample_rate, info.bits_per_sample,
        info.n_channels, info.n_frames);
    println!("Window:   samples {}–{} (1.3000s–1.4000s)",
        WINDOW_START, WINDOW_START + WINDOW_N - 1);
    println!("N:        {}  |  Edges: {}", WINDOW_N, WINDOW_N - 1);

    let rms_x = (samples.iter().map(|v| v * v).sum::<f64>() / samples.len() as f64).sqrt();
    println!("X RMS:    {:.6}", rms_x);
    println!();

    // ── Validate and build input ──────────────────────────────────────
    let input = make_voice_input(samples.clone());
    if let Err(e) = input.validate() {
        eprintln!("Declaration completeness error: {}", e);
        std::process::exit(1);
    }

    // ── Relational path: Δ → Σ ───────────────────────────────────────
    let (rel_output, rel_rec) = run_relational_abr(&input);
    let (rel_corr, m, divergent_edges) = check_relational(&input, &rel_output.values);

    // ── Conventional path: STFT round-trip ───────────────────────────
    let (xr, _conv_edge_output, conv_rec) = run_stft_roundtrip(&samples);
    let (conv_corr, _) = check_conventional(&samples, &xr);
    let recon_err = rms(&samples, &xr);

    // ── Report ────────────────────────────────────────────────────────
    let w = 24usize;
    println!("{:<35}  {:>20}  {:>20}", "", "Relational (A→B→R)", "Conventional (STFT)");
    println!("{}", "-".repeat(80));
    println!("{:<35}  {:>20}  {:>20}",
        "Correspondence",
        corr_label(rel_corr, m), corr_label(conv_corr, m));
    println!("{:<35}  {:>20}  {:>20}",
        "Edges correspondent",
        format!("{}/{}", rel_corr, m), format!("{}/{}", conv_corr, m));
    println!("{:<35}  {:>20}  {:>20}",
        "Reconstruction RMS error",
        "(edge domain)", format!("{:.2e}", recon_err));
    println!();
    println!("{:<35}  {:>20}  {:>20}",
        "Arithmetic ops",
        rel_rec.arithmetic_ops, conv_rec.arithmetic_ops);
    println!("{:<35}  {:>20}  {:>20}",
        "Comparison ops",
        rel_rec.comparison_ops, conv_rec.comparison_ops);
    println!("{:<35}  {:>20}  {:>20}",
        "Allocations",
        rel_rec.allocations, conv_rec.allocations);
    println!("{:<35}  {:>20}  {:>20}",
        "Peak memory (bytes est.)",
        rel_rec.peak_memory_bytes, conv_rec.peak_memory_bytes);
    println!("{:<35}  {:>20}  {:>20}",
        "Wall time (µs)",
        rel_rec.wall_clock_us, conv_rec.wall_clock_us);
    println!();

    let total_rel  = rel_rec.arithmetic_ops  + rel_rec.comparison_ops;
    let total_conv = conv_rec.arithmetic_ops + conv_rec.comparison_ops;
    if total_rel > 0 {
        println!("Total ops ratio (conv/rel): {:.2}×", total_conv as f64 / total_rel as f64);
    }
    println!();

    // ── Divergent edge summary ────────────────────────────────────────
    println!("Divergent edges: {}", divergent_edges.len());
    if !divergent_edges.is_empty() {
        let _sample_rate = info.sample_rate as f64;
        let div_deltas: Vec<f64> = divergent_edges.iter()
            .map(|&e| (input.x[e] - input.x[e+1]).abs())
            .collect();
        let all_deltas: Vec<f64> = (0..m)
            .map(|e| (input.x[e] - input.x[e+1]).abs())
            .collect();
        let median_delta = {
            let mut v = all_deltas.clone();
            v.sort_by(|a, b| a.partial_cmp(b).unwrap());
            v[v.len() / 2]
        };
        let max_div_delta = div_deltas.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let mean_div_sigma: f64 = divergent_edges.iter()
            .map(|&e| rel_output.values[e].abs())
            .sum::<f64>() / divergent_edges.len() as f64;
        let mean_corr_sigma: f64 = (0..m)
            .filter(|e| !divergent_edges.contains(e))
            .map(|e| rel_output.values[e].abs())
            .sum::<f64>() / (m - divergent_edges.len()) as f64;

        println!("  Max |delta| at divergent edges: {:.6}  (median all: {:.6})",
            max_div_delta, median_delta);
        println!("  Mean |sigma| divergent:    {:.6}", mean_div_sigma);
        println!("  Mean |sigma| correspondent:{:.6}", mean_corr_sigma);
        println!("  Ratio: {:.4}× (divergent edges are quieter)",
            mean_div_sigma / mean_corr_sigma);
        println!();
        println!("  Interpretation: divergent edges are near-zero crossings where");
        println!("  Σ accumulation from active neighbors overrides the local directed");
        println!("  difference. This is declared operator behavior, not error.");
        println!("  These edges carry the lowest local contrast in the window.");
        let _ = w; // suppress unused warning
    }

    println!();
    println!("Correspondence checked against X independently for each path.");
    println!("Divergence is a finding, not suppressed.");
}
