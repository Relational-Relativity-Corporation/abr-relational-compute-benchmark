// run_phase0.rs — abr-relational-compute-benchmark
// Metatron Dynamics, Inc. Kernel V8. Bounded over D. No claim beyond D.
//
// Phase 0 experiment: synthetic gradient field at N = 10², 10³, 10⁴, 10⁵.
// Runs both paths, checks correspondence, records compute.
// Prints results. Does not interpret them.

use abr_relational_compute_benchmark::{
    adapter_synthetic::make_synthetic_input,
    run_experiment,
    types::CorrespondenceState,
};

fn main() {
    println!("abr-relational-compute-benchmark — Phase 0");
    println!("Metatron Dynamics, Inc. Kernel V8");
    println!("Bounded over D. No claim beyond D.");
    println!();
    println!("Input: synthetic gradient field x[i] = N−1−i");
    println!("       1D directed chain i → i+1, direction from observed ordering");
    println!("Relational path:    Primary kernel Δ → Σ");
    println!("Conventional path:  mean subtraction + normalization");
    println!("Output requirement: sign(output[e]) = sign(x[s] − x[t])");
    println!();

    let tiers: &[usize] = &[100, 1_000, 10_000, 100_000];

    println!("{:<10}  {:<12}  {:<14}  {:<14}  {:<14}  {:<14}  {:<14}  {:<14}",
        "N", "path",
        "corresponds?", "arith ops", "cmp ops",
        "allocs", "peak mem (B)", "wall (µs)");
    println!("{}", "-".repeat(118));

    for &n in tiers {
        let input = make_synthetic_input(n);
        let result = run_experiment(&input);

        for path_result in [&result.relational, &result.conventional] {
            let corr_str = match &path_result.correspondence {
                CorrespondenceState::Correspondent => "YES".to_string(),
                CorrespondenceState::PartiallyCorrespondent { corresponding, total } =>
                    format!("PARTIAL {}/{}", corresponding, total),
                CorrespondenceState::Divergent => "DIVERGENT".to_string(),
                CorrespondenceState::NotEvaluable(e) => format!("NOT-EVAL: {}", e),
            };
            let c = &path_result.compute;
            println!("{:<10}  {:<12}  {:<14}  {:<14}  {:<14}  {:<14}  {:<14}  {:<14}",
                n,
                path_result.path_label.chars().take(12).collect::<String>(),
                corr_str,
                c.arithmetic_ops,
                c.comparison_ops,
                c.allocations,
                c.peak_memory_bytes,
                c.wall_clock_us,
            );
        }
        println!();
    }

    println!("Correspondence is checked against X independently for each path.");
    println!("Compute comparison is recorded after correspondence is established.");
    println!("Divergence, if any, is returned to Origin — it is not suppressed.");
}
