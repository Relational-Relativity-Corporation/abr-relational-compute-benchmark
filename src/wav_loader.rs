// wav_loader.rs — abr-relational-compute-benchmark
// Metatron Dynamics, Inc. Kernel V8. Bounded over D. No claim beyond D.
//
// Minimal WAV loader for 16-bit stereo PCM files.
// Reads the declared window from the L channel only.
// No dependencies beyond std.

use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;

#[derive(Debug)]
pub struct WavInfo {
    pub sample_rate: u32,
    pub bits_per_sample: u16,
    pub n_channels: u16,
    pub n_frames: usize,
}

/// Read a window of samples from a 16-bit stereo PCM WAV file.
/// Returns normalized f64 samples in [-1, 1] from the L channel.
///
/// # Arguments
/// * `path`         — path to the WAV file
/// * `start_sample` — first sample index (frame index, not byte)
/// * `n_samples`    — number of samples to read
pub fn read_wav_window(
    path: &Path,
    start_sample: usize,
    n_samples: usize,
) -> io::Result<(WavInfo, Vec<f64>)> {
    let mut f = File::open(path)?;

    // ── RIFF header ───────────────────────────────────────────────────
    let mut riff = [0u8; 12];
    f.read_exact(&mut riff)?;
    if &riff[0..4] != b"RIFF" || &riff[8..12] != b"WAVE" {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "not a RIFF/WAVE file"));
    }

    // ── Chunk scan ────────────────────────────────────────────────────
    let mut sample_rate = 0u32;
    let mut bits_per_sample = 0u16;
    let mut n_channels = 0u16;
    let mut data_offset: u64 = 0;
    let mut data_size: u32 = 0;

    loop {
        let mut chunk_id = [0u8; 4];
        if f.read_exact(&mut chunk_id).is_err() { break; }
        let mut size_buf = [0u8; 4];
        f.read_exact(&mut size_buf)?;
        let chunk_size = u32::from_le_bytes(size_buf);

        match &chunk_id {
            b"fmt " => {
                let mut fmt = vec![0u8; chunk_size as usize];
                f.read_exact(&mut fmt)?;
                // audio format (1=PCM), n_channels, sample_rate,
                // byte_rate, block_align, bits_per_sample
                n_channels     = u16::from_le_bytes([fmt[2], fmt[3]]);
                sample_rate    = u32::from_le_bytes([fmt[4], fmt[5], fmt[6], fmt[7]]);
                bits_per_sample = u16::from_le_bytes([fmt[14], fmt[15]]);
            }
            b"data" => {
                data_offset = f.stream_position()?;
                data_size   = chunk_size;
                break;
            }
            _ => {
                f.seek(SeekFrom::Current(chunk_size as i64))?;
            }
        }
    }

    if data_offset == 0 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "no data chunk found"));
    }
    if bits_per_sample != 16 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("expected 16-bit PCM, got {}-bit", bits_per_sample),
        ));
    }

    let bytes_per_frame = (n_channels as usize) * (bits_per_sample as usize / 8);
    let n_frames_total = data_size as usize / bytes_per_frame;

    let info = WavInfo { sample_rate, bits_per_sample, n_channels, n_frames: n_frames_total };

    // ── Read declared window — L channel only ─────────────────────────
    let end_sample = (start_sample + n_samples).min(n_frames_total);
    let actual_n = end_sample - start_sample;

    let byte_offset = data_offset + (start_sample * bytes_per_frame) as u64;
    f.seek(SeekFrom::Start(byte_offset))?;

    let mut samples = Vec::with_capacity(actual_n);
    let mut frame_buf = vec![0u8; bytes_per_frame];
    let scale = 1.0 / 32768.0_f64;

    for _ in 0..actual_n {
        f.read_exact(&mut frame_buf)?;
        // L channel: bytes 0–1 (little-endian i16)
        let raw = i16::from_le_bytes([frame_buf[0], frame_buf[1]]);
        samples.push(raw as f64 * scale);
    }

    Ok((info, samples))
}
