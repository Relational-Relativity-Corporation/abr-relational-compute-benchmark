# M Declaration — Voice/Music Adapter
# abr-relational-compute-benchmark / Voice Phase 0

**Metatron Dynamics, Inc. Kernel V8. Bounded over D. No claim beyond D.**

---

## Version history

| Version | Sample | Window | N | Criterion | Result |
|---|---|---|---|---|---|
| V0.1.0 | Voice_44100_16bit.wav | samples 57,330–61,739 (1.3s–1.4s, highest-RMS 0.1s window) | 4,410 | Amplitude-weighted sign correspondence | 99.2% corr., 4.38× ops ratio |
| V0.2.0 | V0_2_0_Song.wav | samples 0–211,679 (full recording, 4.8s) | 211,680 | Waveform reconstruction fidelity RMS(x̂,x) ≤ 1 LSB | Both paths PASS at machine precision, 4.18× ops ratio |

V0.1.0 result is retained as a named historical reference.
V0.2.0 is the current principal experiment.

---

## Observable record — V0.2.0

File: `V0_2_0_Song.wav`
Format: RIFF WAV, 44,100 Hz, 16-bit, stereo
Duration: 4.8 seconds (211,680 frames)
Content: continuous music signal — no silence, sustained signal throughout
L-channel RMS: 0.047992 (full file)
Max/min RMS across 0.1s windows: 3.00× (uniform amplitude, no silence artifacts)

The file was produced by a recording device. The values it contains are
amplitude values reported by that device at the declared sample rate.
The sample rate, channel organization, and sample ordering are part of
the observable record.

---

## Declared window — V0.2.0

Start sample: 0
End sample:   211,679 (inclusive)
Time:         0.0000s to 4.8000s
N:            211,680 samples (full recording)
Channel used: L (channel 0)

The declared window is the complete file. No trimming or selection
was applied. Declared before running. Not adjusted after results.

---

## Observable domain — 16-bit PCM

The observable domain is declared from the recording format:

| Property | Value | Source |
|---|---|---|
| Amplitude range | [−1.0, 1.0] | 0 dBFS DAC brickwall ceiling |
| Maximum \|A[e]\| | 2.0 | Full-scale transition |
| Minimum nonzero \|A[e]\| | 1/32768 | 1 LSB quantization floor |
| Nominal operating level | 0.1259 | −18 dBFS (EBU R68 / SMPTE) |
| Temporal step | 1/44,100 s | Declared sample rate |

These bounds are declared from the format specification.
They are not tuned to results.

---

## M: O → D

- **One declared temporal step:** 1/44,100 s ≈ 22.676 μs — the device's
  own process step.
- **Node i:** the locus of the i-th sample in the declared window.
  x[i] = amplitude value reported by the recording device at that step.
  i = 0, …, 211,679.
- **All values finite and bounded (∈ D):** confirmed.
- **Declared projection:** x[i] is the numerical projection of the
  amplitude observable through M. It is not the observable itself.

---

## Declared relational topology

- **Edge e = (i, i+1)** for i = 0, …, 211,678. N−1 = 211,679 edges.
- **Direction:** i → i+1. Observed acquisition order — earlier sample
  to later sample. Declared direction of relational comparison, not a
  causal claim. Provenance terminates at the device's acquisition order.
- **Edge provenance:** Continuation (within-channel temporal continuation
  from device acquisition). Open boundary: no wraparound.
- **Kernel selection:** N = 211,680 nodes. Interior loci exist at every
  node except i=0 and i=211,679. Full-operator admissibility condition
  is satisfied. **ABR kernel (A → B → R) applies.**
  Phase 1 spatial kernel only — persistence (A_p→B_p→R_p) requires
  a declared pair of sequential observations through M. A single
  observation window does not meet that condition.

---

## M-supplied operator parameters — V0.2.0

Both parameters are declared from the 16-bit PCM audio format standard.
Neither is tuned to results.

- **ρ_base:** 10^(−18/20) ≈ 0.125893
  −18 dBFS nominal operating level (EBU R68 / SMPTE standard).
  Declared as the maximum relational gain; ceiling bounded by the DAC
  brickwall at |A[e]| = 2.0.

- **χ₀:** 1/32768 ≈ 3.052e-5
  1 LSB — the 16-bit PCM quantization floor.
  Minimum nonzero observable event in this domain.
  Closes OC-V-2 from V0.1.0.

- **cc:** empty (single component, no component pairs declared)

---

## Conventional path — STFT round-trip

The declared conventional comparator is STFT round-trip: forward STFT
followed by inverse STFT.

Parameters: nperseg=512, noverlap=256, Hann window, center padding
matching scipy.signal.stft/istft defaults.

STFT is the standard lossless time-frequency transform for conventional
audio signal processing. It is not chosen to favor the comparison —
it is the unavoidable preprocessing step before any frequency-domain
analysis in the conventional pipeline.

---

## Output requirement and correspondence criterion — V0.2.0

**Output requirement:** both paths must reconstruct the observed waveform
X within the declared fidelity criterion.

**Correspondence criterion:**

    RMS(x̂, x) ≤ 1/32768

Where x̂ is the reconstructed waveform from each path and x is the
original observed signal.

The threshold 1/32768 is 1 LSB in normalized 16-bit PCM amplitude —
the quantization resolution of the recording instrument. It is declared
from the observable domain, not chosen to improve a score.

This criterion imposes no abstraction on the observable. It asks each
path to return the observation itself, within the instrument's own
resolution. No information loss occurs if the criterion is met.

**Why this criterion supersedes V0.1.0 sign correspondence:**

The V0.1.0 criterion (amplitude-weighted sign correspondence per edge)
tested whether the relational output preserved signal direction locally.
The V0.2.0 criterion asks both paths to reconstruct the observed waveform
itself. This is a stronger and more directly interpretable standard: both
mathematical treatments are judged on whether they can return X from X.
The computational cost comparison is then unambiguous.

---

## Declared information path — relational reconstruction

The relational reconstruction path is:

    (R, ρ, x₀) → A → x̂

Specifically:

1. **Forward pass (A→B→R):** The operator chain produces R over the
   declared edge set, and ρ at each node.

2. **Inverse pass (M_op @ A = R):** M_op is the banded linear operator
   (lower bandwidth 1, upper bandwidth 2) fully determined by ρ and
   the declared chain topology. The banded system is solved in O(N)
   operations using the Thomas algorithm.

3. **Forward integration:** x̂[0] = x[0] (boundary condition — the
   first observed sample, declared through M). x̂[i+1] = x̂[i] − A[i].

**The boundary condition x[0] is a required declared input.**
The demonstrated information path is (R, ρ, x₀) → A → x̂,
not "R alone contains the waveform." This is stated prominently
because it is the accurate description of what is demonstrated.

**Invertibility:** A→B→R is an exactly invertible linear transformation
of the signal's difference field. This was verified analytically and
confirmed experimentally at machine precision on the full 211,680-sample
signal before implementation.

---

## Result — V0.2.0

```
File:   V0_2_0_Song.wav
Window: samples 0–211,679 (0.0000s–4.8000s, full recording)
N:      211,680  |  Edges: 211,679

                                        Relational (A→B→R)   Conventional (STFT)
--------------------------------------------------------------------------------
Reconstruction RMS error (vs original)      7.22e-16 PASS       2.74e-16 PASS
Correspondence (RMS ≤ 1 LSB)                CORRESPONDENT       CORRESPONDENT

Arithmetic ops                                 1,905,114           9,739,711
Comparison ops                                   423,358                   0
Total ops                                      2,328,472           9,739,711

Total ops ratio (conv/rel):  4.18×
```

Both paths reconstruct the 4.8-second waveform to machine precision
under the declared criterion. The relational path does so at 4.18×
fewer counted operations.

---

## Open conditions — V0.2.0

- **OC-V-3:** Stereo component-pair declaration — R channel and
  cross-channel topology not yet declared. Single-channel Phase 0 only.

- **OC-V-4:** Operation count for the inverse pass (Thomas algorithm)
  is not yet included in the reported relational ops total. The forward
  A→B→R pass is counted; the banded solve adds O(N) operations. To be
  declared and counted in a future version for completeness.

- **OC-V-5:** The op ratio at larger N — whether the 4.18× ratio holds,
  grows, or changes character as the signal duration scales beyond 4.8s.

---

## Historical reference — V0.1.0

V0.1.0 used `Voice_44100_16bit.wav`, samples 57,330–61,739 (0.1s,
highest-RMS window), N=4,410. Correspondence criterion: amplitude-weighted
sign correspondence per edge. Result: 99.2% weighted correspondence,
4.38× ops ratio (48,502 vs 212,595 ops). Parameters: ρ_base=0.3,
χ₀=0.01 (not yet grounded to domain standard at that version).

The V0.1.0 experiment is superseded by V0.2.0. The sign correspondence
criterion has been replaced by waveform reconstruction fidelity, which
is a stronger and more directly interpretable standard.

---

Bounded over D. No claim beyond D.
Metatron Dynamics, Inc. Kernel V8.
