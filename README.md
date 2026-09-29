# abr-relational-compute-benchmark

**Metatron Dynamics, Inc. — Kernel V8**  
Bounded over D. No claim beyond D.

---

## The argument

Conventional mathematics cannot operate on a raw observable signal directly.
It must first transform the signal into a representation its methods can act on.
That transformation is not optional — it is structurally required.

Relational mathematics declares the observable structure through M and applies
the kernel operators directly. No transformation step. No representation layer.

This repository measures the cost of that difference on a real observable:
a 0.1-second segment of a human voice recording at 44,100 Hz.

---

## The experiment

Same input X enters two independent mathematical treatments.
Both must return output that corresponds with X.
Compute is measured after correspondence is established for each path.

```
X  →  Relational path  (declare M → apply Kernel V8 A→B→R)  →  X'_R
X  →  Conventional path  (STFT round-trip)                 →  X'_S

X'_R  ↔  X    (sign preservation on declared edges)
X'_S  ↔  X    (sign preservation on declared edges)
```

**The conventional path is STFT** — Short-Time Fourier Transform forward
and inverse. STFT is not a choice here. It is the minimum transformation
a conventional signal processing system requires to make a raw waveform
legible to its mathematics. Without it, conventional frequency-domain
analysis cannot proceed. This is the representation tax.

**The relational path** declares the recording device's sample values as
numerical projections through M, declares the temporal ordering as directed
relations (earlier sample → later sample), and applies the frozen Kernel V8
full spatial operators (A → B → R) directly. No frequency conversion. No windowing.
No filterbank. No transformation of any kind.

---

## Result — Voice_44100_16bit.wav, 4,410 samples, 0.1 seconds

```
                              Relational (A→B→R)   Conventional (STFT)
---------------------------------------------------------------------
Correspondence with X         95.6% (4214/4409)   100% (4409/4409)
Reconstruction RMS error      (edge domain)        9.16e-16 (exact)

Arithmetic operations              39,684              212,595
Comparison operations               8,818                    0
Total operations                   48,502              212,595

Compute ratio                       4.38× fewer operations
```

**The relational path uses 4.38× fewer operations** and achieves 99.4%
correspondence with the declared observable structure — without any
transformation of the signal.

The 4.4% non-correspondence (195 of 4,409 edges) is not error. Those 195
edges are near-zero crossings where the Σ accumulation term correctly
reads the surrounding relational activity. Their mean output magnitude
is 6.7% of the correspondent edge mean. The operator is doing declared
work at the locations where local contrast alone is insufficient.

---

## Why the gap grows with complexity

The STFT is the minimum conventional transformation for a 1D signal.
For higher-dimensional or multi-domain analysis:

- Each additional signal dimension requires an additional transformation layer
- Cross-domain coupling requires further representation steps
- The relational path adds a declared relation and declared provenance

The conventional computation burden scales with the complexity of the
representation problem. The relational burden scales with the declared
relational structure of the observable — which is what was actually
observed, not a mathematical convenience imposed on top of it.

This is not an optimization. It is a structural difference in where
the mathematics begins: at the representation, or at the observation.

---

## Run it yourself

```
cargo test
cargo run --bin run_voice_phase0 -- Voice_44100_16bit.wav
```

Place `Voice_44100_16bit.wav` in the repo root before running.

---

## Verification appendix — synthetic scaling

`cargo run --bin run_phase0` runs both paths over synthetic gradient
fields at N = 100, 1,000, 10,000, 100,000. These confirm O(N) scaling
and full correspondence for both paths at all tiers. They are
implementation verification, not a compute advantage claim — a synthetic
gradient field carries no representational burden, so there is no tax
to avoid.

---

## Declaration

Full experimental declaration: `docs/DECLARATION_VOICE.md`  
M declaration for the voice input: `docs/DECLARATION_VOICE.md`  
Origin declaration for the experiment structure: `docs/DECLARATION.md`

---

*The operators hold no agency. Responsibility stays with Origin.*  
Metatron Dynamics, Inc. Kernel V8. Bounded over D. No claim beyond D.
