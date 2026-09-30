#!/usr/bin/env python3
# HISTORICAL REFERENCE ONLY — superseded by experiments/run_voice_phase0.rs (V0.2.0)
#
# This file describes the V0.1.0 experiment structure using the Δ→Σ (Primary)
# kernel and the 0.1s highest-RMS window of Voice_44100_16bit.wav.
# It is retained for reference only and is not part of the active V0.2.0
# execution surface. Do not run as a current benchmark.
#
# Current experiment: experiments/run_voice_phase0.rs
# Current declaration: docs/DECLARATION_VOICE.md
"""
run_voice_phase0.py — abr-relational-compute-benchmark
Metatron Dynamics, Inc. Kernel V8. Bounded over D. No claim beyond D.

Voice experiment — corrected.

Experimental control: X in, X out. Both paths must return output in the
sample domain that corresponds with X. Compute is compared after
correspondence is established for each path independently.

Relational path:    Primary kernel Δ → Σ. Output is over declared edges.
                    Correspondence: sign(Σ[e]) = sign(x[s] - x[t]).

Conventional path:  STFT round-trip (forward STFT + inverse STFT).
                    STFT is the standard lossless time-frequency transform
                    for conventional voice processing. It is unavoidable —
                    raw sample values are not directly legible to downstream
                    frequency-domain systems. The round-trip reconstructs
                    exactly to the sample domain (zero reconstruction error).
                    Correspondence: sign(xr[e] - xr[e+1]) = sign(x[e] - x[e+1]).

MFCC was removed as the conventional path: MFCC discards phase by design
and cannot reconstruct to the sample domain with acceptable fidelity.
It is not a round-trip operation and therefore cannot meet the
input=output experimental requirement. That is a property of MFCC,
not a failing — it was not designed for this task.

See docs/DECLARATION_VOICE.md for the full M declaration.
"""

import sys, time, math
import numpy as np
from scipy.signal import stft, istft

SAMPLE_RATE  = 44100
WINDOW_START = 57330
WINDOW_N     = 4410
RHO_BASE     = 0.3
CHI_0        = 0.01
ZERO_TOL     = 1e-10

# STFT parameters — declared before running
STFT_NPERSEG = 512
STFT_NOVERLAP = 256


# ── Relational path: Primary kernel Δ → Σ ────────────────────────────────

def run_relational_primary(x):
    ops = {'arithmetic': 0, 'comparison': 0, 'allocations': 0, 'peak_bytes': 0}
    t0 = time.perf_counter_ns()
    N = len(x)
    M = N - 1

    # Δ
    ops['allocations'] += 1
    delta = np.empty(M)
    for e in range(M):
        delta[e] = x[e] - x[e + 1]
        ops['arithmetic'] += 1
    ops['peak_bytes'] += M * 8

    # ρ — per-node χ from incident edges (chain: each node touches ≤2 edges)
    ops['allocations'] += 1
    node_chi = np.zeros(N)
    for e in range(M):
        av = abs(delta[e])
        ops['arithmetic'] += 1
        ops['comparison'] += 2
        if av > node_chi[e]:     node_chi[e]     = av
        if av > node_chi[e + 1]: node_chi[e + 1] = av
    ops['peak_bytes'] += N * 8

    ops['allocations'] += 1
    rho = np.empty(M)
    for e in range(M):
        chi = node_chi[e]
        rho[e] = RHO_BASE * chi / (CHI_0 + chi)
        ops['arithmetic'] += 3
    ops['peak_bytes'] += M * 8

    # Σ — chain topology: adj⁺[e] = {e+1}, adj⁻[e] = {e-1}
    ops['allocations'] += 1
    sigma = np.empty(M)
    for e in range(M):
        fwd = delta[e + 1] if e + 1 < M else 0.0
        bwd = delta[e - 1] if e - 1 >= 0 else 0.0
        sigma[e] = delta[e] + rho[e] * (fwd - bwd)
        ops['arithmetic'] += 3
    ops['peak_bytes'] += M * 8

    elapsed_us = (time.perf_counter_ns() - t0) // 1000
    return sigma, ops, elapsed_us


# ── Conventional path: STFT round-trip ───────────────────────────────────

def run_stft_roundtrip(x):
    """
    Forward STFT + inverse STFT. Reconstructs exactly to sample domain.
    Operations counted: FFT butterfly operations (N·log₂N per frame,
    forward and inverse), windowing multiplies, overlap-add additions.
    """
    ops = {'arithmetic': 0, 'comparison': 0, 'allocations': 0, 'peak_bytes': 0}
    t0 = time.perf_counter_ns()
    N = len(x)
    nperseg   = STFT_NPERSEG
    noverlap  = STFT_NOVERLAP
    hop       = nperseg - noverlap
    n_frames  = 1 + (N - nperseg) // hop

    # Windowing (Hann window, applied per frame)
    ops['allocations'] += 1
    window = np.hanning(nperseg)
    ops['arithmetic'] += nperseg * 2   # cos + scale per sample

    # Forward STFT
    fft_ops_per_frame = int(nperseg * math.log2(nperseg))  # butterfly ops
    n_bins = nperseg // 2 + 1
    ops['allocations'] += 1
    _, _, Zxx = stft(x, fs=SAMPLE_RATE, nperseg=nperseg, noverlap=noverlap,
                     window='hann')
    ops['arithmetic'] += n_frames * (fft_ops_per_frame + nperseg)  # FFT + window
    ops['peak_bytes'] += n_frames * n_bins * 16  # complex128

    # Inverse STFT — same cost as forward + overlap-add
    ops['allocations'] += 1
    _, xr_full = istft(Zxx, fs=SAMPLE_RATE, nperseg=nperseg, noverlap=noverlap,
                       window='hann')
    ops['arithmetic'] += n_frames * (fft_ops_per_frame + nperseg)  # IFFT + window
    ops['arithmetic'] += N                                           # overlap-add
    ops['peak_bytes'] += len(xr_full) * 8

    # Trim to N
    xr = xr_full[:N] if len(xr_full) >= N else np.pad(xr_full, (0, N - len(xr_full)))
    ops['allocations'] += 1
    ops['peak_bytes'] += N * 8

    elapsed_us = (time.perf_counter_ns() - t0) // 1000
    return xr, ops, elapsed_us


# ── Correspondence check ──────────────────────────────────────────────────

def check_correspondence_relational(x, sigma):
    """Relational: sign(sigma[e]) = sign(x[i] - x[i+1])"""
    M = len(x) - 1
    corr = 0
    for e in range(M):
        exp = x[e] - x[e + 1]
        act = sigma[e]
        if abs(exp) <= ZERO_TOL and abs(act) <= ZERO_TOL:
            corr += 1
        elif exp * act > 0:
            corr += 1
    return corr, M

def check_correspondence_conventional(x, xr):
    """Conventional: sign(xr[i] - xr[i+1]) = sign(x[i] - x[i+1])"""
    M = len(x) - 1
    corr = 0
    for e in range(M):
        exp = x[e]  - x[e + 1]
        act = xr[e] - xr[e + 1]
        if abs(exp) <= ZERO_TOL and abs(act) <= ZERO_TOL:
            corr += 1
        elif exp * act > 0:
            corr += 1
    return corr, M


# ── Main ──────────────────────────────────────────────────────────────────

def main():
    try:
        import soundfile as sf
    except ImportError:
        print("soundfile not available"); sys.exit(1)

    print("abr-relational-compute-benchmark — Voice Phase 0 (corrected)")
    print("Metatron Dynamics, Inc. Kernel V8. Bounded over D. No claim beyond D.")
    print()
    print(f"Window:   samples {WINDOW_START}–{WINDOW_START+WINDOW_N-1}")
    print(f"          ({WINDOW_START/SAMPLE_RATE:.4f}s – {(WINDOW_START+WINDOW_N)/SAMPLE_RATE:.4f}s)")
    print(f"N:        {WINDOW_N} samples  |  Edges: {WINDOW_N-1}")
    print(f"Paths:    Relational (Δ→Σ)  vs  Conventional (STFT round-trip)")
    print(f"Control:  X in → [path] → X' out; both checked against X")
    print()

    data, sr = sf.read('/mnt/user-data/uploads/Voice_44100_16bit.wav')
    assert sr == SAMPLE_RATE
    x = data[WINDOW_START:WINDOW_START + WINDOW_N, 0].copy()
    assert len(x) == WINDOW_N and np.all(np.isfinite(x))

    rms_x = np.sqrt(np.mean(x**2))
    print(f"X:  min={x.min():.6f}  max={x.max():.6f}  RMS={rms_x:.6f}")
    print()

    # Run paths
    sigma, rel_ops, rel_us  = run_relational_primary(x)
    xr,    conv_ops, conv_us = run_stft_roundtrip(x)

    # Correspondence — each against X independently
    rel_corr,  M = check_correspondence_relational(x, sigma)
    conv_corr, _ = check_correspondence_conventional(x, xr)

    rms_err = np.sqrt(np.mean((x - xr)**2))

    # Report
    def state(n, total):
        if n == total: return 'CORRESPONDENT'
        if n == 0:     return 'DIVERGENT'
        return f'PARTIAL {n}/{total}'

    print(f"{'':35s}  {'Relational (Δ→Σ)':>20s}  {'Conventional (STFT)':>20s}")
    print("-" * 80)
    print(f"{'Correspondence':35s}  {state(rel_corr,M):>20s}  {state(conv_corr,M):>20s}")
    print(f"{'Edges correspondent':35s}  {rel_corr:>20,d}  {conv_corr:>20,d}")
    print(f"{'Total edges':35s}  {M:>20,d}  {M:>20,d}")
    print(f"{'Reconstruction RMS error':35s}  {'(edge domain)':>20s}  {rms_err:>20.2e}")
    print()
    print(f"{'Arithmetic ops':35s}  {rel_ops['arithmetic']:>20,d}  {conv_ops['arithmetic']:>20,d}")
    print(f"{'Comparison ops':35s}  {rel_ops['comparison']:>20,d}  {conv_ops['comparison']:>20,d}")
    print(f"{'Allocations':35s}  {rel_ops['allocations']:>20,d}  {conv_ops['allocations']:>20,d}")
    print(f"{'Peak memory (bytes est.)':35s}  {rel_ops['peak_bytes']:>20,d}  {conv_ops['peak_bytes']:>20,d}")
    print(f"{'Wall time (µs)':35s}  {rel_us:>20,d}  {conv_us:>20,d}")
    print()

    total_rel  = rel_ops['arithmetic']  + rel_ops['comparison']
    total_conv = conv_ops['arithmetic'] + conv_ops['comparison']
    ratio = total_conv / total_rel if total_rel > 0 else float('inf')
    print(f"Total ops ratio (conv/rel): {ratio:.2f}×")
    print()
    print("Correspondence checked against X independently for each path.")
    print("Divergence, if any, is a finding returned to Origin — not suppressed.")
