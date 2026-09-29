// types.rs — abr-relational-compute-benchmark
// Metatron Dynamics, Inc. Kernel V8. Bounded over D. No claim beyond D.
//
// Declared types for the benchmark experiment.
// No domain knowledge here. The adapter supplies M, E, P.

// ── Input contract ────────────────────────────────────────────────────────
//
// I = (X, M, E, P)
//
// X    — observed values as a flat Vec<f64>, finite and bounded (∈ D).
//         Interpreted as data[0][i] — single component, node i — for Phase 0.
// M    — declared measurement projection: a label naming what observable
//         organization X carries. The executor does not interpret it.
//         The adapter declares it. Carried for provenance record only.
// E    — declared edge list: (source_index, target_index) pairs, with
//         provenance label. Direction comes from the observed organization
//         of X declared by the adapter — not from index order alone.
// P    — M-supplied kernel parameters: ρ_base, χ₀. cc is empty for
//         single-component Phase 0 (no component pairs declared).

#[derive(Clone, Debug)]
pub struct BenchmarkInput {
    /// Observed values — finite, bounded. data[i] = value at node i.
    pub x: Vec<f64>,
    /// Declared measurement projection label (provenance record).
    pub m_label: String,
    /// Declared edges: (source, target). Direction from adapter declaration.
    pub edges: Vec<(usize, usize)>,
    /// Edge provenance labels — one per edge.
    pub edge_provenance: Vec<String>,
    /// M-supplied ρ_base. No default. Must be declared.
    pub rho_base: f64,
    /// M-supplied χ₀. No default. Must be declared. Must be > 0.
    pub chi_0: f64,
}

impl BenchmarkInput {
    /// Validate that all required declarations are present and values are in D.
    pub fn validate(&self) -> Result<(), String> {
        if self.x.is_empty() {
            return Err("X is empty — no declared observations".into());
        }
        if self.x.iter().any(|v| !v.is_finite()) {
            return Err("X contains non-finite value — not in D".into());
        }
        if self.edges.is_empty() {
            return Err("No edges declared — topology required".into());
        }
        if self.edges.len() != self.edge_provenance.len() {
            return Err("Edge provenance count must match edge count".into());
        }
        let n = self.x.len();
        for &(s, t) in &self.edges {
            if s >= n || t >= n {
                return Err(format!(
                    "Edge ({},{}) references node outside declared range [0,{})",
                    s, t, n
                ));
            }
            if s == t {
                return Err(format!(
                    "Edge ({},{}) is a self-loop — inadmissible (ring prohibition)",
                    s, t
                ));
            }
        }
        if !self.rho_base.is_finite() {
            return Err("ρ_base must be declared by M as a finite value".into());
        }
        if !self.chi_0.is_finite() || self.chi_0 <= 0.0 {
            return Err("χ₀ must be declared by M as a finite positive value".into());
        }
        Ok(())
    }

    pub fn n_nodes(&self) -> usize { self.x.len() }
    pub fn n_edges(&self) -> usize { self.edges.len() }
}

// ── Output ────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct PathOutput {
    /// Calculated result at each declared edge.
    /// output[e] corresponds to edge e = (source, target).
    pub values: Vec<f64>,
    /// How many declared edges are in the output.
    pub n_edges: usize,
}

// ── Correspondence result ─────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
pub enum CorrespondenceState {
    /// Output corresponds with X under the declared criterion.
    Correspondent,
    /// Some edges correspond, some do not.
    PartiallyCorrespondent { corresponding: usize, total: usize },
    /// Output does not correspond.
    Divergent,
    /// Required declaration missing; execution stopped.
    NotEvaluable(String),
}

impl CorrespondenceState {
    pub fn is_fully_correspondent(&self) -> bool {
        matches!(self, CorrespondenceState::Correspondent)
    }
}

// ── Instrumentation ───────────────────────────────────────────────────────
//
// These are measurement quantities, not kernel quantities.
// They do not enter the operator chain.

#[derive(Clone, Debug, Default)]
pub struct ComputeRecord {
    /// Arithmetic operations executed (additions, subtractions, multiplications,
    /// divisions). Counted explicitly in the implementation, not estimated.
    pub arithmetic_ops: u64,
    /// Comparison operations executed (>, <, ==, abs).
    pub comparison_ops: u64,
    /// Memory allocations (Vec::new, push requiring reallocation, etc.)
    pub allocations: u64,
    /// Peak memory in bytes (estimated from declared data structures).
    pub peak_memory_bytes: u64,
    /// Wall-clock execution time in microseconds.
    pub wall_clock_us: u64,
    /// Pairwise comparisons (declared where applicable, 0 otherwise).
    pub pairwise_comparisons: u64,
}

impl ComputeRecord {
    pub fn new() -> Self { Self::default() }
}

// ── Experiment result ─────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct ExperimentResult {
    pub input_n: usize,
    pub relational: SinglePathResult,
    pub conventional: SinglePathResult,
}

#[derive(Clone, Debug)]
pub struct SinglePathResult {
    pub path_label: String,
    pub output: Option<PathOutput>,
    pub correspondence: CorrespondenceState,
    pub compute: ComputeRecord,
}
