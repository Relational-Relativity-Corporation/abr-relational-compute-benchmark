// phase0_tests.rs — abr-relational-compute-benchmark
// Metatron Dynamics, Inc. Kernel V8. Bounded over D. No claim beyond D.
//
// Phase 0 test suite.
//
// Test classification (Kernel V8 convention):
//   [IMPL]  — implementation verification
//   [MATH]  — mathematical verification of declared formula
//   [REPR]  — forward reproducibility / determinism
//   [CORR]  — correspondence check

use abr_relational_compute_benchmark::{
    adapter_synthetic::make_synthetic_input,
    conventional_path::run_conventional,
    correspondence::check_correspondence,
    relational_path::run_relational_primary,
    run_experiment,
    types::CorrespondenceState,
};

// ── Input validation ──────────────────────────────────────────────────────

// [IMPL] Valid input passes validation.
#[test]
fn valid_input_passes_validation() {
    let input = make_synthetic_input(10);
    assert!(input.validate().is_ok());
}

// [IMPL] Empty X is rejected.
#[test]
fn empty_x_rejected() {
    let mut input = make_synthetic_input(4);
    input.x.clear();
    assert!(input.validate().is_err());
}

// [IMPL] Non-finite value in X is rejected (not in D).
#[test]
fn non_finite_x_rejected() {
    let mut input = make_synthetic_input(4);
    input.x[1] = f64::NAN;
    assert!(input.validate().is_err());
}

// [IMPL] χ₀ = 0 is rejected (not evaluable).
#[test]
fn chi_0_zero_rejected() {
    let mut input = make_synthetic_input(4);
    input.chi_0 = 0.0;
    assert!(input.validate().is_err());
}

// [IMPL] χ₀ < 0 is rejected.
#[test]
fn chi_0_negative_rejected() {
    let mut input = make_synthetic_input(4);
    input.chi_0 = -1.0;
    assert!(input.validate().is_err());
}

// [IMPL] Self-loop edge is rejected (ring prohibition).
#[test]
fn self_loop_edge_rejected() {
    let mut input = make_synthetic_input(4);
    input.edges[0] = (0, 0);
    assert!(input.validate().is_err());
}

// ── Relational path — Δ ──────────────────────────────────────────────────

// [MATH] Δ(x)[e] = x[s] − x[t] for each declared edge.
// Gradient field: x[i] = N−1−i. Edge (0,1): x[0]−x[1] = (N−1) − (N−2) = 1.
#[test]
fn relational_delta_directed_difference() {
    let input = make_synthetic_input(4);
    // x = [3, 2, 1, 0]. Edge (0,1): Δ = 3−2 = 1.
    // After Σ accumulation, output[0] ≥ Δ[0] because adj⁺(0) adds forward
    // contributions. Sign must be preserved.
    let (out, _) = run_relational_primary(&input);
    // All outputs should be positive (gradient descends left to right,
    // all directed differences are positive).
    assert!(out.values.iter().all(|&v| v > 0.0),
        "all Σ outputs must be positive for a strictly descending gradient field");
}

// [MATH] Uniform field: Δ = 0 everywhere; Σ = 0 everywhere.
#[test]
fn relational_uniform_field_zero_output() {
    let n = 10;
    let mut input = make_synthetic_input(n);
    input.x = vec![5.0; n]; // uniform
    let (out, _) = run_relational_primary(&input);
    assert!(out.values.iter().all(|&v| v.abs() < 1e-12),
        "uniform field must produce zero output from Δ → Σ");
}

// [REPR] Two independent applications from identical declared inputs agree exactly.
#[test]
fn relational_determinism() {
    let input = make_synthetic_input(100);
    let (out1, _) = run_relational_primary(&input);
    let (out2, _) = run_relational_primary(&input);
    assert_eq!(out1.values, out2.values,
        "two forward applications from identical declared inputs must be identical");
}

// ── Conventional path ─────────────────────────────────────────────────────

// [IMPL] Conventional path produces output of correct length.
#[test]
fn conventional_output_length() {
    let input = make_synthetic_input(100);
    let (out, _) = run_conventional(&input);
    assert_eq!(out.n_edges, input.n_edges());
    assert_eq!(out.values.len(), input.n_edges());
}

// [MATH] Conventional: uniform field produces zero output (mean = value,
// centered = 0 everywhere, differences = 0).
#[test]
fn conventional_uniform_field_zero_output() {
    let n = 10;
    let mut input = make_synthetic_input(n);
    input.x = vec![3.0; n];
    let (out, _) = run_conventional(&input);
    assert!(out.values.iter().all(|&v| v.abs() < 1e-12),
        "uniform field: conventional output must be zero everywhere");
}

// [REPR] Conventional path determinism.
#[test]
fn conventional_determinism() {
    let input = make_synthetic_input(100);
    let (out1, _) = run_conventional(&input);
    let (out2, _) = run_conventional(&input);
    assert_eq!(out1.values, out2.values,
        "conventional path must be deterministic");
}

// ── Correspondence ────────────────────────────────────────────────────────

// [CORR] Relational path corresponds for gradient field at T1 (N=100).
#[test]
fn relational_corresponds_t1() {
    let input = make_synthetic_input(100);
    let (out, _) = run_relational_primary(&input);
    let corr = check_correspondence(&input, &out);
    assert_eq!(corr, CorrespondenceState::Correspondent,
        "relational path must correspond for N=100 gradient field");
}

// [CORR] Conventional path corresponds for gradient field at T1 (N=100).
#[test]
fn conventional_corresponds_t1() {
    let input = make_synthetic_input(100);
    let (out, _) = run_conventional(&input);
    let corr = check_correspondence(&input, &out);
    assert_eq!(corr, CorrespondenceState::Correspondent,
        "conventional path must correspond for N=100 gradient field");
}

// [CORR] Both paths correspond at all four Phase 0 tiers.
#[test]
fn both_paths_correspond_all_tiers() {
    for &n in &[100usize, 1_000, 10_000, 100_000] {
        let input = make_synthetic_input(n);

        let (rel_out, _) = run_relational_primary(&input);
        let rel_corr = check_correspondence(&input, &rel_out);
        assert_eq!(rel_corr, CorrespondenceState::Correspondent,
            "relational path must correspond at N={}", n);

        let (conv_out, _) = run_conventional(&input);
        let conv_corr = check_correspondence(&input, &conv_out);
        assert_eq!(conv_corr, CorrespondenceState::Correspondent,
            "conventional path must correspond at N={}", n);
    }
}

// ── Experiment runner ─────────────────────────────────────────────────────

// [IMPL] run_experiment returns results for both paths.
#[test]
fn experiment_runner_produces_results() {
    let input = make_synthetic_input(100);
    let result = run_experiment(&input);
    assert_eq!(result.input_n, 100);
    assert!(result.relational.output.is_some());
    assert!(result.conventional.output.is_some());
}

// [IMPL] NotEvaluable is returned when input is invalid.
#[test]
fn experiment_runner_not_evaluable_on_invalid_input() {
    let mut input = make_synthetic_input(4);
    input.chi_0 = 0.0;
    let result = run_experiment(&input);
    assert!(matches!(
        result.relational.correspondence,
        CorrespondenceState::NotEvaluable(_)
    ));
}
