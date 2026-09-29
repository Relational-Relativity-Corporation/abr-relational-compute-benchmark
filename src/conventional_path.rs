// conventional_path.rs — abr-relational-compute-benchmark
// Metatron Dynamics, Inc. Kernel V8. Bounded over D. No claim beyond D.
//
// Conventional path — Phase 0: mean subtraction followed by normalization.
//
// Declaration (DECLARATION.md):
//   This is the minimal conventional reduction that meets the Phase 0 output
//   requirement: preserve differentiable structure across the field. It is
//   not a heavyweight pipeline. It was declared before running. It is not
//   adjusted after seeing results.
//
// What it computes:
//   mean = (1/n) · Σᵢ x[i]
//   centered[i] = x[i] − mean
//   std  = sqrt((1/n) · Σᵢ centered[i]²)     (or 1.0 if std ≈ 0)
//   norm[i] = centered[i] / std
//
//   output[e] = norm[s] − norm[t]  for each declared edge e = (s, t)
//
// The output is produced over the same declared edge list as the relational
// path, so correspondence is checked on the same set of (s, t) pairs.

use crate::types::{BenchmarkInput, ComputeRecord, PathOutput};
use std::time::Instant;

pub fn run_conventional(input: &BenchmarkInput) -> (PathOutput, ComputeRecord) {
    let mut rec = ComputeRecord::new();
    let start = Instant::now();

    let n = input.n_nodes();
    let m = input.n_edges();
    let x = &input.x;
    let edges = &input.edges;

    // Mean
    let sum: f64 = x.iter().sum();
    rec.arithmetic_ops += n as u64; // n additions
    let mean = sum / n as f64;
    rec.arithmetic_ops += 1; // divide

    // Center
    rec.allocations += 1;
    let centered: Vec<f64> = x.iter().map(|v| v - mean).collect();
    rec.arithmetic_ops += n as u64; // n subtractions

    // Std
    let var_sum: f64 = centered.iter().map(|v| v * v).sum();
    rec.arithmetic_ops += 2 * n as u64; // n multiplies + n additions
    let variance = var_sum / n as f64;
    rec.arithmetic_ops += 1;
    let std = variance.sqrt();
    rec.arithmetic_ops += 1;

    // Normalize
    rec.allocations += 1;
    let norm: Vec<f64> = if std > 1e-10 {
        rec.comparison_ops += 1;
        centered.iter().map(|v| {
            rec.arithmetic_ops += 1;
            v / std
        }).collect()
    } else {
        rec.comparison_ops += 1;
        centered.clone()
    };

    // Output: directed difference of normalized values over declared edges
    rec.allocations += 1;
    let values: Vec<f64> = edges.iter().map(|&(s, t)| {
        rec.arithmetic_ops += 1;
        norm[s] - norm[t]
    }).collect();

    rec.wall_clock_us = start.elapsed().as_micros() as u64;

    // Peak memory: x (already allocated by caller), centered (n), norm (n),
    // values (m)
    rec.peak_memory_bytes = ((n + n + m) * 8) as u64;

    (
        PathOutput { values, n_edges: m },
        rec,
    )
}
