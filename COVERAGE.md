# COVERAGE

Coverage summary for `apple-cf` `v0.11.0`. The symbol counts come from the 0.10.0 audit against `MacOSX26.2.sdk` ([`COVERAGE_AUDIT.md`](COVERAGE_AUDIT.md)); they have not been regenerated against newer SDKs.

## What the numbers measure

- `VERIFIED`: 2727 symbols, `EXEMPT`: 138, `GAPS`: 0, i.e. 95.18% of all audited declarations and 100% of the non-exempt ones.
- `VERIFIED` means the declaration is reachable from Rust, not that it has a safe wrapper. 2371 of the 2727 verified symbols are only exposed as unsafe bindgen declarations in `apple_cf::raw`; about 356 are used by the safe wrappers described below.
- [`COVERAGE_AUDIT_V2.md`](COVERAGE_AUDIT_V2.md) is not an independent full audit. It samples the 300 most-wrapped CoreFoundation and CoreMedia symbols plus every CoreVideo, IOSurface and Dispatch symbol, so its 100% says nothing about the unsampled surface, and most of its `VERIFIED` rows are also `raw` declarations.

Legend:

- ✅ safe wrapper covering the common use of the header
- 🟡 partial safe wrapper; the rest of the header is only in `apple_cf::raw`
- 🔧 raw bindings only
- ⏭️ exempt: deprecated or unavailable declaration, private inline helper, or Objective-C protocol marker without a standalone C ABI

## Safe wrapper surface in v0.11.0

| Surface | Headers | Status | Notes |
|---|---|---:|---|
| CF values | `CFBase.h`, `CFString.h`, `CFNumber.h`, `CFData.h`, `CFDate.h`, `CFUUID.h`, `CFError.h` | ✅ | Type-erased `CFType`, UTF-8/UTF-16 strings, 64-bit and float numbers, data, dates, UUIDs, errors. |
| CF collections | `CFArray.h`, `CFDictionary.h`, `CFBag.h`, `CFSet.h`, `CFAttributedString.h`, `CFPropertyList.h` | 🟡 | Creation from values, lookup, iteration, mutable sets and property-list encode/decode. No mutable arrays or dictionaries, no attributed-string attributes. |
| `CFTree.h` | `CFTree.h` | 🔧 | `cf::CFTree` is a Swift-side tree of retained values, not a `CFTreeRef`; the `CFTree*` functions are only in `raw`. |
| CF resources, locale and formatting | `CFURL.h`, `CFBundle.h`, `CFLocale.h`, `CFCalendar.h`, `CFTimeZone.h`, `CFCharacterSet.h`, `CFNumberFormatter.h`, `CFDateFormatter.h`, `CFPreferences.h`, `CFFileSecurity.h`, `CFXMLParser.h` | 🟡 | A few entry points per header (URL creation and paths, bundle resources, identifiers, time-zone offsets, set membership, formatting, app preferences, file owner and mode, XML entity escaping). |
| CF runtime | `CFNotificationCenter.h`, `CFRunLoop.h`, `CFMessagePort.h`, `CFStream.h`, `CFSocket.h`, `CFFileDescriptor.h` | 🟡 | Posting and observing notifications, running the current run loop, no-op timers, an echo message port and remote requests, bound stream pairs, a UDP socket, owned or borrowed file descriptors. No run-loop sources or observers, no timers, sockets, descriptors or streams with user callbacks. |
| Core Media | `CMSampleBuffer.h`, `CMBlockBuffer.h`, `CMFormatDescription.h`, `CMTime.h`, `CMTimeRange.h`, `CMSync.h`, `CMMetadata.h` | 🟡 | Reading sample buffers (timing, data and image buffers, audio buffer list, sample attachments and sync state), block buffers, format descriptions (audio basics, video dimensions, H.264/HEVC parameter sets, metadata descriptions), `CMTime`/`CMTimeRange`, `CMClock` and `CMTimebase`. No sample-buffer creation, timing-info arrays or PCM copy helpers. |
| Core Video and IOSurface | `CVBuffer.h`, `CVImageBuffer.h`, `CVPixelBuffer.h`, `CVPixelBufferPool.h`, `CVMetalTextureCache.h`, `IOSurfaceRef.h` | 🟡 | Buffer and surface lifetime, geometry, attachments, pools and lock guards; zero-copy byte views are `unsafe`. No IOSurface lookup by ID, Mach port or XPC object, and no Metal texture creation from the cache. |
| Dispatch | `dispatch/queue.h`, `dispatch/group.h`, `dispatch/semaphore.h`, `dispatch/source.h`, `dispatch/time.h` | 🟡 | Serial, concurrent, main and global queues, `dispatch_async`, `dispatch_async_and_wait`, `dispatch_apply`, `dispatch_after`, groups, semaphores and a fire-counting timer source. No sources with user event handlers, work items or dispatch I/O. |
| Core Graphics | `CGGeometry.h`, `CGAffineTransform.h`, `CGColorSpace.h`, `CGImage.h`, `CGBitmapContext.h`, `CGContext.h` | 🟡 | Value types, device and named color spaces, image metadata and PNG export, 8-bit bitmap contexts with basic drawing. |
| `raw` | CoreFoundation, CoreMedia, CoreVideo, IOSurface and Dispatch umbrella headers | 🔧 | Unsafe bindgen declarations for the whole audited surface, plus hand-written inline helpers. |

## Remaining exempt families

| API family | Headers | Status | Reason |
|---|---|---:|---|
| Carbon resource-map bundle APIs (`CFBundleOpenBundleResourceMap`, etc.) | `CFBundle.h` | ⏭️ | Deprecated Carbon-era surface. |
| `CFURLAccess.h` helpers | `CFURLAccess.h` | ⏭️ | Deprecated file-URL access layer. |
| Legacy XML parser / document-node graph beyond entity helpers | `CFXMLNode.h`, `CFXMLParser.h` | ⏭️ | Deprecated XML parser/document graph. |
| `CVDisplayLink` family | `CVDisplayLink.h` | ⏭️ | Deprecated on macOS 15. |
| Objective-C `OS_dispatch_*` protocol markers | `dispatch/object.h` | ⏭️ | Header-only ObjC protocol declarations; Rust binds the usable C ABI via `dispatch_*_t` aliases in `apple_cf::raw`. |
| Private `CFSwap` helper union | `CFByteOrder.h` | ⏭️ | Inline implementation detail, not a standalone public API surface. |

See [`COVERAGE_AUDIT.md`](COVERAGE_AUDIT.md) for the symbol-by-symbol table; its "Wrapped by" column reflects 0.10.0.
