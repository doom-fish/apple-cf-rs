# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.6.2]

### Added

- New `raw` module with exhaustive low-level CoreFoundation / CoreMedia / CoreVideo / IOSurface / Dispatch bindings generated from the active macOS SDK, plus hand-written coverage for inline helpers like `CFByteOrder*`, `CFString*InlineBuffer`, `CMTag*`, `CMTimebase*` compatibility shims, `dispatch_get_main_queue`, and the remaining CVMetal declarations.
- Smoke example `15_raw_bindings` and matching `raw_bindings_tests` coverage harness for the new exhaustive raw surface.

### Changed

- `COVERAGE_AUDIT.md` now reports `0` remaining gaps and `100.00%` coverable coverage (`95.18%` overall once deprecated / unavailable exemptions are included).
- README / COVERAGE docs refreshed for the new `apple_cf::raw` module.
- Dispatch queue creation bridge renamed to `acf_dispatch_queue_create` to avoid colliding with the system `dispatch_queue_create` symbol now exposed through `apple_cf::raw`.
- `Cargo.toml` version bumped to `0.6.2`.

## [0.6.1]

### Added

- `cf::CFSet` / `CFMutableSet` and `cf::CFPropertyList`, including Swift bridge coverage, examples, and smoke tests.
- `dispatch_queue::dispatch_async`, `dispatch_async_and_wait`, and `dispatch_apply` safe helpers.
- `cm::CMMetadataFormatDescription` plus metadata-description constants, constructors, identifier lookup, and merge/extend helpers.
- New numbered example `14_cm_metadata_format_description` and matching CoreMedia metadata smoke tests.

### Changed

- `COVERAGE_AUDIT.md` refreshed for the highest-value remaining gaps; deprecated `CVDisplayLink` symbols now live in the exempt bucket.
- README / COVERAGE docs refreshed for the new CoreFoundation, Dispatch, and CoreMedia surface.
- `Cargo.toml` version bumped to `0.6.1`.

## [0.6.0]

### Added

- **`cf` module** — safe Core Foundation wrappers for:
  - value types: `CFType`, `CFString`, `CFNumber`, `CFData`, `CFDate`, `CFUUID`, `CFError`
  - collections: `CFArray`, `CFDictionary` / `CFDict`, `CFBag`, `CFTree`, `CFAttributedString`
  - resources / locale / formatting: `CFURL`, `CFBundle`, `CFLocale`, `CFCalendar`, `CFTimeZone`, `CFCharacterSet`, `CFNumberFormatter`, `CFDateFormatter`, `CFPreferences`, `CFFileSecurity`, `CFXML`
  - runtime helpers: `CFNotificationCenter`, `CFRunLoop`, `CFTimer`, `CFMessagePort`, `CFStreamPair`, `CFSocket`, `CFFileDescriptor`
- **Dispatch sync primitives** — `DispatchGroup`, `DispatchSemaphore`, and timer-backed `DispatchSource` in `dispatch_queue`.
- **CoreMedia time extras** — `CMTimeRange`, `CMClock::host_time_clock()`, and `CMTimebase`.
- **CoreVideo extras** — `CVBuffer`, `CVImageBuffer`, `CVMetalTextureCache`.
- Eight new numbered examples (`06_` through `13_`) covering the new CoreFoundation / Dispatch / CoreMedia / CoreVideo surface.
- Seven new test files covering the new wrappers.
- `COVERAGE.md` header-audit summary for the Wave-C sweep.

### Changed

- README refreshed for the expanded CoreFoundation / Dispatch / media coverage.
- `Cargo.toml` version bumped to `0.6.0`.
- `build.rs` now links the `Metal` framework for `CVMetalTextureCache` support.

## [0.5.0]

### Added

- **`cg::CGContext`** — safe Core Graphics bitmap-context wrapper with RGBA8 and grayscale constructors, byte accessors, path/rect drawing, transforms, graphics-state save/restore, image drawing, and bitmap snapshots.
- `CGImage::save_png()` helper backed by the existing ImageIO Swift bridge so bitmap snapshots can be written to disk without extra dependencies.
- Smoke example `05_cgcontext_smoke` proving a 64×64 offscreen `CGContext` can draw shapes, snapshot to `CGImage`, export a PNG, and verify pixel contents.

### Changed

- `cg` module docs now cover both value types and bitmap drawing wrappers.

### Added

- **`cv` module** — `CVPixelBuffer` and `CVPixelBufferPool` carved out of
  `screencapturekit-rs`. Wraps the CoreVideo primitives that pair an
  IOSurface with format metadata.
- `cv` feature flag (on by default; implies `iosurface`).
- `CoreVideoBridge` Swift target with the underlying `cv_pixel_buffer_*`
  and `cv_pixel_buffer_pool_*` `@_cdecl` exports.
- `CVPixelBuffer::create_with_io_surface(&IOSurface)` lets downstream
  consumers (e.g. `vision-rs`) ingest live capture data without a PNG
  round-trip.
- Smoke test `04_cv_pixel_buffer` proves the IOSurface ↔ CVPixelBuffer
  round-trip: write `[0xDE, 0xAD, 0xBE, 0xEF]` via IOSurface, read back
  via the wrapped CVPixelBuffer, verify identical bytes and identical
  IOSurface id on the round-trip.
- API harness extended to CVPixelBuffer + CVPixelBufferPool — 7/7 tests
  pass at 100% coverable.

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
