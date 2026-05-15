# apple-cf

Safe, dependency-free Rust bindings for Apple's shared **Core\*** frameworks — the foundation underneath the [doom-fish](https://github.com/doom-fish) macOS Rust suite.

> **Status:** experimental. Carved out of [`screencapturekit-rs`](https://github.com/doom-fish/screencapturekit-rs); more frameworks (CoreMedia, CoreVideo, Metal) will land in subsequent releases.

## What's in the box

| Module | Framework | Feature flag | Status |
|---|---|---|---|
| [`cg`](src/cg) | CoreGraphics value types (`CGRect`, `CGPoint`, `CGSize`) | `cg` | ✅ |
| [`iosurface`](src/iosurface) | IOSurface (zero-copy GPU buffers, multi-planar formats) | `iosurface` | ✅ |
| [`dispatch_queue`](src/dispatch_queue.rs) | Grand Central Dispatch queues + QoS | `dispatch` | ✅ |
| [`cm`](src/cm) | CoreMedia (`CMSampleBuffer`, `CMTime`, `CMBlockBuffer`, `CMFormatDescription`) | `cm` | ✅ |
| [`utils`](src/utils) | Shared FFI helpers (always on) | — | ✅ |
| `cv` | CoreVideo (`CVPixelBuffer`, `CVPixelBufferPool`) | `cv` | 🚧 planned |
| `metal` | Metal (`MTLDevice`, `MTLTexture`) | `metal` | 🚧 planned |

## Why this crate exists

Every doom-fish crate that wraps a media-adjacent Apple framework needs the same primitives — `CGRect`, `CVPixelBuffer`, `IOSurface`, dispatch queues. Instead of re-vendoring those inside every crate (and drifting), this crate owns them once.

```
┌────────────────────────────────────────────────────────────┐
│ screencapturekit / videotoolbox / avfoundation / vision    │
│           ↓ depends on ↓                                   │
│ ┌────────────────────────────────────────────────────────┐ │
│ │  apple-cf  (this crate)                                │ │
│ └────────────────────────────────────────────────────────┘ │
│           ↓ Swift @_cdecl bridge ↓                         │
│ Apple Core* frameworks                                     │
└────────────────────────────────────────────────────────────┘
```

## Requirements

- macOS 13.0+
- Xcode toolchain (Swift 5.9+); Command Line Tools alone is **not** enough — `xcrun --sdk macosx --show-sdk-version` must succeed
- Apple Silicon or Intel Mac

## Installation

```toml
[dependencies]
apple-cf = "0.1"
```

Or pick only the frameworks you need:

```toml
[dependencies]
apple-cf = { version = "0.1", default-features = false, features = ["cg", "iosurface"] }
```

## Quick examples

### Allocate an IOSurface and write a pixel

```rust,no_run
use apple_cf::iosurface::{IOSurface, IOSurfaceLockOptions};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let pixel_format = u32::from_be_bytes(*b"BGRA");
let surface = IOSurface::create(16, 16, pixel_format, 4)
    .ok_or("failed to allocate")?;

let mut guard = surface
    .lock(IOSurfaceLockOptions::NONE)
    .map_err(|c| format!("lock failed: {c}"))?;
if let Some(bytes) = guard.as_slice_mut() {
    bytes[0] = 0xFF;
}
# Ok(())
# }
```

### Create a custom dispatch queue

```rust
use apple_cf::dispatch_queue::{DispatchQoS, DispatchQueue};

let q = DispatchQueue::new("com.example.work", DispatchQoS::UserInitiated);
```

### Work with CGRect

```rust
use apple_cf::cg::CGRect;

let r = CGRect::new(10.0, 20.0, 800.0, 600.0);
assert_eq!(r.center().x, 410.0);
```

## Architecture

This crate uses the same Swift-bridge pattern as the rest of the doom-fish crates:

- `swift-bridge/Sources/<Framework>Bridge/` exposes a thin `@_cdecl` C surface
- `src/ffi/mod.rs` declares the matching `extern "C"` bindings
- `src/<framework>/` provides the safe Rust API on top

The Rust crate has **zero runtime dependencies**.

## Roadmap

1. **CoreMedia** — `CMSampleBuffer`, `CMTime`, `CMFormatDescription`, `CMBlockBuffer` (carve out from screencapturekit-rs, leaving `SCStreamFrameInfo` attachment readers in place upstream)
2. **CoreVideo** — `CVPixelBuffer`, `CVPixelBufferPool`
3. **Metal** — `MTLDevice`, `MTLTexture`, `MTLCommandQueue`
4. **CoreFoundation** — opt-in basic types if needed by downstream crates

Migrating screencapturekit-rs to consume `apple-cf` is the proof point that closes this story.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
