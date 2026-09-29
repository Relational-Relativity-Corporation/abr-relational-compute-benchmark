# abr-relational-compute-benchmark

**Metatron Dynamics, Inc. — Kernel V8**  
Bounded over D. No claim beyond D.

---

## What this is

A compute comparison experiment.

Same input X enters two independent mathematical treatments. Both must
produce output that corresponds with X under the same declared criterion.
Compute is measured after correspondence is established. Neither path is
adjusted based on the other's result.

```
X  →  relational path  (M → D → topology → Kernel V8 Primary: Δ → Σ)  →  X'_R
X  →  conventional path  (mean subtraction + normalization)             →  X'_S

X'_R  ↔  X    (sign preservation on declared edges)
X'_S  ↔  X    (sign preservation on declared edges)

C_R  vs  C_S  (arithmetic ops, comparisons, allocations, memory, wall time)
```

This is not a claim that relational mathematics is cheaper. It is an
experiment designed to measure that question reproducibly.

---

## Phase 0 — synthetic structured fields

N = 10², 10³, 10⁴, 10⁵. Gradient field. 1D directed chain. Single component.

These tiers separate the compute-scaling question from any domain-specific
declaration question. Domain adapters are Phase 1.

See `docs/DECLARATION.md` for the full experimental declaration including
M, topology, output requirement, and declared open conditions.

---

## Run

```
cargo test
cargo run --bin run_phase0
```

---

## What the output means

The experiment table shows, for each N and each path:
- Whether the path corresponds with X (sign preservation on declared edges)
- Arithmetic operations, comparisons, allocations, peak memory, wall time

Divergence is not suppressed. It is recorded and returned to Origin.

---

## Phase 1 (declared, not yet built)

Domain adapters: synthetic structured field → image → voice → language →
thermal telemetry.

Full ABR kernel (A → B → R) replaces Primary kernel when M establishes at
least one interior locus (full-operator admissibility condition, Kernel V8).

The conventional comparison path for Phase 1 will be declared before running.

---

Metatron Dynamics, Inc. Kernel V8. Bounded over D. No claim beyond D.
