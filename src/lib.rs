// lib.rs — abr-relational-compute-benchmark
// Metatron Dynamics, Inc. Kernel V8. Bounded over D. No claim beyond D.

pub mod adapter_synthetic;
pub mod adapter_voice;
pub mod conventional_path;
pub mod conventional_stft;
pub mod correspondence;
pub mod relational_path;
pub mod types;
pub mod wav_loader;

use types::{
    BenchmarkInput, CorrespondenceState, ExperimentResult, SinglePathResult,
};

/// Run both paths (Primary Δ→Σ and conventional mean-sub+norm) over the
/// declared input. Used for Phase 0 synthetic experiments.
pub fn run_experiment(input: &BenchmarkInput) -> ExperimentResult {
    if let Err(e) = input.validate() {
        let not_evaluable = SinglePathResult {
            path_label: "not evaluable".into(),
            output: None,
            correspondence: CorrespondenceState::NotEvaluable(e.clone()),
            compute: Default::default(),
        };
        return ExperimentResult {
            input_n: input.n_nodes(),
            relational: not_evaluable.clone(),
            conventional: not_evaluable,
        };
    }

    let (rel_output, rel_compute) = relational_path::run_relational_primary(input);
    let rel_correspondence = correspondence::check_correspondence(input, &rel_output);

    let (conv_output, conv_compute) = conventional_path::run_conventional(input);
    let conv_correspondence = correspondence::check_correspondence(input, &conv_output);

    ExperimentResult {
        input_n: input.n_nodes(),
        relational: SinglePathResult {
            path_label: "relational-primary (Δ→Σ)".into(),
            output: Some(rel_output),
            correspondence: rel_correspondence,
            compute: rel_compute,
        },
        conventional: SinglePathResult {
            path_label: "conventional (mean-sub + norm)".into(),
            output: Some(conv_output),
            correspondence: conv_correspondence,
            compute: conv_compute,
        },
    }
}
