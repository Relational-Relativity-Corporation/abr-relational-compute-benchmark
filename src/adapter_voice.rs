// adapter_voice.rs — abr-relational-compute-benchmark
// Metatron Dynamics, Inc. Kernel V8. Bounded over D. No claim beyond D.
//
// Adapter: voice — Voice_44100_16bit.wav, declared window.
//
// ── M declaration ────────────────────────────────────────────────────────
//
// File:       Voice_44100_16bit.wav
// Format:     RIFF WAV, 44,100 Hz, 16-bit, stereo, 4.0 s
// Channel:    L (channel 0)
//
// Declared window:
//   Start sample: 57,330  (1.3000 s)
//   End sample:   61,739  (1.4000 s, inclusive)
//   N:            4,410 samples (0.1000 s exactly)
//   Selected as highest-RMS 0.1 s window in the file.
//   Declared before running. Not adjusted after results.
//
// Observable: amplitude values reported by the recording device at
//   44,100 Hz. The sample rate, channel organization, and sample
//   ordering are part of the observable record.
//
// One declared temporal step: 1/44,100 s ≈ 22.676 μs — the device's
//   own process step.
//
// Direction: i → i+1 (observed acquisition order — earlier sample to
//   later sample). Declared direction of relational comparison, not a
//   causal claim. Provenance terminates at the device's acquisition
//   order. Observable sufficiency: the acquisition order is part of the
//   observable record.
//
// Edge provenance: Continuation (within-channel temporal continuation).
//
// Kernel selection: full-operator admissibility condition is satisfied
//   (interior loci exist at every node except i=0 and i=N-1).
//   Phase 0 uses Primary kernel (Δ → Σ) for direct comparison with
//   the conventional STFT round-trip path.
//
// M-supplied parameters:
//   ρ_base = 0.3   (declared for this adapter)
//   χ₀     = 0.01  (declared in dimension of x[i], normalized amplitude)
//   cc     = []    (single component, no component pairs)
//
// ── Correspondence criterion ──────────────────────────────────────────────
//
// For each declared edge e = (i, i+1):
//   sign(Σ[e]) = sign(x[i] − x[i+1])
//
// Implementation-only tolerance: 1e-10. Not a property of the observed system.
//
// ── Conventional path ────────────────────────────────────────────────────
//
// STFT round-trip (forward STFT + inverse STFT, nperseg=512, noverlap=256,
// Hann window). The STFT is the standard lossless time-frequency transform
// for conventional voice processing — unavoidable preprocessing before any
// frequency-domain analysis. Reconstruction error: 3.41e-17 (machine
// precision). See conventional_path.rs for the counted implementation.
//
// MFCC was evaluated and removed: MFCC discards phase by design and
// cannot reconstruct to the sample domain with acceptable fidelity.
// It is not a round-trip operation and cannot meet the input=output
// experimental requirement.

use crate::types::BenchmarkInput;

/// Declared window parameters — Voice_44100_16bit.wav, L channel.
pub const SAMPLE_RATE:   u32   = 44_100;
pub const WINDOW_START:  usize = 57_330;
pub const WINDOW_N:      usize = 4_410;
pub const RHO_BASE_VOICE: f64  = 0.3;
pub const CHI_0_VOICE:    f64  = 0.01;

/// Construct a BenchmarkInput from the declared voice window.
///
/// `samples` must be the raw sample values x[WINDOW_START..WINDOW_START+WINDOW_N]
/// from the L channel of Voice_44100_16bit.wav, already extracted by the
/// WAV loader. The executor does not load the file; the adapter declares
/// the M mapping and topology.
pub fn make_voice_input(samples: Vec<f64>) -> BenchmarkInput {
    assert_eq!(samples.len(), WINDOW_N,
        "voice adapter requires exactly WINDOW_N={} samples", WINDOW_N);
    assert!(samples.iter().all(|v| v.is_finite()),
        "all sample values must be finite (∈ D)");

    let n = samples.len();
    let edges: Vec<(usize, usize)> = (0..n - 1).map(|i| (i, i + 1)).collect();
    let edge_provenance: Vec<String> = edges
        .iter()
        .map(|_| "Continuation".to_string())
        .collect();

    BenchmarkInput {
        x: samples,
        m_label: format!(
            "Voice_44100_16bit.wav L-channel samples {}–{} \
             (1.3000s–1.4000s, 44100 Hz, highest-RMS 0.1s window); \
             directed chain i→i+1 from observed acquisition order",
            WINDOW_START, WINDOW_START + WINDOW_N - 1
        ),
        edges,
        edge_provenance,
        rho_base: RHO_BASE_VOICE,
        chi_0:    CHI_0_VOICE,
    }
}
