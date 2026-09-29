# Origin Declaration — abr-relational-compute-benchmark

**Metatron Dynamics, Inc.**
Kernel V8. Bounded over D. No claim beyond D.

---

## Purpose

This repository is a compute comparison experiment.

It does not claim that the relational path is correct or that the conventional
path is incorrect. It does not claim that one path is computationally superior
before running the experiment. It measures what each path actually does and
compares both outputs against the same input.

---

## Experimental structure

```
X  →  relational path  (M → D → declared topology → Kernel V8)  →  X'_R
X  →  conventional path  (reduction pipeline)                   →  X'_S

X'_R  ↔  X    (output correspondence check)
X'_S  ↔  X    (output correspondence check)

C_R  vs  C_S  (compute comparison, recorded after correspondence established)
```

X is identical for both paths. The output requirement is that both paths
return a result that corresponds with the structure of X. The compute
comparison is only meaningful after correspondence is established for each path.

---

## What is being compared

**Relational path:** the declared observable organization of X enters through
M. A finite numerical projection is produced in D. A declared relational
topology is constructed over that projection. The frozen Kernel V8 operators
(Δ → Σ for Primary, A → B → R for full ABR) are applied unchanged. The
output is compared with X.

**Conventional path:** X enters a reduction pipeline appropriate to the input
type. The pipeline is declared before running. Its output is compared with X
using the same correspondence criterion as the relational path.

---

## What is not being compared

- The relational path against a task the conventional path was not asked to
  perform.
- The relational path against an arbitrarily large or capable conventional
  system.
- Two paths over different inputs.
- Outputs against each other. Both outputs answer to X independently.

---

## Input contract

An input instance I = (X, M, E, P) where:

- **X** — the observed values, finite and bounded. A NodeField in the Kernel
  V8 sense: data[c][i], component c, node i. All values finite (∈ D).
- **M** — the declared measurement projection: what observable organization X
  carries and how it is projected into D. Declared per input type by the
  adapter.
- **E** — the declared relational topology: which loci are related, in which
  direction, with what provenance. Declared per input type by the adapter.
  Direction comes from the observed organization of X (position, ordering,
  adjacency, or another directly recorded structural distinction in X).
- **P** — the M-supplied kernel parameters: ρ_base, χ₀, and cc[p] for each
  declared component pair. No defaults. Declared per analysis.

The executor does not know the domain. The adapter declares M, E, and P.

---

## Output requirement

Both paths must return a result whose structure corresponds with X under a
declared correspondence criterion. The criterion is declared per input tier
before running.

For synthetic structured fields: exact correspondence (implementation-only
tolerance declared per tier for floating-point arithmetic; this tolerance is
not a property of the observed system).

---

## Outcome states

A result may be:

1. **Correspondent** — output corresponds with X under the declared criterion.
2. **Partially correspondent** — some structure corresponds, some does not.
3. **Divergent** — output does not correspond.
4. **Not evaluable** — a required declaration is missing; execution stops at
   that boundary.

Divergence is not failure. It is a finding returned to Origin.
Neither path establishes ground truth. Both answer to X.

---

## Compute instrumentation

Recorded per run, per path, per input size N:

- Arithmetic operations executed (counted, not estimated)
- Comparisons executed
- Memory allocations
- Peak memory (bytes)
- Wall-clock execution time (μs)
- Pairwise comparisons (where applicable)

These are instrumentation measurements. They are not kernel quantities and do
not enter the operator chain.

The interesting experimental quantity is not C_R < C_S in isolation. It is:

    (validated correspondence, C_R) vs (validated correspondence, C_S)

across increasing N and across input adapters.

---

## Input tiers — Phase 0

Synthetic structured numerical fields at increasing N. No real-world domain.
Domain adapters are Phase 1 and beyond.

| Tier | N | Structure |
|------|---|-----------|
| T1   | 100 (10²)  | 1D directed chain, single component, gradient field |
| T2   | 1,000 (10³) | 1D directed chain, single component, gradient field |
| T3   | 10,000 (10⁴) | 1D directed chain, single component, gradient field |
| T4   | 100,000 (10⁵) | 1D directed chain, single component, gradient field |

All four tiers use identical structure; only N varies. This separates the
compute-scaling question from any declaration question.

---

## Kernel V8 usage

The Kernel V8 operators are used as declared in `operators.rs`. They are not
rewritten, renamed, or extended for this repository. The executor constructs
the declared inputs and calls the frozen functions.

**Kernel selection for Phase 0 synthetic fields:**
A 1D directed chain with N nodes has interior loci (every node except the
terminal). The full-operator admissibility condition is satisfied. The ABR
kernel (A → B → R) applies.

Phase 0 uses the Primary kernel (Δ → Σ) as the relational path for the
initial comparison, because the conventional path (mean-subtraction over the
same chain) is structurally comparable to Δ → Σ. The ABR full path is
Phase 1, where the conventional comparison path will be correspondingly
declared.

---

## Conventional path — Phase 0

Mean subtraction followed by normalization over the same 1D field. This is
the minimal conventional reduction that meets the output requirement (preserve
differentiable structure across the field). It is not a heavyweight pipeline.
It is the smallest conventional treatment that does the same job.

Declared before running. Not adjusted after seeing results.

---

## Correspondence criterion — Phase 0

For each node i, the output value at i must preserve the sign and approximate
magnitude of the input contrast at i relative to its declared neighbors.
Formally: for each declared edge e = (s, t), sign(output[s] - output[t]) =
sign(X[s] - X[t]).

Implementation-only tolerance: 1e-10 for floating-point zero comparisons.
This tolerance is not a property of the observed system.

---

## What this repository does not do

- Rewrite or reinterpret Kernel V8.
- Declare relational structure on behalf of Origin.
- Adjust declarations to obtain correspondence.
- Interpret operator output as domain-specific meaning before correspondence
  is established.
- Claim compute advantage before running the experiment.

---

## Open conditions at declaration

- OC-BCH-1: Whether the correspondence criterion declared here (sign
  preservation) is sufficient or whether a stronger criterion is needed for
  Phase 1 real-domain adapters. To be determined when Phase 1 adapters are
  declared.
- OC-BCH-2: Whether the conventional path declared for Phase 0 (mean
  subtraction + normalization) is the minimal adequate comparator or whether
  a different conventional treatment better represents the comparison class.
  Declared before Phase 1.
- OC-BCH-3: Whether compute instrumentation (operation counting) is
  implementation-accurate or requires hardware-level validation.

---

Bounded over D. No claim beyond D.
Metatron Dynamics, Inc. Kernel V8.
