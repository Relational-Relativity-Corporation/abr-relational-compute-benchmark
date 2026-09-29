// correspondence.rs — abr-relational-compute-benchmark
// Metatron Dynamics, Inc. Kernel V8. Bounded over D. No claim beyond D.
//
// Correspondence criterion — Phase 0 (declared in DECLARATION.md):
//
//   For each declared edge e = (s, t):
//   sign(output[e]) = sign(X[s] − X[t])
//
//   Both paths are checked against X independently.
//   The two paths are not compared against each other.
//
// Implementation-only numerical tolerance: 1e-10 for zero comparisons.
// This tolerance is not a property of the observed system.

use crate::types::{BenchmarkInput, CorrespondenceState, PathOutput};

const ZERO_TOL: f64 = 1e-10;

/// Check whether a path output corresponds with X under the Phase 0 criterion.
///
/// For each declared edge e = (s, t):
///   expected_sign = sign(X[s] − X[t])
///   actual_sign   = sign(output.values[e])
///
/// Returns CorrespondenceState.
pub fn check_correspondence(
    input: &BenchmarkInput,
    output: &PathOutput,
) -> CorrespondenceState {
    let edges = &input.edges;
    let x = &input.x;

    assert_eq!(output.values.len(), edges.len(),
        "output length must match declared edge count");

    let total = edges.len();
    let mut corresponding = 0usize;

    for (e, &(s, t)) in edges.iter().enumerate() {
        let expected = x[s] - x[t];
        let actual   = output.values[e];

        // Both zero: corresponding (no distinction to preserve).
        // Sign match: corresponding.
        // Sign mismatch: not corresponding.
        let matches = if expected.abs() <= ZERO_TOL && actual.abs() <= ZERO_TOL {
            true
        } else if expected.abs() <= ZERO_TOL || actual.abs() <= ZERO_TOL {
            // One zero, one not — check if near-zero
            expected.abs() <= ZERO_TOL && actual.abs() <= ZERO_TOL
        } else {
            expected.signum() == actual.signum()
        };

        if matches { corresponding += 1; }
    }

    if corresponding == total {
        CorrespondenceState::Correspondent
    } else if corresponding == 0 {
        CorrespondenceState::Divergent
    } else {
        CorrespondenceState::PartiallyCorrespondent { corresponding, total }
    }
}
