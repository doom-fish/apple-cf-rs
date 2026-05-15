# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
