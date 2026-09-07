# COVERAGE

Wave-C audit for `apple-cf` `v0.10.0` against the active macOS SDK headers (`MacOSX26.2.sdk`).

## Summary

- `VERIFIED`: 2727 symbols
- `EXEMPT`: 138 symbols
- `GAPS`: 0 symbols
- `COVERAGE_PCT`: 95.18% of all audited public declarations
- `COVERABLE_COVERAGE_PCT`: 100.00% of non-exempt declarations

Legend:

- ✅ implemented — ergonomic safe Rust wrapper or exhaustive raw binding present
- ⏭️ exempt — deprecated / unavailable declaration, private inline helper, or Objective-C protocol marker without a standalone C ABI

## What v0.10.0 covers

| Surface | Headers | Status | Notes |
|---|---|---:|---|
| Ergonomic `cf` wrappers | `CFBase.h`, `CFString.h`, `CFNumber.h`, `CFData.h`, `CFDate.h`, `CFUUID.h`, `CFError.h`, `CFArray.h`, `CFDictionary.h`, `CFBag.h`, `CFSet.h`, `CFPropertyList.h`, `CFTree.h`, `CFAttributedString.h`, `CFURL.h`, `CFBundle.h`, `CFLocale.h`, `CFCalendar.h`, `CFTimeZone.h`, `CFCharacterSet.h`, `CFNumberFormatter.h`, `CFDateFormatter.h`, `CFPreferences.h`, `CFFileSecurity.h`, `CFXMLNode.h`, `CFXMLParser.h`, `CFNotificationCenter.h`, `CFRunLoop.h`, `CFMessagePort.h`, `CFStream.h`, `CFSocket.h`, `CFFileDescriptor.h` | ✅ | Existing safe wrappers remain the primary API, with examples and smoke tests. |
| Ergonomic `cm` wrappers | `CMSampleBuffer.h`, `CMBlockBuffer.h`, `CMFormatDescription.h`, `CMTime.h`, `CMTimeRange.h`, `CMSync.h`, `CMMetadata.h` | ✅ | Safe wrappers cover the common media stack (`CMSampleBuffer`, `CMBlockBuffer`, `CMFormatDescription`, `CMMetadataFormatDescription`, `CMTime`, `CMTimebase`). |
| Ergonomic `cv` / `iosurface` wrappers | `CVBuffer.h`, `CVImageBuffer.h`, `CVPixelBuffer.h`, `CVPixelBufferPool.h`, `CVMetalTextureCache.h`, `IOSurfaceRef.h` | ✅ | Safe object/lifetime wrappers remain available; zero-copy Rust references require explicit unsafe aliasing guarantees, and pool allocation/flush policy uses direct Core Video calls. |
| Ergonomic `dispatch_queue` wrappers | `dispatch/queue.h`, `dispatch/group.h`, `dispatch/semaphore.h`, `dispatch/source.h` | ✅ | Safe queue, async/apply, group, semaphore, and timer-source helpers remain available. |
| Exhaustive `raw` module | CoreFoundation / CoreMedia / CoreVideo / IOSurface / Dispatch umbrella headers | ✅ | New `apple_cf::raw` exposes the full audited low-level surface, including long-tail constants, inline helpers, CMTag/CMSync compatibility helpers, CVMetal, and dispatch main-queue access. |

## Remaining exempt families

| API family | Headers | Status | Reason |
|---|---|---:|---|
| Carbon resource-map bundle APIs (`CFBundleOpenBundleResourceMap`, etc.) | `CFBundle.h` | ⏭️ | Deprecated Carbon-era surface. |
| `CFURLAccess.h` helpers | `CFURLAccess.h` | ⏭️ | Deprecated file-URL access layer. |
| Legacy XML parser / document-node graph beyond entity helpers | `CFXMLNode.h`, `CFXMLParser.h` | ⏭️ | Deprecated XML parser/document graph. |
| `CVDisplayLink` family | `CVDisplayLink.h` | ⏭️ | Deprecated on macOS 15. |
| Objective-C `OS_dispatch_*` protocol markers | `dispatch/object.h` | ⏭️ | Header-only ObjC protocol declarations; Rust binds the usable C ABI via `dispatch_*_t` aliases in `apple_cf::raw`. |
| Private `CFSwap` helper union | `CFByteOrder.h` | ⏭️ | Inline implementation detail, not a standalone public API surface. |

See [`COVERAGE_AUDIT.md`](COVERAGE_AUDIT.md) for the full symbol-by-symbol audit.
