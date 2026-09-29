// correspondence.rs — abr-relational-compute-benchmark
// Metatron Dynamics, Inc. Kernel V8. Bounded over D. No claim beyond D.
//
// Correspondence criterion — amplitude-weighted (declared 2026-09-29).
//
// The observable provides amplitude through M. The amplitude of each
// declared relation |A[e]| = |x[s] - x[t]| is the physical weight of
// that relation. Correspondence is evaluated weighted by that amplitude.
//
// Weighted correspondence = Σ|A[e]| over correspondent edges
//                         / Σ|A[e]| over all edges
//
// This is not a threshold chosen to improve a score. It is the amplitude
// information M already provides, applied consistently to the comparison.
// A sign correspondence at |A[e]| = 0.0001 and at |A[e]| = 0.4 are not
// equivalent observational events. The physical weight distinguishes them.
//
// Binary correspondence (edge count) is also reported for completeness.
// The amplitude-weighted result is the primary declared criterion.
//
// Implementation-only tolerance: 1e-10 for zero comparisons.
// Not a property of the observed system.

use crate::types::{BenchmarkInput, CorrespondenceState, PathOutput};

const ZERO_TOL: f64 = 1e-10;

/// Compute amplitude-weighted and binary correspondence.
///
/// Returns (weighted_pct, binary_correspondent, total_edges, divergent_indices).
pub fn check_correspondence_weighted(
    input: &BenchmarkInput,
    output: &PathOutput,
) -> (f64, usize, usize, Vec<usize>) {
    let edges = &input.edges;
    let x = &input.x;
    let m = edges.len();

    assert_eq!(output.values.len(), m,
        "output length must match declared edge count");

    let mut total_weight   = 0.0_f64;
    let mut weight_corr    = 0.0_f64;
    let mut binary_corr    = 0usize;
    let mut divergent      = Vec::new();

    for e in 0..m {
        let (s, t) = edges[e];
        let a_e  = x[s] - x[t];
        let r_e  = output.values[e];
        let w    = a_e.abs();

        total_weight += w;

        let correspondent =
            (a_e.abs() <= ZERO_TOL && r_e.abs() <= ZERO_TOL)
            || (a_e * r_e > 0.0);

        if correspondent {
            weight_corr += w;
            binary_corr += 1;
        } else {
            divergent.push(e);
        }
    }

    let weighted_pct = if total_weight > ZERO_TOL {
        100.0 * weight_corr / total_weight
    } else {
        100.0
    };

    (weighted_pct, binary_corr, m, divergent)
}

/// Legacy binary check — kept for synthetic experiment compatibility.
pub fn check_correspondence(
    input: &BenchmarkInput,
    output: &PathOutput,
) -> CorrespondenceState {
    let (_, binary_corr, total, _) = check_correspondence_weighted(input, output);
    if binary_corr == total {
        CorrespondenceState::Correspondent
    } else if binary_corr == 0 {
        CorrespondenceState::Divergent
    } else {
        CorrespondenceState::PartiallyCorrespondent {
            corresponding: binary_corr,
            total,
        }
    }
}
