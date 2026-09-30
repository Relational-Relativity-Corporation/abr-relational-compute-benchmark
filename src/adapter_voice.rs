// adapter_voice.rs — abr-relational-compute-benchmark
// Metatron Dynamics, Inc. Kernel V8. Bounded over D. No claim beyond D.
//
// Adapter: voice/music — V0_2_0_Song.wav, full recording.
//
// ── M declaration ────────────────────────────────────────────────────────
//
// File:       V0_2_0_Song.wav
// Format:     RIFF WAV, 44,100 Hz, 16-bit, stereo, 4.8 s
// Channel:    L (channel 0)
//
// Declared window:
//   Start sample: 0        (0.0000 s)
//   End sample:   211,679  (4.8000 s, inclusive)
//   N:            211,680 samples (4.8000 s exactly)
//   Full recording — all frames in the file.
//   Declared before running. Not adjusted after results.
//
// V0.1.0 reference window — Voice_44100_16bit.wav:
//   Start: 57,330 (1.3000 s), End: 61,739 (1.4000 s), N=4,410
//   Selected as highest-RMS 0.1 s window. Retained as named reference.
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
//   Full spatial kernel A→B→R applies.
//
// M-supplied parameters (declared from 16-bit PCM digital audio format standard):
//   ρ_base = 10^(-18/20) ≈ 0.1259  (-18 dBFS nominal operating level, EBU R68 / SMPTE)
//                                    Standard DAW/broadcast reference level.
//                                    Declared from domain standard, not tuned to result.
//   χ₀     = 1/32768 ≈ 3.05e-5     (1 LSB — 16-bit PCM quantization floor)
//                                    Minimum nonzero observable event in this domain.
//                                    Declared from format specification, not tuned to result.
//   cc     = []                     (single component, no component pairs)
//
// Observable domain bounds (16-bit PCM, 44,100 Hz):
//   Amplitude range:           [-1.0, 1.0]   (0 dBFS DAC brickwall ceiling)
//   Maximum |A[e]|:            2.0            (full-scale transition)
//   Minimum nonzero |A[e]|:    1/32768        (1 LSB quantization step = χ₀)
//   Nominal operating level:   0.1259         (-18 dBFS = ρ_base)
//   Temporal step:             1/44100 s      (declared sample rate)
//
// ── Correspondence criterion ──────────────────────────────────────────────
//
// For each declared edge e = (i, i+1):
//   sign(R[e]) corresponds with sign(x[i] − x[i+1])
//
// Amplitude-weighted: weight = |x[i] − x[i+1]| per edge.
// Implementation-only tolerance: 1e-10. Not a property of the observed system.
//
// ── Conventional path ────────────────────────────────────────────────────
//
// STFT round-trip (forward STFT + inverse STFT, nperseg=512, noverlap=256,
// Hann window). Reconstruction RMS error confirmed at machine precision.
// See conventional_stft.rs for the counted implementation.

use crate::types::BenchmarkInput;

/// Declared window parameters — V0_2_0_Song.wav, L channel, full recording.
pub const SAMPLE_RATE:    u32   = 44_100;
pub const WINDOW_START:   usize = 0;
pub const WINDOW_N:       usize = 211_680;
pub const RHO_BASE_VOICE: f64   = 0.125_892_541;      // -18 dBFS nominal operating level (EBU R68 / SMPTE)
                                                       // = 10^(-18/20) linear amplitude; standard DAW reference
pub const CHI_0_VOICE:    f64   = 1.0 / 32768.0;      // 1 LSB — 16-bit PCM quantization floor
                                                       // minimum nonzero observable event in this domain

// V0.1.0 reference window — Voice_44100_16bit.wav, highest-RMS 0.1 s:
pub const PHASE0_WINDOW_START: usize = 57_330;
pub const PHASE0_WINDOW_N:     usize = 4_410;

/// Construct a BenchmarkInput from the declared window.
///
/// `samples` must be the raw L-channel sample values
/// x[WINDOW_START..WINDOW_START+WINDOW_N] from V0_2_0_Song.wav,
/// already extracted by the WAV loader.
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
            "V0_2_0_Song.wav L-channel samples {}–{} \
             (0.0000s–4.8000s, 44100 Hz, full recording); \
             directed chain i→i+1 from observed acquisition order",
            WINDOW_START, WINDOW_START + WINDOW_N - 1
        ),
        edges,
        edge_provenance,
        rho_base: RHO_BASE_VOICE,
        chi_0:    CHI_0_VOICE,
    }
}
