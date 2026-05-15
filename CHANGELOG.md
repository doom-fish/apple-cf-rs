# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **`cm` module** — CoreMedia value types and reference-counted wrappers
  carved out of `screencapturekit-rs`:
  - `CMTime` / `CMSampleTimingInfo` (pure value types, 0 deps)
  - `CMSampleBuffer` — safe Drop/Clone wrapper with accessors for PTS, DTS,
    duration, num_samples, validity, format description, data buffer, and
    raw image-buffer pointer hand-off. SCStreamFrameInfo attachment readers
    intentionally **not** ported — those stay in screencapturekit-rs.
  - `CMBlockBuffer` — Drop/Clone wrapper with data length, contiguous-range
    check, byte-copy, data pointer access, and create-with-data / create-empty
    constructors.
  - `CMFormatDescription` — Drop/Clone wrapper with media type / subtype /
    extensions, plus audio-specific accessors (sample rate, channel count,
    bits-per-channel, bytes-per-frame, format flags).
  - `audio` — `AudioBuffer` / `AudioBufferList` / `AudioBufferListRaw`
    bridging types ported verbatim.
- **CoreMediaBridge Swift target** with 28 `@_cdecl` exports covering the
  generic CMSampleBuffer / CMBlockBuffer / CMFormatDescription surface.
- **`cm` feature flag** (on by default) so audio-only consumers can opt
  out of the CoreMedia symbols.
- Smoke test `03_cm_sample_buffer` proves end-to-end retain/release across
  the videotoolbox ↔ apple-cf boundary: encodes one H.264 frame, wraps the
  resulting CMSampleBuffer in our safe type, and inspects PTS/data-buffer/
  format-description with real values (`vide` / `avc1`, 142 bytes of H.264).
- API coverage harness extended to CMSampleBuffer / CMBlockBuffer /
  CMFormatDescription — 5/5 tests pass at 100% coverable coverage.

### Changed

- Re-exports from `prelude`: `CMTime`, `CMSampleBuffer`, `CMBlockBuffer`,
  `CMFormatDescription` join the ergonomic prelude (gated on `cm` feature).


### Added

- Initial scaffold carved out of `screencapturekit-rs`.
- `cg` — CoreGraphics value types (`CGRect`, `CGPoint`, `CGSize`).
- `iosurface` — full `IOSurface` API (single- and multi-planar, lock/unlock,
  use-count tracking, properties).
- `dispatch_queue` — `DispatchQueue` + `DispatchQoS`.
- `utils` — `FourCharCode`, `SyncCompletion` / `AsyncCompletion`,
  `ffi_string_owned`, `panic_safe` callback wrapper.
- Swift bridge with separate `CoreGraphicsBridge`, `IOSurfaceBridge`,
  `DispatchBridge` targets aggregated under a single static
  `AppleCFBridge` library.
- `acf_free_string` centralised heap-string deallocator.
- Two smoke-test examples that exercise the full Rust → C FFI → Swift →
  Apple framework path.

### Planned

- `cm` (CoreMedia) once `SCStreamFrameInfo` attachments are decoupled
  upstream in screencapturekit-rs.
- `cv` (CoreVideo).
- `metal` (Metal).
