# M Declaration — Voice Adapter
# abr-relational-compute-benchmark / Phase 1 Voice

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
most active speech content. It is not adjusted after seeing results.

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
  earlier sample to later sample. It is a declared direction of
  relational comparison, not a claim of causation or propagation.
  Observable sufficiency: the device's acquisition order is part of the
  observable record. Provenance terminates here.
- **Edge provenance:** Continuation (within-channel temporal continuation
  from device acquisition). Open boundary: no wraparound.
- **Kernel selection:** N = 4,410 nodes; interior loci exist at every
  node except i=0 and i=4409. Full-operator admissibility condition
  is satisfied. **ABR kernel (A → B → R) applies for Phase 1.**
  Phase 0 of this voice experiment uses Primary (Δ → Σ) for direct
  comparison with the conventional path.

---

## M-supplied operator parameters

- **ρ_base:** 0.3 (declared for this adapter)
- **χ₀:** 0.01 (declared in the dimension of x[i], which is normalized
  amplitude ∈ [−1, 1]. χ₀ = 0.01 is approximately one unit of the
  smallest 16-bit step at this scale.)
- **cc:** empty (single component, no component pairs declared)

These are declared before running. Not adjusted after results.

---

## Conventional path — MFCC pipeline

The conventional path applies the standard MFCC (Mel-Frequency Cepstral
Coefficients) pipeline to the same declared window X.

**Why MFCC:** MFCC is the standard feature extraction step for voice in
conventional signal processing and machine learning pipelines. It exists
because raw waveform sample values are not directly legible to downstream
statistical systems — the transformation is unavoidable, not constructed
for this experiment.

**Pipeline steps (declared before running):**

1. **Pre-emphasis:** x'[i] = x[i] − 0.97 · x[i−1]
   (standard filter; 0.97 is the conventional coefficient)
2. **Frame windowing:** Hamming window, frame size 512 samples,
   hop size 256 samples
3. **FFT:** 512-point FFT per frame → power spectrum
4. **Mel filterbank:** 26 triangular filters on the mel scale,
   20 Hz to 20,000 Hz
5. **Log:** log of filterbank energies
6. **DCT:** 13 coefficients (standard MFCC count)

Output: 13 MFCC coefficients per frame. Number of frames from 4,410
samples with frame=512, hop=256: approximately 16 frames.

The MFCC pipeline is applied using scipy/numpy, counting arithmetic
operations explicitly as in Phase 0.

---

## Output requirement and correspondence criterion

**Output requirement:** both paths must produce output that preserves
the declared temporal structure of X — specifically, the sign and
approximate magnitude of adjacent-sample contrasts across the declared
window.

**Correspondence criterion:**
For each declared edge e = (i, i+1):
  sign(output[e]) = sign(x[i] − x[i+1])

For the MFCC path: the output is reconstructed to the edge domain by
computing adjacent-frame differences of the first MFCC coefficient
(DC energy), which tracks the envelope of the signal.

**Implementation-only tolerance:** 1e-10 for zero comparisons.
Not a property of the observed system.

---

## What this experiment does not do

- Classify the voice content (no emotion, speaker, or phoneme labels)
- Claim that the MFCC path is wrong
- Claim that the relational path produces better audio features
- Adjust either path based on the other's output
- Interpret operator output as domain-specific meaning before
  correspondence is established

---

## Open conditions

- OC-V-1: Whether adjacent-frame MFCC differences are the right
  conventional-path output for the correspondence criterion, or whether
  a different MFCC-domain comparison is more appropriate. Declared
  before Phase 1 full-ABR run.
- OC-V-2: Whether the χ₀ = 0.01 declaration is appropriate for 16-bit
  normalized amplitude, or requires measurement-record grounding.
- OC-V-3: Whether a two-channel (stereo) component-pair declaration
  would be more informative than single-channel Phase 0.

---

Bounded over D. No claim beyond D.
Metatron Dynamics, Inc. Kernel V8.
