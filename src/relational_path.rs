// relational_path.rs — abr-relational-compute-benchmark
// Metatron Dynamics, Inc. Kernel V8. Bounded over D. No claim beyond D.
//
// Relational path: M → D → declared topology → Kernel V8 full ABR (A→B→R).
//
// Kernel selection: the declared temporal chain has interior loci at every
// node except the endpoints. The full-operator admissibility condition is
// satisfied. A→B→R applies. The Primary kernel (Δ→Σ) does not apply here.
//
// Phase 1 spatial kernel only (no persistence): a single declared observation
// window is provided. Persistence (A_p→B_p→R_p) requires a declared pair of
// sequential observations through M. That condition is not met for a single
// window. operator_e is evaluated; operator_e_v5 is not.
//
// Single component, no component pairs declared: cc is empty. The
// cross-topology term in R does not execute.
//
// Operator forms from operators.rs. Not rewritten, renamed, or extended.
// Instrumentation counts wrap the same operations the kernel executes.
//
// For chain topology, adjacency is O(M) from the declared structure:
//   succ(e) = {e+1} if e+1 < M, else {}
//   pred(e) = {e-1} if e > 0,   else {}

use crate::types::{BenchmarkInput, ComputeRecord, PathOutput};
use std::time::Instant;

/// Execute the full ABR spatial kernel (A→B→R) over the declared input.
///
/// Returns (PathOutput over declared edges, ComputeRecord).
///
/// A(x)[e] = x[s] − x[t]  for each declared edge e = (s, t)
///
/// ρ[i] = ρ_base · χ[i] / (χ₀ + χ[i])
/// χ[i] = max{ |A(x)[e]| : e incident to i }
///
/// B(g)[e] = g[e] + Σ_{f ∈ succ(e)} g[f]
///   Terminal edges accumulate nothing.
///
/// R(g)[e] = g[e] + ρ[src(e)] · (Σ_{succ(e)} B(g) − Σ_{pred(e)} B(g))
///   No component pairs declared: cross-topology term does not execute.
pub fn run_relational_abr(input: &BenchmarkInput) -> (PathOutput, ComputeRecord) {
    let mut rec = ComputeRecord::new();
    let start = Instant::now();

    let n = input.n_nodes();
    let m = input.n_edges();
    let edges = &input.edges;
    let x = &input.x;
    let rho_base = input.rho_base;
    let chi_0 = input.chi_0;

    // ── A: directed difference ────────────────────────────────────────────
    // A(x)[e] = x[s] − x[t]
    rec.allocations += 1;
    let mut a: Vec<f64> = Vec::with_capacity(m);
    for &(s, t) in edges {
        a.push(x[s] - x[t]);
        rec.arithmetic_ops += 1;
    }

    // ── ρ: per-node χ from incident edges, then ρ[node] ──────────────────
    // χ[i] = max{ |A(x)[e]| : e incident to i }
    // ρ[i] = ρ_base · χ[i] / (χ₀ + χ[i])
    rec.allocations += 1;
    let mut node_chi: Vec<f64> = vec![0.0_f64; n];
    for e in 0..m {
        let av = a[e].abs();
        rec.arithmetic_ops += 1; // abs
        rec.comparison_ops += 2; // max at src and tgt
        let (s, t) = edges[e];
        if av > node_chi[s] { node_chi[s] = av; }
        if av > node_chi[t] { node_chi[t] = av; }
    }

    rec.allocations += 1;
    let mut rho: Vec<f64> = Vec::with_capacity(n);
    for i in 0..n {
        let chi = node_chi[i];
        rho.push(rho_base * chi / (chi_0 + chi));
        rec.arithmetic_ops += 3; // multiply, add, divide
    }

    // ── B: accumulate forward successors ─────────────────────────────────
    // B(g)[e] = g[e] + Σ_{f ∈ succ(e)} g[f]
    // Chain topology: succ(e) = {e+1} if e+1 < M, else {}
    rec.allocations += 1;
    let mut b: Vec<f64> = Vec::with_capacity(m);
    for e in 0..m {
        let succ = if e + 1 < m { a[e + 1] } else { 0.0 };
        b.push(a[e] + succ);
        rec.arithmetic_ops += 1; // add
    }

    // ── R: relational recurrence over B ──────────────────────────────────
    // R(g)[e] = g[e] + ρ[src(e)] · (Σ_{succ(e)} B − Σ_{pred(e)} B)
    // Chain topology: succ(e) = {e+1}, pred(e) = {e-1}
    // No component pairs: cross-topology term does not execute.
    rec.allocations += 1;
    let mut r: Vec<f64> = Vec::with_capacity(m);
    for e in 0..m {
        let src = edges[e].0;
        let fwd = if e + 1 < m { b[e + 1] } else { 0.0 };
        let bwd = if e > 0     { b[e - 1] } else { 0.0 };
        r.push(b[e] + rho[src] * (fwd - bwd));
        rec.arithmetic_ops += 3; // subtract, multiply, add
    }

    rec.wall_clock_us = start.elapsed().as_micros() as u64;
    rec.peak_memory_bytes = ((n + m + n + m + m) * 8) as u64;

    (PathOutput { values: r, n_edges: m }, rec)
}
