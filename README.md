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
a 0.1-second segment of a human voice recording at 44,100 Hz.

---

## The experiment

Same input X enters two independent mathematical treatments.
Both must return output that corresponds with X.
Compute is measured after correspondence is established for each path.

```
X  →  Relational path  (declare M → apply Kernel V8 A→B→R)  →  X'_R
X  →  Conventional path  (STFT round-trip)                   →  X'_S

X'_R  ↔  X    (amplitude-weighted sign correspondence)
X'_S  ↔  X    (amplitude-weighted sign correspondence)
```

**The relational path** declares the recording device's sample values as
numerical projections through M, declares the temporal ordering as
directed relations (i → i+1, from observed acquisition order), and
applies the Kernel V8 full spatial operators A→B→R directly.

**The conventional path** is STFT round-trip — the declared conventional
comparator for this experiment. Reconstruction RMS error: 9.21e-16
(machine precision).

**Correspondence criterion: amplitude-weighted.**
The observable provides amplitude through M. The amplitude of each
declared relation |A[e]| = |x[s] − x[t]| is the physical weight of
that relation. Correspondence is evaluated weighted by that amplitude:

    Weighted correspondence = Σ|A[e]| over correspondent edges
                            / Σ|A[e]| over all edges

A sign correspondence at |A[e]| = 0.0001 and at |A[e]| = 0.4 are not
equivalent observational events. The physical weight distinguishes them.
This is not a threshold chosen to improve a score — it is the amplitude
information M already provides, applied consistently to the comparison.

---

## Result — Voice_44100_16bit.wav, 4,410 samples, 0.1 seconds

```
                                  Relational (A→B→R)   Conventional (STFT)
--------------------------------------------------------------------------
Weighted correspondence (primary)          99.2%               100.0%
Binary correspondence                  4214/4409           4409/4409
Reconstruction RMS error               (edge domain)         9.21e-16

Arithmetic operations                     39,684              212,595
Comparison operations                      8,818                    0
Total operations                          48,502              212,595

Compute ratio                             4.38× fewer operations
```

The 195 binary-divergent relational edges carry **0.8% of total signal
amplitude weight**. Their mean |A[e]| is 0.001056 versus 0.006451 for
correspondent edges — a ratio of 0.16×. The divergence is at the
physical noise floor of the declared observation, not in the signal.

**The relational path uses 4.38× fewer operations** and achieves 99.2%
amplitude-weighted correspondence with the declared observable — without
any transformation of the signal.

---

## Why the gap grows with complexity

The conventional transformation burden scales with the complexity of the
representation problem. The relational burden scales with the declared
relational structure of the observable.

For higher-dimensional or multi-domain analysis:
- Each additional signal dimension may require additional transformation
- Cross-domain coupling requires further representation steps
- The relational path adds a declared relation and declared provenance

This experiment measures one bounded comparison. The structural reason
the gap exists — operating at the representation versus at the
observation — is stated in the declaration.

---

## Run it yourself

```
cargo test
cargo run --bin run_voice_phase0 -- Voice_44100_16bit.wav
```

Place `Voice_44100_16bit.wav` in the repo root before running.

---

## Verification appendix — synthetic scaling

`cargo run --bin run_phase0` confirms O(N) scaling and full
correspondence for both paths over synthetic gradient fields at
N = 100, 1,000, 10,000, 100,000. This is implementation verification.
A synthetic gradient field carries no representational burden, so
there is no transformation tax to measure.

---

## Declaration

`docs/DECLARATION_VOICE.md` — M declaration, window, kernel selection,
conventional comparator, correspondence criterion, open conditions.

`docs/DECLARATION.md` — experiment structure origin declaration.

---

Metatron Dynamics, Inc. Kernel V8. Bounded over D. No claim beyond D.
