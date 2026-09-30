# abr-relational-compute-benchmark

**Metatron Dynamics, Inc. — Kernel V8**
Bounded over D. No claim beyond D.

---

## The argument

Conventional mathematics requires a transformation step before it can
operate on a raw observable signal. That transformation has a cost.

Relational mathematics declares the observable structure through M and
applies the kernel operators directly. No transformation step.

This repository measures the cost of that difference on a real observable:
a 4.8-second music recording at 44,100 Hz, 16-bit PCM.

---

## The experiment — V0.2.0

The same input X enters two independent mathematical treatments.
Both must reconstruct X within the declared fidelity criterion.
Compute is measured after correspondence is established for each path.

```
X  →  Relational path  (declare M → apply Kernel V8 A→B→R → invert → x̂_R)
X  →  Conventional path  (STFT round-trip → x̂_S)

Both judged against: RMS(x̂, x) ≤ 1/32768  (1 LSB, 16-bit PCM)
```

**The relational path** declares the recording device's sample values as
numerical projections through M, declares the temporal ordering as
directed relations (i → i+1, from observed acquisition order), applies
the Kernel V8 full spatial operators A→B→R, then reconstructs the
waveform via the declared inverse operator: solve M_op @ A = R (banded
O(N) linear system), integrate forward from boundary condition x[0].

The demonstrated information path is:

    (R, ρ, x₀)  →  A  →  x̂

The boundary condition x[0] — the first observed sample — is a declared
input through M. This is stated prominently because it is the accurate
description of what is demonstrated.

**The conventional path** is STFT round-trip — forward STFT followed by
inverse STFT (nperseg=512, noverlap=256, Hann window). The standard
lossless time-frequency transform for conventional audio processing.

**Correspondence criterion: waveform reconstruction fidelity.**

    RMS(x̂, x) ≤ 1/32768

1/32768 is 1 LSB in normalized 16-bit PCM amplitude — the quantization
resolution of the recording instrument. Declared from the observable
domain. Not chosen to improve a score. Both paths are held to the same
standard.

---

## Result — V0.2.0 (principal)

**Sample:** `V0_2_0_Song.wav` — continuous music, 4.8 seconds, 44,100 Hz,
16-bit stereo, L channel. No silence. Full recording declared as window.

```
                                    Relational (A→B→R)   Conventional (STFT)
----------------------------------------------------------------------------
Reconstruction RMS error                 7.22e-16 PASS       2.74e-16 PASS
Correspondence (RMS ≤ 1 LSB)            CORRESPONDENT       CORRESPONDENT

Arithmetic ops                               1,905,114           9,739,711
Comparison ops                                 423,358                   0
Total ops                                    2,328,472           9,739,711

Total ops ratio (conv/rel):   4.18×  fewer operations on the relational path
```

Both paths reconstruct the 4.8-second waveform to machine precision —
eight orders of magnitude below the declared 1 LSB threshold. The
relational path does so at **4.18× fewer counted operations**.

Both reconstructed WAV files are audibly indistinguishable from the
original under direct A/B listening comparison.

---

## Result — V0.1.0 (historical reference)

**Sample:** `Voice_44100_16bit.wav` — human voice, samples 57,330–61,739
(1.3s–1.4s, highest-RMS 0.1s window), N=4,410.
Criterion: amplitude-weighted sign correspondence per edge.

```
                                    Relational (A→B→R)   Conventional (STFT)
----------------------------------------------------------------------------
Weighted correspondence (primary)          99.2%               100.0%
Binary correspondence                  4,214/4,409         4,409/4,409

Arithmetic ops                              39,684             212,595
Comparison ops                               8,818                   0
Total ops                                   48,502             212,595

Total ops ratio (conv/rel):   4.38×
```

The V0.1.0 experiment used a sign correspondence criterion, which has
been superseded in V0.2.0 by waveform reconstruction fidelity. The
V0.1.0 result is retained here as a named prior result.

---

## Why the gap exists

The conventional transformation burden scales with the complexity of the
representation problem. The relational burden scales with the declared
relational structure of the observable.

For higher-dimensional or multi-domain analysis:
- Each additional signal dimension may require additional transformation
- Cross-domain coupling requires further representation steps
- The relational path adds a declared relation and declared provenance

This experiment measures one bounded comparison on a real biological
signal. The structural reason the gap exists — operating at the
observation rather than at the representation — is stated in the
declaration.

---

## Run it yourself

```
cargo test
cargo run --bin run_voice_phase0 -- V0_2_0_Song.wav
```

Place `V0_2_0_Song.wav` in the repo root before running. The experiment
produces two playable WAV files: `output_relational.wav` and
`output_stft.wav`.

For the V0.1.0 voice experiment:
```
cargo run --bin run_voice_phase0 -- Voice_44100_16bit.wav
```

---

## Verification appendix — synthetic scaling

`cargo run --bin run_phase0` confirms O(N) scaling and full
correspondence for both paths over synthetic gradient fields at
N = 100, 1,000, 10,000, 100,000. This is implementation verification.
A synthetic gradient field carries no representational burden, so
there is no transformation tax to measure.

---

## Declaration

`docs/DECLARATION_VOICE.md` — M declaration, observable domain, kernel
selection, conventional comparator, correspondence criterion, reconstruction
information path, open conditions, version history.

`docs/DECLARATION.md` — experiment structure origin declaration.

---

Metatron Dynamics, Inc. Kernel V8. Bounded over D. No claim beyond D.
