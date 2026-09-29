# M Declaration — Voice Adapter
# abr-relational-compute-benchmark / Voice Phase 0

**Metatron Dynamics, Inc. Kernel V8. Bounded over D. No claim beyond D.**

---

## Observable record

File: `Voice_44100_16bit.wav`
Format: RIFF WAV, 44,100 Hz, 16-bit, stereo, 4.0 seconds
Channels: 2 (L, R) — two independently recorded amplitude streams

The file was produced by a recording device. The values it contains are
amplitude values reported by that device at the declared sample rate.
The sample rate, channel organization, and sample ordering are part of
the observable record — they are what the device reports.

---

## Declared window

Start sample: 57,330
End sample:   61,739 (inclusive)
Time:         1.3000 s to 1.4000 s
N:            4,410 samples
Duration:     0.1000 s exactly (4410 / 44100)
Channel used: L (channel 0)
RMS:          0.14957 (highest-energy 0.1s window in the file)

This window was declared before running the experiment. It was selected
as the highest-RMS 0.1s window in the file — the region containing the
most active speech content. It is not adjusted after results.

---

## M: O → D

- **One declared temporal step:** 1/44,100 s ≈ 22.676 μs — the device's
  own process step. Not an externally imposed increment.
- **Node i:** the locus of the i-th sample in the declared window.
  x[i] = amplitude value reported by the recording device at that step.
  i = 0, …, 4409.
- **All values finite and bounded (∈ D):** confirmed.
- **Declared projection:** x[i] is the numerical projection of the
  amplitude observable through M. It is not the observable itself.

---

## Declared relational topology

- **Edge e = (i, i+1)** for i = 0, …, 4408. N−1 = 4,409 edges.
- **Direction:** i → i+1. This is the observed acquisition order —
  earlier sample to later sample. Declared direction of relational
  comparison, not a causal claim. Provenance terminates at the device's
  acquisition order.
- **Edge provenance:** Continuation (within-channel temporal continuation
  from device acquisition). Open boundary: no wraparound.
- **Kernel selection:** N = 4,410 nodes; interior loci exist at every
  node except i=0 and i=4409. Full-operator admissibility condition
  is satisfied. **ABR kernel (A → B → R) applies.**
  Phase 1 spatial kernel only — persistence (A_p→B_p→R_p) requires
  a declared pair of sequential observations through M. A single
  observation window does not meet that condition.

---

## M-supplied operator parameters

- **ρ_base:** 0.3 (declared for this adapter)
- **χ₀:** 0.01 (declared in the dimension of x[i], normalized amplitude)
- **cc:** empty (single component, no component pairs declared)

---

## Conventional path — STFT round-trip

The declared conventional comparator is STFT (Short-Time Fourier
Transform) round-trip: forward STFT followed by inverse STFT.

Parameters: nperseg=512, noverlap=256, Hann window, center padding
matching scipy.signal.stft/istft defaults.

Reconstruction RMS error: 9.21e-16 (machine precision confirmed).

STFT is the declared conventional comparator for this experiment.
No universal claim is made about all conventional signal processing.

---

## Output requirement and correspondence criterion

**Output requirement:** both paths must produce output that corresponds
with X under the declared criterion.

**Correspondence criterion:**
For each declared edge e = (i, i+1):
  sign(output[e]) = sign(x[i] − x[i+1])

**Implementation-only tolerance:** 1e-10 for zero comparisons.
Not a property of the observed system.

---

## Open conditions

- OC-V-1: Partition of correspondence result by observed signal amplitude
  — characterize where divergence occurs relative to the signal's
  amplitude distribution. Declared before further analysis; threshold
  derived from the observable record, not chosen to improve the score.
- OC-V-2: Whether the χ₀ = 0.01 declaration is appropriate for 16-bit
  normalized amplitude, or requires measurement-record grounding.
- OC-V-3: Whether a two-channel (stereo) component-pair declaration
  would be more informative than single-channel Phase 0.

---

Bounded over D. No claim beyond D.
Metatron Dynamics, Inc. Kernel V8.
