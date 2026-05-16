# COVERAGE

Wave-C audit for `apple-cf` `v0.6.1` against the active macOS SDK headers (`MacOSX26.2.sdk`).

Legend:

- ✅ implemented — ergonomic safe Rust wrapper present, with examples/tests
- 🟡 partial — intentionally narrow wrapper around a broader Apple surface
- ⏭️ skipped — deprecated / legacy / platform-specific API family not exposed by this crate

## CoreFoundation logical areas

| Area | Headers | Status | Notes |
|---|---|---:|---|
| `CFType` | `CFBase.h` | ✅ | Type-erased wrapper with retain/release/hash/equality/description. |
| `CFString` | `CFString.h` | ✅ | UTF-8 construction, length, Rust string conversion. |
| `CFNumber` | `CFNumber.h` | ✅ | `i64` / `u64` / `f64` creation and conversion helpers. |
| `CFData` | `CFData.h` | ✅ | Byte-copy construction and extraction. |
| `CFDate` | `CFDate.h` | ✅ | Absolute time + `SystemTime` conversion. |
| `CFUUID` | `CFUUID.h` | ✅ | Random generation, parse, string, bytes. |
| `CFError` | `CFError.h` | ✅ | Domain/code/description/failure-reason helpers. |
| `CFArray` | `CFArray.h` | ✅ | Typed construction, count, retained element access. |
| `CFDictionary` / `CFDict` | `CFDictionary.h` | ✅ | Pair construction, lookup, keys/values arrays. |
| `CFBag` | `CFBag.h` | ✅ | Construction, count, contains, multiplicity. |
| `CFSet` | `CFSet.h` | ✅ | Immutable + mutable set wrappers with membership, retained element access, and apply helpers. |
| `CFPropertyList` | `CFPropertyList.h` | ✅ | Parse, serialize, deep-copy, validate, and stream/write helpers. |
| `CFTree` | `CFTree.h` | ✅ | Tree nodes with payload + child append/access. |
| `CFAttributedString` | `CFAttributedString.h` | ✅ | Plain-string construction and string/length access. |
| `CFURL` | `CFURL.h` | ✅ | URL-string and file-path constructors, string/path accessors. |
| `CFBundle` | `CFBundle.h` | ✅ | Main/from-URL bundle wrappers, identifier/resource lookup. |
| `CFLocale` | `CFLocale.h` | ✅ | Current/custom locale + identifier access. |
| `CFCalendar` | `CFCalendar.h` | ✅ | Current/custom calendar + time-zone management. |
| `CFTimeZone` | `CFTimeZone.h` | ✅ | Current/custom time zone + GMT offset. |
| `CFCharacterSet` | `CFCharacterSet.h` | ✅ | Create from string, invert, membership tests. |
| `CFNumberFormatter` | `CFNumberFormatter.h` | ✅ | Format/parse numbers with locale/style. |
| `CFDateFormatter` | `CFDateFormatter.h` | ✅ | Format dates with locale/style. |
| `CFPreferences` | `CFPreferences.h` | ✅ | App-scoped get/set/synchronize helpers. |
| `CFFileSecurity` | `CFFileSecurity.h` | ✅ | Owner UUID + mode accessors. |
| `CFXML` | `CFXMLNode.h`, `CFXMLParser.h` | ✅ | Entity escape/unescape helpers retained as the still-useful non-parser surface. |
| `CFNotificationCenter` | `CFNotificationCenter.h` | ✅ | Local/distributed/Darwin centers + post helpers. |
| `CFRunLoop` | `CFRunLoop.h` | ✅ | Current/main loop access, default-mode run, stop/wake, add timer. |
| `CFTimer` | `CFRunLoop.h` | ✅ | No-op callback timer creation + validity/invalidate/fire. |
| `CFMessagePort` | `CFMessagePort.h` | ✅ | Echo-local port helper, remote connect, request/reply bytes. |
| `CFStream` | `CFStream.h` | ✅ | Bound read/write stream pair (`CFStreamPair`) with open/read/write/close. |
| `CFSocket` | `CFSocket.h` | ✅ | UDP/IPv4 socket creation, native descriptor, invalidate/validity. |
| `CFFileDescriptor` | `CFFileDescriptor.h` | ✅ | Native descriptor wrapper with invalidate. |

## CoreMedia / CoreVideo / IOSurface / Dispatch logical areas

| Area | Headers | Status | Notes |
|---|---|---:|---|
| `CMSampleBuffer` | `CMSampleBuffer.h` | ✅ | Existing safe wrapper + smoke example + coverage harness. |
| `CMBlockBuffer` | `CMBlockBuffer.h` | ✅ | Existing wrapper + coverage harness. |
| `CMFormatDescription` | `CMFormatDescription.h` | ✅ | Existing wrapper + coverage harness. |
| `CMMetadataFormatDescription` | `CMFormatDescription.h` | ✅ | Metadata-specific constructors, extension constants, identifier/key lookup, and merge/extend helpers. |
| `CMTime` | `CMTime.h` | ✅ | Existing value-type wrapper. |
| `CMTimeRange` | `CMTimeRange.h` | ✅ | Added range helpers (`end`, containment, intersection, union). |
| `CMTimebase` | `CMTimebase.h` | ✅ | Added timebase wrapper with master clock, time, rate. |
| `CVPixelBuffer` | `CVPixelBuffer.h` | ✅ | Existing safe wrapper + coverage harness. |
| `CVBuffer` | `CVBuffer.h` | ✅ | Added attachment helpers. |
| `CVImageBuffer` | `CVImageBuffer.h` | ✅ | Added encoded/display size + clean-rect helpers. |
| `CVMetalTextureCache` | `CVMetalTextureCache.h` | ✅ | Added system-default cache creation + flush wrapper. |
| `IOSurface` | `IOSurfaceRef.h` | ✅ | Existing wrapper + coverage harness. |
| `DispatchQueue` | `dispatch/queue.h` | ✅ | Existing queue wrapper. |
| `dispatch_async` / `dispatch_async_and_wait` / `dispatch_apply` | `dispatch/queue.h`, `dispatch/apply.h` | ✅ | Safe closure-based helpers bridged through Swift queue equivalents because the direct `_f` entry points are unavailable to Swift. |
| `DispatchGroup` | `dispatch/group.h` | ✅ | Added group creation/enter/leave/wait wrapper. |
| `DispatchSemaphore` | `dispatch/semaphore.h` | ✅ | Added semaphore create/signal/wait wrapper. |
| `DispatchSource` | `dispatch/source.h` | ✅ | Timer-backed source wrapper with resume/cancel/fire-count. |

## Skipped / deferred Apple API families

| API family | Headers | Status | Reason |
|---|---|---:|---|
| Carbon resource-map bundle APIs (`CFBundleOpenBundleResourceMap`, etc.) | `CFBundle.h` | ⏭️ | Deprecated Carbon-era surface; not useful for modern media crates. |
| `CFURLAccess.h` helpers | `CFURLAccess.h` | ⏭️ | Deprecated file-URL access layer. |
| Legacy XML parser / document-node graph beyond entity helpers | `CFXMLNode.h`, `CFXMLParser.h` | ⏭️ | Deprecated XML parser surface; only entity helpers retained. |
| IOSurface Mach-port / property-dictionary families | `IOSurfaceRef.h` | ⏭️ | Already intentionally omitted by the coverage harness as legacy / niche IPC surface. |
| `CMSampleBufferCreate*`, `CMBlockBufferAppend*`, `CMFormatDescriptionCreate*` heavy constructors | `CMSampleBuffer.h`, `CMBlockBuffer.h`, `CMFormatDescription.h` | ⏭️ | Existing harness intentionally omits these constructor-heavy / legacy callback entry points; safe read-side coverage remains 100% for the crate surface. |
| `CVMetalTextureCacheCreateTextureFromImage` | `CVMetalTextureCache.h` | ⏭️ | Requires public Metal texture wrapper surface that still lives in `apple-metal`; cache creation/flush covered here. |
| `CVDisplayLink` family | `CVDisplayLink.h` | ⏭️ | Deprecated on macOS 15; tracked as exempt in `COVERAGE_AUDIT.md` instead of adding new wrappers. |
| Non-timer dispatch sources | `dispatch/source.h` | ⏭️ | Timer source covered here; file, process, signal, mach, and vnode source flavors remain for a future dedicated surface. |
