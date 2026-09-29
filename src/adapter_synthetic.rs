// adapter_synthetic.rs — abr-relational-compute-benchmark
// Metatron Dynamics, Inc. Kernel V8. Bounded over D. No claim beyond D.
//
// Adapter: synthetic structured numerical field.
//
// ── M declaration (adapter, Phase 0) ─────────────────────────────────────
//
// X: a 1D gradient field of N declared loci.
//    x[i] = (N − 1 − i) as f64    (i = 0, …, N−1)
//    Each locus i occupies declared position i in a finite sequence.
//
// Observable organization: the N values occupy N declared positions in a
// finite ordered sequence. Position is part of the observable record.
// Adjacent positions (i, i+1) are directly observed as neighboring.
//
// Provenance termination (DECLARATION.md §2): the observed positional
// organization is sufficient provenance for the declared adjacency.
// Generator and Verifier must not demand an observation behind this
// observation.
//
// Direction: the sequence is declared left-to-right: i → i+1.
// This is a declared direction of relational comparison, not a causal claim.
// It represents the observed ordering of the sequence. The reverse direction
// (i+1 → i) is not independently declared and is therefore not used.
//
// Edge provenance: Continuation (within-family spatial continuation,
// declared from the observed positional organization).
//
// ── P declaration ─────────────────────────────────────────────────────────
//
// ρ_base = 0.3    (declared for this adapter)
// χ₀     = 1.0    (declared for this adapter; carries dimension of x[i])
//
// These are declared before running. They are not adjusted after results.

use crate::types::BenchmarkInput;

/// Construct a Phase 0 synthetic benchmark input of size N.
///
/// X: gradient field x[i] = (N − 1 − i) as f64.
/// E: directed chain i → i+1 for i in 0..N−1.
/// P: ρ_base = 0.3, χ₀ = 1.0.
pub fn make_synthetic_input(n: usize) -> BenchmarkInput {
    assert!(n >= 2, "N must be at least 2 to declare any edges");

    let x: Vec<f64> = (0..n).map(|i| (n - 1 - i) as f64).collect();
    let edges: Vec<(usize, usize)> = (0..n - 1).map(|i| (i, i + 1)).collect();
    let edge_provenance: Vec<String> = edges
        .iter()
        .map(|_| "Continuation".to_string())
        .collect();

    BenchmarkInput {
        x,
        m_label: format!(
            "synthetic-gradient-N{}: x[i]=(N-1-i), 1D directed chain i→i+1, \
             direction from observed positional ordering",
            n
        ),
        edges,
        edge_provenance,
        rho_base: 0.3,
        chi_0: 1.0,
    }
}
