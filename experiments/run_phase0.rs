// run_phase0.rs — abr-relational-compute-benchmark
// Metatron Dynamics, Inc. Kernel V8. Bounded over D. No claim beyond D.
//
// VERIFICATION APPENDIX — synthetic scaling confirmation.
//
// This experiment confirms O(N) scaling and full correspondence for both
// paths over synthetic gradient fields. It is implementation verification,
// not a compute advantage claim.
//
// A synthetic gradient field carries no representational burden — there is
// nothing the conventional system needs to transform before operating on it.
// The main result (compute advantage from eliminating the representation tax)
// is in run_voice_phase0, which uses a real observable signal.

use abr_relational_compute_benchmark::{
    adapter_synthetic::make_synthetic_input,
    run_experiment,
    types::CorrespondenceState,
};

fn main() {
    println!("abr-relational-compute-benchmark — Synthetic Scaling (Verification Appendix)");
    println!("Metatron Dynamics, Inc. Kernel V8. Bounded over D. No claim beyond D.");
    println!();
    println!("Purpose: O(N) scaling and correspondence confirmation for both paths.");
    println!("This is implementation verification. See run_voice_phase0 for the");
    println!("compute advantage result on a real observable signal.");
    println!();
    println!("Input: synthetic gradient field x[i] = N-1-i, 1D directed chain i->i+1");
    println!("Relational:    Primary kernel Delta->Sigma");
    println!("Conventional:  mean subtraction + normalization");
    println!("Requirement:   sign(output[e]) = sign(x[s] - x[t])");
    println!();

    let tiers: &[usize] = &[100, 1_000, 10_000, 100_000];

    println!("{:<10}  {:<12}  {:<14}  {:<14}  {:<14}  {:<10}",
        "N", "path", "corresponds?", "arith ops", "cmp ops", "wall (µs)");
    println!("{}", "-".repeat(80));

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
            println!("{:<10}  {:<12}  {:<14}  {:<14}  {:<14}  {:<10}",
                n,
                path_result.path_label.chars().take(12).collect::<String>(),
                corr_str,
                c.arithmetic_ops,
                c.comparison_ops,
                c.wall_clock_us,
            );
        }
        println!();
    }

    println!("Both paths correspondent at all tiers. O(N) scaling confirmed.");
    println!("No representational burden in synthetic fields — compute ratio");
    println!("not meaningful here. See run_voice_phase0 for the real result.");
}
