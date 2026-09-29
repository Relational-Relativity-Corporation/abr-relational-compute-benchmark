// relational_path.rs — abr-relational-compute-benchmark
// Metatron Dynamics, Inc. Kernel V8. Bounded over D. No claim beyond D.
//
// Relational path: M → D → declared topology → Kernel V8 (Primary: Δ → Σ).
//
// Phase 0 uses the Primary kernel. The ABR full path (A → B → R) is Phase 1.
//
// Operator forms are taken verbatim from operators.rs. They are not rewritten,
// renamed, or extended. The instrumentation counters wrap the same operations
// the kernel executes.

use crate::types::{BenchmarkInput, ComputeRecord, PathOutput};
use std::time::Instant;

/// Execute the Primary kernel path (Δ → Σ) over the declared input.
///
/// Returns (PathOutput, ComputeRecord).
///
/// Δ(x)[e] = x[s] − x[t]  for each declared edge e = (s, t).
/// Σ(g)[e] = g[e] + ρ[e] · (Σ_{adj⁺(e)} g[e'] − Σ_{adj⁻(e)} g[e'])
///
/// ρ[e] = ρ_base · χ[s] / (χ₀ + χ[s])
/// χ[s] = max{ |Δ(x)[e']| : e' incident to s }
///
/// All formulas from operators.rs. Instrumentation counts wrap each operation.
pub fn run_relational_primary(input: &BenchmarkInput) -> (PathOutput, ComputeRecord) {
    let mut rec = ComputeRecord::new();
    let start = Instant::now();

    let n = input.n_nodes();
    let m = input.n_edges();
    let edges = &input.edges;
    let x = &input.x;
    let rho_base = input.rho_base;
    let chi_0 = input.chi_0;

    // ── Δ: directed difference ────────────────────────────────────────────
    // Δ(x)[e] = x[s] − x[t]
    rec.allocations += 1;
    let mut delta: Vec<f64> = Vec::with_capacity(m);
    for &(s, t) in edges {
        delta.push(x[s] - x[t]);
        rec.arithmetic_ops += 1;
    }

    // ── ρ: compute per-node χ, then ρ[e] at source of each edge ──────────
    // χ[i] = max{ |Δ(x)[e']| : e' incident to i }
    // ρ[e] = ρ_base · χ[src(e)] / (χ₀ + χ[src(e)])

    // Build node → incident edges index
    rec.allocations += 1;
    let mut node_incident: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (e, &(s, t)) in edges.iter().enumerate() {
        node_incident[s].push(e);
        node_incident[t].push(e);
    }

    rec.allocations += 1;
    let mut node_chi: Vec<f64> = vec![0.0_f64; n];
    for i in 0..n {
        for &e in &node_incident[i] {
            let abs_val = delta[e].abs();
            rec.arithmetic_ops += 1; // abs
            rec.comparison_ops += 1; // max comparison
            if abs_val > node_chi[i] {
                node_chi[i] = abs_val;
            }
        }
    }

    rec.allocations += 1;
    let mut rho: Vec<f64> = Vec::with_capacity(m);
    for &(s, _) in edges {
        let chi = node_chi[s];
        // ρ_base · chi / (chi_0 + chi)
        let r = rho_base * chi / (chi_0 + chi);
        rec.arithmetic_ops += 3; // multiply, add, divide
        rho.push(r);
    }

    // ── adj⁺ and adj⁻ for Σ ──────────────────────────────────────────────
    // adj⁺[e] = edges f where source(f) = target(e)
    // adj⁻[e] = edges f where target(f) = source(e)
    rec.allocations += 2;
    let mut adj_plus: Vec<Vec<usize>> = vec![Vec::new(); m];
    let mut adj_minus: Vec<Vec<usize>> = vec![Vec::new(); m];
    for (e, &(_, et)) in edges.iter().enumerate() {
        for (f, &(fs, _)) in edges.iter().enumerate() {
            rec.comparison_ops += 1;
            if fs == et { adj_plus[e].push(f); }
        }
    }
    for (e, &(es, _)) in edges.iter().enumerate() {
        for (f, &(_, ft)) in edges.iter().enumerate() {
            rec.comparison_ops += 1;
            if ft == es { adj_minus[e].push(f); }
        }
    }
    // adj construction: O(m²) comparisons — declared and counted.

    // ── Σ: relational accumulation ─────────────────────────────────────────
    // Σ(g)[e] = g[e] + ρ[e] · (Σ_{adj⁺} g − Σ_{adj⁻} g)
    rec.allocations += 1;
    let mut sigma: Vec<f64> = Vec::with_capacity(m);
    for e in 0..m {
        let fwd: f64 = adj_plus[e].iter().map(|&f| delta[f]).sum();
        let bwd: f64 = adj_minus[e].iter().map(|&p| delta[p]).sum();
        rec.arithmetic_ops += adj_plus[e].len() as u64;  // fwd sum
        rec.arithmetic_ops += adj_minus[e].len() as u64; // bwd sum
        let val = delta[e] + rho[e] * (fwd - bwd);
        rec.arithmetic_ops += 3; // subtract, multiply, add
        sigma.push(val);
    }

    rec.wall_clock_us = start.elapsed().as_micros() as u64;

    // Peak memory estimate: x (n f64) + delta (m) + node_chi (n) + rho (m)
    //   + sigma (m) + adjacency structures (rough: 2*m*avg_degree)
    rec.peak_memory_bytes = ((n + m + n + m + m) * 8) as u64;

    (
        PathOutput { values: sigma, n_edges: m },
        rec,
    )
}
