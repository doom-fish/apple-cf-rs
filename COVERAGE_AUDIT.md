# apple-cf-rs coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 2865
VERIFIED: 356
GAPS: 2386
EXEMPT: 123
COVERAGE_PCT: 12.43%

## Notes

- Combined audit of CoreFoundation, CoreMedia, CoreVideo, IOSurface, and Dispatch.
- Public declarations were enumerated from MacOSX26.2.sdk via clang AST + header scans.
- Declarations unavailable on macOS were filtered out; deprecated declarations are retained as EXEMPT.
- VERIFIED means the declaration is directly referenced by apple-cf-rs wrapper implementations or represented by a public wrapper type.
- Rows whose header provenance resolved to `?` are tracked under the `Unscoped` bucket in the framework breakdown.
- v0.6.1 focuses the highest-value missing families (`CFSet`, `CFPropertyList`, `CMMetadataFormatDescription`, and dispatch async/apply); the remaining 2k+ long-tail gaps are documented as low priority for now.

## Framework breakdown

| Framework | Verified | Gaps | Exempt |
| --- | ---: | ---: | ---: |
| CoreFoundation | 205 | 1078 | 79 |
| CoreMedia | 69 | 707 | 8 |
| CoreVideo | 45 | 239 | 35 |
| IOSurface | 26 | 66 | 1 |
| Dispatch | 11 | 139 | 0 |
| Unscoped (`?`) | 0 | 157 | 0 |

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| CFArrayCreate | function | CoreFoundation/CFArray.h | cf::CFArray |
| CFArrayGetCount | function | CoreFoundation/CFArray.h | cf::CFArray |
| CFArrayGetTypeID | function | CoreFoundation/CFArray.h | cf::CFArray |
| CFArrayGetValueAtIndex | function | CoreFoundation/CFArray.h | cf::CFArray |
| CFArrayRef | typedef struct | CoreFoundation/CFArray.h | cf::CFArray |
| CFAttributedStringCreate | function | CoreFoundation/CFAttributedString.h | cf::CFAttributedString |
| CFAttributedStringGetLength | function | CoreFoundation/CFAttributedString.h | cf::CFAttributedString |
| CFAttributedStringGetString | function | CoreFoundation/CFAttributedString.h | cf::CFAttributedString |
| CFAttributedStringGetTypeID | function | CoreFoundation/CFAttributedString.h | cf::CFAttributedString |
| CFAttributedStringRef | typedef struct | CoreFoundation/CFAttributedString.h | cf::CFAttributedString |
| CFBagContainsValue | function | CoreFoundation/CFBag.h | cf::CFBag |
| CFBagCreate | function | CoreFoundation/CFBag.h | cf::CFBag |
| CFBagGetCount | function | CoreFoundation/CFBag.h | cf::CFBag |
| CFBagGetCountOfValue | function | CoreFoundation/CFBag.h | cf::CFBag |
| CFBagGetTypeID | function | CoreFoundation/CFBag.h | cf::CFBag |
| CFBagRef | typedef struct | CoreFoundation/CFBag.h | cf::CFBag |
| CFCopyDescription | function | CoreFoundation/CFBase.h | cf::CFType |
| CFEqual | function | CoreFoundation/CFBase.h | cf::CFType |
| CFGetTypeID | function | CoreFoundation/CFBase.h | cf::CFType |
| CFHash | function | CoreFoundation/CFBase.h | cf::CFType |
| CFRelease | function | CoreFoundation/CFBase.h | cf::CFType |
| CFRetain | function | CoreFoundation/CFBase.h | cf::CFType |
| CFStringRef | typedef struct | CoreFoundation/CFBase.h | cf::CFString |
| CFBundleCopyBundleURL | function | CoreFoundation/CFBundle.h | cf::CFBundle |
| CFBundleCopyResourceURL | function | CoreFoundation/CFBundle.h | cf::CFBundle |
| CFBundleCreate | function | CoreFoundation/CFBundle.h | cf::CFBundle |
| CFBundleGetIdentifier | function | CoreFoundation/CFBundle.h | cf::CFBundle |
| CFBundleGetMainBundle | function | CoreFoundation/CFBundle.h | cf::CFBundle |
| CFBundleGetTypeID | function | CoreFoundation/CFBundle.h | cf::CFBundle |
| CFBundleRef | typedef struct | CoreFoundation/CFBundle.h | cf::CFBundle |
| CFCalendarCopyCurrent | function | CoreFoundation/CFCalendar.h | cf::CFCalendar |
| CFCalendarCopyTimeZone | function | CoreFoundation/CFCalendar.h | cf::CFCalendar |
| CFCalendarCreateWithIdentifier | function | CoreFoundation/CFCalendar.h | cf::CFCalendar |
| CFCalendarGetIdentifier | function | CoreFoundation/CFCalendar.h | cf::CFCalendar |
| CFCalendarGetTypeID | function | CoreFoundation/CFCalendar.h | cf::CFCalendar |
| CFCalendarRef | typedef struct | CoreFoundation/CFCalendar.h | cf::CFCalendar |
| CFCalendarSetTimeZone | function | CoreFoundation/CFCalendar.h | cf::CFCalendar |
| CFCharacterSetCreateInvertedSet | function | CoreFoundation/CFCharacterSet.h | cf::CFCharacterSet |
| CFCharacterSetCreateWithCharactersInString | function | CoreFoundation/CFCharacterSet.h | cf::CFCharacterSet |
| CFCharacterSetGetTypeID | function | CoreFoundation/CFCharacterSet.h | cf::CFCharacterSet |
| CFCharacterSetIsCharacterMember | function | CoreFoundation/CFCharacterSet.h | cf::CFCharacterSet |
| CFCharacterSetRef | typedef struct | CoreFoundation/CFCharacterSet.h | cf::CFCharacterSet |
| CFDataCreate | function | CoreFoundation/CFData.h | cf::CFData |
| CFDataGetBytePtr | function | CoreFoundation/CFData.h | cf::CFData |
| CFDataGetLength | function | CoreFoundation/CFData.h | cf::CFData |
| CFDataGetTypeID | function | CoreFoundation/CFData.h | cf::CFData |
| CFDataRef | typedef struct | CoreFoundation/CFData.h | cf::CFData |
| CFAbsoluteTimeGetCurrent | function | CoreFoundation/CFDate.h | cf::CFDate |
| CFDateCreate | function | CoreFoundation/CFDate.h | cf::CFDate |
| CFDateGetAbsoluteTime | function | CoreFoundation/CFDate.h | cf::CFDate |
| CFDateGetTypeID | function | CoreFoundation/CFDate.h | cf::CFDate |
| CFDateRef | typedef struct | CoreFoundation/CFDate.h | cf::CFDate |
| CFTimeZoneRef | typedef struct | CoreFoundation/CFDate.h | cf::CFTimeZone |
| CFDateFormatterCreate | function | CoreFoundation/CFDateFormatter.h | cf::CFDateFormatter |
| CFDateFormatterCreateStringWithDate | function | CoreFoundation/CFDateFormatter.h | cf::CFDateFormatter |
| CFDateFormatterGetTypeID | function | CoreFoundation/CFDateFormatter.h | cf::CFDateFormatter |
| CFDateFormatterRef | typedef struct | CoreFoundation/CFDateFormatter.h | cf::CFDateFormatter |
| CFDateFormatterStyle | typedef enum | CoreFoundation/CFDateFormatter.h | cf::CFDateFormatter |
| CFDictionaryContainsKey | function | CoreFoundation/CFDictionary.h | cf::CFDictionary |
| CFDictionaryCreate | function | CoreFoundation/CFDictionary.h | cf::CFDictionary |
| CFDictionaryGetCount | function | CoreFoundation/CFDictionary.h | cf::CFDictionary |
| CFDictionaryGetKeysAndValues | function | CoreFoundation/CFDictionary.h | cf::CFDictionary |
| CFDictionaryGetTypeID | function | CoreFoundation/CFDictionary.h | cf::CFDictionary |
| CFDictionaryGetValue | function | CoreFoundation/CFDictionary.h | cf::CFDictionary |
| CFDictionaryRef | typedef struct | CoreFoundation/CFDictionary.h | cf::CFDictionary |
| CFErrorCopyDescription | function | CoreFoundation/CFError.h | cf::CFError |
| CFErrorCopyFailureReason | function | CoreFoundation/CFError.h | cf::CFError |
| CFErrorCreate | function | CoreFoundation/CFError.h | cf::CFError |
| CFErrorGetCode | function | CoreFoundation/CFError.h | cf::CFError |
| CFErrorGetDomain | function | CoreFoundation/CFError.h | cf::CFError |
| CFErrorGetTypeID | function | CoreFoundation/CFError.h | cf::CFError |
| CFErrorRef | typedef struct | CoreFoundation/CFError.h | cf::CFError |
| CFFileDescriptorCreate | function | CoreFoundation/CFFileDescriptor.h | cf::CFFileDescriptor |
| CFFileDescriptorGetNativeDescriptor | function | CoreFoundation/CFFileDescriptor.h | cf::CFFileDescriptor |
| CFFileDescriptorGetTypeID | function | CoreFoundation/CFFileDescriptor.h | cf::CFFileDescriptor |
| CFFileDescriptorInvalidate | function | CoreFoundation/CFFileDescriptor.h | cf::CFFileDescriptor |
| CFFileDescriptorRef | typedef struct | CoreFoundation/CFFileDescriptor.h | cf::CFFileDescriptor |
| CFFileSecurityCopyOwnerUUID | function | CoreFoundation/CFFileSecurity.h | cf::CFFileSecurity |
| CFFileSecurityCreate | function | CoreFoundation/CFFileSecurity.h | cf::CFFileSecurity |
| CFFileSecurityGetMode | function | CoreFoundation/CFFileSecurity.h | cf::CFFileSecurity |
| CFFileSecurityGetTypeID | function | CoreFoundation/CFFileSecurity.h | cf::CFFileSecurity |
| CFFileSecurityRef | typedef struct | CoreFoundation/CFFileSecurity.h | cf::CFFileSecurity |
| CFFileSecuritySetMode | function | CoreFoundation/CFFileSecurity.h | cf::CFFileSecurity |
| CFFileSecuritySetOwnerUUID | function | CoreFoundation/CFFileSecurity.h | cf::CFFileSecurity |
| CFLocaleCopyCurrent | function | CoreFoundation/CFLocale.h | cf::CFLocale |
| CFLocaleCreate | function | CoreFoundation/CFLocale.h | cf::CFLocale |
| CFLocaleGetIdentifier | function | CoreFoundation/CFLocale.h | cf::CFLocale |
| CFLocaleGetTypeID | function | CoreFoundation/CFLocale.h | cf::CFLocale |
| CFLocaleRef | typedef struct | CoreFoundation/CFLocale.h | cf::CFLocale |
| CFMessagePortCreateLocal | function | CoreFoundation/CFMessagePort.h | cf::CFMessagePort |
| CFMessagePortCreateRemote | function | CoreFoundation/CFMessagePort.h | cf::CFMessagePort |
| CFMessagePortCreateRunLoopSource | function | CoreFoundation/CFMessagePort.h | cf::CFMessagePort |
| CFMessagePortGetTypeID | function | CoreFoundation/CFMessagePort.h | cf::CFMessagePort |
| CFMessagePortInvalidate | function | CoreFoundation/CFMessagePort.h | cf::CFMessagePort |
| CFMessagePortRef | typedef struct | CoreFoundation/CFMessagePort.h | cf::CFMessagePort |
| CFMessagePortSendRequest | function | CoreFoundation/CFMessagePort.h | cf::CFMessagePort |
| CFNotificationCenterGetDarwinNotifyCenter | function | CoreFoundation/CFNotificationCenter.h | cf::CFNotificationCenter |
| CFNotificationCenterGetDistributedCenter | function | CoreFoundation/CFNotificationCenter.h | cf::CFNotificationCenter |
| CFNotificationCenterGetLocalCenter | function | CoreFoundation/CFNotificationCenter.h | cf::CFNotificationCenter |
| CFNotificationCenterGetTypeID | function | CoreFoundation/CFNotificationCenter.h | cf::CFNotificationCenter |
| CFNotificationCenterPostNotification | function | CoreFoundation/CFNotificationCenter.h | cf::CFNotificationCenter |
| CFNotificationCenterRef | typedef struct | CoreFoundation/CFNotificationCenter.h | cf::CFNotificationCenter |
| CFNumberCreate | function | CoreFoundation/CFNumber.h | cf::CFNumber |
| CFNumberGetTypeID | function | CoreFoundation/CFNumber.h | cf::CFNumber |
| CFNumberGetValue | function | CoreFoundation/CFNumber.h | cf::CFNumber |
| CFNumberIsFloatType | function | CoreFoundation/CFNumber.h | cf::CFNumber |
| CFNumberRef | typedef struct | CoreFoundation/CFNumber.h | cf::CFNumber |
| CFNumberFormatterCreate | function | CoreFoundation/CFNumberFormatter.h | cf::CFNumberFormatter |
| CFNumberFormatterCreateNumberFromString | function | CoreFoundation/CFNumberFormatter.h | cf::CFNumberFormatter |
| CFNumberFormatterCreateStringWithNumber | function | CoreFoundation/CFNumberFormatter.h | cf::CFNumberFormatter |
| CFNumberFormatterGetTypeID | function | CoreFoundation/CFNumberFormatter.h | cf::CFNumberFormatter |
| CFNumberFormatterRef | typedef struct | CoreFoundation/CFNumberFormatter.h | cf::CFNumberFormatter |
| CFNumberFormatterStyle | typedef enum | CoreFoundation/CFNumberFormatter.h | cf::CFNumberFormatter |
| CFPreferencesAppSynchronize | function | CoreFoundation/CFPreferences.h | cf::CFPreferences |
| CFPreferencesCopyAppValue | function | CoreFoundation/CFPreferences.h | cf::CFPreferences |
| CFPreferencesSetAppValue | function | CoreFoundation/CFPreferences.h | cf::CFPreferences |
| CFPropertyListCreateData | function | CoreFoundation/CFPropertyList.h | cf::CFPropertyList |
| CFPropertyListCreateDeepCopy | function | CoreFoundation/CFPropertyList.h | cf::CFPropertyList |
| CFPropertyListCreateWithData | function | CoreFoundation/CFPropertyList.h | cf::CFPropertyList |
| CFPropertyListCreateWithStream | function | CoreFoundation/CFPropertyList.h | cf::CFPropertyList |
| CFPropertyListFormat | typedef enum | CoreFoundation/CFPropertyList.h | cf::CFPropertyListFormat |
| CFPropertyListIsValid | function | CoreFoundation/CFPropertyList.h | cf::CFPropertyList |
| CFPropertyListMutabilityOptions | typedef enum | CoreFoundation/CFPropertyList.h | cf::CFPropertyListMutabilityOptions |
| CFPropertyListWrite | function | CoreFoundation/CFPropertyList.h | cf::CFPropertyList |
| CFRunLoopAddSource | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop |
| CFRunLoopAddTimer | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop |
| CFRunLoopGetCurrent | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop |
| CFRunLoopGetMain | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop |
| CFRunLoopGetTypeID | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop |
| CFRunLoopRef | typedef struct | CoreFoundation/CFRunLoop.h | cf::CFRunLoop |
| CFRunLoopRunInMode | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop |
| CFRunLoopRunResult | typedef enum | CoreFoundation/CFRunLoop.h | cf::CFRunLoop |
| CFRunLoopStop | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop |
| CFRunLoopTimerCreate | function | CoreFoundation/CFRunLoop.h | cf::CFTimer |
| CFRunLoopTimerCreateWithHandler | function | CoreFoundation/CFRunLoop.h | cf::CFTimer |
| CFRunLoopTimerGetTypeID | function | CoreFoundation/CFRunLoop.h | cf::CFTimer |
| CFRunLoopTimerInvalidate | function | CoreFoundation/CFRunLoop.h | cf::CFTimer |
| CFRunLoopTimerIsValid | function | CoreFoundation/CFRunLoop.h | cf::CFTimer |
| CFRunLoopTimerRef | typedef struct | CoreFoundation/CFRunLoop.h | cf::CFTimer |
| CFRunLoopTimerSetNextFireDate | function | CoreFoundation/CFRunLoop.h | cf::CFTimer |
| CFRunLoopWakeUp | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop |
| CFMutableSetRef | typedef struct | CoreFoundation/CFSet.h | cf::CFMutableSet |
| CFSetAddValue | function | CoreFoundation/CFSet.h | cf::CFMutableSet |
| CFSetApplyFunction | function | CoreFoundation/CFSet.h | cf::CFSet / cf::CFMutableSet |
| CFSetCallBacks | typedef struct | CoreFoundation/CFSet.h | cf::CFSetCallbacks |
| CFSetContainsValue | function | CoreFoundation/CFSet.h | cf::CFSet / cf::CFMutableSet |
| CFSetCreate | function | CoreFoundation/CFSet.h | cf::CFSet |
| CFSetCreateCopy | function | CoreFoundation/CFSet.h | cf::CFSet / cf::CFMutableSet |
| CFSetCreateMutable | function | CoreFoundation/CFSet.h | cf::CFMutableSet |
| CFSetCreateMutableCopy | function | CoreFoundation/CFSet.h | cf::CFSet / cf::CFMutableSet |
| CFSetGetCount | function | CoreFoundation/CFSet.h | cf::CFSet / cf::CFMutableSet |
| CFSetGetCountOfValue | function | CoreFoundation/CFSet.h | cf::CFSet / cf::CFMutableSet |
| CFSetGetTypeID | function | CoreFoundation/CFSet.h | cf::CFSet / cf::CFMutableSet |
| CFSetGetValue | function | CoreFoundation/CFSet.h | cf::CFSet |
| CFSetGetValueIfPresent | function | CoreFoundation/CFSet.h | cf::CFSet |
| CFSetGetValues | function | CoreFoundation/CFSet.h | cf::CFSet / cf::CFMutableSet |
| CFSetRef | typedef struct | CoreFoundation/CFSet.h | cf::CFSet |
| CFSetRemoveAllValues | function | CoreFoundation/CFSet.h | cf::CFMutableSet |
| CFSetRemoveValue | function | CoreFoundation/CFSet.h | cf::CFMutableSet |
| CFSetReplaceValue | function | CoreFoundation/CFSet.h | cf::CFMutableSet |
| CFSetSetValue | function | CoreFoundation/CFSet.h | cf::CFMutableSet |
| kCFCopyStringSetCallBacks | constant | CoreFoundation/CFSet.h | cf::CFSetCallbacks::CopyString |
| kCFTypeSetCallBacks | constant | CoreFoundation/CFSet.h | cf::CFSetCallbacks::Type |
| CFSocketCreate | function | CoreFoundation/CFSocket.h | cf::CFSocket |
| CFSocketGetNative | function | CoreFoundation/CFSocket.h | cf::CFSocket |
| CFSocketGetTypeID | function | CoreFoundation/CFSocket.h | cf::CFSocket |
| CFSocketInvalidate | function | CoreFoundation/CFSocket.h | cf::CFSocket |
| CFSocketIsValid | function | CoreFoundation/CFSocket.h | cf::CFSocket |
| CFSocketRef | typedef struct | CoreFoundation/CFSocket.h | cf::CFSocket |
| CFReadStreamClose | function | CoreFoundation/CFStream.h | cf::CFStreamPair |
| CFReadStreamGetTypeID | function | CoreFoundation/CFStream.h | cf::CFStreamPair |
| CFReadStreamOpen | function | CoreFoundation/CFStream.h | cf::CFStreamPair |
| CFReadStreamRead | function | CoreFoundation/CFStream.h | cf::CFStreamPair |
| CFReadStreamRef | typedef struct | CoreFoundation/CFStream.h | cf::CFStreamPair |
| CFStreamCreateBoundPair | function | CoreFoundation/CFStream.h | cf::CFStreamPair |
| CFWriteStreamClose | function | CoreFoundation/CFStream.h | cf::CFStreamPair |
| CFWriteStreamGetTypeID | function | CoreFoundation/CFStream.h | cf::CFStreamPair |
| CFWriteStreamOpen | function | CoreFoundation/CFStream.h | cf::CFStreamPair |
| CFWriteStreamRef | typedef struct | CoreFoundation/CFStream.h | cf::CFStreamPair |
| CFWriteStreamWrite | function | CoreFoundation/CFStream.h | cf::CFStreamPair |
| CFStringCreateWithCString | function | CoreFoundation/CFString.h | cf::CFString |
| CFStringGetCString | function | CoreFoundation/CFString.h | cf::CFString |
| CFStringGetLength | function | CoreFoundation/CFString.h | cf::CFString |
| CFStringGetMaximumSizeForEncoding | function | CoreFoundation/CFString.h | cf::CFString |
| CFStringGetTypeID | function | CoreFoundation/CFString.h | cf::CFString |
| CFTimeZoneCopySystem | function | CoreFoundation/CFTimeZone.h | cf::CFTimeZone |
| CFTimeZoneCreateWithName | function | CoreFoundation/CFTimeZone.h | cf::CFTimeZone |
| CFTimeZoneGetName | function | CoreFoundation/CFTimeZone.h | cf::CFTimeZone |
| CFTimeZoneGetSecondsFromGMT | function | CoreFoundation/CFTimeZone.h | cf::CFTimeZone |
| CFTimeZoneGetTypeID | function | CoreFoundation/CFTimeZone.h | cf::CFTimeZone |
| CFURLCopyFileSystemPath | function | CoreFoundation/CFURL.h | cf::CFURL |
| CFURLCreateWithFileSystemPath | function | CoreFoundation/CFURL.h | cf::CFURL |
| CFURLCreateWithString | function | CoreFoundation/CFURL.h | cf::CFURL |
| CFURLGetString | function | CoreFoundation/CFURL.h | cf::CFURL |
| CFURLGetTypeID | function | CoreFoundation/CFURL.h | cf::CFURL |
| CFURLHasDirectoryPath | function | CoreFoundation/CFURL.h | cf::CFURL |
| CFURLRef | typedef struct | CoreFoundation/CFURL.h | cf::CFURL |
| CFUUIDCreate | function | CoreFoundation/CFUUID.h | cf::CFUUID |
| CFUUIDCreateFromString | function | CoreFoundation/CFUUID.h | cf::CFUUID |
| CFUUIDCreateString | function | CoreFoundation/CFUUID.h | cf::CFUUID |
| CFUUIDGetTypeID | function | CoreFoundation/CFUUID.h | cf::CFUUID |
| CFUUIDGetUUIDBytes | function | CoreFoundation/CFUUID.h | cf::CFUUID |
| CFUUIDRef | typedef struct | CoreFoundation/CFUUID.h | cf::CFUUID |
| CFXMLCreateStringByEscapingEntities | function | CoreFoundation/CFXMLParser.h | cf::CFXML |
| CFXMLCreateStringByUnescapingEntities | function | CoreFoundation/CFXMLParser.h | cf::CFXML |
| CMBlockBufferCopyDataBytes | function | CoreMedia/CMBlockBuffer.h | cm::CMBlockBuffer |
| CMBlockBufferCreateEmpty | function | CoreMedia/CMBlockBuffer.h | cm::CMBlockBuffer |
| CMBlockBufferCreateWithMemoryBlock | function | CoreMedia/CMBlockBuffer.h | cm::CMBlockBuffer |
| CMBlockBufferGetDataLength | function | CoreMedia/CMBlockBuffer.h | cm::CMBlockBuffer |
| CMBlockBufferGetDataPointer | function | CoreMedia/CMBlockBuffer.h | cm::CMBlockBuffer |
| CMBlockBufferIsEmpty | function | CoreMedia/CMBlockBuffer.h | cm::CMBlockBuffer |
| CMBlockBufferIsRangeContiguous | function | CoreMedia/CMBlockBuffer.h | cm::CMBlockBuffer |
| CMBlockBufferRef | typedef struct | CoreMedia/CMBlockBuffer.h | cm::CMBlockBuffer |
| CMBlockBufferReplaceDataBytes | function | CoreMedia/CMBlockBuffer.h | cm::CMBlockBuffer |
| CMAudioFormatDescriptionGetStreamBasicDescription | function | CoreMedia/CMFormatDescription.h | cm::CMFormatDescription |
| CMFormatDescriptionGetExtensions | function | CoreMedia/CMFormatDescription.h | cm::CMFormatDescription |
| CMFormatDescriptionGetMediaSubType | function | CoreMedia/CMFormatDescription.h | cm::CMFormatDescription |
| CMFormatDescriptionGetMediaType | function | CoreMedia/CMFormatDescription.h | cm::CMFormatDescription |
| CMFormatDescriptionRef | typedef struct | CoreMedia/CMFormatDescription.h | cm::CMFormatDescription |
| CMMetadataFormatDescriptionCreateByMergingMetadataFormatDescriptions | function | CoreMedia/CMFormatDescription.h | cm::CMMetadataFormatDescription |
| CMMetadataFormatDescriptionCreateWithKeys | function | CoreMedia/CMFormatDescription.h | cm::CMMetadataFormatDescription |
| CMMetadataFormatDescriptionCreateWithMetadataFormatDescriptionAndMetadataSpecifications | function | CoreMedia/CMFormatDescription.h | cm::CMMetadataFormatDescription |
| CMMetadataFormatDescriptionCreateWithMetadataSpecifications | function | CoreMedia/CMFormatDescription.h | cm::CMMetadataFormatDescription |
| CMMetadataFormatDescriptionGetIdentifiers | function | CoreMedia/CMFormatDescription.h | cm::CMMetadataFormatDescription |
| CMMetadataFormatDescriptionGetKeyWithLocalID | function | CoreMedia/CMFormatDescription.h | cm::CMMetadataFormatDescription |
| CMSampleBufferDataIsReady | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer |
| CMSampleBufferGetDataBuffer | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer |
| CMSampleBufferGetDecodeTimeStamp | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer |
| CMSampleBufferGetDuration | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer |
| CMSampleBufferGetFormatDescription | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer |
| CMSampleBufferGetImageBuffer | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer |
| CMSampleBufferGetNumSamples | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer |
| CMSampleBufferGetPresentationTimeStamp | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer |
| CMSampleBufferIsValid | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer |
| CMSampleBufferRef | typedef struct | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer |
| CMSampleTimingInfo | typedef struct | CoreMedia/CMSampleBuffer.h | cm::CMTime |
| CMTime | typedef struct | CoreMedia/CMSampleBuffer.h | cm::CMTime |
| CMClockGetHostTimeClock | function | CoreMedia/CMSync.h | cm::CMClock |
| CMClockRef | typedef struct | CoreMedia/CMSync.h | cm::CMClock |
| CMTimebaseCreateWithMasterClock | function | CoreMedia/CMSync.h | cm::CMTimebase |
| CMTimebaseGetRate | function | CoreMedia/CMSync.h | cm::CMTimebase |
| CMTimebaseGetTime | function | CoreMedia/CMSync.h | cm::CMTimebase |
| CMTimebaseRef | typedef struct | CoreMedia/CMSync.h | cm::CMTimebase |
| CMTimebaseSetRate | function | CoreMedia/CMSync.h | cm::CMTimebase |
| CMTimebaseSetTime | function | CoreMedia/CMSync.h | cm::CMTimebase |
| CMTimeAdd | function | CoreMedia/CMTime.h | cm::CMTime |
| CMTimeCompare | function | CoreMedia/CMTime.h | cm::CMTime |
| CMTimeConvertScale | function | CoreMedia/CMTime.h | cm::CMTime |
| CMTimeMakeWithSeconds | function | CoreMedia/CMTime.h | cm::CMTime |
| CMTimeMultiply | function | CoreMedia/CMTime.h | cm::CMTime |
| CMTimeMultiplyByFloat64 | function | CoreMedia/CMTime.h | cm::CMTime |
| CMTimeSubtract | function | CoreMedia/CMTime.h | cm::CMTime |
| CMTimeRange | typedef struct | CoreMedia/CMTimeRange.h | cm::CMTime |
| CMTimeRangeContainsTime | function | CoreMedia/CMTimeRange.h | cm::CMTime |
| CMTimeRangeContainsTimeRange | function | CoreMedia/CMTimeRange.h | cm::CMTime |
| CMTimeRangeGetEnd | function | CoreMedia/CMTimeRange.h | cm::CMTime |
| CMTimeRangeGetIntersection | function | CoreMedia/CMTimeRange.h | cm::CMTime |
| CMTimeRangeGetUnion | function | CoreMedia/CMTimeRange.h | cm::CMTime |
| CVAttachmentMode | typedef enum | CoreVideo/CVBuffer.h | cv::CVAttachmentMode |
| CVBufferCopyAttachment | function | CoreVideo/CVBuffer.h | cv::CVBuffer |
| CVBufferCopyAttachments | function | CoreVideo/CVBuffer.h | cv::CVBuffer |
| CVBufferRef | typedef struct | CoreVideo/CVBuffer.h | cv::CVBuffer |
| CVBufferRelease | function | CoreVideo/CVBuffer.h | cv::CVBuffer |
| CVBufferRemoveAllAttachments | function | CoreVideo/CVBuffer.h | cv::CVBuffer |
| CVBufferRetain | function | CoreVideo/CVBuffer.h | cv::CVBuffer |
| CVBufferSetAttachment | function | CoreVideo/CVBuffer.h | cv::CVBuffer |
| CVImageBufferGetCleanRect | function | CoreVideo/CVImageBuffer.h | cv::CVImageBuffer |
| CVImageBufferGetDisplaySize | function | CoreVideo/CVImageBuffer.h | cv::CVImageBuffer |
| CVImageBufferGetEncodedSize | function | CoreVideo/CVImageBuffer.h | cv::CVImageBuffer |
| CVMetalTextureCacheCreate | function | CoreVideo/CVMetalTextureCache.h | cv::CVMetalTextureCache |
| CVMetalTextureCacheFlush | function | CoreVideo/CVMetalTextureCache.h | cv::CVMetalTextureCache |
| CVMetalTextureCacheGetTypeID | function | CoreVideo/CVMetalTextureCache.h | cv::CVMetalTextureCache |
| CVMetalTextureCacheRef | typedef struct | CoreVideo/CVMetalTextureCache.h | cv::CVMetalTextureCache |
| CVPixelBufferCreate | function | CoreVideo/CVPixelBuffer.h | cv::CVPixelBuffer |
| CVPixelBufferCreateWithBytes | function | CoreVideo/CVPixelBuffer.h | cv::CVPixelBuffer |
| CVPixelBufferCreateWithPlanarBytes | function | CoreVideo/CVPixelBuffer.h | cv::CVPixelBuffer |
| CVPixelBufferFillExtendedPixels | function | CoreVideo/CVPixelBuffer.h | cv::CVPixelBuffer |
| CVPixelBufferGetBaseAddress | function | CoreVideo/CVPixelBuffer.h | cv::CVPixelBuffer |
| CVPixelBufferGetBaseAddressOfPlane | function | CoreVideo/CVPixelBuffer.h | cv::CVPixelBuffer |
| CVPixelBufferGetBytesPerRow | function | CoreVideo/CVPixelBuffer.h | cv::CVPixelBuffer |
| CVPixelBufferGetBytesPerRowOfPlane | function | CoreVideo/CVPixelBuffer.h | cv::CVPixelBuffer |
| CVPixelBufferGetDataSize | function | CoreVideo/CVPixelBuffer.h | cv::CVPixelBuffer |
| CVPixelBufferGetExtendedPixels | function | CoreVideo/CVPixelBuffer.h | cv::CVPixelBuffer |
| CVPixelBufferGetHeight | function | CoreVideo/CVPixelBuffer.h | cv::CVPixelBuffer |
| CVPixelBufferGetHeightOfPlane | function | CoreVideo/CVPixelBuffer.h | cv::CVPixelBuffer |
| CVPixelBufferGetPixelFormatType | function | CoreVideo/CVPixelBuffer.h | cv::CVPixelBuffer |
| CVPixelBufferGetPlaneCount | function | CoreVideo/CVPixelBuffer.h | cv::CVPixelBuffer |
| CVPixelBufferGetTypeID | function | CoreVideo/CVPixelBuffer.h | cv::CVPixelBuffer |
| CVPixelBufferGetWidth | function | CoreVideo/CVPixelBuffer.h | cv::CVPixelBuffer |
| CVPixelBufferGetWidthOfPlane | function | CoreVideo/CVPixelBuffer.h | cv::CVPixelBuffer |
| CVPixelBufferIsPlanar | function | CoreVideo/CVPixelBuffer.h | cv::CVPixelBuffer |
| CVPixelBufferLockBaseAddress | function | CoreVideo/CVPixelBuffer.h | cv::CVPixelBuffer |
| CVPixelBufferLockFlags | typedef enum | CoreVideo/CVPixelBuffer.h | cv::CVPixelBuffer |
| CVPixelBufferUnlockBaseAddress | function | CoreVideo/CVPixelBuffer.h | cv::CVPixelBuffer |
| CVPixelBufferCreateWithIOSurface | function | CoreVideo/CVPixelBufferIOSurface.h | cv::CVPixelBuffer |
| CVPixelBufferGetIOSurface | function | CoreVideo/CVPixelBufferIOSurface.h | cv::CVPixelBuffer |
| CVPixelBufferPoolCreate | function | CoreVideo/CVPixelBufferPool.h | cv::CVPixelBufferPool |
| CVPixelBufferPoolCreatePixelBuffer | function | CoreVideo/CVPixelBufferPool.h | cv::CVPixelBufferPool |
| CVPixelBufferPoolFlush | function | CoreVideo/CVPixelBufferPool.h | cv::CVPixelBufferPool |
| CVPixelBufferPoolGetAttributes | function | CoreVideo/CVPixelBufferPool.h | cv::CVPixelBufferPool |
| CVPixelBufferPoolGetPixelBufferAttributes | function | CoreVideo/CVPixelBufferPool.h | cv::CVPixelBufferPool |
| CVPixelBufferPoolGetTypeID | function | CoreVideo/CVPixelBufferPool.h | cv::CVPixelBufferPool |
| CVPixelBufferPoolRef | typedef struct | CoreVideo/CVPixelBufferPool.h | cv::CVPixelBufferPool |
| dispatch_group_t | typedef struct | Dispatch/group.h | DispatchGroup |
| dispatch_apply | function | Dispatch/queue.h | dispatch_queue::dispatch_apply |
| dispatch_apply_f | function | Dispatch/queue.h | dispatch_queue::dispatch_apply (Swift bridge uses `_f` callback form internally) |
| dispatch_async | function | Dispatch/queue.h | dispatch_queue::dispatch_async |
| dispatch_async_and_wait | function | Dispatch/queue.h | dispatch_queue::dispatch_async_and_wait |
| dispatch_async_and_wait_f | function | Dispatch/queue.h | dispatch_queue::dispatch_async_and_wait (Swift bridge uses `_f` callback form internally) |
| dispatch_async_f | function | Dispatch/queue.h | dispatch_queue::dispatch_async (Swift bridge uses `_f` callback form internally) |
| dispatch_queue_create | function | Dispatch/queue.h | DispatchQueue |
| dispatch_queue_t | typedef struct | Dispatch/queue.h | DispatchQueue |
| dispatch_semaphore_t | typedef struct | Dispatch/semaphore.h | DispatchSemaphore |
| dispatch_source_t | typedef struct | Dispatch/source.h | DispatchSource |
| IOSurfaceDecrementUseCount | function | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceGetAllocSize | function | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceGetBaseAddress | function | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceGetBaseAddressOfPlane | function | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceGetBytesPerElement | function | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceGetBytesPerElementOfPlane | function | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceGetBytesPerRow | function | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceGetBytesPerRowOfPlane | function | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceGetElementHeight | function | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceGetElementHeightOfPlane | function | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceGetElementWidth | function | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceGetElementWidthOfPlane | function | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceGetHeight | function | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceGetHeightOfPlane | function | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceGetID | function | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceGetPixelFormat | function | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceGetPlaneCount | function | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceGetSeed | function | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceGetWidth | function | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceGetWidthOfPlane | function | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceIncrementUseCount | function | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceIsInUse | function | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceLock | function | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceRef | typedef struct | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceUnlock | function | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceLockOptions | typedef enum | IOSurface/IOSurfaceTypes.h | iosurface::IOSurface |
| kCMFormatDescriptionExtensionKey_MetadataKeyTable | constant | CoreMedia/CMFormatDescription.h | cm::format_description::format_description_extension_keys::metadata_key_table |
| kCMMetadataFormatDescriptionKey_ConformingDataTypes | constant | CoreMedia/CMFormatDescription.h | cm::format_description::metadata_description_keys::conforming_data_types |
| kCMMetadataFormatDescriptionKey_DataType | constant | CoreMedia/CMFormatDescription.h | cm::format_description::metadata_description_keys::data_type |
| kCMMetadataFormatDescriptionKey_DataTypeNamespace | constant | CoreMedia/CMFormatDescription.h | cm::format_description::metadata_description_keys::data_type_namespace |
| kCMMetadataFormatDescriptionKey_LanguageTag | constant | CoreMedia/CMFormatDescription.h | cm::format_description::metadata_description_keys::language_tag |
| kCMMetadataFormatDescriptionKey_LocalID | constant | CoreMedia/CMFormatDescription.h | cm::format_description::metadata_description_keys::local_id |
| kCMMetadataFormatDescriptionKey_Namespace | constant | CoreMedia/CMFormatDescription.h | cm::format_description::metadata_description_keys::namespace |
| kCMMetadataFormatDescriptionKey_SetupData | constant | CoreMedia/CMFormatDescription.h | cm::format_description::metadata_description_keys::setup_data |
| kCMMetadataFormatDescriptionKey_StructuralDependency | constant | CoreMedia/CMFormatDescription.h | cm::format_description::metadata_description_keys::structural_dependency |
| kCMMetadataFormatDescriptionKey_Value | constant | CoreMedia/CMFormatDescription.h | cm::format_description::metadata_description_keys::value |
| kCMMetadataFormatDescriptionMetadataSpecificationKey_DataType | constant | CoreMedia/CMFormatDescription.h | cm::format_description::metadata_specification_keys::data_type |
| kCMMetadataFormatDescriptionMetadataSpecificationKey_ExtendedLanguageTag | constant | CoreMedia/CMFormatDescription.h | cm::format_description::metadata_specification_keys::extended_language_tag |
| kCMMetadataFormatDescriptionMetadataSpecificationKey_Identifier | constant | CoreMedia/CMFormatDescription.h | cm::format_description::metadata_specification_keys::identifier |
| kCMMetadataFormatDescriptionMetadataSpecificationKey_SetupData | constant | CoreMedia/CMFormatDescription.h | cm::format_description::metadata_specification_keys::setup_data |
| kCMMetadataFormatDescriptionMetadataSpecificationKey_StructuralDependency | constant | CoreMedia/CMFormatDescription.h | cm::format_description::metadata_specification_keys::structural_dependency |
| kCMMetadataFormatDescription_StructuralDependencyKey_DependencyIsInvalidFlag | constant | CoreMedia/CMFormatDescription.h | cm::format_description::metadata_structural_dependency_keys::dependency_is_invalid_flag |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| CFSwap | struct | ? | No safe wrapper or bridge entry point was detected. |
| CFArrayAppendArray | function | CoreFoundation/CFArray.h | cf::CFArray is covered, but this public declaration is not exposed. |
| CFArrayAppendValue | function | CoreFoundation/CFArray.h | cf::CFArray is covered, but this public declaration is not exposed. |
| CFArrayApplyFunction | function | CoreFoundation/CFArray.h | cf::CFArray is covered, but this public declaration is not exposed. |
| CFArrayBSearchValues | function | CoreFoundation/CFArray.h | cf::CFArray is covered, but this public declaration is not exposed. |
| CFArrayCallBacks | typedef struct | CoreFoundation/CFArray.h | cf::CFArray is covered, but this public declaration is not exposed. |
| CFArrayContainsValue | function | CoreFoundation/CFArray.h | cf::CFArray is covered, but this public declaration is not exposed. |
| CFArrayCreateCopy | function | CoreFoundation/CFArray.h | cf::CFArray is covered, but this public declaration is not exposed. |
| CFArrayCreateMutable | function | CoreFoundation/CFArray.h | cf::CFArray is covered, but this public declaration is not exposed. |
| CFArrayCreateMutableCopy | function | CoreFoundation/CFArray.h | cf::CFArray is covered, but this public declaration is not exposed. |
| CFArrayExchangeValuesAtIndices | function | CoreFoundation/CFArray.h | cf::CFArray is covered, but this public declaration is not exposed. |
| CFArrayGetCountOfValue | function | CoreFoundation/CFArray.h | cf::CFArray is covered, but this public declaration is not exposed. |
| CFArrayGetFirstIndexOfValue | function | CoreFoundation/CFArray.h | cf::CFArray is covered, but this public declaration is not exposed. |
| CFArrayGetLastIndexOfValue | function | CoreFoundation/CFArray.h | cf::CFArray is covered, but this public declaration is not exposed. |
| CFArrayGetValues | function | CoreFoundation/CFArray.h | cf::CFArray is covered, but this public declaration is not exposed. |
| CFArrayInsertValueAtIndex | function | CoreFoundation/CFArray.h | cf::CFArray is covered, but this public declaration is not exposed. |
| CFArrayRemoveAllValues | function | CoreFoundation/CFArray.h | cf::CFArray is covered, but this public declaration is not exposed. |
| CFArrayRemoveValueAtIndex | function | CoreFoundation/CFArray.h | cf::CFArray is covered, but this public declaration is not exposed. |
| CFArrayReplaceValues | function | CoreFoundation/CFArray.h | cf::CFArray is covered, but this public declaration is not exposed. |
| CFArraySetValueAtIndex | function | CoreFoundation/CFArray.h | cf::CFArray is covered, but this public declaration is not exposed. |
| CFArraySortValues | function | CoreFoundation/CFArray.h | cf::CFArray is covered, but this public declaration is not exposed. |
| CFMutableArrayRef | typedef struct | CoreFoundation/CFArray.h | No safe wrapper or bridge entry point was detected. |
| kCFTypeArrayCallBacks | constant | CoreFoundation/CFArray.h | No safe wrapper or bridge entry point was detected. |
| CFAttributedStringBeginEditing | function | CoreFoundation/CFAttributedString.h | cf::CFAttributedString is covered, but this public declaration is not exposed. |
| CFAttributedStringCreateCopy | function | CoreFoundation/CFAttributedString.h | cf::CFAttributedString is covered, but this public declaration is not exposed. |
| CFAttributedStringCreateMutable | function | CoreFoundation/CFAttributedString.h | cf::CFAttributedString is covered, but this public declaration is not exposed. |
| CFAttributedStringCreateMutableCopy | function | CoreFoundation/CFAttributedString.h | cf::CFAttributedString is covered, but this public declaration is not exposed. |
| CFAttributedStringCreateWithSubstring | function | CoreFoundation/CFAttributedString.h | cf::CFAttributedString is covered, but this public declaration is not exposed. |
| CFAttributedStringEndEditing | function | CoreFoundation/CFAttributedString.h | cf::CFAttributedString is covered, but this public declaration is not exposed. |
| CFAttributedStringGetAttribute | function | CoreFoundation/CFAttributedString.h | cf::CFAttributedString is covered, but this public declaration is not exposed. |
| CFAttributedStringGetAttributeAndLongestEffectiveRange | function | CoreFoundation/CFAttributedString.h | cf::CFAttributedString is covered, but this public declaration is not exposed. |
| CFAttributedStringGetAttributes | function | CoreFoundation/CFAttributedString.h | cf::CFAttributedString is covered, but this public declaration is not exposed. |
| CFAttributedStringGetAttributesAndLongestEffectiveRange | function | CoreFoundation/CFAttributedString.h | cf::CFAttributedString is covered, but this public declaration is not exposed. |
| CFAttributedStringGetBidiLevelsAndResolvedDirections | function | CoreFoundation/CFAttributedString.h | cf::CFAttributedString is covered, but this public declaration is not exposed. |
| CFAttributedStringGetMutableString | function | CoreFoundation/CFAttributedString.h | cf::CFAttributedString is covered, but this public declaration is not exposed. |
| CFAttributedStringGetStatisticalWritingDirections | function | CoreFoundation/CFAttributedString.h | cf::CFAttributedString is covered, but this public declaration is not exposed. |
| CFAttributedStringRemoveAttribute | function | CoreFoundation/CFAttributedString.h | cf::CFAttributedString is covered, but this public declaration is not exposed. |
| CFAttributedStringReplaceAttributedString | function | CoreFoundation/CFAttributedString.h | cf::CFAttributedString is covered, but this public declaration is not exposed. |
| CFAttributedStringReplaceString | function | CoreFoundation/CFAttributedString.h | cf::CFAttributedString is covered, but this public declaration is not exposed. |
| CFAttributedStringSetAttribute | function | CoreFoundation/CFAttributedString.h | cf::CFAttributedString is covered, but this public declaration is not exposed. |
| CFAttributedStringSetAttributes | function | CoreFoundation/CFAttributedString.h | cf::CFAttributedString is covered, but this public declaration is not exposed. |
| CFMutableAttributedStringRef | typedef struct | CoreFoundation/CFAttributedString.h | No safe wrapper or bridge entry point was detected. |
| CFBagAddValue | function | CoreFoundation/CFBag.h | cf::CFBag is covered, but this public declaration is not exposed. |
| CFBagApplyFunction | function | CoreFoundation/CFBag.h | cf::CFBag is covered, but this public declaration is not exposed. |
| CFBagCallBacks | typedef struct | CoreFoundation/CFBag.h | cf::CFBag is covered, but this public declaration is not exposed. |
| CFBagCreateCopy | function | CoreFoundation/CFBag.h | cf::CFBag is covered, but this public declaration is not exposed. |
| CFBagCreateMutable | function | CoreFoundation/CFBag.h | cf::CFBag is covered, but this public declaration is not exposed. |
| CFBagCreateMutableCopy | function | CoreFoundation/CFBag.h | cf::CFBag is covered, but this public declaration is not exposed. |
| CFBagGetValue | function | CoreFoundation/CFBag.h | cf::CFBag is covered, but this public declaration is not exposed. |
| CFBagGetValueIfPresent | function | CoreFoundation/CFBag.h | cf::CFBag is covered, but this public declaration is not exposed. |
| CFBagGetValues | function | CoreFoundation/CFBag.h | cf::CFBag is covered, but this public declaration is not exposed. |
| CFBagRemoveAllValues | function | CoreFoundation/CFBag.h | cf::CFBag is covered, but this public declaration is not exposed. |
| CFBagRemoveValue | function | CoreFoundation/CFBag.h | cf::CFBag is covered, but this public declaration is not exposed. |
| CFBagReplaceValue | function | CoreFoundation/CFBag.h | cf::CFBag is covered, but this public declaration is not exposed. |
| CFBagSetValue | function | CoreFoundation/CFBag.h | cf::CFBag is covered, but this public declaration is not exposed. |
| CFMutableBagRef | typedef struct | CoreFoundation/CFBag.h | No safe wrapper or bridge entry point was detected. |
| kCFCopyStringBagCallBacks | constant | CoreFoundation/CFBag.h | No safe wrapper or bridge entry point was detected. |
| kCFTypeBagCallBacks | constant | CoreFoundation/CFBag.h | No safe wrapper or bridge entry point was detected. |
| CFAllocatorAllocate | function | CoreFoundation/CFBase.h | No safe wrapper or bridge entry point was detected. |
| CFAllocatorAllocateBytes | function | CoreFoundation/CFBase.h | No safe wrapper or bridge entry point was detected. |
| CFAllocatorAllocateTyped | function | CoreFoundation/CFBase.h | No safe wrapper or bridge entry point was detected. |
| CFAllocatorContext | typedef struct | CoreFoundation/CFBase.h | No safe wrapper or bridge entry point was detected. |
| CFAllocatorCreate | function | CoreFoundation/CFBase.h | No safe wrapper or bridge entry point was detected. |
| CFAllocatorDeallocate | function | CoreFoundation/CFBase.h | No safe wrapper or bridge entry point was detected. |
| CFAllocatorGetContext | function | CoreFoundation/CFBase.h | No safe wrapper or bridge entry point was detected. |
| CFAllocatorGetDefault | function | CoreFoundation/CFBase.h | No safe wrapper or bridge entry point was detected. |
| CFAllocatorGetPreferredSizeForSize | function | CoreFoundation/CFBase.h | No safe wrapper or bridge entry point was detected. |
| CFAllocatorGetTypeID | function | CoreFoundation/CFBase.h | No safe wrapper or bridge entry point was detected. |
| CFAllocatorReallocate | function | CoreFoundation/CFBase.h | No safe wrapper or bridge entry point was detected. |
| CFAllocatorReallocateBytes | function | CoreFoundation/CFBase.h | No safe wrapper or bridge entry point was detected. |
| CFAllocatorReallocateTyped | function | CoreFoundation/CFBase.h | No safe wrapper or bridge entry point was detected. |
| CFAllocatorRef | typedef struct | CoreFoundation/CFBase.h | No safe wrapper or bridge entry point was detected. |
| CFAllocatorSetDefault | function | CoreFoundation/CFBase.h | No safe wrapper or bridge entry point was detected. |
| CFAutorelease | function | CoreFoundation/CFBase.h | No safe wrapper or bridge entry point was detected. |
| CFComparisonResult | typedef enum | CoreFoundation/CFBase.h | No safe wrapper or bridge entry point was detected. |
| CFCopyTypeIDDescription | function | CoreFoundation/CFBase.h | No safe wrapper or bridge entry point was detected. |
| CFGetAllocator | function | CoreFoundation/CFBase.h | No safe wrapper or bridge entry point was detected. |
| CFGetRetainCount | function | CoreFoundation/CFBase.h | No safe wrapper or bridge entry point was detected. |
| CFMakeCollectable | function | CoreFoundation/CFBase.h | No safe wrapper or bridge entry point was detected. |
| CFMutableStringRef | typedef struct | CoreFoundation/CFBase.h | No safe wrapper or bridge entry point was detected. |
| CFNullGetTypeID | function | CoreFoundation/CFBase.h | No safe wrapper or bridge entry point was detected. |
| CFNullRef | typedef struct | CoreFoundation/CFBase.h | No safe wrapper or bridge entry point was detected. |
| CFRange | typedef struct | CoreFoundation/CFBase.h | No safe wrapper or bridge entry point was detected. |
| CFRangeMake | function | CoreFoundation/CFBase.h | No safe wrapper or bridge entry point was detected. |
| kCFAllocatorDefault | constant | CoreFoundation/CFBase.h | No safe wrapper or bridge entry point was detected. |
| kCFAllocatorMalloc | constant | CoreFoundation/CFBase.h | No safe wrapper or bridge entry point was detected. |
| kCFAllocatorMallocZone | constant | CoreFoundation/CFBase.h | No safe wrapper or bridge entry point was detected. |
| kCFAllocatorNull | constant | CoreFoundation/CFBase.h | No safe wrapper or bridge entry point was detected. |
| kCFAllocatorSystemDefault | constant | CoreFoundation/CFBase.h | No safe wrapper or bridge entry point was detected. |
| kCFAllocatorUseContext | constant | CoreFoundation/CFBase.h | No safe wrapper or bridge entry point was detected. |
| kCFCoreFoundationVersionNumber | constant | CoreFoundation/CFBase.h | No safe wrapper or bridge entry point was detected. |
| kCFNotFound | constant | CoreFoundation/CFBase.h | No safe wrapper or bridge entry point was detected. |
| kCFNull | constant | CoreFoundation/CFBase.h | No safe wrapper or bridge entry point was detected. |
| CFBinaryHeapAddValue | function | CoreFoundation/CFBinaryHeap.h | No safe wrapper or bridge entry point was detected. |
| CFBinaryHeapApplyFunction | function | CoreFoundation/CFBinaryHeap.h | No safe wrapper or bridge entry point was detected. |
| CFBinaryHeapCallBacks | typedef struct | CoreFoundation/CFBinaryHeap.h | No safe wrapper or bridge entry point was detected. |
| CFBinaryHeapCompareContext | typedef struct | CoreFoundation/CFBinaryHeap.h | No safe wrapper or bridge entry point was detected. |
| CFBinaryHeapContainsValue | function | CoreFoundation/CFBinaryHeap.h | No safe wrapper or bridge entry point was detected. |
| CFBinaryHeapCreate | function | CoreFoundation/CFBinaryHeap.h | No safe wrapper or bridge entry point was detected. |
| CFBinaryHeapCreateCopy | function | CoreFoundation/CFBinaryHeap.h | No safe wrapper or bridge entry point was detected. |
| CFBinaryHeapGetCount | function | CoreFoundation/CFBinaryHeap.h | No safe wrapper or bridge entry point was detected. |
| CFBinaryHeapGetCountOfValue | function | CoreFoundation/CFBinaryHeap.h | No safe wrapper or bridge entry point was detected. |
| CFBinaryHeapGetMinimum | function | CoreFoundation/CFBinaryHeap.h | No safe wrapper or bridge entry point was detected. |
| CFBinaryHeapGetMinimumIfPresent | function | CoreFoundation/CFBinaryHeap.h | No safe wrapper or bridge entry point was detected. |
| CFBinaryHeapGetTypeID | function | CoreFoundation/CFBinaryHeap.h | No safe wrapper or bridge entry point was detected. |
| CFBinaryHeapGetValues | function | CoreFoundation/CFBinaryHeap.h | No safe wrapper or bridge entry point was detected. |
| CFBinaryHeapRef | typedef struct | CoreFoundation/CFBinaryHeap.h | No safe wrapper or bridge entry point was detected. |
| CFBinaryHeapRemoveAllValues | function | CoreFoundation/CFBinaryHeap.h | No safe wrapper or bridge entry point was detected. |
| CFBinaryHeapRemoveMinimumValue | function | CoreFoundation/CFBinaryHeap.h | No safe wrapper or bridge entry point was detected. |
| kCFStringBinaryHeapCallBacks | constant | CoreFoundation/CFBinaryHeap.h | No safe wrapper or bridge entry point was detected. |
| CFBitVectorContainsBit | function | CoreFoundation/CFBitVector.h | No safe wrapper or bridge entry point was detected. |
| CFBitVectorCreate | function | CoreFoundation/CFBitVector.h | No safe wrapper or bridge entry point was detected. |
| CFBitVectorCreateCopy | function | CoreFoundation/CFBitVector.h | No safe wrapper or bridge entry point was detected. |
| CFBitVectorCreateMutable | function | CoreFoundation/CFBitVector.h | No safe wrapper or bridge entry point was detected. |
| CFBitVectorCreateMutableCopy | function | CoreFoundation/CFBitVector.h | No safe wrapper or bridge entry point was detected. |
| CFBitVectorFlipBitAtIndex | function | CoreFoundation/CFBitVector.h | No safe wrapper or bridge entry point was detected. |
| CFBitVectorFlipBits | function | CoreFoundation/CFBitVector.h | No safe wrapper or bridge entry point was detected. |
| CFBitVectorGetBitAtIndex | function | CoreFoundation/CFBitVector.h | No safe wrapper or bridge entry point was detected. |
| CFBitVectorGetBits | function | CoreFoundation/CFBitVector.h | No safe wrapper or bridge entry point was detected. |
| CFBitVectorGetCount | function | CoreFoundation/CFBitVector.h | No safe wrapper or bridge entry point was detected. |
| CFBitVectorGetCountOfBit | function | CoreFoundation/CFBitVector.h | No safe wrapper or bridge entry point was detected. |
| CFBitVectorGetFirstIndexOfBit | function | CoreFoundation/CFBitVector.h | No safe wrapper or bridge entry point was detected. |
| CFBitVectorGetLastIndexOfBit | function | CoreFoundation/CFBitVector.h | No safe wrapper or bridge entry point was detected. |
| CFBitVectorGetTypeID | function | CoreFoundation/CFBitVector.h | No safe wrapper or bridge entry point was detected. |
| CFBitVectorRef | typedef struct | CoreFoundation/CFBitVector.h | No safe wrapper or bridge entry point was detected. |
| CFBitVectorSetAllBits | function | CoreFoundation/CFBitVector.h | No safe wrapper or bridge entry point was detected. |
| CFBitVectorSetBitAtIndex | function | CoreFoundation/CFBitVector.h | No safe wrapper or bridge entry point was detected. |
| CFBitVectorSetBits | function | CoreFoundation/CFBitVector.h | No safe wrapper or bridge entry point was detected. |
| CFBitVectorSetCount | function | CoreFoundation/CFBitVector.h | No safe wrapper or bridge entry point was detected. |
| CFMutableBitVectorRef | typedef struct | CoreFoundation/CFBitVector.h | No safe wrapper or bridge entry point was detected. |
| CFBundleCopyAuxiliaryExecutableURL | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleCopyBuiltInPlugInsURL | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleCopyBundleLocalizations | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleCopyExecutableArchitectures | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleCopyExecutableArchitecturesForURL | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleCopyExecutableURL | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleCopyInfoDictionaryForURL | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleCopyInfoDictionaryInDirectory | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleCopyLocalizationsForPreferences | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleCopyLocalizationsForURL | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleCopyLocalizedString | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleCopyLocalizedStringForLocalizations | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleCopyPreferredLocalizationsFromArray | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleCopyPrivateFrameworksURL | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleCopyResourceURLForLocalization | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleCopyResourceURLInDirectory | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleCopyResourceURLsOfType | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleCopyResourceURLsOfTypeForLocalization | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleCopyResourceURLsOfTypeInDirectory | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleCopyResourcesDirectoryURL | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleCopySharedFrameworksURL | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleCopySharedSupportURL | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleCopySupportFilesDirectoryURL | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleCreateBundlesFromDirectory | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleGetAllBundles | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleGetBundleWithIdentifier | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleGetDataPointerForName | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleGetDataPointersForNames | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleGetDevelopmentRegion | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleGetFunctionPointerForName | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleGetFunctionPointersForNames | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleGetInfoDictionary | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleGetLocalInfoDictionary | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleGetPackageInfo | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleGetPackageInfoInDirectory | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleGetPlugIn | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleGetValueForInfoDictionaryKey | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleGetVersionNumber | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleIsArchitectureLoadable | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleIsExecutableLoadable | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleIsExecutableLoadableForURL | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleIsExecutableLoaded | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleLoadExecutable | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleLoadExecutableAndReturnError | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundlePreflightExecutable | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFBundleUnloadExecutable | function | CoreFoundation/CFBundle.h | cf::CFBundle is covered, but this public declaration is not exposed. |
| CFPlugInRef | typedef struct | CoreFoundation/CFBundle.h | No safe wrapper or bridge entry point was detected. |
| kCFBundleDevelopmentRegionKey | constant | CoreFoundation/CFBundle.h | No safe wrapper or bridge entry point was detected. |
| kCFBundleExecutableKey | constant | CoreFoundation/CFBundle.h | No safe wrapper or bridge entry point was detected. |
| kCFBundleIdentifierKey | constant | CoreFoundation/CFBundle.h | No safe wrapper or bridge entry point was detected. |
| kCFBundleInfoDictionaryVersionKey | constant | CoreFoundation/CFBundle.h | No safe wrapper or bridge entry point was detected. |
| kCFBundleLocalizationsKey | constant | CoreFoundation/CFBundle.h | No safe wrapper or bridge entry point was detected. |
| kCFBundleNameKey | constant | CoreFoundation/CFBundle.h | No safe wrapper or bridge entry point was detected. |
| kCFBundleVersionKey | constant | CoreFoundation/CFBundle.h | No safe wrapper or bridge entry point was detected. |
| CFByteOrderGetCurrent | function | CoreFoundation/CFByteOrder.h | No safe wrapper or bridge entry point was detected. |
| CFConvertDoubleHostToSwapped | function | CoreFoundation/CFByteOrder.h | No safe wrapper or bridge entry point was detected. |
| CFConvertDoubleSwappedToHost | function | CoreFoundation/CFByteOrder.h | No safe wrapper or bridge entry point was detected. |
| CFConvertFloat32HostToSwapped | function | CoreFoundation/CFByteOrder.h | No safe wrapper or bridge entry point was detected. |
| CFConvertFloat32SwappedToHost | function | CoreFoundation/CFByteOrder.h | No safe wrapper or bridge entry point was detected. |
| CFConvertFloat64HostToSwapped | function | CoreFoundation/CFByteOrder.h | No safe wrapper or bridge entry point was detected. |
| CFConvertFloat64SwappedToHost | function | CoreFoundation/CFByteOrder.h | No safe wrapper or bridge entry point was detected. |
| CFConvertFloatHostToSwapped | function | CoreFoundation/CFByteOrder.h | No safe wrapper or bridge entry point was detected. |
| CFConvertFloatSwappedToHost | function | CoreFoundation/CFByteOrder.h | No safe wrapper or bridge entry point was detected. |
| CFSwapInt16 | function | CoreFoundation/CFByteOrder.h | No safe wrapper or bridge entry point was detected. |
| CFSwapInt16BigToHost | function | CoreFoundation/CFByteOrder.h | No safe wrapper or bridge entry point was detected. |
| CFSwapInt16HostToBig | function | CoreFoundation/CFByteOrder.h | No safe wrapper or bridge entry point was detected. |
| CFSwapInt16HostToLittle | function | CoreFoundation/CFByteOrder.h | No safe wrapper or bridge entry point was detected. |
| CFSwapInt16LittleToHost | function | CoreFoundation/CFByteOrder.h | No safe wrapper or bridge entry point was detected. |
| CFSwapInt32 | function | CoreFoundation/CFByteOrder.h | No safe wrapper or bridge entry point was detected. |
| CFSwapInt32BigToHost | function | CoreFoundation/CFByteOrder.h | No safe wrapper or bridge entry point was detected. |
| CFSwapInt32HostToBig | function | CoreFoundation/CFByteOrder.h | No safe wrapper or bridge entry point was detected. |
| CFSwapInt32HostToLittle | function | CoreFoundation/CFByteOrder.h | No safe wrapper or bridge entry point was detected. |
| CFSwapInt32LittleToHost | function | CoreFoundation/CFByteOrder.h | No safe wrapper or bridge entry point was detected. |
| CFSwapInt64 | function | CoreFoundation/CFByteOrder.h | No safe wrapper or bridge entry point was detected. |
| CFSwapInt64BigToHost | function | CoreFoundation/CFByteOrder.h | No safe wrapper or bridge entry point was detected. |
| CFSwapInt64HostToBig | function | CoreFoundation/CFByteOrder.h | No safe wrapper or bridge entry point was detected. |
| CFSwapInt64HostToLittle | function | CoreFoundation/CFByteOrder.h | No safe wrapper or bridge entry point was detected. |
| CFSwapInt64LittleToHost | function | CoreFoundation/CFByteOrder.h | No safe wrapper or bridge entry point was detected. |
| CFSwappedFloat32 | typedef struct | CoreFoundation/CFByteOrder.h | No safe wrapper or bridge entry point was detected. |
| CFSwappedFloat64 | typedef struct | CoreFoundation/CFByteOrder.h | No safe wrapper or bridge entry point was detected. |
| CFCalendarAddComponents | function | CoreFoundation/CFCalendar.h | cf::CFCalendar is covered, but this public declaration is not exposed. |
| CFCalendarComposeAbsoluteTime | function | CoreFoundation/CFCalendar.h | cf::CFCalendar is covered, but this public declaration is not exposed. |
| CFCalendarCopyLocale | function | CoreFoundation/CFCalendar.h | cf::CFCalendar is covered, but this public declaration is not exposed. |
| CFCalendarDecomposeAbsoluteTime | function | CoreFoundation/CFCalendar.h | cf::CFCalendar is covered, but this public declaration is not exposed. |
| CFCalendarGetComponentDifference | function | CoreFoundation/CFCalendar.h | cf::CFCalendar is covered, but this public declaration is not exposed. |
| CFCalendarGetFirstWeekday | function | CoreFoundation/CFCalendar.h | cf::CFCalendar is covered, but this public declaration is not exposed. |
| CFCalendarGetMaximumRangeOfUnit | function | CoreFoundation/CFCalendar.h | cf::CFCalendar is covered, but this public declaration is not exposed. |
| CFCalendarGetMinimumDaysInFirstWeek | function | CoreFoundation/CFCalendar.h | cf::CFCalendar is covered, but this public declaration is not exposed. |
| CFCalendarGetMinimumRangeOfUnit | function | CoreFoundation/CFCalendar.h | cf::CFCalendar is covered, but this public declaration is not exposed. |
| CFCalendarGetOrdinalityOfUnit | function | CoreFoundation/CFCalendar.h | cf::CFCalendar is covered, but this public declaration is not exposed. |
| CFCalendarGetRangeOfUnit | function | CoreFoundation/CFCalendar.h | cf::CFCalendar is covered, but this public declaration is not exposed. |
| CFCalendarGetTimeRangeOfUnit | function | CoreFoundation/CFCalendar.h | cf::CFCalendar is covered, but this public declaration is not exposed. |
| CFCalendarSetFirstWeekday | function | CoreFoundation/CFCalendar.h | cf::CFCalendar is covered, but this public declaration is not exposed. |
| CFCalendarSetLocale | function | CoreFoundation/CFCalendar.h | cf::CFCalendar is covered, but this public declaration is not exposed. |
| CFCalendarSetMinimumDaysInFirstWeek | function | CoreFoundation/CFCalendar.h | cf::CFCalendar is covered, but this public declaration is not exposed. |
| CFCalendarUnit | typedef enum | CoreFoundation/CFCalendar.h | cf::CFCalendar is covered, but this public declaration is not exposed. |
| CFCharacterSetAddCharactersInRange | function | CoreFoundation/CFCharacterSet.h | cf::CFCharacterSet is covered, but this public declaration is not exposed. |
| CFCharacterSetAddCharactersInString | function | CoreFoundation/CFCharacterSet.h | cf::CFCharacterSet is covered, but this public declaration is not exposed. |
| CFCharacterSetCreateBitmapRepresentation | function | CoreFoundation/CFCharacterSet.h | cf::CFCharacterSet is covered, but this public declaration is not exposed. |
| CFCharacterSetCreateCopy | function | CoreFoundation/CFCharacterSet.h | cf::CFCharacterSet is covered, but this public declaration is not exposed. |
| CFCharacterSetCreateMutable | function | CoreFoundation/CFCharacterSet.h | cf::CFCharacterSet is covered, but this public declaration is not exposed. |
| CFCharacterSetCreateMutableCopy | function | CoreFoundation/CFCharacterSet.h | cf::CFCharacterSet is covered, but this public declaration is not exposed. |
| CFCharacterSetCreateWithBitmapRepresentation | function | CoreFoundation/CFCharacterSet.h | cf::CFCharacterSet is covered, but this public declaration is not exposed. |
| CFCharacterSetCreateWithCharactersInRange | function | CoreFoundation/CFCharacterSet.h | cf::CFCharacterSet is covered, but this public declaration is not exposed. |
| CFCharacterSetGetPredefined | function | CoreFoundation/CFCharacterSet.h | cf::CFCharacterSet is covered, but this public declaration is not exposed. |
| CFCharacterSetHasMemberInPlane | function | CoreFoundation/CFCharacterSet.h | cf::CFCharacterSet is covered, but this public declaration is not exposed. |
| CFCharacterSetIntersect | function | CoreFoundation/CFCharacterSet.h | cf::CFCharacterSet is covered, but this public declaration is not exposed. |
| CFCharacterSetInvert | function | CoreFoundation/CFCharacterSet.h | cf::CFCharacterSet is covered, but this public declaration is not exposed. |
| CFCharacterSetIsLongCharacterMember | function | CoreFoundation/CFCharacterSet.h | cf::CFCharacterSet is covered, but this public declaration is not exposed. |
| CFCharacterSetIsSupersetOfSet | function | CoreFoundation/CFCharacterSet.h | cf::CFCharacterSet is covered, but this public declaration is not exposed. |
| CFCharacterSetPredefinedSet | typedef enum | CoreFoundation/CFCharacterSet.h | cf::CFCharacterSet is covered, but this public declaration is not exposed. |
| CFCharacterSetRemoveCharactersInRange | function | CoreFoundation/CFCharacterSet.h | cf::CFCharacterSet is covered, but this public declaration is not exposed. |
| CFCharacterSetRemoveCharactersInString | function | CoreFoundation/CFCharacterSet.h | cf::CFCharacterSet is covered, but this public declaration is not exposed. |
| CFCharacterSetUnion | function | CoreFoundation/CFCharacterSet.h | cf::CFCharacterSet is covered, but this public declaration is not exposed. |
| CFMutableCharacterSetRef | typedef struct | CoreFoundation/CFCharacterSet.h | No safe wrapper or bridge entry point was detected. |
| CFDataAppendBytes | function | CoreFoundation/CFData.h | cf::CFData is covered, but this public declaration is not exposed. |
| CFDataCreateCopy | function | CoreFoundation/CFData.h | cf::CFData is covered, but this public declaration is not exposed. |
| CFDataCreateMutable | function | CoreFoundation/CFData.h | cf::CFData is covered, but this public declaration is not exposed. |
| CFDataCreateMutableCopy | function | CoreFoundation/CFData.h | cf::CFData is covered, but this public declaration is not exposed. |
| CFDataCreateWithBytesNoCopy | function | CoreFoundation/CFData.h | cf::CFData is covered, but this public declaration is not exposed. |
| CFDataDeleteBytes | function | CoreFoundation/CFData.h | cf::CFData is covered, but this public declaration is not exposed. |
| CFDataFind | function | CoreFoundation/CFData.h | cf::CFData is covered, but this public declaration is not exposed. |
| CFDataGetBytes | function | CoreFoundation/CFData.h | cf::CFData is covered, but this public declaration is not exposed. |
| CFDataGetMutableBytePtr | function | CoreFoundation/CFData.h | cf::CFData is covered, but this public declaration is not exposed. |
| CFDataIncreaseLength | function | CoreFoundation/CFData.h | cf::CFData is covered, but this public declaration is not exposed. |
| CFDataReplaceBytes | function | CoreFoundation/CFData.h | cf::CFData is covered, but this public declaration is not exposed. |
| CFDataSearchFlags | typedef enum | CoreFoundation/CFData.h | cf::CFData is covered, but this public declaration is not exposed. |
| CFDataSetLength | function | CoreFoundation/CFData.h | cf::CFData is covered, but this public declaration is not exposed. |
| CFMutableDataRef | typedef struct | CoreFoundation/CFData.h | No safe wrapper or bridge entry point was detected. |
| CFDateCompare | function | CoreFoundation/CFDate.h | cf::CFDate is covered, but this public declaration is not exposed. |
| CFDateGetTimeIntervalSinceDate | function | CoreFoundation/CFDate.h | cf::CFDate is covered, but this public declaration is not exposed. |
| kCFAbsoluteTimeIntervalSince1904 | constant | CoreFoundation/CFDate.h | No safe wrapper or bridge entry point was detected. |
| kCFAbsoluteTimeIntervalSince1970 | constant | CoreFoundation/CFDate.h | No safe wrapper or bridge entry point was detected. |
| CFDateFormatterCopyProperty | function | CoreFoundation/CFDateFormatter.h | cf::CFDateFormatter is covered, but this public declaration is not exposed. |
| CFDateFormatterCreateDateFormatFromTemplate | function | CoreFoundation/CFDateFormatter.h | cf::CFDateFormatter is covered, but this public declaration is not exposed. |
| CFDateFormatterCreateDateFromString | function | CoreFoundation/CFDateFormatter.h | cf::CFDateFormatter is covered, but this public declaration is not exposed. |
| CFDateFormatterCreateISO8601Formatter | function | CoreFoundation/CFDateFormatter.h | cf::CFDateFormatter is covered, but this public declaration is not exposed. |
| CFDateFormatterCreateStringWithAbsoluteTime | function | CoreFoundation/CFDateFormatter.h | cf::CFDateFormatter is covered, but this public declaration is not exposed. |
| CFDateFormatterGetAbsoluteTimeFromString | function | CoreFoundation/CFDateFormatter.h | cf::CFDateFormatter is covered, but this public declaration is not exposed. |
| CFDateFormatterGetDateStyle | function | CoreFoundation/CFDateFormatter.h | cf::CFDateFormatter is covered, but this public declaration is not exposed. |
| CFDateFormatterGetFormat | function | CoreFoundation/CFDateFormatter.h | cf::CFDateFormatter is covered, but this public declaration is not exposed. |
| CFDateFormatterGetLocale | function | CoreFoundation/CFDateFormatter.h | cf::CFDateFormatter is covered, but this public declaration is not exposed. |
| CFDateFormatterGetTimeStyle | function | CoreFoundation/CFDateFormatter.h | cf::CFDateFormatter is covered, but this public declaration is not exposed. |
| CFDateFormatterSetFormat | function | CoreFoundation/CFDateFormatter.h | cf::CFDateFormatter is covered, but this public declaration is not exposed. |
| CFDateFormatterSetProperty | function | CoreFoundation/CFDateFormatter.h | cf::CFDateFormatter is covered, but this public declaration is not exposed. |
| CFISO8601DateFormatOptions | typedef enum | CoreFoundation/CFDateFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFDateFormatterAMSymbol | constant | CoreFoundation/CFDateFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFDateFormatterCalendar | constant | CoreFoundation/CFDateFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFDateFormatterCalendarName | constant | CoreFoundation/CFDateFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFDateFormatterDefaultDate | constant | CoreFoundation/CFDateFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFDateFormatterDefaultFormat | constant | CoreFoundation/CFDateFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFDateFormatterDoesRelativeDateFormattingKey | constant | CoreFoundation/CFDateFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFDateFormatterEraSymbols | constant | CoreFoundation/CFDateFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFDateFormatterGregorianStartDate | constant | CoreFoundation/CFDateFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFDateFormatterIsLenient | constant | CoreFoundation/CFDateFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFDateFormatterLongEraSymbols | constant | CoreFoundation/CFDateFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFDateFormatterMonthSymbols | constant | CoreFoundation/CFDateFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFDateFormatterPMSymbol | constant | CoreFoundation/CFDateFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFDateFormatterQuarterSymbols | constant | CoreFoundation/CFDateFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFDateFormatterShortMonthSymbols | constant | CoreFoundation/CFDateFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFDateFormatterShortQuarterSymbols | constant | CoreFoundation/CFDateFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFDateFormatterShortStandaloneMonthSymbols | constant | CoreFoundation/CFDateFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFDateFormatterShortStandaloneQuarterSymbols | constant | CoreFoundation/CFDateFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFDateFormatterShortStandaloneWeekdaySymbols | constant | CoreFoundation/CFDateFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFDateFormatterShortWeekdaySymbols | constant | CoreFoundation/CFDateFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFDateFormatterStandaloneMonthSymbols | constant | CoreFoundation/CFDateFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFDateFormatterStandaloneQuarterSymbols | constant | CoreFoundation/CFDateFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFDateFormatterStandaloneWeekdaySymbols | constant | CoreFoundation/CFDateFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFDateFormatterTimeZone | constant | CoreFoundation/CFDateFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFDateFormatterTwoDigitStartDate | constant | CoreFoundation/CFDateFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFDateFormatterVeryShortMonthSymbols | constant | CoreFoundation/CFDateFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFDateFormatterVeryShortStandaloneMonthSymbols | constant | CoreFoundation/CFDateFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFDateFormatterVeryShortStandaloneWeekdaySymbols | constant | CoreFoundation/CFDateFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFDateFormatterVeryShortWeekdaySymbols | constant | CoreFoundation/CFDateFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFDateFormatterWeekdaySymbols | constant | CoreFoundation/CFDateFormatter.h | No safe wrapper or bridge entry point was detected. |
| CFDictionaryAddValue | function | CoreFoundation/CFDictionary.h | cf::CFDictionary is covered, but this public declaration is not exposed. |
| CFDictionaryApplyFunction | function | CoreFoundation/CFDictionary.h | cf::CFDictionary is covered, but this public declaration is not exposed. |
| CFDictionaryContainsValue | function | CoreFoundation/CFDictionary.h | cf::CFDictionary is covered, but this public declaration is not exposed. |
| CFDictionaryCreateCopy | function | CoreFoundation/CFDictionary.h | cf::CFDictionary is covered, but this public declaration is not exposed. |
| CFDictionaryCreateMutable | function | CoreFoundation/CFDictionary.h | cf::CFDictionary is covered, but this public declaration is not exposed. |
| CFDictionaryCreateMutableCopy | function | CoreFoundation/CFDictionary.h | cf::CFDictionary is covered, but this public declaration is not exposed. |
| CFDictionaryGetCountOfKey | function | CoreFoundation/CFDictionary.h | cf::CFDictionary is covered, but this public declaration is not exposed. |
| CFDictionaryGetCountOfValue | function | CoreFoundation/CFDictionary.h | cf::CFDictionary is covered, but this public declaration is not exposed. |
| CFDictionaryGetValueIfPresent | function | CoreFoundation/CFDictionary.h | cf::CFDictionary is covered, but this public declaration is not exposed. |
| CFDictionaryKeyCallBacks | typedef struct | CoreFoundation/CFDictionary.h | cf::CFDictionary is covered, but this public declaration is not exposed. |
| CFDictionaryRemoveAllValues | function | CoreFoundation/CFDictionary.h | cf::CFDictionary is covered, but this public declaration is not exposed. |
| CFDictionaryRemoveValue | function | CoreFoundation/CFDictionary.h | cf::CFDictionary is covered, but this public declaration is not exposed. |
| CFDictionaryReplaceValue | function | CoreFoundation/CFDictionary.h | cf::CFDictionary is covered, but this public declaration is not exposed. |
| CFDictionarySetValue | function | CoreFoundation/CFDictionary.h | cf::CFDictionary is covered, but this public declaration is not exposed. |
| CFDictionaryValueCallBacks | typedef struct | CoreFoundation/CFDictionary.h | cf::CFDictionary is covered, but this public declaration is not exposed. |
| CFMutableDictionaryRef | typedef struct | CoreFoundation/CFDictionary.h | No safe wrapper or bridge entry point was detected. |
| kCFCopyStringDictionaryKeyCallBacks | constant | CoreFoundation/CFDictionary.h | No safe wrapper or bridge entry point was detected. |
| kCFTypeDictionaryKeyCallBacks | constant | CoreFoundation/CFDictionary.h | No safe wrapper or bridge entry point was detected. |
| kCFTypeDictionaryValueCallBacks | constant | CoreFoundation/CFDictionary.h | No safe wrapper or bridge entry point was detected. |
| CFErrorCopyRecoverySuggestion | function | CoreFoundation/CFError.h | cf::CFError is covered, but this public declaration is not exposed. |
| CFErrorCopyUserInfo | function | CoreFoundation/CFError.h | cf::CFError is covered, but this public declaration is not exposed. |
| CFErrorCreateWithUserInfoKeysAndValues | function | CoreFoundation/CFError.h | cf::CFError is covered, but this public declaration is not exposed. |
| kCFErrorDescriptionKey | constant | CoreFoundation/CFError.h | No safe wrapper or bridge entry point was detected. |
| kCFErrorDomainCocoa | constant | CoreFoundation/CFError.h | No safe wrapper or bridge entry point was detected. |
| kCFErrorDomainMach | constant | CoreFoundation/CFError.h | No safe wrapper or bridge entry point was detected. |
| kCFErrorDomainOSStatus | constant | CoreFoundation/CFError.h | No safe wrapper or bridge entry point was detected. |
| kCFErrorDomainPOSIX | constant | CoreFoundation/CFError.h | No safe wrapper or bridge entry point was detected. |
| kCFErrorFilePathKey | constant | CoreFoundation/CFError.h | No safe wrapper or bridge entry point was detected. |
| kCFErrorLocalizedDescriptionKey | constant | CoreFoundation/CFError.h | No safe wrapper or bridge entry point was detected. |
| kCFErrorLocalizedFailureKey | constant | CoreFoundation/CFError.h | No safe wrapper or bridge entry point was detected. |
| kCFErrorLocalizedFailureReasonKey | constant | CoreFoundation/CFError.h | No safe wrapper or bridge entry point was detected. |
| kCFErrorLocalizedRecoverySuggestionKey | constant | CoreFoundation/CFError.h | No safe wrapper or bridge entry point was detected. |
| kCFErrorURLKey | constant | CoreFoundation/CFError.h | No safe wrapper or bridge entry point was detected. |
| kCFErrorUnderlyingErrorKey | constant | CoreFoundation/CFError.h | No safe wrapper or bridge entry point was detected. |
| CFFileDescriptorContext | typedef struct | CoreFoundation/CFFileDescriptor.h | cf::CFFileDescriptor is covered, but this public declaration is not exposed. |
| CFFileDescriptorCreateRunLoopSource | function | CoreFoundation/CFFileDescriptor.h | cf::CFFileDescriptor is covered, but this public declaration is not exposed. |
| CFFileDescriptorDisableCallBacks | function | CoreFoundation/CFFileDescriptor.h | cf::CFFileDescriptor is covered, but this public declaration is not exposed. |
| CFFileDescriptorEnableCallBacks | function | CoreFoundation/CFFileDescriptor.h | cf::CFFileDescriptor is covered, but this public declaration is not exposed. |
| CFFileDescriptorGetContext | function | CoreFoundation/CFFileDescriptor.h | cf::CFFileDescriptor is covered, but this public declaration is not exposed. |
| CFFileDescriptorIsValid | function | CoreFoundation/CFFileDescriptor.h | cf::CFFileDescriptor is covered, but this public declaration is not exposed. |
| CFFileSecurityClearOptions | typedef enum | CoreFoundation/CFFileSecurity.h | cf::CFFileSecurity is covered, but this public declaration is not exposed. |
| CFFileSecurityClearProperties | function | CoreFoundation/CFFileSecurity.h | cf::CFFileSecurity is covered, but this public declaration is not exposed. |
| CFFileSecurityCopyAccessControlList | function | CoreFoundation/CFFileSecurity.h | cf::CFFileSecurity is covered, but this public declaration is not exposed. |
| CFFileSecurityCopyGroupUUID | function | CoreFoundation/CFFileSecurity.h | cf::CFFileSecurity is covered, but this public declaration is not exposed. |
| CFFileSecurityCreateCopy | function | CoreFoundation/CFFileSecurity.h | cf::CFFileSecurity is covered, but this public declaration is not exposed. |
| CFFileSecurityGetGroup | function | CoreFoundation/CFFileSecurity.h | cf::CFFileSecurity is covered, but this public declaration is not exposed. |
| CFFileSecurityGetOwner | function | CoreFoundation/CFFileSecurity.h | cf::CFFileSecurity is covered, but this public declaration is not exposed. |
| CFFileSecuritySetAccessControlList | function | CoreFoundation/CFFileSecurity.h | cf::CFFileSecurity is covered, but this public declaration is not exposed. |
| CFFileSecuritySetGroup | function | CoreFoundation/CFFileSecurity.h | cf::CFFileSecurity is covered, but this public declaration is not exposed. |
| CFFileSecuritySetGroupUUID | function | CoreFoundation/CFFileSecurity.h | cf::CFFileSecurity is covered, but this public declaration is not exposed. |
| CFFileSecuritySetOwner | function | CoreFoundation/CFFileSecurity.h | cf::CFFileSecurity is covered, but this public declaration is not exposed. |
| CFLocaleCopyAvailableLocaleIdentifiers | function | CoreFoundation/CFLocale.h | cf::CFLocale is covered, but this public declaration is not exposed. |
| CFLocaleCopyCommonISOCurrencyCodes | function | CoreFoundation/CFLocale.h | cf::CFLocale is covered, but this public declaration is not exposed. |
| CFLocaleCopyDisplayNameForPropertyValue | function | CoreFoundation/CFLocale.h | cf::CFLocale is covered, but this public declaration is not exposed. |
| CFLocaleCopyISOCountryCodes | function | CoreFoundation/CFLocale.h | cf::CFLocale is covered, but this public declaration is not exposed. |
| CFLocaleCopyISOCurrencyCodes | function | CoreFoundation/CFLocale.h | cf::CFLocale is covered, but this public declaration is not exposed. |
| CFLocaleCopyISOLanguageCodes | function | CoreFoundation/CFLocale.h | cf::CFLocale is covered, but this public declaration is not exposed. |
| CFLocaleCopyPreferredLanguages | function | CoreFoundation/CFLocale.h | cf::CFLocale is covered, but this public declaration is not exposed. |
| CFLocaleCreateCanonicalLanguageIdentifierFromString | function | CoreFoundation/CFLocale.h | cf::CFLocale is covered, but this public declaration is not exposed. |
| CFLocaleCreateCanonicalLocaleIdentifierFromScriptManagerCodes | function | CoreFoundation/CFLocale.h | cf::CFLocale is covered, but this public declaration is not exposed. |
| CFLocaleCreateCanonicalLocaleIdentifierFromString | function | CoreFoundation/CFLocale.h | cf::CFLocale is covered, but this public declaration is not exposed. |
| CFLocaleCreateComponentsFromLocaleIdentifier | function | CoreFoundation/CFLocale.h | cf::CFLocale is covered, but this public declaration is not exposed. |
| CFLocaleCreateCopy | function | CoreFoundation/CFLocale.h | cf::CFLocale is covered, but this public declaration is not exposed. |
| CFLocaleCreateLocaleIdentifierFromComponents | function | CoreFoundation/CFLocale.h | cf::CFLocale is covered, but this public declaration is not exposed. |
| CFLocaleCreateLocaleIdentifierFromWindowsLocaleCode | function | CoreFoundation/CFLocale.h | cf::CFLocale is covered, but this public declaration is not exposed. |
| CFLocaleGetLanguageCharacterDirection | function | CoreFoundation/CFLocale.h | cf::CFLocale is covered, but this public declaration is not exposed. |
| CFLocaleGetLanguageLineDirection | function | CoreFoundation/CFLocale.h | cf::CFLocale is covered, but this public declaration is not exposed. |
| CFLocaleGetSystem | function | CoreFoundation/CFLocale.h | cf::CFLocale is covered, but this public declaration is not exposed. |
| CFLocaleGetValue | function | CoreFoundation/CFLocale.h | cf::CFLocale is covered, but this public declaration is not exposed. |
| CFLocaleGetWindowsLocaleCodeFromLocaleIdentifier | function | CoreFoundation/CFLocale.h | cf::CFLocale is covered, but this public declaration is not exposed. |
| CFLocaleLanguageDirection | typedef enum | CoreFoundation/CFLocale.h | cf::CFLocale is covered, but this public declaration is not exposed. |
| kCFBanglaCalendar | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFBuddhistCalendar | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFChineseCalendar | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFDangiCalendar | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFGregorianCalendar | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFGujaratiCalendar | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFHebrewCalendar | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFISO8601Calendar | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFIndianCalendar | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFIslamicCalendar | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFIslamicCivilCalendar | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFIslamicTabularCalendar | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFIslamicUmmAlQuraCalendar | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFJapaneseCalendar | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFKannadaCalendar | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFLocaleAlternateQuotationBeginDelimiterKey | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFLocaleAlternateQuotationEndDelimiterKey | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFLocaleCalendar | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFLocaleCalendarIdentifier | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFLocaleCollationIdentifier | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFLocaleCollatorIdentifier | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFLocaleCountryCode | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFLocaleCurrencyCode | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFLocaleCurrencySymbol | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFLocaleCurrentLocaleDidChangeNotification | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFLocaleDecimalSeparator | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFLocaleExemplarCharacterSet | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFLocaleGroupingSeparator | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFLocaleIdentifier | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFLocaleLanguageCode | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFLocaleMeasurementSystem | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFLocaleQuotationBeginDelimiterKey | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFLocaleQuotationEndDelimiterKey | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFLocaleScriptCode | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFLocaleUsesMetricSystem | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFLocaleVariantCode | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFMalayalamCalendar | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFMarathiCalendar | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFOdiaCalendar | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFPersianCalendar | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFRepublicOfChinaCalendar | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFTamilCalendar | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFTeluguCalendar | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFVietnameseCalendar | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| kCFVikramCalendar | constant | CoreFoundation/CFLocale.h | No safe wrapper or bridge entry point was detected. |
| CFMachPortContext | typedef struct | CoreFoundation/CFMachPort.h | No safe wrapper or bridge entry point was detected. |
| CFMachPortCreate | function | CoreFoundation/CFMachPort.h | No safe wrapper or bridge entry point was detected. |
| CFMachPortCreateRunLoopSource | function | CoreFoundation/CFMachPort.h | No safe wrapper or bridge entry point was detected. |
| CFMachPortCreateWithPort | function | CoreFoundation/CFMachPort.h | No safe wrapper or bridge entry point was detected. |
| CFMachPortGetContext | function | CoreFoundation/CFMachPort.h | No safe wrapper or bridge entry point was detected. |
| CFMachPortGetInvalidationCallBack | function | CoreFoundation/CFMachPort.h | No safe wrapper or bridge entry point was detected. |
| CFMachPortGetPort | function | CoreFoundation/CFMachPort.h | No safe wrapper or bridge entry point was detected. |
| CFMachPortGetTypeID | function | CoreFoundation/CFMachPort.h | No safe wrapper or bridge entry point was detected. |
| CFMachPortInvalidate | function | CoreFoundation/CFMachPort.h | No safe wrapper or bridge entry point was detected. |
| CFMachPortIsValid | function | CoreFoundation/CFMachPort.h | No safe wrapper or bridge entry point was detected. |
| CFMachPortRef | typedef struct | CoreFoundation/CFMachPort.h | No safe wrapper or bridge entry point was detected. |
| CFMachPortSetInvalidationCallBack | function | CoreFoundation/CFMachPort.h | No safe wrapper or bridge entry point was detected. |
| CFMessagePortContext | typedef struct | CoreFoundation/CFMessagePort.h | cf::CFMessagePort is covered, but this public declaration is not exposed. |
| CFMessagePortGetContext | function | CoreFoundation/CFMessagePort.h | cf::CFMessagePort is covered, but this public declaration is not exposed. |
| CFMessagePortGetInvalidationCallBack | function | CoreFoundation/CFMessagePort.h | cf::CFMessagePort is covered, but this public declaration is not exposed. |
| CFMessagePortGetName | function | CoreFoundation/CFMessagePort.h | cf::CFMessagePort is covered, but this public declaration is not exposed. |
| CFMessagePortIsRemote | function | CoreFoundation/CFMessagePort.h | cf::CFMessagePort is covered, but this public declaration is not exposed. |
| CFMessagePortIsValid | function | CoreFoundation/CFMessagePort.h | cf::CFMessagePort is covered, but this public declaration is not exposed. |
| CFMessagePortSetDispatchQueue | function | CoreFoundation/CFMessagePort.h | cf::CFMessagePort is covered, but this public declaration is not exposed. |
| CFMessagePortSetInvalidationCallBack | function | CoreFoundation/CFMessagePort.h | cf::CFMessagePort is covered, but this public declaration is not exposed. |
| CFMessagePortSetName | function | CoreFoundation/CFMessagePort.h | cf::CFMessagePort is covered, but this public declaration is not exposed. |
| CFNotificationCenterAddObserver | function | CoreFoundation/CFNotificationCenter.h | cf::CFNotificationCenter is covered, but this public declaration is not exposed. |
| CFNotificationCenterPostNotificationWithOptions | function | CoreFoundation/CFNotificationCenter.h | cf::CFNotificationCenter is covered, but this public declaration is not exposed. |
| CFNotificationCenterRemoveEveryObserver | function | CoreFoundation/CFNotificationCenter.h | cf::CFNotificationCenter is covered, but this public declaration is not exposed. |
| CFNotificationCenterRemoveObserver | function | CoreFoundation/CFNotificationCenter.h | cf::CFNotificationCenter is covered, but this public declaration is not exposed. |
| CFNotificationSuspensionBehavior | typedef enum | CoreFoundation/CFNotificationCenter.h | No safe wrapper or bridge entry point was detected. |
| CFBooleanGetTypeID | function | CoreFoundation/CFNumber.h | No safe wrapper or bridge entry point was detected. |
| CFBooleanGetValue | function | CoreFoundation/CFNumber.h | No safe wrapper or bridge entry point was detected. |
| CFBooleanRef | typedef struct | CoreFoundation/CFNumber.h | No safe wrapper or bridge entry point was detected. |
| CFNumberCompare | function | CoreFoundation/CFNumber.h | cf::CFNumber is covered, but this public declaration is not exposed. |
| CFNumberGetByteSize | function | CoreFoundation/CFNumber.h | cf::CFNumber is covered, but this public declaration is not exposed. |
| CFNumberGetType | function | CoreFoundation/CFNumber.h | cf::CFNumber is covered, but this public declaration is not exposed. |
| CFNumberType | typedef enum | CoreFoundation/CFNumber.h | cf::CFNumber is covered, but this public declaration is not exposed. |
| kCFBooleanFalse | constant | CoreFoundation/CFNumber.h | No safe wrapper or bridge entry point was detected. |
| kCFBooleanTrue | constant | CoreFoundation/CFNumber.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberNaN | constant | CoreFoundation/CFNumber.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberNegativeInfinity | constant | CoreFoundation/CFNumber.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberPositiveInfinity | constant | CoreFoundation/CFNumber.h | No safe wrapper or bridge entry point was detected. |
| CFNumberFormatterCopyProperty | function | CoreFoundation/CFNumberFormatter.h | cf::CFNumberFormatter is covered, but this public declaration is not exposed. |
| CFNumberFormatterCreateStringWithValue | function | CoreFoundation/CFNumberFormatter.h | cf::CFNumberFormatter is covered, but this public declaration is not exposed. |
| CFNumberFormatterGetDecimalInfoForCurrencyCode | function | CoreFoundation/CFNumberFormatter.h | cf::CFNumberFormatter is covered, but this public declaration is not exposed. |
| CFNumberFormatterGetFormat | function | CoreFoundation/CFNumberFormatter.h | cf::CFNumberFormatter is covered, but this public declaration is not exposed. |
| CFNumberFormatterGetLocale | function | CoreFoundation/CFNumberFormatter.h | cf::CFNumberFormatter is covered, but this public declaration is not exposed. |
| CFNumberFormatterGetStyle | function | CoreFoundation/CFNumberFormatter.h | cf::CFNumberFormatter is covered, but this public declaration is not exposed. |
| CFNumberFormatterGetValueFromString | function | CoreFoundation/CFNumberFormatter.h | cf::CFNumberFormatter is covered, but this public declaration is not exposed. |
| CFNumberFormatterOptionFlags | typedef enum | CoreFoundation/CFNumberFormatter.h | cf::CFNumberFormatter is covered, but this public declaration is not exposed. |
| CFNumberFormatterPadPosition | typedef enum | CoreFoundation/CFNumberFormatter.h | cf::CFNumberFormatter is covered, but this public declaration is not exposed. |
| CFNumberFormatterRoundingMode | typedef enum | CoreFoundation/CFNumberFormatter.h | cf::CFNumberFormatter is covered, but this public declaration is not exposed. |
| CFNumberFormatterSetFormat | function | CoreFoundation/CFNumberFormatter.h | cf::CFNumberFormatter is covered, but this public declaration is not exposed. |
| CFNumberFormatterSetProperty | function | CoreFoundation/CFNumberFormatter.h | cf::CFNumberFormatter is covered, but this public declaration is not exposed. |
| kCFNumberFormatterAlwaysShowDecimalSeparator | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterCurrencyCode | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterCurrencyDecimalSeparator | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterCurrencyGroupingSeparator | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterCurrencySymbol | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterDecimalSeparator | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterDefaultFormat | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterExponentSymbol | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterFormatWidth | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterGroupingSeparator | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterGroupingSize | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterInfinitySymbol | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterInternationalCurrencySymbol | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterIsLenient | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterMaxFractionDigits | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterMaxIntegerDigits | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterMaxSignificantDigits | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterMinFractionDigits | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterMinGroupingDigits | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterMinIntegerDigits | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterMinSignificantDigits | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterMinusSign | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterMultiplier | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterNaNSymbol | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterNegativePrefix | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterNegativeSuffix | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterPaddingCharacter | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterPaddingPosition | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterPerMillSymbol | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterPercentSymbol | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterPlusSign | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterPositivePrefix | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterPositiveSuffix | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterRoundingIncrement | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterRoundingMode | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterSecondaryGroupingSize | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterUseGroupingSeparator | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterUseSignificantDigits | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| kCFNumberFormatterZeroSymbol | constant | CoreFoundation/CFNumberFormatter.h | No safe wrapper or bridge entry point was detected. |
| CFPlugInAddInstanceForFactory | function | CoreFoundation/CFPlugIn.h | No safe wrapper or bridge entry point was detected. |
| CFPlugInCreate | function | CoreFoundation/CFPlugIn.h | No safe wrapper or bridge entry point was detected. |
| CFPlugInFindFactoriesForPlugInType | function | CoreFoundation/CFPlugIn.h | No safe wrapper or bridge entry point was detected. |
| CFPlugInFindFactoriesForPlugInTypeInPlugIn | function | CoreFoundation/CFPlugIn.h | No safe wrapper or bridge entry point was detected. |
| CFPlugInGetBundle | function | CoreFoundation/CFPlugIn.h | No safe wrapper or bridge entry point was detected. |
| CFPlugInGetTypeID | function | CoreFoundation/CFPlugIn.h | No safe wrapper or bridge entry point was detected. |
| CFPlugInInstanceCreate | function | CoreFoundation/CFPlugIn.h | No safe wrapper or bridge entry point was detected. |
| CFPlugInInstanceCreateWithInstanceDataSize | function | CoreFoundation/CFPlugIn.h | No safe wrapper or bridge entry point was detected. |
| CFPlugInInstanceGetFactoryName | function | CoreFoundation/CFPlugIn.h | No safe wrapper or bridge entry point was detected. |
| CFPlugInInstanceGetInstanceData | function | CoreFoundation/CFPlugIn.h | No safe wrapper or bridge entry point was detected. |
| CFPlugInInstanceGetInterfaceFunctionTable | function | CoreFoundation/CFPlugIn.h | No safe wrapper or bridge entry point was detected. |
| CFPlugInInstanceGetTypeID | function | CoreFoundation/CFPlugIn.h | No safe wrapper or bridge entry point was detected. |
| CFPlugInInstanceRef | typedef struct | CoreFoundation/CFPlugIn.h | No safe wrapper or bridge entry point was detected. |
| CFPlugInIsLoadOnDemand | function | CoreFoundation/CFPlugIn.h | No safe wrapper or bridge entry point was detected. |
| CFPlugInRegisterFactoryFunction | function | CoreFoundation/CFPlugIn.h | No safe wrapper or bridge entry point was detected. |
| CFPlugInRegisterFactoryFunctionByName | function | CoreFoundation/CFPlugIn.h | No safe wrapper or bridge entry point was detected. |
| CFPlugInRegisterPlugInType | function | CoreFoundation/CFPlugIn.h | No safe wrapper or bridge entry point was detected. |
| CFPlugInRemoveInstanceForFactory | function | CoreFoundation/CFPlugIn.h | No safe wrapper or bridge entry point was detected. |
| CFPlugInSetLoadOnDemand | function | CoreFoundation/CFPlugIn.h | No safe wrapper or bridge entry point was detected. |
| CFPlugInUnregisterFactory | function | CoreFoundation/CFPlugIn.h | No safe wrapper or bridge entry point was detected. |
| CFPlugInUnregisterPlugInType | function | CoreFoundation/CFPlugIn.h | No safe wrapper or bridge entry point was detected. |
| kCFPlugInDynamicRegisterFunctionKey | constant | CoreFoundation/CFPlugIn.h | No safe wrapper or bridge entry point was detected. |
| kCFPlugInDynamicRegistrationKey | constant | CoreFoundation/CFPlugIn.h | No safe wrapper or bridge entry point was detected. |
| kCFPlugInFactoriesKey | constant | CoreFoundation/CFPlugIn.h | No safe wrapper or bridge entry point was detected. |
| kCFPlugInTypesKey | constant | CoreFoundation/CFPlugIn.h | No safe wrapper or bridge entry point was detected. |
| kCFPlugInUnloadFunctionKey | constant | CoreFoundation/CFPlugIn.h | No safe wrapper or bridge entry point was detected. |
| CFPreferencesAddSuitePreferencesToApp | function | CoreFoundation/CFPreferences.h | cf::CFPreferences is covered, but this public declaration is not exposed. |
| CFPreferencesAppValueIsForced | function | CoreFoundation/CFPreferences.h | cf::CFPreferences is covered, but this public declaration is not exposed. |
| CFPreferencesCopyKeyList | function | CoreFoundation/CFPreferences.h | cf::CFPreferences is covered, but this public declaration is not exposed. |
| CFPreferencesCopyMultiple | function | CoreFoundation/CFPreferences.h | cf::CFPreferences is covered, but this public declaration is not exposed. |
| CFPreferencesCopyValue | function | CoreFoundation/CFPreferences.h | cf::CFPreferences is covered, but this public declaration is not exposed. |
| CFPreferencesGetAppBooleanValue | function | CoreFoundation/CFPreferences.h | cf::CFPreferences is covered, but this public declaration is not exposed. |
| CFPreferencesGetAppIntegerValue | function | CoreFoundation/CFPreferences.h | cf::CFPreferences is covered, but this public declaration is not exposed. |
| CFPreferencesRemoveSuitePreferencesFromApp | function | CoreFoundation/CFPreferences.h | cf::CFPreferences is covered, but this public declaration is not exposed. |
| CFPreferencesSetMultiple | function | CoreFoundation/CFPreferences.h | cf::CFPreferences is covered, but this public declaration is not exposed. |
| CFPreferencesSetValue | function | CoreFoundation/CFPreferences.h | cf::CFPreferences is covered, but this public declaration is not exposed. |
| CFPreferencesSynchronize | function | CoreFoundation/CFPreferences.h | cf::CFPreferences is covered, but this public declaration is not exposed. |
| kCFPreferencesAnyApplication | constant | CoreFoundation/CFPreferences.h | No safe wrapper or bridge entry point was detected. |
| kCFPreferencesAnyHost | constant | CoreFoundation/CFPreferences.h | No safe wrapper or bridge entry point was detected. |
| kCFPreferencesAnyUser | constant | CoreFoundation/CFPreferences.h | No safe wrapper or bridge entry point was detected. |
| kCFPreferencesCurrentApplication | constant | CoreFoundation/CFPreferences.h | No safe wrapper or bridge entry point was detected. |
| kCFPreferencesCurrentHost | constant | CoreFoundation/CFPreferences.h | No safe wrapper or bridge entry point was detected. |
| kCFPreferencesCurrentUser | constant | CoreFoundation/CFPreferences.h | No safe wrapper or bridge entry point was detected. |
| CFRunLoopActivity | typedef enum | CoreFoundation/CFRunLoop.h | cf::CFRunLoop is covered, but this public declaration is not exposed. |
| CFRunLoopAddCommonMode | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop is covered, but this public declaration is not exposed. |
| CFRunLoopAddObserver | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop is covered, but this public declaration is not exposed. |
| CFRunLoopContainsObserver | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop is covered, but this public declaration is not exposed. |
| CFRunLoopContainsSource | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop is covered, but this public declaration is not exposed. |
| CFRunLoopContainsTimer | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop is covered, but this public declaration is not exposed. |
| CFRunLoopCopyAllModes | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop is covered, but this public declaration is not exposed. |
| CFRunLoopCopyCurrentMode | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop is covered, but this public declaration is not exposed. |
| CFRunLoopGetNextTimerFireDate | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop is covered, but this public declaration is not exposed. |
| CFRunLoopIsWaiting | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop is covered, but this public declaration is not exposed. |
| CFRunLoopObserverContext | typedef struct | CoreFoundation/CFRunLoop.h | cf::CFRunLoop is covered, but this public declaration is not exposed. |
| CFRunLoopObserverCreate | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop is covered, but this public declaration is not exposed. |
| CFRunLoopObserverCreateWithHandler | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop is covered, but this public declaration is not exposed. |
| CFRunLoopObserverDoesRepeat | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop is covered, but this public declaration is not exposed. |
| CFRunLoopObserverGetActivities | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop is covered, but this public declaration is not exposed. |
| CFRunLoopObserverGetContext | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop is covered, but this public declaration is not exposed. |
| CFRunLoopObserverGetOrder | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop is covered, but this public declaration is not exposed. |
| CFRunLoopObserverGetTypeID | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop is covered, but this public declaration is not exposed. |
| CFRunLoopObserverInvalidate | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop is covered, but this public declaration is not exposed. |
| CFRunLoopObserverIsValid | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop is covered, but this public declaration is not exposed. |
| CFRunLoopObserverRef | typedef struct | CoreFoundation/CFRunLoop.h | cf::CFRunLoop is covered, but this public declaration is not exposed. |
| CFRunLoopPerformBlock | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop is covered, but this public declaration is not exposed. |
| CFRunLoopRemoveObserver | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop is covered, but this public declaration is not exposed. |
| CFRunLoopRemoveSource | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop is covered, but this public declaration is not exposed. |
| CFRunLoopRemoveTimer | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop is covered, but this public declaration is not exposed. |
| CFRunLoopRun | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop is covered, but this public declaration is not exposed. |
| CFRunLoopSourceContext | typedef struct | CoreFoundation/CFRunLoop.h | cf::CFRunLoop is covered, but this public declaration is not exposed. |
| CFRunLoopSourceContext1 | typedef struct | CoreFoundation/CFRunLoop.h | cf::CFRunLoop is covered, but this public declaration is not exposed. |
| CFRunLoopSourceCreate | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop is covered, but this public declaration is not exposed. |
| CFRunLoopSourceGetContext | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop is covered, but this public declaration is not exposed. |
| CFRunLoopSourceGetOrder | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop is covered, but this public declaration is not exposed. |
| CFRunLoopSourceGetTypeID | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop is covered, but this public declaration is not exposed. |
| CFRunLoopSourceInvalidate | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop is covered, but this public declaration is not exposed. |
| CFRunLoopSourceIsValid | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop is covered, but this public declaration is not exposed. |
| CFRunLoopSourceRef | typedef struct | CoreFoundation/CFRunLoop.h | cf::CFRunLoop is covered, but this public declaration is not exposed. |
| CFRunLoopSourceSignal | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop is covered, but this public declaration is not exposed. |
| CFRunLoopTimerContext | typedef struct | CoreFoundation/CFRunLoop.h | cf::CFTimer is covered, but this public declaration is not exposed. |
| CFRunLoopTimerDoesRepeat | function | CoreFoundation/CFRunLoop.h | cf::CFTimer is covered, but this public declaration is not exposed. |
| CFRunLoopTimerGetContext | function | CoreFoundation/CFRunLoop.h | cf::CFTimer is covered, but this public declaration is not exposed. |
| CFRunLoopTimerGetInterval | function | CoreFoundation/CFRunLoop.h | cf::CFTimer is covered, but this public declaration is not exposed. |
| CFRunLoopTimerGetNextFireDate | function | CoreFoundation/CFRunLoop.h | cf::CFTimer is covered, but this public declaration is not exposed. |
| CFRunLoopTimerGetOrder | function | CoreFoundation/CFRunLoop.h | cf::CFTimer is covered, but this public declaration is not exposed. |
| CFRunLoopTimerGetTolerance | function | CoreFoundation/CFRunLoop.h | cf::CFTimer is covered, but this public declaration is not exposed. |
| CFRunLoopTimerSetTolerance | function | CoreFoundation/CFRunLoop.h | cf::CFTimer is covered, but this public declaration is not exposed. |
| kCFRunLoopCommonModes | constant | CoreFoundation/CFRunLoop.h | No safe wrapper or bridge entry point was detected. |
| kCFRunLoopDefaultMode | constant | CoreFoundation/CFRunLoop.h | No safe wrapper or bridge entry point was detected. |
| CFSocketCallBackType | typedef enum | CoreFoundation/CFSocket.h | cf::CFSocket is covered, but this public declaration is not exposed. |
| CFSocketConnectToAddress | function | CoreFoundation/CFSocket.h | cf::CFSocket is covered, but this public declaration is not exposed. |
| CFSocketContext | typedef struct | CoreFoundation/CFSocket.h | cf::CFSocket is covered, but this public declaration is not exposed. |
| CFSocketCopyAddress | function | CoreFoundation/CFSocket.h | cf::CFSocket is covered, but this public declaration is not exposed. |
| CFSocketCopyPeerAddress | function | CoreFoundation/CFSocket.h | cf::CFSocket is covered, but this public declaration is not exposed. |
| CFSocketCopyRegisteredSocketSignature | function | CoreFoundation/CFSocket.h | cf::CFSocket is covered, but this public declaration is not exposed. |
| CFSocketCopyRegisteredValue | function | CoreFoundation/CFSocket.h | cf::CFSocket is covered, but this public declaration is not exposed. |
| CFSocketCreateConnectedToSocketSignature | function | CoreFoundation/CFSocket.h | cf::CFSocket is covered, but this public declaration is not exposed. |
| CFSocketCreateRunLoopSource | function | CoreFoundation/CFSocket.h | cf::CFSocket is covered, but this public declaration is not exposed. |
| CFSocketCreateWithNative | function | CoreFoundation/CFSocket.h | cf::CFSocket is covered, but this public declaration is not exposed. |
| CFSocketCreateWithSocketSignature | function | CoreFoundation/CFSocket.h | cf::CFSocket is covered, but this public declaration is not exposed. |
| CFSocketDisableCallBacks | function | CoreFoundation/CFSocket.h | cf::CFSocket is covered, but this public declaration is not exposed. |
| CFSocketEnableCallBacks | function | CoreFoundation/CFSocket.h | cf::CFSocket is covered, but this public declaration is not exposed. |
| CFSocketError | typedef enum | CoreFoundation/CFSocket.h | cf::CFSocket is covered, but this public declaration is not exposed. |
| CFSocketGetContext | function | CoreFoundation/CFSocket.h | cf::CFSocket is covered, but this public declaration is not exposed. |
| CFSocketGetDefaultNameRegistryPortNumber | function | CoreFoundation/CFSocket.h | cf::CFSocket is covered, but this public declaration is not exposed. |
| CFSocketGetSocketFlags | function | CoreFoundation/CFSocket.h | cf::CFSocket is covered, but this public declaration is not exposed. |
| CFSocketRegisterSocketSignature | function | CoreFoundation/CFSocket.h | cf::CFSocket is covered, but this public declaration is not exposed. |
| CFSocketRegisterValue | function | CoreFoundation/CFSocket.h | cf::CFSocket is covered, but this public declaration is not exposed. |
| CFSocketSendData | function | CoreFoundation/CFSocket.h | cf::CFSocket is covered, but this public declaration is not exposed. |
| CFSocketSetAddress | function | CoreFoundation/CFSocket.h | cf::CFSocket is covered, but this public declaration is not exposed. |
| CFSocketSetDefaultNameRegistryPortNumber | function | CoreFoundation/CFSocket.h | cf::CFSocket is covered, but this public declaration is not exposed. |
| CFSocketSetSocketFlags | function | CoreFoundation/CFSocket.h | cf::CFSocket is covered, but this public declaration is not exposed. |
| CFSocketSignature | typedef struct | CoreFoundation/CFSocket.h | cf::CFSocket is covered, but this public declaration is not exposed. |
| CFSocketUnregister | function | CoreFoundation/CFSocket.h | cf::CFSocket is covered, but this public declaration is not exposed. |
| kCFSocketCommandKey | constant | CoreFoundation/CFSocket.h | No safe wrapper or bridge entry point was detected. |
| kCFSocketErrorKey | constant | CoreFoundation/CFSocket.h | No safe wrapper or bridge entry point was detected. |
| kCFSocketNameKey | constant | CoreFoundation/CFSocket.h | No safe wrapper or bridge entry point was detected. |
| kCFSocketRegisterCommand | constant | CoreFoundation/CFSocket.h | No safe wrapper or bridge entry point was detected. |
| kCFSocketResultKey | constant | CoreFoundation/CFSocket.h | No safe wrapper or bridge entry point was detected. |
| kCFSocketRetrieveCommand | constant | CoreFoundation/CFSocket.h | No safe wrapper or bridge entry point was detected. |
| kCFSocketValueKey | constant | CoreFoundation/CFSocket.h | No safe wrapper or bridge entry point was detected. |
| CFReadStreamCopyDispatchQueue | function | CoreFoundation/CFStream.h | cf::CFStreamPair is covered, but this public declaration is not exposed. |
| CFReadStreamCopyError | function | CoreFoundation/CFStream.h | cf::CFStreamPair is covered, but this public declaration is not exposed. |
| CFReadStreamCopyProperty | function | CoreFoundation/CFStream.h | cf::CFStreamPair is covered, but this public declaration is not exposed. |
| CFReadStreamCreateWithBytesNoCopy | function | CoreFoundation/CFStream.h | cf::CFStreamPair is covered, but this public declaration is not exposed. |
| CFReadStreamCreateWithFile | function | CoreFoundation/CFStream.h | cf::CFStreamPair is covered, but this public declaration is not exposed. |
| CFReadStreamGetBuffer | function | CoreFoundation/CFStream.h | cf::CFStreamPair is covered, but this public declaration is not exposed. |
| CFReadStreamGetError | function | CoreFoundation/CFStream.h | cf::CFStreamPair is covered, but this public declaration is not exposed. |
| CFReadStreamGetStatus | function | CoreFoundation/CFStream.h | cf::CFStreamPair is covered, but this public declaration is not exposed. |
| CFReadStreamHasBytesAvailable | function | CoreFoundation/CFStream.h | cf::CFStreamPair is covered, but this public declaration is not exposed. |
| CFReadStreamScheduleWithRunLoop | function | CoreFoundation/CFStream.h | cf::CFStreamPair is covered, but this public declaration is not exposed. |
| CFReadStreamSetClient | function | CoreFoundation/CFStream.h | cf::CFStreamPair is covered, but this public declaration is not exposed. |
| CFReadStreamSetDispatchQueue | function | CoreFoundation/CFStream.h | cf::CFStreamPair is covered, but this public declaration is not exposed. |
| CFReadStreamSetProperty | function | CoreFoundation/CFStream.h | cf::CFStreamPair is covered, but this public declaration is not exposed. |
| CFReadStreamUnscheduleFromRunLoop | function | CoreFoundation/CFStream.h | cf::CFStreamPair is covered, but this public declaration is not exposed. |
| CFStreamClientContext | typedef struct | CoreFoundation/CFStream.h | cf::CFStreamPair is covered, but this public declaration is not exposed. |
| CFStreamError | typedef struct | CoreFoundation/CFStream.h | cf::CFStreamPair is covered, but this public declaration is not exposed. |
| CFStreamErrorDomain | typedef enum | CoreFoundation/CFStream.h | cf::CFStreamPair is covered, but this public declaration is not exposed. |
| CFStreamEventType | typedef enum | CoreFoundation/CFStream.h | cf::CFStreamPair is covered, but this public declaration is not exposed. |
| CFStreamStatus | typedef enum | CoreFoundation/CFStream.h | cf::CFStreamPair is covered, but this public declaration is not exposed. |
| CFWriteStreamCanAcceptBytes | function | CoreFoundation/CFStream.h | cf::CFStreamPair is covered, but this public declaration is not exposed. |
| CFWriteStreamCopyDispatchQueue | function | CoreFoundation/CFStream.h | cf::CFStreamPair is covered, but this public declaration is not exposed. |
| CFWriteStreamCopyError | function | CoreFoundation/CFStream.h | cf::CFStreamPair is covered, but this public declaration is not exposed. |
| CFWriteStreamCopyProperty | function | CoreFoundation/CFStream.h | cf::CFStreamPair is covered, but this public declaration is not exposed. |
| CFWriteStreamCreateWithAllocatedBuffers | function | CoreFoundation/CFStream.h | cf::CFStreamPair is covered, but this public declaration is not exposed. |
| CFWriteStreamCreateWithBuffer | function | CoreFoundation/CFStream.h | cf::CFStreamPair is covered, but this public declaration is not exposed. |
| CFWriteStreamCreateWithFile | function | CoreFoundation/CFStream.h | cf::CFStreamPair is covered, but this public declaration is not exposed. |
| CFWriteStreamGetError | function | CoreFoundation/CFStream.h | cf::CFStreamPair is covered, but this public declaration is not exposed. |
| CFWriteStreamGetStatus | function | CoreFoundation/CFStream.h | cf::CFStreamPair is covered, but this public declaration is not exposed. |
| CFWriteStreamScheduleWithRunLoop | function | CoreFoundation/CFStream.h | cf::CFStreamPair is covered, but this public declaration is not exposed. |
| CFWriteStreamSetClient | function | CoreFoundation/CFStream.h | cf::CFStreamPair is covered, but this public declaration is not exposed. |
| CFWriteStreamSetDispatchQueue | function | CoreFoundation/CFStream.h | cf::CFStreamPair is covered, but this public declaration is not exposed. |
| CFWriteStreamSetProperty | function | CoreFoundation/CFStream.h | cf::CFStreamPair is covered, but this public declaration is not exposed. |
| CFWriteStreamUnscheduleFromRunLoop | function | CoreFoundation/CFStream.h | cf::CFStreamPair is covered, but this public declaration is not exposed. |
| kCFStreamErrorDomainSOCKS | constant | CoreFoundation/CFStream.h | No safe wrapper or bridge entry point was detected. |
| kCFStreamErrorDomainSSL | constant | CoreFoundation/CFStream.h | No safe wrapper or bridge entry point was detected. |
| kCFStreamPropertyAppendToFile | constant | CoreFoundation/CFStream.h | No safe wrapper or bridge entry point was detected. |
| kCFStreamPropertyDataWritten | constant | CoreFoundation/CFStream.h | No safe wrapper or bridge entry point was detected. |
| kCFStreamPropertyFileCurrentOffset | constant | CoreFoundation/CFStream.h | No safe wrapper or bridge entry point was detected. |
| kCFStreamPropertySOCKSPassword | constant | CoreFoundation/CFStream.h | No safe wrapper or bridge entry point was detected. |
| kCFStreamPropertySOCKSProxy | constant | CoreFoundation/CFStream.h | No safe wrapper or bridge entry point was detected. |
| kCFStreamPropertySOCKSProxyHost | constant | CoreFoundation/CFStream.h | No safe wrapper or bridge entry point was detected. |
| kCFStreamPropertySOCKSProxyPort | constant | CoreFoundation/CFStream.h | No safe wrapper or bridge entry point was detected. |
| kCFStreamPropertySOCKSUser | constant | CoreFoundation/CFStream.h | No safe wrapper or bridge entry point was detected. |
| kCFStreamPropertySOCKSVersion | constant | CoreFoundation/CFStream.h | No safe wrapper or bridge entry point was detected. |
| kCFStreamPropertyShouldCloseNativeSocket | constant | CoreFoundation/CFStream.h | No safe wrapper or bridge entry point was detected. |
| kCFStreamPropertySocketNativeHandle | constant | CoreFoundation/CFStream.h | No safe wrapper or bridge entry point was detected. |
| kCFStreamPropertySocketRemoteHostName | constant | CoreFoundation/CFStream.h | No safe wrapper or bridge entry point was detected. |
| kCFStreamPropertySocketRemotePortNumber | constant | CoreFoundation/CFStream.h | No safe wrapper or bridge entry point was detected. |
| kCFStreamPropertySocketSecurityLevel | constant | CoreFoundation/CFStream.h | No safe wrapper or bridge entry point was detected. |
| kCFStreamSocketSOCKSVersion4 | constant | CoreFoundation/CFStream.h | No safe wrapper or bridge entry point was detected. |
| kCFStreamSocketSOCKSVersion5 | constant | CoreFoundation/CFStream.h | No safe wrapper or bridge entry point was detected. |
| kCFStreamSocketSecurityLevelNegotiatedSSL | constant | CoreFoundation/CFStream.h | No safe wrapper or bridge entry point was detected. |
| kCFStreamSocketSecurityLevelNone | constant | CoreFoundation/CFStream.h | No safe wrapper or bridge entry point was detected. |
| kCFStreamSocketSecurityLevelTLSv1 | constant | CoreFoundation/CFStream.h | No safe wrapper or bridge entry point was detected. |
| CFShow | function | CoreFoundation/CFString.h | No safe wrapper or bridge entry point was detected. |
| CFShowStr | function | CoreFoundation/CFString.h | No safe wrapper or bridge entry point was detected. |
| CFStringAppend | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringAppendCString | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringAppendCharacters | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringAppendFormat | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringAppendFormatAndArguments | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringAppendPascalString | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringBuiltInEncodings | typedef enum | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringCapitalize | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringCompare | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringCompareFlags | typedef enum | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringCompareWithOptions | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringCompareWithOptionsAndLocale | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringConvertEncodingToIANACharSetName | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringConvertEncodingToNSStringEncoding | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringConvertEncodingToWindowsCodepage | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringConvertIANACharSetNameToEncoding | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringConvertNSStringEncodingToEncoding | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringConvertWindowsCodepageToEncoding | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringCreateArrayBySeparatingStrings | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringCreateArrayWithFindResults | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringCreateByCombiningStrings | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringCreateCopy | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringCreateExternalRepresentation | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringCreateFromExternalRepresentation | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringCreateMutable | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringCreateMutableCopy | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringCreateMutableWithExternalCharactersNoCopy | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringCreateStringWithValidatedFormat | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringCreateStringWithValidatedFormatAndArguments | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringCreateWithBytes | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringCreateWithBytesNoCopy | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringCreateWithCStringNoCopy | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringCreateWithCharacters | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringCreateWithCharactersNoCopy | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringCreateWithFileSystemRepresentation | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringCreateWithFormat | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringCreateWithFormatAndArguments | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringCreateWithPascalString | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringCreateWithPascalStringNoCopy | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringCreateWithSubstring | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringDelete | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringFind | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringFindAndReplace | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringFindCharacterFromSet | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringFindWithOptions | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringFindWithOptionsAndLocale | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringFold | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringGetBytes | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringGetCStringPtr | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringGetCharacterAtIndex | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringGetCharacterFromInlineBuffer | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringGetCharacters | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringGetCharactersPtr | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringGetDoubleValue | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringGetFastestEncoding | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringGetFileSystemRepresentation | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringGetHyphenationLocationBeforeIndex | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringGetIntValue | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringGetLineBounds | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringGetListOfAvailableEncodings | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringGetLongCharacterForSurrogatePair | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringGetMaximumSizeOfFileSystemRepresentation | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringGetMostCompatibleMacStringEncoding | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringGetNameOfEncoding | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringGetParagraphBounds | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringGetPascalString | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringGetPascalStringPtr | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringGetRangeOfComposedCharactersAtIndex | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringGetSmallestEncoding | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringGetSurrogatePairForLongCharacter | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringGetSystemEncoding | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringHasPrefix | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringHasSuffix | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringInitInlineBuffer | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringInlineBuffer | typedef struct | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringInsert | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringIsEncodingAvailable | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringIsHyphenationAvailableForLocale | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringIsSurrogateHighCharacter | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringIsSurrogateLowCharacter | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringLowercase | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringNormalizationForm | typedef enum | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringNormalize | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringPad | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringReplace | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringReplaceAll | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringSetExternalCharactersNoCopy | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringTransform | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringTrim | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringTrimWhitespace | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringUppercase | function | CoreFoundation/CFString.h | cf::CFString is covered, but this public declaration is not exposed. |
| kCFStringTransformFullwidthHalfwidth | constant | CoreFoundation/CFString.h | No safe wrapper or bridge entry point was detected. |
| kCFStringTransformHiraganaKatakana | constant | CoreFoundation/CFString.h | No safe wrapper or bridge entry point was detected. |
| kCFStringTransformLatinArabic | constant | CoreFoundation/CFString.h | No safe wrapper or bridge entry point was detected. |
| kCFStringTransformLatinCyrillic | constant | CoreFoundation/CFString.h | No safe wrapper or bridge entry point was detected. |
| kCFStringTransformLatinGreek | constant | CoreFoundation/CFString.h | No safe wrapper or bridge entry point was detected. |
| kCFStringTransformLatinHangul | constant | CoreFoundation/CFString.h | No safe wrapper or bridge entry point was detected. |
| kCFStringTransformLatinHebrew | constant | CoreFoundation/CFString.h | No safe wrapper or bridge entry point was detected. |
| kCFStringTransformLatinHiragana | constant | CoreFoundation/CFString.h | No safe wrapper or bridge entry point was detected. |
| kCFStringTransformLatinKatakana | constant | CoreFoundation/CFString.h | No safe wrapper or bridge entry point was detected. |
| kCFStringTransformLatinThai | constant | CoreFoundation/CFString.h | No safe wrapper or bridge entry point was detected. |
| kCFStringTransformMandarinLatin | constant | CoreFoundation/CFString.h | No safe wrapper or bridge entry point was detected. |
| kCFStringTransformStripCombiningMarks | constant | CoreFoundation/CFString.h | No safe wrapper or bridge entry point was detected. |
| kCFStringTransformStripDiacritics | constant | CoreFoundation/CFString.h | No safe wrapper or bridge entry point was detected. |
| kCFStringTransformToLatin | constant | CoreFoundation/CFString.h | No safe wrapper or bridge entry point was detected. |
| kCFStringTransformToUnicodeName | constant | CoreFoundation/CFString.h | No safe wrapper or bridge entry point was detected. |
| kCFStringTransformToXMLHex | constant | CoreFoundation/CFString.h | No safe wrapper or bridge entry point was detected. |
| CFStringEncodings | typedef enum | CoreFoundation/CFStringEncodingExt.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringTokenizerAdvanceToNextToken | function | CoreFoundation/CFStringTokenizer.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringTokenizerCopyBestStringLanguage | function | CoreFoundation/CFStringTokenizer.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringTokenizerCopyCurrentTokenAttribute | function | CoreFoundation/CFStringTokenizer.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringTokenizerCreate | function | CoreFoundation/CFStringTokenizer.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringTokenizerGetCurrentSubTokens | function | CoreFoundation/CFStringTokenizer.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringTokenizerGetCurrentTokenRange | function | CoreFoundation/CFStringTokenizer.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringTokenizerGetTypeID | function | CoreFoundation/CFStringTokenizer.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringTokenizerGoToTokenAtIndex | function | CoreFoundation/CFStringTokenizer.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringTokenizerRef | typedef struct | CoreFoundation/CFStringTokenizer.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringTokenizerSetString | function | CoreFoundation/CFStringTokenizer.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFStringTokenizerTokenType | typedef enum | CoreFoundation/CFStringTokenizer.h | cf::CFString is covered, but this public declaration is not exposed. |
| CFTimeZoneCopyAbbreviation | function | CoreFoundation/CFTimeZone.h | cf::CFTimeZone is covered, but this public declaration is not exposed. |
| CFTimeZoneCopyAbbreviationDictionary | function | CoreFoundation/CFTimeZone.h | cf::CFTimeZone is covered, but this public declaration is not exposed. |
| CFTimeZoneCopyDefault | function | CoreFoundation/CFTimeZone.h | cf::CFTimeZone is covered, but this public declaration is not exposed. |
| CFTimeZoneCopyKnownNames | function | CoreFoundation/CFTimeZone.h | cf::CFTimeZone is covered, but this public declaration is not exposed. |
| CFTimeZoneCopyLocalizedName | function | CoreFoundation/CFTimeZone.h | cf::CFTimeZone is covered, but this public declaration is not exposed. |
| CFTimeZoneCreate | function | CoreFoundation/CFTimeZone.h | cf::CFTimeZone is covered, but this public declaration is not exposed. |
| CFTimeZoneCreateWithTimeIntervalFromGMT | function | CoreFoundation/CFTimeZone.h | cf::CFTimeZone is covered, but this public declaration is not exposed. |
| CFTimeZoneGetData | function | CoreFoundation/CFTimeZone.h | cf::CFTimeZone is covered, but this public declaration is not exposed. |
| CFTimeZoneGetDaylightSavingTimeOffset | function | CoreFoundation/CFTimeZone.h | cf::CFTimeZone is covered, but this public declaration is not exposed. |
| CFTimeZoneGetNextDaylightSavingTimeTransition | function | CoreFoundation/CFTimeZone.h | cf::CFTimeZone is covered, but this public declaration is not exposed. |
| CFTimeZoneIsDaylightSavingTime | function | CoreFoundation/CFTimeZone.h | cf::CFTimeZone is covered, but this public declaration is not exposed. |
| CFTimeZoneNameStyle | typedef enum | CoreFoundation/CFTimeZone.h | cf::CFTimeZone is covered, but this public declaration is not exposed. |
| CFTimeZoneResetSystem | function | CoreFoundation/CFTimeZone.h | cf::CFTimeZone is covered, but this public declaration is not exposed. |
| CFTimeZoneSetAbbreviationDictionary | function | CoreFoundation/CFTimeZone.h | cf::CFTimeZone is covered, but this public declaration is not exposed. |
| CFTimeZoneSetDefault | function | CoreFoundation/CFTimeZone.h | cf::CFTimeZone is covered, but this public declaration is not exposed. |
| kCFTimeZoneSystemTimeZoneDidChangeNotification | constant | CoreFoundation/CFTimeZone.h | No safe wrapper or bridge entry point was detected. |
| CFTreeAppendChild | function | CoreFoundation/CFTree.h | cf::CFTree is covered, but this public declaration is not exposed. |
| CFTreeApplyFunctionToChildren | function | CoreFoundation/CFTree.h | cf::CFTree is covered, but this public declaration is not exposed. |
| CFTreeContext | typedef struct | CoreFoundation/CFTree.h | cf::CFTree is covered, but this public declaration is not exposed. |
| CFTreeCreate | function | CoreFoundation/CFTree.h | cf::CFTree is covered, but this public declaration is not exposed. |
| CFTreeFindRoot | function | CoreFoundation/CFTree.h | cf::CFTree is covered, but this public declaration is not exposed. |
| CFTreeGetChildAtIndex | function | CoreFoundation/CFTree.h | cf::CFTree is covered, but this public declaration is not exposed. |
| CFTreeGetChildCount | function | CoreFoundation/CFTree.h | cf::CFTree is covered, but this public declaration is not exposed. |
| CFTreeGetChildren | function | CoreFoundation/CFTree.h | cf::CFTree is covered, but this public declaration is not exposed. |
| CFTreeGetContext | function | CoreFoundation/CFTree.h | cf::CFTree is covered, but this public declaration is not exposed. |
| CFTreeGetFirstChild | function | CoreFoundation/CFTree.h | cf::CFTree is covered, but this public declaration is not exposed. |
| CFTreeGetNextSibling | function | CoreFoundation/CFTree.h | cf::CFTree is covered, but this public declaration is not exposed. |
| CFTreeGetParent | function | CoreFoundation/CFTree.h | cf::CFTree is covered, but this public declaration is not exposed. |
| CFTreeGetTypeID | function | CoreFoundation/CFTree.h | cf::CFTree is covered, but this public declaration is not exposed. |
| CFTreeInsertSibling | function | CoreFoundation/CFTree.h | cf::CFTree is covered, but this public declaration is not exposed. |
| CFTreePrependChild | function | CoreFoundation/CFTree.h | cf::CFTree is covered, but this public declaration is not exposed. |
| CFTreeRef | typedef struct | CoreFoundation/CFTree.h | cf::CFTree is covered, but this public declaration is not exposed. |
| CFTreeRemove | function | CoreFoundation/CFTree.h | cf::CFTree is covered, but this public declaration is not exposed. |
| CFTreeRemoveAllChildren | function | CoreFoundation/CFTree.h | cf::CFTree is covered, but this public declaration is not exposed. |
| CFTreeSetContext | function | CoreFoundation/CFTree.h | cf::CFTree is covered, but this public declaration is not exposed. |
| CFTreeSortChildren | function | CoreFoundation/CFTree.h | cf::CFTree is covered, but this public declaration is not exposed. |
| CFURLBookmarkCreationOptions | typedef enum | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLBookmarkResolutionOptions | typedef enum | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLCanBeDecomposed | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLClearResourcePropertyCache | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLClearResourcePropertyCacheForKey | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLComponentType | typedef enum | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLCopyAbsoluteURL | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLCopyFragment | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLCopyHostName | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLCopyLastPathComponent | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLCopyNetLocation | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLCopyPassword | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLCopyPath | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLCopyPathExtension | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLCopyResourcePropertiesForKeys | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLCopyResourcePropertyForKey | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLCopyResourceSpecifier | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLCopyScheme | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLCopyStrictPath | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLCopyUserName | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLCreateAbsoluteURLWithBytes | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLCreateBookmarkData | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLCreateBookmarkDataFromFile | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLCreateByResolvingBookmarkData | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLCreateCopyAppendingPathComponent | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLCreateCopyAppendingPathExtension | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLCreateCopyDeletingLastPathComponent | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLCreateCopyDeletingPathExtension | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLCreateData | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLCreateFilePathURL | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLCreateFileReferenceURL | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLCreateFromFileSystemRepresentation | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLCreateFromFileSystemRepresentationRelativeToBase | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLCreateResourcePropertiesForKeysFromBookmarkData | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLCreateResourcePropertyForKeyFromBookmarkData | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLCreateStringByReplacingPercentEscapes | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLCreateWithBytes | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLCreateWithFileSystemPathRelativeToBase | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLGetBaseURL | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLGetByteRangeForComponent | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLGetBytes | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLGetFileSystemRepresentation | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLGetPortNumber | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLIsFileReferenceURL | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLResourceIsReachable | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLSetResourcePropertiesForKeys | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLSetResourcePropertyForKey | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLSetTemporaryResourcePropertyForKey | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLStartAccessingSecurityScopedResource | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLStopAccessingSecurityScopedResource | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLWriteBookmarkDataToFile | function | CoreFoundation/CFURL.h | cf::CFURL is covered, but this public declaration is not exposed. |
| kCFURLAddedToDirectoryDateKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLApplicationIsScriptableKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLAttributeModificationDateKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLCanonicalPathKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLContentAccessDateKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLContentModificationDateKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLCreationDateKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLDirectoryEntryCountKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLDocumentIdentifierKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLFileAllocatedSizeKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLFileContentIdentifierKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLFileIdentifierKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLFileResourceIdentifierKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLFileResourceTypeBlockSpecial | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLFileResourceTypeCharacterSpecial | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLFileResourceTypeDirectory | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLFileResourceTypeKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLFileResourceTypeNamedPipe | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLFileResourceTypeRegular | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLFileResourceTypeSocket | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLFileResourceTypeSymbolicLink | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLFileResourceTypeUnknown | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLFileSecurityKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLFileSizeKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLGenerationIdentifierKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLHasHiddenExtensionKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLIsAliasFileKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLIsApplicationKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLIsDirectoryKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLIsExcludedFromBackupKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLIsExecutableKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLIsHiddenKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLIsMountTriggerKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLIsPackageKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLIsPurgeableKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLIsReadableKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLIsRegularFileKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLIsSparseKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLIsSymbolicLinkKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLIsSystemImmutableKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLIsUbiquitousItemKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLIsUserImmutableKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLIsVolumeKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLIsWritableKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLKeysOfUnsetValuesKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLLabelNumberKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLLinkCountKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLLocalizedLabelKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLLocalizedNameKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLLocalizedTypeDescriptionKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLMayHaveExtendedAttributesKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLMayShareFileContentKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLNameKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLParentDirectoryURLKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLPathKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLPreferredIOBlockSizeKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLQuarantinePropertiesKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLTagNamesKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLTotalFileAllocatedSizeKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLTotalFileSizeKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLUbiquitousItemDownloadingErrorKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLUbiquitousItemDownloadingStatusCurrent | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLUbiquitousItemDownloadingStatusDownloaded | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLUbiquitousItemDownloadingStatusNotDownloaded | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLUbiquitousItemHasUnresolvedConflictsKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLUbiquitousItemIsDownloadingKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLUbiquitousItemIsExcludedFromSyncKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLUbiquitousItemIsSyncPausedKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLUbiquitousItemIsUploadedKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLUbiquitousItemIsUploadingKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLUbiquitousItemSupportedSyncControlsKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLUbiquitousItemUploadingErrorKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeAvailableCapacityForImportantUsageKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeAvailableCapacityForOpportunisticUsageKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeAvailableCapacityKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeCreationDateKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeIdentifierKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeIsAutomountedKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeIsBrowsableKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeIsEjectableKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeIsEncryptedKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeIsInternalKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeIsJournalingKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeIsLocalKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeIsReadOnlyKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeIsRemovableKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeIsRootFileSystemKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeLocalizedFormatDescriptionKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeLocalizedNameKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeMaximumFileSizeKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeMountFromLocationKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeNameKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeResourceCountKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeSubtypeKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeSupportsAccessPermissionsKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeSupportsAdvisoryFileLockingKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeSupportsCasePreservedNamesKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeSupportsCaseSensitiveNamesKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeSupportsCompressionKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeSupportsExclusiveRenamingKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeSupportsExtendedSecurityKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeSupportsFileCloningKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeSupportsFileProtectionKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeSupportsHardLinksKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeSupportsImmutableFilesKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeSupportsJournalingKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeSupportsPersistentIDsKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeSupportsRenamingKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeSupportsRootDirectoryDatesKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeSupportsSparseFilesKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeSupportsSwapRenamingKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeSupportsSymbolicLinksKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeSupportsVolumeSizesKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeSupportsZeroRunsKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeTotalCapacityKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeTypeNameKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeURLForRemountingKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeURLKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| kCFURLVolumeUUIDStringKey | constant | CoreFoundation/CFURL.h | No safe wrapper or bridge entry point was detected. |
| CFURLError | typedef enum | CoreFoundation/CFURLAccess.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLEnumeratorCreateForDirectoryURL | function | CoreFoundation/CFURLEnumerator.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLEnumeratorCreateForMountedVolumes | function | CoreFoundation/CFURLEnumerator.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLEnumeratorGetDescendentLevel | function | CoreFoundation/CFURLEnumerator.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLEnumeratorGetNextURL | function | CoreFoundation/CFURLEnumerator.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLEnumeratorGetTypeID | function | CoreFoundation/CFURLEnumerator.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLEnumeratorOptions | typedef enum | CoreFoundation/CFURLEnumerator.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLEnumeratorRef | typedef struct | CoreFoundation/CFURLEnumerator.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLEnumeratorResult | typedef enum | CoreFoundation/CFURLEnumerator.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFURLEnumeratorSkipDescendents | function | CoreFoundation/CFURLEnumerator.h | cf::CFURL is covered, but this public declaration is not exposed. |
| CFUUIDBytes | typedef struct | CoreFoundation/CFUUID.h | cf::CFUUID is covered, but this public declaration is not exposed. |
| CFUUIDCreateFromUUIDBytes | function | CoreFoundation/CFUUID.h | cf::CFUUID is covered, but this public declaration is not exposed. |
| CFUUIDCreateWithBytes | function | CoreFoundation/CFUUID.h | cf::CFUUID is covered, but this public declaration is not exposed. |
| CFUUIDGetConstantUUIDWithBytes | function | CoreFoundation/CFUUID.h | cf::CFUUID is covered, but this public declaration is not exposed. |
| CFUserNotificationCancel | function | CoreFoundation/CFUserNotification.h | No safe wrapper or bridge entry point was detected. |
| CFUserNotificationCheckBoxChecked | function | CoreFoundation/CFUserNotification.h | No safe wrapper or bridge entry point was detected. |
| CFUserNotificationCreate | function | CoreFoundation/CFUserNotification.h | No safe wrapper or bridge entry point was detected. |
| CFUserNotificationCreateRunLoopSource | function | CoreFoundation/CFUserNotification.h | No safe wrapper or bridge entry point was detected. |
| CFUserNotificationDisplayAlert | function | CoreFoundation/CFUserNotification.h | No safe wrapper or bridge entry point was detected. |
| CFUserNotificationDisplayNotice | function | CoreFoundation/CFUserNotification.h | No safe wrapper or bridge entry point was detected. |
| CFUserNotificationGetResponseDictionary | function | CoreFoundation/CFUserNotification.h | No safe wrapper or bridge entry point was detected. |
| CFUserNotificationGetResponseValue | function | CoreFoundation/CFUserNotification.h | No safe wrapper or bridge entry point was detected. |
| CFUserNotificationGetTypeID | function | CoreFoundation/CFUserNotification.h | No safe wrapper or bridge entry point was detected. |
| CFUserNotificationPopUpSelection | function | CoreFoundation/CFUserNotification.h | No safe wrapper or bridge entry point was detected. |
| CFUserNotificationReceiveResponse | function | CoreFoundation/CFUserNotification.h | No safe wrapper or bridge entry point was detected. |
| CFUserNotificationRef | typedef struct | CoreFoundation/CFUserNotification.h | No safe wrapper or bridge entry point was detected. |
| CFUserNotificationSecureTextField | function | CoreFoundation/CFUserNotification.h | No safe wrapper or bridge entry point was detected. |
| CFUserNotificationUpdate | function | CoreFoundation/CFUserNotification.h | No safe wrapper or bridge entry point was detected. |
| kCFUserNotificationAlertHeaderKey | constant | CoreFoundation/CFUserNotification.h | No safe wrapper or bridge entry point was detected. |
| kCFUserNotificationAlertMessageKey | constant | CoreFoundation/CFUserNotification.h | No safe wrapper or bridge entry point was detected. |
| kCFUserNotificationAlertTopMostKey | constant | CoreFoundation/CFUserNotification.h | No safe wrapper or bridge entry point was detected. |
| kCFUserNotificationAlternateButtonTitleKey | constant | CoreFoundation/CFUserNotification.h | No safe wrapper or bridge entry point was detected. |
| kCFUserNotificationCheckBoxTitlesKey | constant | CoreFoundation/CFUserNotification.h | No safe wrapper or bridge entry point was detected. |
| kCFUserNotificationDefaultButtonTitleKey | constant | CoreFoundation/CFUserNotification.h | No safe wrapper or bridge entry point was detected. |
| kCFUserNotificationIconURLKey | constant | CoreFoundation/CFUserNotification.h | No safe wrapper or bridge entry point was detected. |
| kCFUserNotificationKeyboardTypesKey | constant | CoreFoundation/CFUserNotification.h | No safe wrapper or bridge entry point was detected. |
| kCFUserNotificationLocalizationURLKey | constant | CoreFoundation/CFUserNotification.h | No safe wrapper or bridge entry point was detected. |
| kCFUserNotificationOtherButtonTitleKey | constant | CoreFoundation/CFUserNotification.h | No safe wrapper or bridge entry point was detected. |
| kCFUserNotificationPopUpSelectionKey | constant | CoreFoundation/CFUserNotification.h | No safe wrapper or bridge entry point was detected. |
| kCFUserNotificationPopUpTitlesKey | constant | CoreFoundation/CFUserNotification.h | No safe wrapper or bridge entry point was detected. |
| kCFUserNotificationProgressIndicatorValueKey | constant | CoreFoundation/CFUserNotification.h | No safe wrapper or bridge entry point was detected. |
| kCFUserNotificationSoundURLKey | constant | CoreFoundation/CFUserNotification.h | No safe wrapper or bridge entry point was detected. |
| kCFUserNotificationTextFieldTitlesKey | constant | CoreFoundation/CFUserNotification.h | No safe wrapper or bridge entry point was detected. |
| kCFUserNotificationTextFieldValuesKey | constant | CoreFoundation/CFUserNotification.h | No safe wrapper or bridge entry point was detected. |
| CFXMLAttributeDeclarationInfo | typedef struct | CoreFoundation/CFXMLNode.h | cf::CFXML is covered, but this public declaration is not exposed. |
| CFXMLAttributeListDeclarationInfo | typedef struct | CoreFoundation/CFXMLNode.h | cf::CFXML is covered, but this public declaration is not exposed. |
| CFXMLDocumentInfo | typedef struct | CoreFoundation/CFXMLNode.h | cf::CFXML is covered, but this public declaration is not exposed. |
| CFXMLDocumentTypeInfo | typedef struct | CoreFoundation/CFXMLNode.h | cf::CFXML is covered, but this public declaration is not exposed. |
| CFXMLElementInfo | typedef struct | CoreFoundation/CFXMLNode.h | cf::CFXML is covered, but this public declaration is not exposed. |
| CFXMLElementTypeDeclarationInfo | typedef struct | CoreFoundation/CFXMLNode.h | cf::CFXML is covered, but this public declaration is not exposed. |
| CFXMLEntityInfo | typedef struct | CoreFoundation/CFXMLNode.h | cf::CFXML is covered, but this public declaration is not exposed. |
| CFXMLEntityReferenceInfo | typedef struct | CoreFoundation/CFXMLNode.h | cf::CFXML is covered, but this public declaration is not exposed. |
| CFXMLEntityTypeCode | typedef enum | CoreFoundation/CFXMLNode.h | cf::CFXML is covered, but this public declaration is not exposed. |
| CFXMLExternalID | typedef struct | CoreFoundation/CFXMLNode.h | cf::CFXML is covered, but this public declaration is not exposed. |
| CFXMLNodeRef | typedef struct | CoreFoundation/CFXMLNode.h | cf::CFXML is covered, but this public declaration is not exposed. |
| CFXMLNodeTypeCode | typedef enum | CoreFoundation/CFXMLNode.h | cf::CFXML is covered, but this public declaration is not exposed. |
| CFXMLNotationInfo | typedef struct | CoreFoundation/CFXMLNode.h | cf::CFXML is covered, but this public declaration is not exposed. |
| CFXMLProcessingInstructionInfo | typedef struct | CoreFoundation/CFXMLNode.h | cf::CFXML is covered, but this public declaration is not exposed. |
| CFXMLParserCallBacks | typedef struct | CoreFoundation/CFXMLParser.h | cf::CFXML is covered, but this public declaration is not exposed. |
| CFXMLParserContext | typedef struct | CoreFoundation/CFXMLParser.h | cf::CFXML is covered, but this public declaration is not exposed. |
| CFXMLParserOptions | typedef enum | CoreFoundation/CFXMLParser.h | cf::CFXML is covered, but this public declaration is not exposed. |
| CFXMLParserRef | typedef struct | CoreFoundation/CFXMLParser.h | cf::CFXML is covered, but this public declaration is not exposed. |
| CFXMLParserStatusCode | typedef enum | CoreFoundation/CFXMLParser.h | cf::CFXML is covered, but this public declaration is not exposed. |
| kCFXMLTreeErrorDescription | constant | CoreFoundation/CFXMLParser.h | No safe wrapper or bridge entry point was detected. |
| kCFXMLTreeErrorLineNumber | constant | CoreFoundation/CFXMLParser.h | No safe wrapper or bridge entry point was detected. |
| kCFXMLTreeErrorLocation | constant | CoreFoundation/CFXMLParser.h | No safe wrapper or bridge entry point was detected. |
| kCFXMLTreeErrorStatusCode | constant | CoreFoundation/CFXMLParser.h | No safe wrapper or bridge entry point was detected. |
| CM2Header | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CM2Profile | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CM2ProfileHandle | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CM2ProfilePtr | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CM4Header | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMAdaptationMatrixType | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMAppleProfileHeader | struct | ? | No safe wrapper or bridge entry point was detected. |
| CMBitmap | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMBufferLocation | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMCMYColor | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMCMYKColor | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMColor | struct | ? | No safe wrapper or bridge entry point was detected. |
| CMConcatProfileSet | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMCurveType | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMDataType | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMDateTime | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMDateTimeType | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMDeviceInfo | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMDeviceInfoPtr | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMDeviceProfileArray | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMDeviceProfileArrayPtr | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMDeviceProfileInfo | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMDeviceProfileScope | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMDeviceScope | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMFixedXYColor | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMFixedXYZColor | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMFloatBitmap | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMFloatBitmapFlags | typedef enum | ? | No safe wrapper or bridge entry point was detected. |
| CMGrayColor | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMHLSColor | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMHSVColor | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMHandleLocation | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMIntentCRDVMSize | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMLabColor | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMLut16Type | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMLut8Type | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMLuvColor | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMMInfo | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMMakeAndModel | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMMakeAndModelType | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMMeasurementType | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMMultiFunctCLUTType | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMMultiFunctLutA2BType | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMMultiFunctLutB2AType | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMMultiFunctLutType | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMMultiLocalizedUniCodeEntryRec | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMMultiLocalizedUniCodeType | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMMultichannel5Color | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMMultichannel6Color | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMMultichannel7Color | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMMultichannel8Color | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMNamedColor | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMNamedColor2EntryType | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMNamedColor2Type | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMNamedColorType | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMNativeDisplayInfo | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMNativeDisplayInfoType | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMPS2CRDVMSizeType | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMParametricCurveType | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMPathLocation | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMProfLoc | struct | ? | No safe wrapper or bridge entry point was detected. |
| CMProfileIterateData | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMProfileLocation | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMProfileRef | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMProfileSequenceDescType | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMRGBColor | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMS15Fixed16ArrayType | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMSCertificateChainMode | typedef enum | ? | No safe wrapper or bridge entry point was detected. |
| CMSDecoderCopyAllCerts | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSDecoderCopyContent | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSDecoderCopyDetachedContent | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSDecoderCopyEncapsulatedContentType | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSDecoderCopySignerCert | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSDecoderCopySignerEmailAddress | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSDecoderCopySignerSigningTime | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSDecoderCopySignerStatus | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSDecoderCopySignerTimestamp | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSDecoderCopySignerTimestampCertificates | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSDecoderCopySignerTimestampWithPolicy | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSDecoderCreate | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSDecoderFinalizeMessage | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSDecoderGetNumSigners | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSDecoderGetTypeID | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSDecoderIsContentEncrypted | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSDecoderRef | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMSDecoderSetDetachedContent | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSDecoderSetSearchKeychain | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSDecoderUpdateMessage | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSEncode | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSEncodeContent | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSEncoderAddRecipients | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSEncoderAddSignedAttributes | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSEncoderAddSigners | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSEncoderAddSupportingCerts | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSEncoderCopyEncapsulatedContentType | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSEncoderCopyEncodedContent | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSEncoderCopyRecipients | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSEncoderCopySignerTimestamp | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSEncoderCopySignerTimestampWithPolicy | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSEncoderCopySigners | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSEncoderCopySupportingCerts | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSEncoderCreate | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSEncoderGetCertificateChainMode | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSEncoderGetHasDetachedContent | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSEncoderGetTypeID | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSEncoderRef | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMSEncoderSetCertificateChainMode | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSEncoderSetEncapsulatedContentType | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSEncoderSetEncapsulatedContentTypeOID | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSEncoderSetHasDetachedContent | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSEncoderSetSignerAlgorithm | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSEncoderUpdateContent | function | ? | No safe wrapper or bridge entry point was detected. |
| CMSSignedAttributes | typedef enum | ? | No safe wrapper or bridge entry point was detected. |
| CMSSignerStatus | typedef enum | ? | No safe wrapper or bridge entry point was detected. |
| CMScreeningChannelRec | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMScreeningType | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMSignatureType | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMTagElemTable | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMTagRecord | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMTextDescriptionType | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMTextType | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMU16Fixed16ArrayType | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMUInt16ArrayType | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMUInt32ArrayType | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMUInt64ArrayType | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMUInt8ArrayType | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMUcrBgType | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMUnicodeTextType | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMVideoCardGamma | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMVideoCardGammaFormula | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMVideoCardGammaTable | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMVideoCardGammaType | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMViewingConditionsType | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMWorldRef | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMXYZColor | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMXYZType | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| CMYxyColor | typedef struct | ? | No safe wrapper or bridge entry point was detected. |
| kCMMApplyTransformProcName | constant | ? | No safe wrapper or bridge entry point was detected. |
| kCMMCreateTransformPropertyProcName | constant | ? | No safe wrapper or bridge entry point was detected. |
| kCMMInitializeLinkProfileProcName | constant | ? | No safe wrapper or bridge entry point was detected. |
| kCMMInitializeTransformProcName | constant | ? | No safe wrapper or bridge entry point was detected. |
| kCMSEncoderDigestAlgorithmSHA1 | constant | ? | No safe wrapper or bridge entry point was detected. |
| kCMSEncoderDigestAlgorithmSHA256 | constant | ? | No safe wrapper or bridge entry point was detected. |
| CMCopyDictionaryOfAttachments | function | CoreMedia/CMAttachment.h | No safe wrapper or bridge entry point was detected. |
| CMGetAttachment | function | CoreMedia/CMAttachment.h | No safe wrapper or bridge entry point was detected. |
| CMPropagateAttachments | function | CoreMedia/CMAttachment.h | No safe wrapper or bridge entry point was detected. |
| CMRemoveAllAttachments | function | CoreMedia/CMAttachment.h | No safe wrapper or bridge entry point was detected. |
| CMRemoveAttachment | function | CoreMedia/CMAttachment.h | No safe wrapper or bridge entry point was detected. |
| CMSetAttachment | function | CoreMedia/CMAttachment.h | No safe wrapper or bridge entry point was detected. |
| CMSetAttachments | function | CoreMedia/CMAttachment.h | No safe wrapper or bridge entry point was detected. |
| CMAudioDeviceClockCreate | function | CoreMedia/CMAudioDeviceClock.h | No safe wrapper or bridge entry point was detected. |
| CMAudioDeviceClockCreateFromAudioDeviceID | function | CoreMedia/CMAudioDeviceClock.h | No safe wrapper or bridge entry point was detected. |
| CMAudioDeviceClockGetAudioDevice | function | CoreMedia/CMAudioDeviceClock.h | No safe wrapper or bridge entry point was detected. |
| CMAudioDeviceClockSetAudioDeviceID | function | CoreMedia/CMAudioDeviceClock.h | No safe wrapper or bridge entry point was detected. |
| CMAudioDeviceClockSetAudioDeviceUID | function | CoreMedia/CMAudioDeviceClock.h | No safe wrapper or bridge entry point was detected. |
| CMBlockBufferAccessDataBytes | function | CoreMedia/CMBlockBuffer.h | cm::CMBlockBuffer is covered, but this public declaration is not exposed. |
| CMBlockBufferAppendBufferReference | function | CoreMedia/CMBlockBuffer.h | cm::CMBlockBuffer is covered, but this public declaration is not exposed. |
| CMBlockBufferAppendMemoryBlock | function | CoreMedia/CMBlockBuffer.h | cm::CMBlockBuffer is covered, but this public declaration is not exposed. |
| CMBlockBufferAssureBlockMemory | function | CoreMedia/CMBlockBuffer.h | cm::CMBlockBuffer is covered, but this public declaration is not exposed. |
| CMBlockBufferCreateContiguous | function | CoreMedia/CMBlockBuffer.h | cm::CMBlockBuffer is covered, but this public declaration is not exposed. |
| CMBlockBufferCreateWithBufferReference | function | CoreMedia/CMBlockBuffer.h | cm::CMBlockBuffer is covered, but this public declaration is not exposed. |
| CMBlockBufferCustomBlockSource | typedef struct | CoreMedia/CMBlockBuffer.h | cm::CMBlockBuffer is covered, but this public declaration is not exposed. |
| CMBlockBufferFillDataBytes | function | CoreMedia/CMBlockBuffer.h | cm::CMBlockBuffer is covered, but this public declaration is not exposed. |
| CMBlockBufferGetTypeID | function | CoreMedia/CMBlockBuffer.h | cm::CMBlockBuffer is covered, but this public declaration is not exposed. |
| CMBufferCallbacks | typedef struct | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMBufferHandlers | typedef struct | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMBufferQueueCallForEachBuffer | function | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMBufferQueueContainsEndOfData | function | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMBufferQueueCopyHead | function | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMBufferQueueCreate | function | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMBufferQueueCreateWithHandlers | function | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMBufferQueueDequeueAndRetain | function | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMBufferQueueDequeueIfDataReadyAndRetain | function | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMBufferQueueEnqueue | function | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMBufferQueueGetBufferCount | function | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMBufferQueueGetCallbacksForSampleBuffersSortedByOutputPTS | function | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMBufferQueueGetCallbacksForUnsortedSampleBuffers | function | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMBufferQueueGetDuration | function | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMBufferQueueGetEndPresentationTimeStamp | function | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMBufferQueueGetFirstDecodeTimeStamp | function | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMBufferQueueGetFirstPresentationTimeStamp | function | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMBufferQueueGetHead | function | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMBufferQueueGetMaxPresentationTimeStamp | function | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMBufferQueueGetMinDecodeTimeStamp | function | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMBufferQueueGetMinPresentationTimeStamp | function | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMBufferQueueGetTotalSize | function | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMBufferQueueGetTypeID | function | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMBufferQueueInstallTrigger | function | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMBufferQueueInstallTriggerHandler | function | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMBufferQueueInstallTriggerHandlerWithIntegerThreshold | function | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMBufferQueueInstallTriggerWithIntegerThreshold | function | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMBufferQueueIsAtEndOfData | function | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMBufferQueueIsEmpty | function | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMBufferQueueMarkEndOfData | function | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMBufferQueueRef | typedef struct | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMBufferQueueRemoveTrigger | function | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMBufferQueueReset | function | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMBufferQueueResetWithCallback | function | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMBufferQueueSetValidationCallback | function | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMBufferQueueSetValidationHandler | function | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMBufferQueueTestTrigger | function | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMBufferQueueTriggerToken | typedef struct | CoreMedia/CMBufferQueue.h | No safe wrapper or bridge entry point was detected. |
| CMAudioFormatDescriptionCreate | function | CoreMedia/CMFormatDescription.h | cm::CMFormatDescription is covered, but this public declaration is not exposed. |
| CMAudioFormatDescriptionCreateSummary | function | CoreMedia/CMFormatDescription.h | cm::CMFormatDescription is covered, but this public declaration is not exposed. |
| CMAudioFormatDescriptionEqual | function | CoreMedia/CMFormatDescription.h | cm::CMFormatDescription is covered, but this public declaration is not exposed. |
| CMAudioFormatDescriptionGetChannelLayout | function | CoreMedia/CMFormatDescription.h | cm::CMFormatDescription is covered, but this public declaration is not exposed. |
| CMAudioFormatDescriptionGetFormatList | function | CoreMedia/CMFormatDescription.h | cm::CMFormatDescription is covered, but this public declaration is not exposed. |
| CMAudioFormatDescriptionGetMagicCookie | function | CoreMedia/CMFormatDescription.h | cm::CMFormatDescription is covered, but this public declaration is not exposed. |
| CMAudioFormatDescriptionGetMostCompatibleFormat | function | CoreMedia/CMFormatDescription.h | cm::CMFormatDescription is covered, but this public declaration is not exposed. |
| CMAudioFormatDescriptionGetRichestDecodableFormat | function | CoreMedia/CMFormatDescription.h | cm::CMFormatDescription is covered, but this public declaration is not exposed. |
| CMFormatDescriptionCreate | function | CoreMedia/CMFormatDescription.h | cm::CMFormatDescription is covered, but this public declaration is not exposed. |
| CMFormatDescriptionEqual | function | CoreMedia/CMFormatDescription.h | cm::CMFormatDescription is covered, but this public declaration is not exposed. |
| CMFormatDescriptionEqualIgnoringExtensionKeys | function | CoreMedia/CMFormatDescription.h | cm::CMFormatDescription is covered, but this public declaration is not exposed. |
| CMFormatDescriptionGetExtension | function | CoreMedia/CMFormatDescription.h | cm::CMFormatDescription is covered, but this public declaration is not exposed. |
| CMFormatDescriptionGetTypeID | function | CoreMedia/CMFormatDescription.h | cm::CMFormatDescription is covered, but this public declaration is not exposed. |
| CMMuxedFormatDescriptionCreate | function | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| CMTextFormatDescriptionGetDefaultStyle | function | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| CMTextFormatDescriptionGetDefaultTextBox | function | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| CMTextFormatDescriptionGetDisplayFlags | function | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| CMTextFormatDescriptionGetFontName | function | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| CMTextFormatDescriptionGetJustification | function | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| CMTimeCodeFormatDescriptionCreate | function | CoreMedia/CMFormatDescription.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMTimeCodeFormatDescriptionGetFrameDuration | function | CoreMedia/CMFormatDescription.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMTimeCodeFormatDescriptionGetFrameQuanta | function | CoreMedia/CMFormatDescription.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMTimeCodeFormatDescriptionGetTimeCodeFlags | function | CoreMedia/CMFormatDescription.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMVideoDimensions | typedef struct | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| CMVideoFormatDescriptionCopyTagCollectionArray | function | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| CMVideoFormatDescriptionCreate | function | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| CMVideoFormatDescriptionCreateForImageBuffer | function | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| CMVideoFormatDescriptionCreateFromH264ParameterSets | function | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| CMVideoFormatDescriptionCreateFromHEVCParameterSets | function | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| CMVideoFormatDescriptionGetCleanAperture | function | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| CMVideoFormatDescriptionGetDimensions | function | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| CMVideoFormatDescriptionGetExtensionKeysCommonWithImageBuffers | function | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| CMVideoFormatDescriptionGetH264ParameterSetAtIndex | function | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| CMVideoFormatDescriptionGetHEVCParameterSetAtIndex | function | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| CMVideoFormatDescriptionGetPresentationDimensions | function | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| CMVideoFormatDescriptionMatchesImageBuffer | function | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionAlphaChannelMode_PremultipliedAlpha | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionAlphaChannelMode_StraightAlpha | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionCameraCalibrationExtrinsicOriginSource_StereoCameraSystemBaseline | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionCameraCalibrationLensAlgorithmKind_ParametricLens | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionCameraCalibrationLensDomain_Color | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionCameraCalibrationLensRole_Left | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionCameraCalibrationLensRole_Mono | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionCameraCalibrationLensRole_Right | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionCameraCalibration_ExtrinsicOrientationQuaternion | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionCameraCalibration_ExtrinsicOriginSource | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionCameraCalibration_IntrinsicMatrix | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionCameraCalibration_IntrinsicMatrixProjectionOffset | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionCameraCalibration_IntrinsicMatrixReferenceDimensions | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionCameraCalibration_LensAlgorithmKind | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionCameraCalibration_LensDistortions | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionCameraCalibration_LensDomain | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionCameraCalibration_LensFrameAdjustmentsPolynomialX | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionCameraCalibration_LensFrameAdjustmentsPolynomialY | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionCameraCalibration_LensIdentifier | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionCameraCalibration_LensRole | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionCameraCalibration_RadialAngleLimit | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionChromaLocation_Bottom | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionChromaLocation_BottomLeft | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionChromaLocation_Center | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionChromaLocation_DV420 | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionChromaLocation_Left | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionChromaLocation_Top | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionChromaLocation_TopLeft | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionColorPrimaries_DCI_P3 | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionColorPrimaries_EBU_3213 | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionColorPrimaries_ITU_R_2020 | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionColorPrimaries_ITU_R_709_2 | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionColorPrimaries_P22 | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionColorPrimaries_P3_D65 | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionColorPrimaries_SMPTE_C | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionConformsToMPEG2VideoProfile | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_AlphaChannelMode | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_AlternativeTransferCharacteristics | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_AmbientViewingEnvironment | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_AuxiliaryTypeInfo | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_BitsPerComponent | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_BytesPerRow | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_CameraCalibrationDataLensCollection | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_ChromaLocationBottomField | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_ChromaLocationTopField | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_CleanAperture | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_ColorPrimaries | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_ContainsAlphaChannel | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_ContentColorVolume | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_ContentLightLevelInfo | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_ConvertedFromExternalSphericalTags | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_Depth | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_FieldCount | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_FieldDetail | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_FormatName | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_FullRangeVideo | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_GammaLevel | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_HasAdditionalViews | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_HasLeftStereoEyeView | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_HasRightStereoEyeView | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_HeroEye | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_HorizontalDisparityAdjustment | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_HorizontalFieldOfView | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_ICCProfile | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_LogTransferFunction | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_MasteringDisplayColorVolume | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_OriginalCompressionSettings | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_PixelAspectRatio | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_ProjectionKind | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_ProtectedContentOriginalFormat | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_RevisionLevel | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_SampleDescriptionExtensionAtoms | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_SpatialQuality | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_StereoCameraBaseline | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_TemporalQuality | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_TransferFunction | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_Vendor | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_VerbatimISOSampleEntry | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_VerbatimImageDescription | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_VerbatimSampleDescription | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_Version | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_ViewPackingKind | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionExtension_YCbCrMatrix | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionFieldDetail_SpatialFirstLineEarly | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionFieldDetail_SpatialFirstLineLate | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionFieldDetail_TemporalBottomFirst | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionFieldDetail_TemporalTopFirst | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionHeroEye_Left | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionHeroEye_Right | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionKey_CleanApertureHeight | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionKey_CleanApertureHeightRational | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionKey_CleanApertureHorizontalOffset | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionKey_CleanApertureHorizontalOffsetRational | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionKey_CleanApertureVerticalOffset | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionKey_CleanApertureVerticalOffsetRational | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionKey_CleanApertureWidth | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionKey_CleanApertureWidthRational | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionKey_PixelAspectRatioHorizontalSpacing | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionKey_PixelAspectRatioVerticalSpacing | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionLogTransferFunction_AppleLog | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionProjectionKind_AppleImmersiveVideo | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionProjectionKind_Equirectangular | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionProjectionKind_HalfEquirectangular | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionProjectionKind_ParametricImmersive | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionProjectionKind_Rectilinear | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionTransferFunction_ITU_R_2020 | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionTransferFunction_ITU_R_2100_HLG | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionTransferFunction_ITU_R_709_2 | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionTransferFunction_Linear | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionTransferFunction_SMPTE_240M_1995 | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionTransferFunction_SMPTE_ST_2084_PQ | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionTransferFunction_SMPTE_ST_428_1 | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionTransferFunction_UseGamma | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionTransferFunction_sRGB | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionVendor_Apple | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionViewPackingKind_OverUnder | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionViewPackingKind_SideBySide | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionYCbCrMatrix_ITU_R_2020 | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionYCbCrMatrix_ITU_R_601_4 | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionYCbCrMatrix_ITU_R_709_2 | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMFormatDescriptionYCbCrMatrix_SMPTE_240M_1995 | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMTextFormatDescriptionColor_Alpha | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMTextFormatDescriptionColor_Blue | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMTextFormatDescriptionColor_Green | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMTextFormatDescriptionColor_Red | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMTextFormatDescriptionExtension_BackgroundColor | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMTextFormatDescriptionExtension_DefaultFontName | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMTextFormatDescriptionExtension_DefaultStyle | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMTextFormatDescriptionExtension_DefaultTextBox | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMTextFormatDescriptionExtension_DisplayFlags | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMTextFormatDescriptionExtension_FontTable | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMTextFormatDescriptionExtension_HorizontalJustification | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMTextFormatDescriptionExtension_TextJustification | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMTextFormatDescriptionExtension_VerticalJustification | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMTextFormatDescriptionRect_Bottom | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMTextFormatDescriptionRect_Left | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMTextFormatDescriptionRect_Right | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMTextFormatDescriptionRect_Top | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMTextFormatDescriptionStyle_Ascent | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMTextFormatDescriptionStyle_EndChar | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMTextFormatDescriptionStyle_Font | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMTextFormatDescriptionStyle_FontFace | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMTextFormatDescriptionStyle_FontSize | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMTextFormatDescriptionStyle_ForegroundColor | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMTextFormatDescriptionStyle_Height | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMTextFormatDescriptionStyle_StartChar | constant | CoreMedia/CMFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCMTimeCodeFormatDescriptionExtension_SourceReferenceName | constant | CoreMedia/CMFormatDescription.h | cm::CMTime is covered, but this public declaration is not exposed. |
| kCMTimeCodeFormatDescriptionKey_LangCode | constant | CoreMedia/CMFormatDescription.h | cm::CMTime is covered, but this public declaration is not exposed. |
| kCMTimeCodeFormatDescriptionKey_Value | constant | CoreMedia/CMFormatDescription.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMAudioFormatDescriptionCopyAsBigEndianSoundDescriptionBlockBuffer | function | CoreMedia/CMFormatDescriptionBridge.h | cm::CMFormatDescription is covered, but this public declaration is not exposed. |
| CMAudioFormatDescriptionCreateFromBigEndianSoundDescriptionBlockBuffer | function | CoreMedia/CMFormatDescriptionBridge.h | cm::CMFormatDescription is covered, but this public declaration is not exposed. |
| CMAudioFormatDescriptionCreateFromBigEndianSoundDescriptionData | function | CoreMedia/CMFormatDescriptionBridge.h | cm::CMFormatDescription is covered, but this public declaration is not exposed. |
| CMClosedCaptionFormatDescriptionCopyAsBigEndianClosedCaptionDescriptionBlockBuffer | function | CoreMedia/CMFormatDescriptionBridge.h | No safe wrapper or bridge entry point was detected. |
| CMClosedCaptionFormatDescriptionCreateFromBigEndianClosedCaptionDescriptionBlockBuffer | function | CoreMedia/CMFormatDescriptionBridge.h | No safe wrapper or bridge entry point was detected. |
| CMClosedCaptionFormatDescriptionCreateFromBigEndianClosedCaptionDescriptionData | function | CoreMedia/CMFormatDescriptionBridge.h | No safe wrapper or bridge entry point was detected. |
| CMDoesBigEndianSoundDescriptionRequireLegacyCBRSampleTableLayout | function | CoreMedia/CMFormatDescriptionBridge.h | No safe wrapper or bridge entry point was detected. |
| CMMetadataFormatDescriptionCopyAsBigEndianMetadataDescriptionBlockBuffer | function | CoreMedia/CMFormatDescriptionBridge.h | No safe wrapper or bridge entry point was detected. |
| CMMetadataFormatDescriptionCreateFromBigEndianMetadataDescriptionBlockBuffer | function | CoreMedia/CMFormatDescriptionBridge.h | No safe wrapper or bridge entry point was detected. |
| CMMetadataFormatDescriptionCreateFromBigEndianMetadataDescriptionData | function | CoreMedia/CMFormatDescriptionBridge.h | No safe wrapper or bridge entry point was detected. |
| CMSwapBigEndianClosedCaptionDescriptionToHost | function | CoreMedia/CMFormatDescriptionBridge.h | No safe wrapper or bridge entry point was detected. |
| CMSwapBigEndianImageDescriptionToHost | function | CoreMedia/CMFormatDescriptionBridge.h | No safe wrapper or bridge entry point was detected. |
| CMSwapBigEndianMetadataDescriptionToHost | function | CoreMedia/CMFormatDescriptionBridge.h | No safe wrapper or bridge entry point was detected. |
| CMSwapBigEndianSoundDescriptionToHost | function | CoreMedia/CMFormatDescriptionBridge.h | No safe wrapper or bridge entry point was detected. |
| CMSwapBigEndianTextDescriptionToHost | function | CoreMedia/CMFormatDescriptionBridge.h | No safe wrapper or bridge entry point was detected. |
| CMSwapBigEndianTimeCodeDescriptionToHost | function | CoreMedia/CMFormatDescriptionBridge.h | No safe wrapper or bridge entry point was detected. |
| CMSwapHostEndianClosedCaptionDescriptionToBig | function | CoreMedia/CMFormatDescriptionBridge.h | No safe wrapper or bridge entry point was detected. |
| CMSwapHostEndianImageDescriptionToBig | function | CoreMedia/CMFormatDescriptionBridge.h | No safe wrapper or bridge entry point was detected. |
| CMSwapHostEndianMetadataDescriptionToBig | function | CoreMedia/CMFormatDescriptionBridge.h | No safe wrapper or bridge entry point was detected. |
| CMSwapHostEndianSoundDescriptionToBig | function | CoreMedia/CMFormatDescriptionBridge.h | No safe wrapper or bridge entry point was detected. |
| CMSwapHostEndianTextDescriptionToBig | function | CoreMedia/CMFormatDescriptionBridge.h | No safe wrapper or bridge entry point was detected. |
| CMSwapHostEndianTimeCodeDescriptionToBig | function | CoreMedia/CMFormatDescriptionBridge.h | No safe wrapper or bridge entry point was detected. |
| CMTextFormatDescriptionCopyAsBigEndianTextDescriptionBlockBuffer | function | CoreMedia/CMFormatDescriptionBridge.h | No safe wrapper or bridge entry point was detected. |
| CMTextFormatDescriptionCreateFromBigEndianTextDescriptionBlockBuffer | function | CoreMedia/CMFormatDescriptionBridge.h | No safe wrapper or bridge entry point was detected. |
| CMTextFormatDescriptionCreateFromBigEndianTextDescriptionData | function | CoreMedia/CMFormatDescriptionBridge.h | No safe wrapper or bridge entry point was detected. |
| CMTimeCodeFormatDescriptionCopyAsBigEndianTimeCodeDescriptionBlockBuffer | function | CoreMedia/CMFormatDescriptionBridge.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMTimeCodeFormatDescriptionCreateFromBigEndianTimeCodeDescriptionBlockBuffer | function | CoreMedia/CMFormatDescriptionBridge.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMTimeCodeFormatDescriptionCreateFromBigEndianTimeCodeDescriptionData | function | CoreMedia/CMFormatDescriptionBridge.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMVideoFormatDescriptionCopyAsBigEndianImageDescriptionBlockBuffer | function | CoreMedia/CMFormatDescriptionBridge.h | No safe wrapper or bridge entry point was detected. |
| CMVideoFormatDescriptionCreateFromBigEndianImageDescriptionBlockBuffer | function | CoreMedia/CMFormatDescriptionBridge.h | No safe wrapper or bridge entry point was detected. |
| CMVideoFormatDescriptionCreateFromBigEndianImageDescriptionData | function | CoreMedia/CMFormatDescriptionBridge.h | No safe wrapper or bridge entry point was detected. |
| kCMImageDescriptionFlavor_3GPFamily | constant | CoreMedia/CMFormatDescriptionBridge.h | No safe wrapper or bridge entry point was detected. |
| kCMImageDescriptionFlavor_ISOFamily | constant | CoreMedia/CMFormatDescriptionBridge.h | No safe wrapper or bridge entry point was detected. |
| kCMImageDescriptionFlavor_ISOFamilyWithAppleExtensions | constant | CoreMedia/CMFormatDescriptionBridge.h | No safe wrapper or bridge entry point was detected. |
| kCMImageDescriptionFlavor_QuickTimeMovie | constant | CoreMedia/CMFormatDescriptionBridge.h | No safe wrapper or bridge entry point was detected. |
| kCMSoundDescriptionFlavor_3GPFamily | constant | CoreMedia/CMFormatDescriptionBridge.h | No safe wrapper or bridge entry point was detected. |
| kCMSoundDescriptionFlavor_ISOFamily | constant | CoreMedia/CMFormatDescriptionBridge.h | No safe wrapper or bridge entry point was detected. |
| kCMSoundDescriptionFlavor_QuickTimeMovie | constant | CoreMedia/CMFormatDescriptionBridge.h | No safe wrapper or bridge entry point was detected. |
| kCMSoundDescriptionFlavor_QuickTimeMovieV2 | constant | CoreMedia/CMFormatDescriptionBridge.h | No safe wrapper or bridge entry point was detected. |
| CMMemoryPoolCreate | function | CoreMedia/CMMemoryPool.h | No safe wrapper or bridge entry point was detected. |
| CMMemoryPoolFlush | function | CoreMedia/CMMemoryPool.h | No safe wrapper or bridge entry point was detected. |
| CMMemoryPoolGetAllocator | function | CoreMedia/CMMemoryPool.h | No safe wrapper or bridge entry point was detected. |
| CMMemoryPoolGetTypeID | function | CoreMedia/CMMemoryPool.h | No safe wrapper or bridge entry point was detected. |
| CMMemoryPoolInvalidate | function | CoreMedia/CMMemoryPool.h | No safe wrapper or bridge entry point was detected. |
| CMMemoryPoolRef | typedef struct | CoreMedia/CMMemoryPool.h | No safe wrapper or bridge entry point was detected. |
| kCMMemoryPoolOption_AgeOutPeriod | constant | CoreMedia/CMMemoryPool.h | No safe wrapper or bridge entry point was detected. |
| CMMetadataCreateIdentifierForKeyAndKeySpace | function | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMMetadataCreateKeyFromIdentifier | function | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMMetadataCreateKeyFromIdentifierAsCFData | function | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMMetadataCreateKeySpaceFromIdentifier | function | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMMetadataDataTypeRegistryDataTypeConformsToDataType | function | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMMetadataDataTypeRegistryDataTypeIsBaseDataType | function | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMMetadataDataTypeRegistryDataTypeIsRegistered | function | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMMetadataDataTypeRegistryGetBaseDataTypeForConformingDataType | function | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMMetadataDataTypeRegistryGetBaseDataTypes | function | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMMetadataDataTypeRegistryGetConformingDataTypes | function | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMMetadataDataTypeRegistryGetDataTypeDescription | function | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMMetadataDataTypeRegistryRegisterDataType | function | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataBaseDataType_AffineTransformF64 | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataBaseDataType_BMP | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataBaseDataType_DimensionsF32 | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataBaseDataType_ExtendedRasterRectangleValue | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataBaseDataType_Float32 | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataBaseDataType_Float64 | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataBaseDataType_GIF | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataBaseDataType_JPEG | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataBaseDataType_JSON | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataBaseDataType_PNG | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataBaseDataType_PerspectiveTransformF64 | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataBaseDataType_PointF32 | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataBaseDataType_PolygonF32 | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataBaseDataType_PolylineF32 | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataBaseDataType_RasterRectangleValue | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataBaseDataType_RawData | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataBaseDataType_RectF32 | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataBaseDataType_SInt16 | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataBaseDataType_SInt32 | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataBaseDataType_SInt64 | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataBaseDataType_SInt8 | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataBaseDataType_UInt16 | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataBaseDataType_UInt32 | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataBaseDataType_UInt64 | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataBaseDataType_UInt8 | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataBaseDataType_UTF16 | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataBaseDataType_UTF8 | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataDataType_QuickTimeMetadataDirection | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataDataType_QuickTimeMetadataLocation_ISO6709 | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataDataType_QuickTimeMetadataMilliLux | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataDataType_QuickTimeMetadataUUID | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataIdentifier_QuickTimeMetadataDirection_Facing | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataIdentifier_QuickTimeMetadataDisplayMaskRectangleMono | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataIdentifier_QuickTimeMetadataDisplayMaskRectangleStereoLeft | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataIdentifier_QuickTimeMetadataDisplayMaskRectangleStereoRight | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataIdentifier_QuickTimeMetadataLivePhotoStillImageTransform | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataIdentifier_QuickTimeMetadataLivePhotoStillImageTransformReferenceDimensions | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataIdentifier_QuickTimeMetadataLocation_ISO6709 | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataIdentifier_QuickTimeMetadataPreferredAffineTransform | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataIdentifier_QuickTimeMetadataPresentationImmersiveMedia | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataIdentifier_QuickTimeMetadataSceneIlluminance | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataIdentifier_QuickTimeMetadataSegmentIdentifier | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataIdentifier_QuickTimeMetadataSpatialAudioMix | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataIdentifier_QuickTimeMetadataVideoOrientation | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataKeySpace_HLSDateRange | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataKeySpace_ID3 | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataKeySpace_ISOUserData | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataKeySpace_Icy | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataKeySpace_QuickTimeMetadata | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataKeySpace_QuickTimeUserData | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMMetadataKeySpace_iTunes | constant | CoreMedia/CMMetadata.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMAudioSampleBufferCreateReadyWithPacketDescriptions | function | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| CMAudioSampleBufferCreateWithPacketDescriptions | function | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| CMAudioSampleBufferCreateWithPacketDescriptionsAndMakeDataReadyHandler | function | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| CMSampleBufferCallBlockForEachSample | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer is covered, but this public declaration is not exposed. |
| CMSampleBufferCallForEachSample | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer is covered, but this public declaration is not exposed. |
| CMSampleBufferCopyPCMDataIntoAudioBufferList | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer is covered, but this public declaration is not exposed. |
| CMSampleBufferCopySampleBufferForRange | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer is covered, but this public declaration is not exposed. |
| CMSampleBufferCreate | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer is covered, but this public declaration is not exposed. |
| CMSampleBufferCreateCopy | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer is covered, but this public declaration is not exposed. |
| CMSampleBufferCreateCopyWithNewTiming | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer is covered, but this public declaration is not exposed. |
| CMSampleBufferCreateForImageBuffer | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer is covered, but this public declaration is not exposed. |
| CMSampleBufferCreateForImageBufferWithMakeDataReadyHandler | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer is covered, but this public declaration is not exposed. |
| CMSampleBufferCreateReady | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer is covered, but this public declaration is not exposed. |
| CMSampleBufferCreateReadyWithImageBuffer | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer is covered, but this public declaration is not exposed. |
| CMSampleBufferCreateWithMakeDataReadyHandler | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer is covered, but this public declaration is not exposed. |
| CMSampleBufferGetAudioBufferListWithRetainedBlockBuffer | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer is covered, but this public declaration is not exposed. |
| CMSampleBufferGetAudioStreamPacketDescriptions | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer is covered, but this public declaration is not exposed. |
| CMSampleBufferGetAudioStreamPacketDescriptionsPtr | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer is covered, but this public declaration is not exposed. |
| CMSampleBufferGetOutputDecodeTimeStamp | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer is covered, but this public declaration is not exposed. |
| CMSampleBufferGetOutputDuration | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer is covered, but this public declaration is not exposed. |
| CMSampleBufferGetOutputPresentationTimeStamp | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer is covered, but this public declaration is not exposed. |
| CMSampleBufferGetOutputSampleTimingInfoArray | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer is covered, but this public declaration is not exposed. |
| CMSampleBufferGetSampleAttachmentsArray | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer is covered, but this public declaration is not exposed. |
| CMSampleBufferGetSampleSize | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer is covered, but this public declaration is not exposed. |
| CMSampleBufferGetSampleSizeArray | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer is covered, but this public declaration is not exposed. |
| CMSampleBufferGetSampleTimingInfo | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer is covered, but this public declaration is not exposed. |
| CMSampleBufferGetSampleTimingInfoArray | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer is covered, but this public declaration is not exposed. |
| CMSampleBufferGetTotalSampleSize | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer is covered, but this public declaration is not exposed. |
| CMSampleBufferGetTypeID | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer is covered, but this public declaration is not exposed. |
| CMSampleBufferHasDataFailed | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer is covered, but this public declaration is not exposed. |
| CMSampleBufferInvalidate | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer is covered, but this public declaration is not exposed. |
| CMSampleBufferMakeDataReady | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer is covered, but this public declaration is not exposed. |
| CMSampleBufferSetDataBuffer | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer is covered, but this public declaration is not exposed. |
| CMSampleBufferSetDataBufferFromAudioBufferList | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer is covered, but this public declaration is not exposed. |
| CMSampleBufferSetDataFailed | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer is covered, but this public declaration is not exposed. |
| CMSampleBufferSetDataReady | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer is covered, but this public declaration is not exposed. |
| CMSampleBufferSetInvalidateCallback | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer is covered, but this public declaration is not exposed. |
| CMSampleBufferSetInvalidateHandler | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer is covered, but this public declaration is not exposed. |
| CMSampleBufferSetOutputPresentationTimeStamp | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer is covered, but this public declaration is not exposed. |
| CMSampleBufferTrackDataReadiness | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer is covered, but this public declaration is not exposed. |
| kCMHEVCTemporalLevelInfoKey_ConstraintIndicatorFlags | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMHEVCTemporalLevelInfoKey_LevelIndex | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMHEVCTemporalLevelInfoKey_ProfileCompatibilityFlags | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMHEVCTemporalLevelInfoKey_ProfileIndex | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMHEVCTemporalLevelInfoKey_ProfileSpace | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMHEVCTemporalLevelInfoKey_TemporalLevel | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMHEVCTemporalLevelInfoKey_TierFlag | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleAttachmentKey_AudioIndependentSampleDecoderRefreshCount | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleAttachmentKey_CryptorSubsampleAuxiliaryData | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleAttachmentKey_DependsOnOthers | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleAttachmentKey_DisplayImmediately | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleAttachmentKey_DoNotDisplay | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleAttachmentKey_EarlierDisplayTimesAllowed | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleAttachmentKey_HDR10PlusPerFrameData | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleAttachmentKey_HEVCStepwiseTemporalSubLayerAccess | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleAttachmentKey_HEVCSyncSampleNALUnitType | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleAttachmentKey_HEVCTemporalLevelInfo | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleAttachmentKey_HEVCTemporalSubLayerAccess | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleAttachmentKey_HasRedundantCoding | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleAttachmentKey_IsDependedOnByOthers | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleAttachmentKey_NotSync | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleAttachmentKey_PartialSync | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleAttachmentKey_PostDecodeProcessingMetadata | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferAttachmentKey_CameraIntrinsicMatrix | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferAttachmentKey_DisplayEmptyMediaImmediately | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferAttachmentKey_DrainAfterDecoding | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferAttachmentKey_DroppedFrameReason | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferAttachmentKey_DroppedFrameReasonInfo | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferAttachmentKey_EmptyMedia | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferAttachmentKey_EndsPreviousSampleDuration | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferAttachmentKey_FillDiscontinuitiesWithSilence | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferAttachmentKey_ForceKeyFrame | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferAttachmentKey_GradualDecoderRefresh | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferAttachmentKey_PermanentEmptyMedia | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferAttachmentKey_PostNotificationWhenConsumed | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferAttachmentKey_ResetDecoderBeforeDecoding | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferAttachmentKey_ResumeOutput | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferAttachmentKey_Reverse | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferAttachmentKey_SampleReferenceByteOffset | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferAttachmentKey_SampleReferenceURL | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferAttachmentKey_SpeedMultiplier | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferAttachmentKey_StillImageLensStabilizationInfo | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferAttachmentKey_TransitionID | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferAttachmentKey_TrimDurationAtEnd | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferAttachmentKey_TrimDurationAtStart | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferConduitNotificationParameter_MaxUpcomingOutputPTS | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferConduitNotificationParameter_MinUpcomingOutputPTS | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferConduitNotificationParameter_ResumeTag | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferConduitNotificationParameter_UpcomingOutputPTSRangeMayOverlapQueuedOutputPTSRange | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferConduitNotification_InhibitOutputUntil | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferConduitNotification_ResetOutput | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferConduitNotification_UpcomingOutputPTSRangeChanged | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferConsumerNotification_BufferConsumed | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferDroppedFrameReasonInfo_CameraModeSwitch | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferDroppedFrameReason_Discontinuity | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferDroppedFrameReason_FrameWasLate | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferDroppedFrameReason_OutOfBuffers | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferLensStabilizationInfo_Active | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferLensStabilizationInfo_Off | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferLensStabilizationInfo_OutOfRange | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferLensStabilizationInfo_Unavailable | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferNotificationParameter_OSStatus | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferNotification_DataBecameReady | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMSampleBufferNotification_DataFailed | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCMTimingInfoInvalid | constant | CoreMedia/CMSampleBuffer.h | No safe wrapper or bridge entry point was detected. |
| CMSimpleQueueCreate | function | CoreMedia/CMSimpleQueue.h | No safe wrapper or bridge entry point was detected. |
| CMSimpleQueueDequeue | function | CoreMedia/CMSimpleQueue.h | No safe wrapper or bridge entry point was detected. |
| CMSimpleQueueEnqueue | function | CoreMedia/CMSimpleQueue.h | No safe wrapper or bridge entry point was detected. |
| CMSimpleQueueGetCapacity | function | CoreMedia/CMSimpleQueue.h | No safe wrapper or bridge entry point was detected. |
| CMSimpleQueueGetCount | function | CoreMedia/CMSimpleQueue.h | No safe wrapper or bridge entry point was detected. |
| CMSimpleQueueGetHead | function | CoreMedia/CMSimpleQueue.h | No safe wrapper or bridge entry point was detected. |
| CMSimpleQueueGetTypeID | function | CoreMedia/CMSimpleQueue.h | No safe wrapper or bridge entry point was detected. |
| CMSimpleQueueRef | typedef struct | CoreMedia/CMSimpleQueue.h | No safe wrapper or bridge entry point was detected. |
| CMSimpleQueueReset | function | CoreMedia/CMSimpleQueue.h | No safe wrapper or bridge entry point was detected. |
| CMClockConvertHostTimeToSystemUnits | function | CoreMedia/CMSync.h | cm::CMClock is covered, but this public declaration is not exposed. |
| CMClockGetAnchorTime | function | CoreMedia/CMSync.h | cm::CMClock is covered, but this public declaration is not exposed. |
| CMClockGetTime | function | CoreMedia/CMSync.h | cm::CMClock is covered, but this public declaration is not exposed. |
| CMClockGetTypeID | function | CoreMedia/CMSync.h | cm::CMClock is covered, but this public declaration is not exposed. |
| CMClockInvalidate | function | CoreMedia/CMSync.h | cm::CMClock is covered, but this public declaration is not exposed. |
| CMClockMakeHostTimeFromSystemUnits | function | CoreMedia/CMSync.h | cm::CMClock is covered, but this public declaration is not exposed. |
| CMClockMightDrift | function | CoreMedia/CMSync.h | cm::CMClock is covered, but this public declaration is not exposed. |
| CMSyncConvertTime | function | CoreMedia/CMSync.h | No safe wrapper or bridge entry point was detected. |
| CMSyncGetRelativeRate | function | CoreMedia/CMSync.h | No safe wrapper or bridge entry point was detected. |
| CMSyncGetRelativeRateAndAnchorTime | function | CoreMedia/CMSync.h | No safe wrapper or bridge entry point was detected. |
| CMSyncGetTime | function | CoreMedia/CMSync.h | No safe wrapper or bridge entry point was detected. |
| CMSyncMightDrift | function | CoreMedia/CMSync.h | No safe wrapper or bridge entry point was detected. |
| CMTimebaseAddTimer | function | CoreMedia/CMSync.h | cm::CMTimebase is covered, but this public declaration is not exposed. |
| CMTimebaseAddTimerDispatchSource | function | CoreMedia/CMSync.h | cm::CMTimebase is covered, but this public declaration is not exposed. |
| CMTimebaseCopySource | function | CoreMedia/CMSync.h | cm::CMTimebase is covered, but this public declaration is not exposed. |
| CMTimebaseCopySourceClock | function | CoreMedia/CMSync.h | cm::CMTimebase is covered, but this public declaration is not exposed. |
| CMTimebaseCopySourceTimebase | function | CoreMedia/CMSync.h | cm::CMTimebase is covered, but this public declaration is not exposed. |
| CMTimebaseCopyUltimateSourceClock | function | CoreMedia/CMSync.h | cm::CMTimebase is covered, but this public declaration is not exposed. |
| CMTimebaseCreateWithMasterTimebase | function | CoreMedia/CMSync.h | cm::CMTimebase is covered, but this public declaration is not exposed. |
| CMTimebaseCreateWithSourceClock | function | CoreMedia/CMSync.h | cm::CMTimebase is covered, but this public declaration is not exposed. |
| CMTimebaseCreateWithSourceTimebase | function | CoreMedia/CMSync.h | cm::CMTimebase is covered, but this public declaration is not exposed. |
| CMTimebaseGetEffectiveRate | function | CoreMedia/CMSync.h | cm::CMTimebase is covered, but this public declaration is not exposed. |
| CMTimebaseGetTimeAndRate | function | CoreMedia/CMSync.h | cm::CMTimebase is covered, but this public declaration is not exposed. |
| CMTimebaseGetTimeWithTimeScale | function | CoreMedia/CMSync.h | cm::CMTimebase is covered, but this public declaration is not exposed. |
| CMTimebaseGetTypeID | function | CoreMedia/CMSync.h | cm::CMTimebase is covered, but this public declaration is not exposed. |
| CMTimebaseNotificationBarrier | function | CoreMedia/CMSync.h | cm::CMTimebase is covered, but this public declaration is not exposed. |
| CMTimebaseRemoveTimer | function | CoreMedia/CMSync.h | cm::CMTimebase is covered, but this public declaration is not exposed. |
| CMTimebaseRemoveTimerDispatchSource | function | CoreMedia/CMSync.h | cm::CMTimebase is covered, but this public declaration is not exposed. |
| CMTimebaseSetAnchorTime | function | CoreMedia/CMSync.h | cm::CMTimebase is covered, but this public declaration is not exposed. |
| CMTimebaseSetMasterClock | function | CoreMedia/CMSync.h | cm::CMTimebase is covered, but this public declaration is not exposed. |
| CMTimebaseSetMasterTimebase | function | CoreMedia/CMSync.h | cm::CMTimebase is covered, but this public declaration is not exposed. |
| CMTimebaseSetRateAndAnchorTime | function | CoreMedia/CMSync.h | cm::CMTimebase is covered, but this public declaration is not exposed. |
| CMTimebaseSetSourceClock | function | CoreMedia/CMSync.h | cm::CMTimebase is covered, but this public declaration is not exposed. |
| CMTimebaseSetSourceTimebase | function | CoreMedia/CMSync.h | cm::CMTimebase is covered, but this public declaration is not exposed. |
| CMTimebaseSetTimerDispatchSourceNextFireTime | function | CoreMedia/CMSync.h | cm::CMTimebase is covered, but this public declaration is not exposed. |
| CMTimebaseSetTimerDispatchSourceToFireImmediately | function | CoreMedia/CMSync.h | cm::CMTimebase is covered, but this public declaration is not exposed. |
| CMTimebaseSetTimerNextFireTime | function | CoreMedia/CMSync.h | cm::CMTimebase is covered, but this public declaration is not exposed. |
| CMTimebaseSetTimerToFireImmediately | function | CoreMedia/CMSync.h | cm::CMTimebase is covered, but this public declaration is not exposed. |
| kCMTimebaseNotificationKey_EventTime | constant | CoreMedia/CMSync.h | cm::CMTime is covered, but this public declaration is not exposed. |
| kCMTimebaseNotification_EffectiveRateChanged | constant | CoreMedia/CMSync.h | cm::CMTime is covered, but this public declaration is not exposed. |
| kCMTimebaseNotification_TimeJumped | constant | CoreMedia/CMSync.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMPackingType | typedef enum | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMProjectionType | typedef enum | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMStereoViewComponents | typedef enum | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMStereoViewInterpretationOptions | typedef enum | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTag | typedef struct | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCategory | typedef enum | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCategoryEqualToTagCategory | function | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCategoryValueEqualToValue | function | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCompare | function | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCopyAsDictionary | function | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCopyDescription | function | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagDataType | typedef enum | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagEqualToTag | function | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagError | typedef enum | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagGetCategory | function | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagGetFlagsValue | function | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagGetFloat64Value | function | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagGetOSTypeValue | function | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagGetSInt64Value | function | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagGetValue | function | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagGetValueDataType | function | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagHasCategory | function | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagHasFlagsValue | function | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagHasFloat64Value | function | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagHasOSTypeValue | function | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagHasSInt64Value | function | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagHash | function | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagIsValid | function | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagMakeFromDictionary | function | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagMakeWithFlagsValue | function | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagMakeWithFloat64Value | function | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagMakeWithOSTypeValue | function | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagMakeWithSInt64Value | function | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMTagCategoryKey | constant | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMTagDataTypeKey | constant | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMTagInvalid | constant | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMTagMediaSubTypeMebx | constant | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMTagMediaTypeAudio | constant | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMTagMediaTypeMetadata | constant | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMTagMediaTypeVideo | constant | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMTagPackingTypeNone | constant | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMTagPackingTypeOverUnder | constant | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMTagPackingTypeSideBySide | constant | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMTagProjectionTypeEquirectangular | constant | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMTagProjectionTypeFisheye | constant | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMTagProjectionTypeHalfEquirectangular | constant | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMTagProjectionTypeParametricImmersive | constant | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMTagProjectionTypeRectangular | constant | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMTagStereoInterpretationOrderReversed | constant | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMTagStereoLeftAndRightEye | constant | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMTagStereoLeftEye | constant | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMTagStereoNone | constant | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMTagStereoRightEye | constant | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMTagValueKey | constant | CoreMedia/CMTag.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMMutableTagCollectionRef | typedef struct | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCollectionAddTag | function | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCollectionAddTagsFromArray | function | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCollectionAddTagsFromCollection | function | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCollectionApply | function | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCollectionApplyUntil | function | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCollectionContainsCategory | function | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCollectionContainsSpecifiedTags | function | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCollectionContainsTag | function | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCollectionContainsTagsOfCollection | function | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCollectionCopyAsData | function | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCollectionCopyAsDictionary | function | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCollectionCopyDescription | function | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCollectionCopyTagsOfCategories | function | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCollectionCountTagsWithFilterFunction | function | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCollectionCreate | function | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCollectionCreateCopy | function | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCollectionCreateDifference | function | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCollectionCreateExclusiveOr | function | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCollectionCreateFromData | function | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCollectionCreateFromDictionary | function | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCollectionCreateIntersection | function | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCollectionCreateMutable | function | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCollectionCreateMutableCopy | function | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCollectionCreateUnion | function | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCollectionError | typedef enum | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCollectionGetCount | function | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCollectionGetCountOfCategory | function | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCollectionGetTags | function | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCollectionGetTagsWithCategory | function | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCollectionGetTagsWithFilterFunction | function | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCollectionGetTypeID | function | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCollectionIsEmpty | function | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCollectionRef | typedef struct | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCollectionRemoveAllTags | function | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCollectionRemoveAllTagsOfCategory | function | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTagCollectionRemoveTag | function | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMTagCollectionTagsArrayKey | constant | CoreMedia/CMTagCollection.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMSampleBufferCreateForTaggedBufferGroup | function | CoreMedia/CMTaggedBufferGroup.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMSampleBufferGetTaggedBufferGroup | function | CoreMedia/CMTaggedBufferGroup.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTaggedBufferGroupCreate | function | CoreMedia/CMTaggedBufferGroup.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTaggedBufferGroupCreateCombined | function | CoreMedia/CMTaggedBufferGroup.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTaggedBufferGroupError | typedef enum | CoreMedia/CMTaggedBufferGroup.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTaggedBufferGroupFormatDescriptionCreateForTaggedBufferGroup | function | CoreMedia/CMTaggedBufferGroup.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTaggedBufferGroupFormatDescriptionCreateForTaggedBufferGroupWithExtensions | function | CoreMedia/CMTaggedBufferGroup.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTaggedBufferGroupFormatDescriptionMatchesTaggedBufferGroup | function | CoreMedia/CMTaggedBufferGroup.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTaggedBufferGroupGetCMSampleBufferAtIndex | function | CoreMedia/CMTaggedBufferGroup.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTaggedBufferGroupGetCMSampleBufferForTag | function | CoreMedia/CMTaggedBufferGroup.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTaggedBufferGroupGetCMSampleBufferForTagCollection | function | CoreMedia/CMTaggedBufferGroup.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTaggedBufferGroupGetCVPixelBufferAtIndex | function | CoreMedia/CMTaggedBufferGroup.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTaggedBufferGroupGetCVPixelBufferForTag | function | CoreMedia/CMTaggedBufferGroup.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTaggedBufferGroupGetCVPixelBufferForTagCollection | function | CoreMedia/CMTaggedBufferGroup.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTaggedBufferGroupGetCount | function | CoreMedia/CMTaggedBufferGroup.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTaggedBufferGroupGetNumberOfMatchesForTagCollection | function | CoreMedia/CMTaggedBufferGroup.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTaggedBufferGroupGetTagCollectionAtIndex | function | CoreMedia/CMTaggedBufferGroup.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTaggedBufferGroupGetTypeID | function | CoreMedia/CMTaggedBufferGroup.h | CoreMedia metadata/tagging APIs are not wrapped. |
| CMTaggedBufferGroupRef | typedef struct | CoreMedia/CMTaggedBufferGroup.h | CoreMedia metadata/tagging APIs are not wrapped. |
| kCMTextMarkupAlignmentType_End | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupAlignmentType_Left | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupAlignmentType_Middle | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupAlignmentType_Right | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupAlignmentType_Start | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupAttribute_Alignment | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupAttribute_BackgroundColorARGB | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupAttribute_BaseFontSizePercentageRelativeToVideoHeight | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupAttribute_BoldStyle | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupAttribute_CharacterBackgroundColorARGB | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupAttribute_CharacterEdgeStyle | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupAttribute_FontFamilyName | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupAttribute_FontFamilyNameList | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupAttribute_ForegroundColorARGB | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupAttribute_GenericFontFamilyName | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupAttribute_ItalicStyle | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupAttribute_OrthogonalLinePositionPercentageRelativeToWritingDirection | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupAttribute_RelativeFontSize | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupAttribute_TextPositionPercentageRelativeToWritingDirection | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupAttribute_UnderlineStyle | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupAttribute_VerticalLayout | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupAttribute_WritingDirectionSizePercentage | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupCharacterEdgeStyle_Depressed | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupCharacterEdgeStyle_DropShadow | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupCharacterEdgeStyle_None | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupCharacterEdgeStyle_Raised | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupCharacterEdgeStyle_Uniform | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupGenericFontName_Casual | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupGenericFontName_Cursive | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupGenericFontName_Default | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupGenericFontName_Fantasy | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupGenericFontName_Monospace | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupGenericFontName_MonospaceSansSerif | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupGenericFontName_MonospaceSerif | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupGenericFontName_ProportionalSansSerif | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupGenericFontName_ProportionalSerif | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupGenericFontName_SansSerif | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupGenericFontName_Serif | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextMarkupGenericFontName_SmallCapital | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextVerticalLayout_LeftToRight | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| kCMTextVerticalLayout_RightToLeft | constant | CoreMedia/CMTextMarkup.h | No safe wrapper or bridge entry point was detected. |
| CMTimeAbsoluteValue | function | CoreMedia/CMTime.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMTimeCopyAsDictionary | function | CoreMedia/CMTime.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMTimeCopyDescription | function | CoreMedia/CMTime.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMTimeFlags | typedef enum | CoreMedia/CMTime.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMTimeGetSeconds | function | CoreMedia/CMTime.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMTimeMake | function | CoreMedia/CMTime.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMTimeMakeFromDictionary | function | CoreMedia/CMTime.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMTimeMakeWithEpoch | function | CoreMedia/CMTime.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMTimeMaximum | function | CoreMedia/CMTime.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMTimeMinimum | function | CoreMedia/CMTime.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMTimeMultiplyByRatio | function | CoreMedia/CMTime.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMTimeRoundingMethod | typedef enum | CoreMedia/CMTime.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMTimeShow | function | CoreMedia/CMTime.h | cm::CMTime is covered, but this public declaration is not exposed. |
| kCMTimeEpochKey | constant | CoreMedia/CMTime.h | cm::CMTime is covered, but this public declaration is not exposed. |
| kCMTimeFlagsKey | constant | CoreMedia/CMTime.h | cm::CMTime is covered, but this public declaration is not exposed. |
| kCMTimeIndefinite | constant | CoreMedia/CMTime.h | cm::CMTime is covered, but this public declaration is not exposed. |
| kCMTimeInvalid | constant | CoreMedia/CMTime.h | cm::CMTime is covered, but this public declaration is not exposed. |
| kCMTimeNegativeInfinity | constant | CoreMedia/CMTime.h | cm::CMTime is covered, but this public declaration is not exposed. |
| kCMTimePositiveInfinity | constant | CoreMedia/CMTime.h | cm::CMTime is covered, but this public declaration is not exposed. |
| kCMTimeScaleKey | constant | CoreMedia/CMTime.h | cm::CMTime is covered, but this public declaration is not exposed. |
| kCMTimeValueKey | constant | CoreMedia/CMTime.h | cm::CMTime is covered, but this public declaration is not exposed. |
| kCMTimeZero | constant | CoreMedia/CMTime.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMTimeClampToRange | function | CoreMedia/CMTimeRange.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMTimeFoldIntoRange | function | CoreMedia/CMTimeRange.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMTimeMapDurationFromRangeToRange | function | CoreMedia/CMTimeRange.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMTimeMapTimeFromRangeToRange | function | CoreMedia/CMTimeRange.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMTimeMapping | typedef struct | CoreMedia/CMTimeRange.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMTimeMappingCopyAsDictionary | function | CoreMedia/CMTimeRange.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMTimeMappingCopyDescription | function | CoreMedia/CMTimeRange.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMTimeMappingMake | function | CoreMedia/CMTimeRange.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMTimeMappingMakeEmpty | function | CoreMedia/CMTimeRange.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMTimeMappingMakeFromDictionary | function | CoreMedia/CMTimeRange.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMTimeMappingShow | function | CoreMedia/CMTimeRange.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMTimeRangeCopyAsDictionary | function | CoreMedia/CMTimeRange.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMTimeRangeCopyDescription | function | CoreMedia/CMTimeRange.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMTimeRangeEqual | function | CoreMedia/CMTimeRange.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMTimeRangeFromTimeToTime | function | CoreMedia/CMTimeRange.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMTimeRangeMake | function | CoreMedia/CMTimeRange.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMTimeRangeMakeFromDictionary | function | CoreMedia/CMTimeRange.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CMTimeRangeShow | function | CoreMedia/CMTimeRange.h | cm::CMTime is covered, but this public declaration is not exposed. |
| kCMTimeMappingInvalid | constant | CoreMedia/CMTimeRange.h | cm::CMTime is covered, but this public declaration is not exposed. |
| kCMTimeMappingSourceKey | constant | CoreMedia/CMTimeRange.h | cm::CMTime is covered, but this public declaration is not exposed. |
| kCMTimeMappingTargetKey | constant | CoreMedia/CMTimeRange.h | cm::CMTime is covered, but this public declaration is not exposed. |
| kCMTimeRangeDurationKey | constant | CoreMedia/CMTimeRange.h | cm::CMTime is covered, but this public declaration is not exposed. |
| kCMTimeRangeInvalid | constant | CoreMedia/CMTimeRange.h | cm::CMTime is covered, but this public declaration is not exposed. |
| kCMTimeRangeStartKey | constant | CoreMedia/CMTimeRange.h | cm::CMTime is covered, but this public declaration is not exposed. |
| kCMTimeRangeZero | constant | CoreMedia/CMTimeRange.h | cm::CMTime is covered, but this public declaration is not exposed. |
| CVSMPTETime | typedef struct | CoreVideo/CVBase.h | No safe wrapper or bridge entry point was detected. |
| CVSMPTETimeFlags | typedef enum | CoreVideo/CVBase.h | No safe wrapper or bridge entry point was detected. |
| CVSMPTETimeType | typedef enum | CoreVideo/CVBase.h | No safe wrapper or bridge entry point was detected. |
| CVTime | typedef struct | CoreVideo/CVBase.h | No safe wrapper or bridge entry point was detected. |
| CVTimeFlags | typedef enum | CoreVideo/CVBase.h | No safe wrapper or bridge entry point was detected. |
| CVTimeStampFlags | typedef enum | CoreVideo/CVBase.h | No safe wrapper or bridge entry point was detected. |
| kCVIndefiniteTime | constant | CoreVideo/CVBase.h | No safe wrapper or bridge entry point was detected. |
| kCVZeroTime | constant | CoreVideo/CVBase.h | No safe wrapper or bridge entry point was detected. |
| CVBufferHasAttachment | function | CoreVideo/CVBuffer.h | cv::CVBuffer is covered, but this public declaration is not exposed. |
| CVBufferPropagateAttachments | function | CoreVideo/CVBuffer.h | cv::CVBuffer is covered, but this public declaration is not exposed. |
| CVBufferRemoveAttachment | function | CoreVideo/CVBuffer.h | cv::CVBuffer is covered, but this public declaration is not exposed. |
| CVBufferSetAttachments | function | CoreVideo/CVBuffer.h | cv::CVBuffer is covered, but this public declaration is not exposed. |
| kCVBufferMovieTimeKey | constant | CoreVideo/CVBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVBufferNonPropagatedAttachmentsKey | constant | CoreVideo/CVBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVBufferPropagatedAttachmentsKey | constant | CoreVideo/CVBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVBufferTimeScaleKey | constant | CoreVideo/CVBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVBufferTimeValueKey | constant | CoreVideo/CVBuffer.h | No safe wrapper or bridge entry point was detected. |
| CVGetCurrentHostTime | function | CoreVideo/CVHostTime.h | No safe wrapper or bridge entry point was detected. |
| CVGetHostClockFrequency | function | CoreVideo/CVHostTime.h | No safe wrapper or bridge entry point was detected. |
| CVGetHostClockMinimumTimeDelta | function | CoreVideo/CVHostTime.h | No safe wrapper or bridge entry point was detected. |
| CVColorPrimariesGetIntegerCodePointForString | function | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| CVColorPrimariesGetStringForIntegerCodePoint | function | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| CVImageBufferCreateColorSpaceFromAttachments | function | CoreVideo/CVImageBuffer.h | cv::CVImageBuffer is covered, but this public declaration is not exposed. |
| CVImageBufferGetColorSpace | function | CoreVideo/CVImageBuffer.h | cv::CVImageBuffer is covered, but this public declaration is not exposed. |
| CVImageBufferIsFlipped | function | CoreVideo/CVImageBuffer.h | cv::CVImageBuffer is covered, but this public declaration is not exposed. |
| CVTransferFunctionGetIntegerCodePointForString | function | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| CVTransferFunctionGetStringForIntegerCodePoint | function | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| CVYCbCrMatrixGetIntegerCodePointForString | function | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| CVYCbCrMatrixGetStringForIntegerCodePoint | function | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferAlphaChannelIsOpaque | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferAlphaChannelModeKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferAlphaChannelMode_PremultipliedAlpha | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferAlphaChannelMode_StraightAlpha | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferAmbientViewingEnvironmentKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferCGColorSpaceKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferChromaLocationBottomFieldKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferChromaLocationTopFieldKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferChromaLocation_Bottom | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferChromaLocation_BottomLeft | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferChromaLocation_Center | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferChromaLocation_DV420 | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferChromaLocation_Left | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferChromaLocation_Top | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferChromaLocation_TopLeft | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferChromaSubsamplingKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferChromaSubsampling_411 | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferChromaSubsampling_420 | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferChromaSubsampling_422 | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferCleanApertureHeightKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferCleanApertureHorizontalOffsetKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferCleanApertureKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferCleanApertureVerticalOffsetKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferCleanApertureWidthKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferColorPrimariesKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferColorPrimaries_DCI_P3 | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferColorPrimaries_EBU_3213 | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferColorPrimaries_ITU_R_2020 | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferColorPrimaries_ITU_R_709_2 | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferColorPrimaries_P22 | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferColorPrimaries_P3_D65 | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferColorPrimaries_SMPTE_C | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferContentLightLevelInfoKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferDisplayDimensionsKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferDisplayHeightKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferDisplayMaskRectangleKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferDisplayMaskRectangleStereoLeftKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferDisplayMaskRectangleStereoRightKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferDisplayMaskRectangle_LeftEdgePointsKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferDisplayMaskRectangle_RectangleHeightKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferDisplayMaskRectangle_RectangleLeftKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferDisplayMaskRectangle_RectangleTopKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferDisplayMaskRectangle_RectangleWidthKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferDisplayMaskRectangle_ReferenceRasterHeightKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferDisplayMaskRectangle_ReferenceRasterWidthKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferDisplayMaskRectangle_RightEdgePointsKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferDisplayWidthKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferFieldCountKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferFieldDetailKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferFieldDetailSpatialFirstLineEarly | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferFieldDetailSpatialFirstLineLate | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferFieldDetailTemporalBottomFirst | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferFieldDetailTemporalTopFirst | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferGammaLevelKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferICCProfileKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferLogTransferFunctionKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferLogTransferFunction_AppleLog | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferLogTransferFunction_AppleLog2 | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferMasteringDisplayColorVolumeKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferPixelAspectRatioHorizontalSpacingKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferPixelAspectRatioKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferPixelAspectRatioVerticalSpacingKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferPostDecodeProcessingFrameMetadataKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferPostDecodeProcessingSequenceMetadataKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferPreferredCleanApertureKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferRegionOfInterestKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferSceneIlluminationKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferTransferFunctionKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferTransferFunction_ITU_R_2100_HLG | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferTransferFunction_ITU_R_709_2 | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferTransferFunction_Linear | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferTransferFunction_SMPTE_ST_2084_PQ | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferTransferFunction_SMPTE_ST_428_1 | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferYCbCrMatrixKey | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVImageBufferYCbCrMatrix_ITU_R_709_2 | constant | CoreVideo/CVImageBuffer.h | No safe wrapper or bridge entry point was detected. |
| CVMetalBufferGetBuffer | function | CoreVideo/CVMetalBuffer.h | No safe wrapper or bridge entry point was detected. |
| CVMetalBufferGetTypeID | function | CoreVideo/CVMetalBuffer.h | No safe wrapper or bridge entry point was detected. |
| CVMetalBufferCacheCreate | function | CoreVideo/CVMetalBufferCache.h | No safe wrapper or bridge entry point was detected. |
| CVMetalBufferCacheCreateBufferFromImage | function | CoreVideo/CVMetalBufferCache.h | No safe wrapper or bridge entry point was detected. |
| CVMetalBufferCacheFlush | function | CoreVideo/CVMetalBufferCache.h | No safe wrapper or bridge entry point was detected. |
| CVMetalBufferCacheGetTypeID | function | CoreVideo/CVMetalBufferCache.h | No safe wrapper or bridge entry point was detected. |
| CVMetalBufferCacheRef | typedef struct | CoreVideo/CVMetalBufferCache.h | No safe wrapper or bridge entry point was detected. |
| kCVMetalBufferCacheMaximumBufferAgeKey | constant | CoreVideo/CVMetalBufferCache.h | No safe wrapper or bridge entry point was detected. |
| CVMetalTextureGetCleanTexCoords | function | CoreVideo/CVMetalTexture.h | No safe wrapper or bridge entry point was detected. |
| CVMetalTextureGetTexture | function | CoreVideo/CVMetalTexture.h | No safe wrapper or bridge entry point was detected. |
| CVMetalTextureGetTypeID | function | CoreVideo/CVMetalTexture.h | No safe wrapper or bridge entry point was detected. |
| CVMetalTextureIsFlipped | function | CoreVideo/CVMetalTexture.h | No safe wrapper or bridge entry point was detected. |
| kCVMetalTextureStorageMode | constant | CoreVideo/CVMetalTexture.h | No safe wrapper or bridge entry point was detected. |
| kCVMetalTextureUsage | constant | CoreVideo/CVMetalTexture.h | No safe wrapper or bridge entry point was detected. |
| CVMetalTextureCacheCreateTextureFromImage | function | CoreVideo/CVMetalTextureCache.h | cv::CVMetalTextureCache is covered, but this public declaration is not exposed. |
| kCVMetalTextureCacheMaximumTextureAgeKey | constant | CoreVideo/CVMetalTextureCache.h | No safe wrapper or bridge entry point was detected. |
| CVOpenGLBufferAttach | function | CoreVideo/CVOpenGLBuffer.h | No safe wrapper or bridge entry point was detected. |
| CVOpenGLBufferCreate | function | CoreVideo/CVOpenGLBuffer.h | No safe wrapper or bridge entry point was detected. |
| CVOpenGLBufferGetAttributes | function | CoreVideo/CVOpenGLBuffer.h | No safe wrapper or bridge entry point was detected. |
| CVOpenGLBufferGetTypeID | function | CoreVideo/CVOpenGLBuffer.h | No safe wrapper or bridge entry point was detected. |
| CVOpenGLBufferRelease | function | CoreVideo/CVOpenGLBuffer.h | No safe wrapper or bridge entry point was detected. |
| CVOpenGLBufferRetain | function | CoreVideo/CVOpenGLBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVOpenGLBufferHeight | constant | CoreVideo/CVOpenGLBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVOpenGLBufferInternalFormat | constant | CoreVideo/CVOpenGLBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVOpenGLBufferMaximumMipmapLevel | constant | CoreVideo/CVOpenGLBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVOpenGLBufferTarget | constant | CoreVideo/CVOpenGLBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVOpenGLBufferWidth | constant | CoreVideo/CVOpenGLBuffer.h | No safe wrapper or bridge entry point was detected. |
| CVOpenGLBufferPoolCreate | function | CoreVideo/CVOpenGLBufferPool.h | No safe wrapper or bridge entry point was detected. |
| CVOpenGLBufferPoolCreateOpenGLBuffer | function | CoreVideo/CVOpenGLBufferPool.h | No safe wrapper or bridge entry point was detected. |
| CVOpenGLBufferPoolGetAttributes | function | CoreVideo/CVOpenGLBufferPool.h | No safe wrapper or bridge entry point was detected. |
| CVOpenGLBufferPoolGetOpenGLBufferAttributes | function | CoreVideo/CVOpenGLBufferPool.h | No safe wrapper or bridge entry point was detected. |
| CVOpenGLBufferPoolGetTypeID | function | CoreVideo/CVOpenGLBufferPool.h | No safe wrapper or bridge entry point was detected. |
| CVOpenGLBufferPoolRef | typedef struct | CoreVideo/CVOpenGLBufferPool.h | No safe wrapper or bridge entry point was detected. |
| CVOpenGLBufferPoolRelease | function | CoreVideo/CVOpenGLBufferPool.h | No safe wrapper or bridge entry point was detected. |
| CVOpenGLBufferPoolRetain | function | CoreVideo/CVOpenGLBufferPool.h | No safe wrapper or bridge entry point was detected. |
| kCVOpenGLBufferPoolMaximumBufferAgeKey | constant | CoreVideo/CVOpenGLBufferPool.h | No safe wrapper or bridge entry point was detected. |
| kCVOpenGLBufferPoolMinimumBufferCountKey | constant | CoreVideo/CVOpenGLBufferPool.h | No safe wrapper or bridge entry point was detected. |
| CVOpenGLTextureGetCleanTexCoords | function | CoreVideo/CVOpenGLTexture.h | No safe wrapper or bridge entry point was detected. |
| CVOpenGLTextureGetName | function | CoreVideo/CVOpenGLTexture.h | No safe wrapper or bridge entry point was detected. |
| CVOpenGLTextureGetTarget | function | CoreVideo/CVOpenGLTexture.h | No safe wrapper or bridge entry point was detected. |
| CVOpenGLTextureGetTypeID | function | CoreVideo/CVOpenGLTexture.h | No safe wrapper or bridge entry point was detected. |
| CVOpenGLTextureIsFlipped | function | CoreVideo/CVOpenGLTexture.h | No safe wrapper or bridge entry point was detected. |
| CVOpenGLTextureRelease | function | CoreVideo/CVOpenGLTexture.h | No safe wrapper or bridge entry point was detected. |
| CVOpenGLTextureRetain | function | CoreVideo/CVOpenGLTexture.h | No safe wrapper or bridge entry point was detected. |
| CVOpenGLTextureCacheCreate | function | CoreVideo/CVOpenGLTextureCache.h | No safe wrapper or bridge entry point was detected. |
| CVOpenGLTextureCacheCreateTextureFromImage | function | CoreVideo/CVOpenGLTextureCache.h | No safe wrapper or bridge entry point was detected. |
| CVOpenGLTextureCacheFlush | function | CoreVideo/CVOpenGLTextureCache.h | No safe wrapper or bridge entry point was detected. |
| CVOpenGLTextureCacheGetTypeID | function | CoreVideo/CVOpenGLTextureCache.h | No safe wrapper or bridge entry point was detected. |
| CVOpenGLTextureCacheRef | typedef struct | CoreVideo/CVOpenGLTextureCache.h | No safe wrapper or bridge entry point was detected. |
| CVOpenGLTextureCacheRelease | function | CoreVideo/CVOpenGLTextureCache.h | No safe wrapper or bridge entry point was detected. |
| CVOpenGLTextureCacheRetain | function | CoreVideo/CVOpenGLTextureCache.h | No safe wrapper or bridge entry point was detected. |
| kCVOpenGLTextureCacheChromaSamplingModeAutomatic | constant | CoreVideo/CVOpenGLTextureCache.h | No safe wrapper or bridge entry point was detected. |
| kCVOpenGLTextureCacheChromaSamplingModeBestPerformance | constant | CoreVideo/CVOpenGLTextureCache.h | No safe wrapper or bridge entry point was detected. |
| kCVOpenGLTextureCacheChromaSamplingModeHighestQuality | constant | CoreVideo/CVOpenGLTextureCache.h | No safe wrapper or bridge entry point was detected. |
| kCVOpenGLTextureCacheChromaSamplingModeKey | constant | CoreVideo/CVOpenGLTextureCache.h | No safe wrapper or bridge entry point was detected. |
| CVPixelBufferCopyCreationAttributes | function | CoreVideo/CVPixelBuffer.h | cv::CVPixelBuffer is covered, but this public declaration is not exposed. |
| CVPixelBufferCreateResolvedAttributesDictionary | function | CoreVideo/CVPixelBuffer.h | cv::CVPixelBuffer is covered, but this public declaration is not exposed. |
| CVPixelBufferIsCompatibleWithAttributes | function | CoreVideo/CVPixelBuffer.h | cv::CVPixelBuffer is covered, but this public declaration is not exposed. |
| CVPixelBufferRelease | function | CoreVideo/CVPixelBuffer.h | cv::CVPixelBuffer is covered, but this public declaration is not exposed. |
| CVPixelBufferRetain | function | CoreVideo/CVPixelBuffer.h | cv::CVPixelBuffer is covered, but this public declaration is not exposed. |
| CVPlanarComponentInfo | typedef struct | CoreVideo/CVPixelBuffer.h | No safe wrapper or bridge entry point was detected. |
| CVPlanarPixelBufferInfo | typedef struct | CoreVideo/CVPixelBuffer.h | No safe wrapper or bridge entry point was detected. |
| CVPlanarPixelBufferInfo_YCbCrBiPlanar | typedef struct | CoreVideo/CVPixelBuffer.h | No safe wrapper or bridge entry point was detected. |
| CVPlanarPixelBufferInfo_YCbCrPlanar | typedef struct | CoreVideo/CVPixelBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelBufferBytesPerRowAlignmentKey | constant | CoreVideo/CVPixelBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelBufferCGBitmapContextCompatibilityKey | constant | CoreVideo/CVPixelBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelBufferCGImageCompatibilityKey | constant | CoreVideo/CVPixelBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelBufferExtendedPixelsBottomKey | constant | CoreVideo/CVPixelBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelBufferExtendedPixelsLeftKey | constant | CoreVideo/CVPixelBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelBufferExtendedPixelsRightKey | constant | CoreVideo/CVPixelBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelBufferExtendedPixelsTopKey | constant | CoreVideo/CVPixelBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelBufferHeightKey | constant | CoreVideo/CVPixelBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelBufferIOSurfacePurgeableKey | constant | CoreVideo/CVPixelBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelBufferMemoryAllocatorKey | constant | CoreVideo/CVPixelBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelBufferOpenGLCompatibilityKey | constant | CoreVideo/CVPixelBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelBufferPixelFormatTypeKey | constant | CoreVideo/CVPixelBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelBufferProResRAWKey_BlackLevel | constant | CoreVideo/CVPixelBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelBufferProResRAWKey_ColorMatrix | constant | CoreVideo/CVPixelBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelBufferProResRAWKey_GainFactor | constant | CoreVideo/CVPixelBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelBufferProResRAWKey_MetadataExtension | constant | CoreVideo/CVPixelBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelBufferProResRAWKey_RecommendedCrop | constant | CoreVideo/CVPixelBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelBufferProResRAWKey_SenselSitingOffsets | constant | CoreVideo/CVPixelBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelBufferProResRAWKey_WhiteBalanceBlueFactor | constant | CoreVideo/CVPixelBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelBufferProResRAWKey_WhiteBalanceCCT | constant | CoreVideo/CVPixelBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelBufferProResRAWKey_WhiteBalanceRedFactor | constant | CoreVideo/CVPixelBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelBufferProResRAWKey_WhiteLevel | constant | CoreVideo/CVPixelBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelBufferVersatileBayerKey_BayerPattern | constant | CoreVideo/CVPixelBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelBufferWidthKey | constant | CoreVideo/CVPixelBuffer.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelBufferIOSurfaceCoreAnimationCompatibilityKey | constant | CoreVideo/CVPixelBufferIOSurface.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelBufferIOSurfaceOpenGLFBOCompatibilityKey | constant | CoreVideo/CVPixelBufferIOSurface.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelBufferIOSurfaceOpenGLTextureCompatibilityKey | constant | CoreVideo/CVPixelBufferIOSurface.h | No safe wrapper or bridge entry point was detected. |
| CVPixelBufferPoolCreatePixelBufferWithAuxAttributes | function | CoreVideo/CVPixelBufferPool.h | cv::CVPixelBufferPool is covered, but this public declaration is not exposed. |
| CVPixelBufferPoolFlushFlags | typedef enum | CoreVideo/CVPixelBufferPool.h | cv::CVPixelBufferPool is covered, but this public declaration is not exposed. |
| CVPixelBufferPoolRelease | function | CoreVideo/CVPixelBufferPool.h | cv::CVPixelBufferPool is covered, but this public declaration is not exposed. |
| CVPixelBufferPoolRetain | function | CoreVideo/CVPixelBufferPool.h | cv::CVPixelBufferPool is covered, but this public declaration is not exposed. |
| kCVPixelBufferPoolAllocationThresholdKey | constant | CoreVideo/CVPixelBufferPool.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelBufferPoolFreeBufferNotification | constant | CoreVideo/CVPixelBufferPool.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelBufferPoolMaximumBufferAgeKey | constant | CoreVideo/CVPixelBufferPool.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelBufferPoolMinimumBufferCountKey | constant | CoreVideo/CVPixelBufferPool.h | No safe wrapper or bridge entry point was detected. |
| CVFillExtendedPixelsCallBackData | typedef struct | CoreVideo/CVPixelFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| CVIsCompressedPixelFormatAvailable | function | CoreVideo/CVPixelFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| CVPixelFormatDescriptionArrayCreateWithAllPixelFormatTypes | function | CoreVideo/CVPixelFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| CVPixelFormatDescriptionCreateWithPixelFormatType | function | CoreVideo/CVPixelFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| CVPixelFormatDescriptionRegisterDescriptionWithPixelFormatType | function | CoreVideo/CVPixelFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| CVPixelFormatTypeCopyFourCharCodeString | function | CoreVideo/CVPixelFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelFormatBitsPerBlock | constant | CoreVideo/CVPixelFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelFormatBitsPerComponent | constant | CoreVideo/CVPixelFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelFormatBlackBlock | constant | CoreVideo/CVPixelFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelFormatBlockHeight | constant | CoreVideo/CVPixelFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelFormatBlockHorizontalAlignment | constant | CoreVideo/CVPixelFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelFormatBlockVerticalAlignment | constant | CoreVideo/CVPixelFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelFormatBlockWidth | constant | CoreVideo/CVPixelFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelFormatCGBitmapContextCompatibility | constant | CoreVideo/CVPixelFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelFormatCGBitmapInfo | constant | CoreVideo/CVPixelFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelFormatCodecType | constant | CoreVideo/CVPixelFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelFormatComponentRange | constant | CoreVideo/CVPixelFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelFormatComponentRange_FullRange | constant | CoreVideo/CVPixelFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelFormatComponentRange_VideoRange | constant | CoreVideo/CVPixelFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelFormatComponentRange_WideRange | constant | CoreVideo/CVPixelFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelFormatConstant | constant | CoreVideo/CVPixelFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelFormatContainsAlpha | constant | CoreVideo/CVPixelFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelFormatContainsGrayscale | constant | CoreVideo/CVPixelFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelFormatContainsRGB | constant | CoreVideo/CVPixelFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelFormatContainsSenselArray | constant | CoreVideo/CVPixelFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelFormatContainsYCbCr | constant | CoreVideo/CVPixelFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelFormatFillExtendedPixelsCallback | constant | CoreVideo/CVPixelFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelFormatFourCC | constant | CoreVideo/CVPixelFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelFormatHorizontalSubsampling | constant | CoreVideo/CVPixelFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelFormatName | constant | CoreVideo/CVPixelFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelFormatOpenGLFormat | constant | CoreVideo/CVPixelFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelFormatOpenGLInternalFormat | constant | CoreVideo/CVPixelFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelFormatOpenGLType | constant | CoreVideo/CVPixelFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelFormatPlanes | constant | CoreVideo/CVPixelFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelFormatQDCompatibility | constant | CoreVideo/CVPixelFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| kCVPixelFormatVerticalSubsampling | constant | CoreVideo/CVPixelFormatDescription.h | No safe wrapper or bridge entry point was detected. |
| OS_dispatch_data | protocol | ? | No safe wrapper or bridge entry point was detected. |
| OS_dispatch_group | protocol | ? | No safe wrapper or bridge entry point was detected. |
| OS_dispatch_io | protocol | ? | No safe wrapper or bridge entry point was detected. |
| OS_dispatch_object | protocol | ? | No safe wrapper or bridge entry point was detected. |
| OS_dispatch_queue | protocol | ? | No safe wrapper or bridge entry point was detected. |
| OS_dispatch_queue_attr | protocol | ? | No safe wrapper or bridge entry point was detected. |
| OS_dispatch_queue_concurrent | protocol | ? | No safe wrapper or bridge entry point was detected. |
| OS_dispatch_queue_global | protocol | ? | No safe wrapper or bridge entry point was detected. |
| OS_dispatch_queue_main | protocol | ? | No safe wrapper or bridge entry point was detected. |
| OS_dispatch_queue_serial | protocol | ? | No safe wrapper or bridge entry point was detected. |
| OS_dispatch_queue_serial_executor | protocol | ? | No safe wrapper or bridge entry point was detected. |
| OS_dispatch_semaphore | protocol | ? | No safe wrapper or bridge entry point was detected. |
| OS_dispatch_workloop | protocol | ? | No safe wrapper or bridge entry point was detected. |
| dispatch_function_t | typedef struct | Dispatch/base.h | No safe wrapper or bridge entry point was detected. |
| dispatch_block_cancel | function | Dispatch/block.h | No safe wrapper or bridge entry point was detected. |
| dispatch_block_create | function | Dispatch/block.h | No safe wrapper or bridge entry point was detected. |
| dispatch_block_create_with_qos_class | function | Dispatch/block.h | No safe wrapper or bridge entry point was detected. |
| dispatch_block_flags_t | typedef enum | Dispatch/block.h | No safe wrapper or bridge entry point was detected. |
| dispatch_block_notify | function | Dispatch/block.h | No safe wrapper or bridge entry point was detected. |
| dispatch_block_perform | function | Dispatch/block.h | No safe wrapper or bridge entry point was detected. |
| dispatch_block_testcancel | function | Dispatch/block.h | No safe wrapper or bridge entry point was detected. |
| dispatch_block_wait | function | Dispatch/block.h | No safe wrapper or bridge entry point was detected. |
| dispatch_data_applier_t | typedef struct | Dispatch/data.h | Dispatch data / I/O helpers are not wrapped. |
| dispatch_data_apply | function | Dispatch/data.h | Dispatch data / I/O helpers are not wrapped. |
| dispatch_data_copy_region | function | Dispatch/data.h | Dispatch data / I/O helpers are not wrapped. |
| dispatch_data_create | function | Dispatch/data.h | Dispatch data / I/O helpers are not wrapped. |
| dispatch_data_create_concat | function | Dispatch/data.h | Dispatch data / I/O helpers are not wrapped. |
| dispatch_data_create_map | function | Dispatch/data.h | Dispatch data / I/O helpers are not wrapped. |
| dispatch_data_create_subrange | function | Dispatch/data.h | Dispatch data / I/O helpers are not wrapped. |
| dispatch_data_get_size | function | Dispatch/data.h | Dispatch data / I/O helpers are not wrapped. |
| dispatch_data_s | struct | Dispatch/data.h | Dispatch data / I/O helpers are not wrapped. |
| dispatch_data_t | typedef struct | Dispatch/data.h | Dispatch data / I/O helpers are not wrapped. |
| dispatch_group_async | function | Dispatch/group.h | DispatchGroup is covered, but this public declaration is not exposed. |
| dispatch_group_async_f | function | Dispatch/group.h | DispatchGroup is covered, but this public declaration is not exposed. |
| dispatch_group_create | function | Dispatch/group.h | DispatchGroup is covered, but this public declaration is not exposed. |
| dispatch_group_enter | function | Dispatch/group.h | DispatchGroup is covered, but this public declaration is not exposed. |
| dispatch_group_leave | function | Dispatch/group.h | DispatchGroup is covered, but this public declaration is not exposed. |
| dispatch_group_notify | function | Dispatch/group.h | DispatchGroup is covered, but this public declaration is not exposed. |
| dispatch_group_notify_f | function | Dispatch/group.h | DispatchGroup is covered, but this public declaration is not exposed. |
| dispatch_group_wait | function | Dispatch/group.h | DispatchGroup is covered, but this public declaration is not exposed. |
| dispatch_fd_t | typedef struct | Dispatch/io.h | Dispatch data / I/O helpers are not wrapped. |
| dispatch_io_barrier | function | Dispatch/io.h | Dispatch data / I/O helpers are not wrapped. |
| dispatch_io_close | function | Dispatch/io.h | Dispatch data / I/O helpers are not wrapped. |
| dispatch_io_close_flags_t | typedef struct | Dispatch/io.h | Dispatch data / I/O helpers are not wrapped. |
| dispatch_io_create | function | Dispatch/io.h | Dispatch data / I/O helpers are not wrapped. |
| dispatch_io_create_with_io | function | Dispatch/io.h | Dispatch data / I/O helpers are not wrapped. |
| dispatch_io_create_with_path | function | Dispatch/io.h | Dispatch data / I/O helpers are not wrapped. |
| dispatch_io_get_descriptor | function | Dispatch/io.h | Dispatch data / I/O helpers are not wrapped. |
| dispatch_io_handler_t | typedef struct | Dispatch/io.h | Dispatch data / I/O helpers are not wrapped. |
| dispatch_io_interval_flags_t | typedef struct | Dispatch/io.h | Dispatch data / I/O helpers are not wrapped. |
| dispatch_io_read | function | Dispatch/io.h | Dispatch data / I/O helpers are not wrapped. |
| dispatch_io_set_high_water | function | Dispatch/io.h | Dispatch data / I/O helpers are not wrapped. |
| dispatch_io_set_interval | function | Dispatch/io.h | Dispatch data / I/O helpers are not wrapped. |
| dispatch_io_set_low_water | function | Dispatch/io.h | Dispatch data / I/O helpers are not wrapped. |
| dispatch_io_t | typedef struct | Dispatch/io.h | Dispatch data / I/O helpers are not wrapped. |
| dispatch_io_type_t | typedef struct | Dispatch/io.h | Dispatch data / I/O helpers are not wrapped. |
| dispatch_io_write | function | Dispatch/io.h | Dispatch data / I/O helpers are not wrapped. |
| dispatch_read | function | Dispatch/io.h | Dispatch data / I/O helpers are not wrapped. |
| dispatch_write | function | Dispatch/io.h | Dispatch data / I/O helpers are not wrapped. |
| OS_dispatch_source | protocol | Dispatch/object.h | No safe wrapper or bridge entry point was detected. |
| dispatch_activate | function | Dispatch/object.h | No safe wrapper or bridge entry point was detected. |
| dispatch_block_t | typedef struct | Dispatch/object.h | No safe wrapper or bridge entry point was detected. |
| dispatch_cancel | function | Dispatch/object.h | No safe wrapper or bridge entry point was detected. |
| dispatch_debug | function | Dispatch/object.h | No safe wrapper or bridge entry point was detected. |
| dispatch_debugv | function | Dispatch/object.h | No safe wrapper or bridge entry point was detected. |
| dispatch_get_context | function | Dispatch/object.h | No safe wrapper or bridge entry point was detected. |
| dispatch_notify | function | Dispatch/object.h | No safe wrapper or bridge entry point was detected. |
| dispatch_object_t | typedef struct | Dispatch/object.h | No safe wrapper or bridge entry point was detected. |
| dispatch_qos_class_t | typedef struct | Dispatch/object.h | No safe wrapper or bridge entry point was detected. |
| dispatch_release | function | Dispatch/object.h | No safe wrapper or bridge entry point was detected. |
| dispatch_resume | function | Dispatch/object.h | No safe wrapper or bridge entry point was detected. |
| dispatch_retain | function | Dispatch/object.h | No safe wrapper or bridge entry point was detected. |
| dispatch_set_context | function | Dispatch/object.h | No safe wrapper or bridge entry point was detected. |
| dispatch_set_finalizer_f | function | Dispatch/object.h | No safe wrapper or bridge entry point was detected. |
| dispatch_set_qos_class_floor | function | Dispatch/object.h | No safe wrapper or bridge entry point was detected. |
| dispatch_suspend | function | Dispatch/object.h | No safe wrapper or bridge entry point was detected. |
| dispatch_testcancel | function | Dispatch/object.h | No safe wrapper or bridge entry point was detected. |
| dispatch_wait | function | Dispatch/object.h | No safe wrapper or bridge entry point was detected. |
| dispatch_once | function | Dispatch/once.h | No safe wrapper or bridge entry point was detected. |
| dispatch_once_f | function | Dispatch/once.h | No safe wrapper or bridge entry point was detected. |
| dispatch_once_t | typedef struct | Dispatch/once.h | No safe wrapper or bridge entry point was detected. |
| dispatch_after | function | Dispatch/queue.h | No safe wrapper or bridge entry point was detected. |
| dispatch_after_f | function | Dispatch/queue.h | No safe wrapper or bridge entry point was detected. |
| dispatch_allow_send_signals | function | Dispatch/queue.h | No safe wrapper or bridge entry point was detected. |
| dispatch_assert_queue | function | Dispatch/queue.h | No safe wrapper or bridge entry point was detected. |
| dispatch_assert_queue_barrier | function | Dispatch/queue.h | No safe wrapper or bridge entry point was detected. |
| dispatch_assert_queue_not | function | Dispatch/queue.h | No safe wrapper or bridge entry point was detected. |
| dispatch_autorelease_frequency_t | typedef enum | Dispatch/queue.h | No safe wrapper or bridge entry point was detected. |
| dispatch_barrier_async | function | Dispatch/queue.h | No safe wrapper or bridge entry point was detected. |
| dispatch_barrier_async_and_wait | function | Dispatch/queue.h | No safe wrapper or bridge entry point was detected. |
| dispatch_barrier_async_and_wait_f | function | Dispatch/queue.h | No safe wrapper or bridge entry point was detected. |
| dispatch_barrier_async_f | function | Dispatch/queue.h | No safe wrapper or bridge entry point was detected. |
| dispatch_barrier_sync | function | Dispatch/queue.h | No safe wrapper or bridge entry point was detected. |
| dispatch_barrier_sync_f | function | Dispatch/queue.h | No safe wrapper or bridge entry point was detected. |
| dispatch_get_current_queue | function | Dispatch/queue.h | No safe wrapper or bridge entry point was detected. |
| dispatch_get_global_queue | function | Dispatch/queue.h | No safe wrapper or bridge entry point was detected. |
| dispatch_get_main_queue | function | Dispatch/queue.h | No safe wrapper or bridge entry point was detected. |
| dispatch_get_specific | function | Dispatch/queue.h | No safe wrapper or bridge entry point was detected. |
| dispatch_main | function | Dispatch/queue.h | No safe wrapper or bridge entry point was detected. |
| dispatch_queue_attr_make_initially_inactive | function | Dispatch/queue.h | DispatchQueue is covered, but this public declaration is not exposed. |
| dispatch_queue_attr_make_with_autorelease_frequency | function | Dispatch/queue.h | DispatchQueue is covered, but this public declaration is not exposed. |
| dispatch_queue_attr_make_with_qos_class | function | Dispatch/queue.h | DispatchQueue is covered, but this public declaration is not exposed. |
| dispatch_queue_attr_s | struct | Dispatch/queue.h | DispatchQueue is covered, but this public declaration is not exposed. |
| dispatch_queue_attr_t | typedef struct | Dispatch/queue.h | DispatchQueue is covered, but this public declaration is not exposed. |
| dispatch_queue_concurrent_t | typedef struct | Dispatch/queue.h | DispatchQueue is covered, but this public declaration is not exposed. |
| dispatch_queue_create_with_target | function | Dispatch/queue.h | DispatchQueue is covered, but this public declaration is not exposed. |
| dispatch_queue_get_label | function | Dispatch/queue.h | DispatchQueue is covered, but this public declaration is not exposed. |
| dispatch_queue_get_qos_class | function | Dispatch/queue.h | DispatchQueue is covered, but this public declaration is not exposed. |
| dispatch_queue_get_specific | function | Dispatch/queue.h | DispatchQueue is covered, but this public declaration is not exposed. |
| dispatch_queue_global_t | typedef struct | Dispatch/queue.h | DispatchQueue is covered, but this public declaration is not exposed. |
| dispatch_queue_main_t | typedef struct | Dispatch/queue.h | DispatchQueue is covered, but this public declaration is not exposed. |
| dispatch_queue_priority_t | typedef struct | Dispatch/queue.h | DispatchQueue is covered, but this public declaration is not exposed. |
| dispatch_queue_s | struct | Dispatch/queue.h | DispatchQueue is covered, but this public declaration is not exposed. |
| dispatch_queue_serial_executor_t | typedef struct | Dispatch/queue.h | DispatchQueue is covered, but this public declaration is not exposed. |
| dispatch_queue_serial_t | typedef struct | Dispatch/queue.h | DispatchQueue is covered, but this public declaration is not exposed. |
| dispatch_queue_set_specific | function | Dispatch/queue.h | DispatchQueue is covered, but this public declaration is not exposed. |
| dispatch_set_target_queue | function | Dispatch/queue.h | No safe wrapper or bridge entry point was detected. |
| dispatch_sync | function | Dispatch/queue.h | No safe wrapper or bridge entry point was detected. |
| dispatch_sync_f | function | Dispatch/queue.h | No safe wrapper or bridge entry point was detected. |
| dispatch_semaphore_create | function | Dispatch/semaphore.h | DispatchSemaphore is covered, but this public declaration is not exposed. |
| dispatch_semaphore_signal | function | Dispatch/semaphore.h | DispatchSemaphore is covered, but this public declaration is not exposed. |
| dispatch_semaphore_wait | function | Dispatch/semaphore.h | DispatchSemaphore is covered, but this public declaration is not exposed. |
| dispatch_source_cancel | function | Dispatch/source.h | DispatchSource is covered, but this public declaration is not exposed. |
| dispatch_source_create | function | Dispatch/source.h | DispatchSource is covered, but this public declaration is not exposed. |
| dispatch_source_get_data | function | Dispatch/source.h | DispatchSource is covered, but this public declaration is not exposed. |
| dispatch_source_get_handle | function | Dispatch/source.h | DispatchSource is covered, but this public declaration is not exposed. |
| dispatch_source_get_mask | function | Dispatch/source.h | DispatchSource is covered, but this public declaration is not exposed. |
| dispatch_source_mach_recv_flags_t | typedef struct | Dispatch/source.h | DispatchSource is covered, but this public declaration is not exposed. |
| dispatch_source_mach_send_flags_t | typedef struct | Dispatch/source.h | DispatchSource is covered, but this public declaration is not exposed. |
| dispatch_source_memorypressure_flags_t | typedef struct | Dispatch/source.h | DispatchSource is covered, but this public declaration is not exposed. |
| dispatch_source_merge_data | function | Dispatch/source.h | DispatchSource is covered, but this public declaration is not exposed. |
| dispatch_source_proc_flags_t | typedef struct | Dispatch/source.h | DispatchSource is covered, but this public declaration is not exposed. |
| dispatch_source_set_cancel_handler | function | Dispatch/source.h | DispatchSource is covered, but this public declaration is not exposed. |
| dispatch_source_set_cancel_handler_f | function | Dispatch/source.h | DispatchSource is covered, but this public declaration is not exposed. |
| dispatch_source_set_event_handler | function | Dispatch/source.h | DispatchSource is covered, but this public declaration is not exposed. |
| dispatch_source_set_event_handler_f | function | Dispatch/source.h | DispatchSource is covered, but this public declaration is not exposed. |
| dispatch_source_set_registration_handler | function | Dispatch/source.h | DispatchSource is covered, but this public declaration is not exposed. |
| dispatch_source_set_registration_handler_f | function | Dispatch/source.h | DispatchSource is covered, but this public declaration is not exposed. |
| dispatch_source_set_timer | function | Dispatch/source.h | DispatchSource is covered, but this public declaration is not exposed. |
| dispatch_source_testcancel | function | Dispatch/source.h | DispatchSource is covered, but this public declaration is not exposed. |
| dispatch_source_timer_flags_t | typedef struct | Dispatch/source.h | DispatchSource is covered, but this public declaration is not exposed. |
| dispatch_source_type_s | struct | Dispatch/source.h | DispatchSource is covered, but this public declaration is not exposed. |
| dispatch_source_type_t | typedef struct | Dispatch/source.h | DispatchSource is covered, but this public declaration is not exposed. |
| dispatch_source_vnode_flags_t | typedef struct | Dispatch/source.h | DispatchSource is covered, but this public declaration is not exposed. |
| dispatch_time | function | Dispatch/time.h | No safe wrapper or bridge entry point was detected. |
| dispatch_time_t | typedef struct | Dispatch/time.h | No safe wrapper or bridge entry point was detected. |
| dispatch_walltime | function | Dispatch/time.h | No safe wrapper or bridge entry point was detected. |
| dispatch_workloop_create | function | Dispatch/workloop.h | No safe wrapper or bridge entry point was detected. |
| dispatch_workloop_create_inactive | function | Dispatch/workloop.h | No safe wrapper or bridge entry point was detected. |
| dispatch_workloop_set_autorelease_frequency | function | Dispatch/workloop.h | No safe wrapper or bridge entry point was detected. |
| dispatch_workloop_set_os_workgroup | function | Dispatch/workloop.h | No safe wrapper or bridge entry point was detected. |
| dispatch_workloop_t | typedef struct | Dispatch/workloop.h | No safe wrapper or bridge entry point was detected. |
| IOSurfaceCreateXPCObject | function | IOSurface/IOSurfaceAPI.h | iosurface::IOSurface is covered, but this public declaration is not exposed. |
| IOSurfaceLookupFromXPCObject | function | IOSurface/IOSurfaceAPI.h | iosurface::IOSurface is covered, but this public declaration is not exposed. |
| IOSurfaceAlignProperty | function | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| IOSurfaceAllowsPixelSizeCasting | function | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| IOSurfaceComponentName | typedef enum | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| IOSurfaceComponentRange | typedef enum | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| IOSurfaceComponentType | typedef enum | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| IOSurfaceCopyAllValues | function | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| IOSurfaceCopyValue | function | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| IOSurfaceCreate | function | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| IOSurfaceCreateMachPort | function | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| IOSurfaceGetBitDepthOfComponentOfPlane | function | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| IOSurfaceGetBitOffsetOfComponentOfPlane | function | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| IOSurfaceGetNameOfComponentOfPlane | function | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| IOSurfaceGetNumberOfComponentsOfPlane | function | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| IOSurfaceGetPropertyAlignment | function | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| IOSurfaceGetPropertyMaximum | function | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| IOSurfaceGetRangeOfComponentOfPlane | function | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| IOSurfaceGetSubsampling | function | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| IOSurfaceGetTypeID | function | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| IOSurfaceGetTypeOfComponentOfPlane | function | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| IOSurfaceGetUseCount | function | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| IOSurfaceLookup | function | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| IOSurfaceLookupFromMachPort | function | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| IOSurfaceMemoryLedgerFlags | typedef enum | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| IOSurfaceMemoryLedgerTags | typedef enum | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| IOSurfaceRemoveAllValues | function | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| IOSurfaceRemoveValue | function | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| IOSurfaceSetOwnershipIdentity | function | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| IOSurfaceSetPurgeable | function | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| IOSurfaceSetValue | function | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| IOSurfaceSetValues | function | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| IOSurfaceSubsampling | typedef enum | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| kIOSurfaceAllocSize | constant | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| kIOSurfaceBytesPerElement | constant | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| kIOSurfaceBytesPerRow | constant | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| kIOSurfaceCacheMode | constant | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| kIOSurfaceColorSpace | constant | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| kIOSurfaceContentHeadroom | constant | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| kIOSurfaceElementHeight | constant | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| kIOSurfaceElementWidth | constant | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| kIOSurfaceHeight | constant | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| kIOSurfaceICCProfile | constant | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| kIOSurfaceName | constant | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| kIOSurfaceOffset | constant | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| kIOSurfacePixelFormat | constant | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| kIOSurfacePixelSizeCastingAllowed | constant | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| kIOSurfacePlaneBase | constant | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| kIOSurfacePlaneBitsPerElement | constant | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| kIOSurfacePlaneBytesPerElement | constant | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| kIOSurfacePlaneBytesPerRow | constant | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| kIOSurfacePlaneComponentBitDepths | constant | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| kIOSurfacePlaneComponentBitOffsets | constant | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| kIOSurfacePlaneComponentNames | constant | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| kIOSurfacePlaneComponentRanges | constant | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| kIOSurfacePlaneComponentTypes | constant | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| kIOSurfacePlaneElementHeight | constant | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| kIOSurfacePlaneElementWidth | constant | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| kIOSurfacePlaneHeight | constant | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| kIOSurfacePlaneInfo | constant | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| kIOSurfacePlaneOffset | constant | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| kIOSurfacePlaneSize | constant | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| kIOSurfacePlaneWidth | constant | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| kIOSurfaceSubsampling | constant | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| kIOSurfaceWidth | constant | IOSurface/IOSurfaceRef.h | Property dictionary / Mach-port / ownership helpers are missing. |
| IOSurfacePurgeabilityState | typedef enum | IOSurface/IOSurfaceTypes.h | iosurface::IOSurface is covered, but this public declaration is not exposed. |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| CFBundleCloseBundleResourceMap | function | CoreFoundation/CFBundle.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("The Carbon Resource Manager is deprecated. This should only be used to access Resource Manager-style resources in old bundles.", macosx(10.0, 10.15)) API_UNAVAILABLE(ios, watchos, tvos) |
| CFBundleOpenBundleResourceFiles | function | CoreFoundation/CFBundle.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("The Carbon Resource Manager is deprecated. This should only be used to access Resource Manager-style resources in old bundles.", macosx(10.0, 10.15)) API_UNAVAILABLE(ios, watchos, tvos) |
| CFBundleOpenBundleResourceMap | function | CoreFoundation/CFBundle.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("The Carbon Resource Manager is deprecated. This should only be used to access Resource Manager-style resources in old bundles.", macosx(10.0, 10.15)) API_UNAVAILABLE(ios, watchos, tvos) |
| CFAbsoluteTimeAddGregorianUnits | function | CoreFoundation/CFDate.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use CFCalendar or NSCalendar API instead", macos(10.4, 10.10), ios(2.0, 8.0), watchos(2.0, 2.0), tvos(9.0, 9.0)) |
| CFAbsoluteTimeGetDayOfWeek | function | CoreFoundation/CFDate.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use CFCalendar or NSCalendar API instead", macos(10.4, 10.10), ios(2.0, 8.0), watchos(2.0, 2.0), tvos(9.0, 9.0)) |
| CFAbsoluteTimeGetDayOfYear | function | CoreFoundation/CFDate.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use CFCalendar or NSCalendar API instead", macos(10.4, 10.10), ios(2.0, 8.0), watchos(2.0, 2.0), tvos(9.0, 9.0)) |
| CFAbsoluteTimeGetDifferenceAsGregorianUnits | function | CoreFoundation/CFDate.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use CFCalendar or NSCalendar API instead", macos(10.4, 10.10), ios(2.0, 8.0), watchos(2.0, 2.0), tvos(9.0, 9.0)) |
| CFAbsoluteTimeGetGregorianDate | function | CoreFoundation/CFDate.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use CFCalendar or NSCalendar API instead", macos(10.4, 10.10), ios(2.0, 8.0), watchos(2.0, 2.0), tvos(9.0, 9.0)) |
| CFAbsoluteTimeGetWeekOfYear | function | CoreFoundation/CFDate.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use CFCalendar or NSCalendar API instead", macos(10.4, 10.10), ios(2.0, 8.0), watchos(2.0, 2.0), tvos(9.0, 9.0)) |
| CFGregorianDate | typedef struct | CoreFoundation/CFDate.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use CFCalendar or NSCalendar API instead", macos(10.4, 10.10), ios(2.0, 8.0), watchos(2.0, 2.0), tvos(9.0, 9.0)) |
| CFGregorianDateGetAbsoluteTime | function | CoreFoundation/CFDate.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use CFCalendar or NSCalendar API instead", macos(10.4, 10.10), ios(2.0, 8.0), watchos(2.0, 2.0), tvos(9.0, 9.0)) |
| CFGregorianDateIsValid | function | CoreFoundation/CFDate.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use CFCalendar or NSCalendar API instead", macos(10.4, 10.10), ios(2.0, 8.0), watchos(2.0, 2.0), tvos(9.0, 9.0)) |
| CFGregorianUnitFlags | typedef enum | CoreFoundation/CFDate.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use CFCalendar or NSCalendar API instead", macos(10.4, 10.10), ios(2.0, 8.0), watchos(2.0, 2.0), tvos(9.0, 9.0)) |
| CFGregorianUnits | typedef struct | CoreFoundation/CFDate.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use CFCalendar or NSCalendar API instead", macos(10.4, 10.10), ios(2.0, 8.0), watchos(2.0, 2.0), tvos(9.0, 9.0)) |
| CFPreferencesCopyApplicationList | function | CoreFoundation/CFPreferences.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Unsupported API", macos(10.0,10.9), ios(2.0,7.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFPropertyListCreateFromStream | function | CoreFoundation/CFPropertyList.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use CFPropertyListCreateWithStream instead.", macos(10.2,10.10), ios(2.0,8.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFPropertyListCreateFromXMLData | function | CoreFoundation/CFPropertyList.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use CFPropertyListCreateWithData instead.", macos(10.0,10.10), ios(2.0,8.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFPropertyListCreateXMLData | function | CoreFoundation/CFPropertyList.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use CFPropertyListCreateData instead.", macos(10.0,10.10), ios(2.0,8.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFPropertyListWriteToStream | function | CoreFoundation/CFPropertyList.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use CFPropertyListWrite instead.", macos(10.2,10.10), ios(2.0,8.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFStreamCreatePairWithPeerSocketSignature | function | CoreFoundation/CFStream.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use nw_connection_t in Network framework instead", macos(10.1, API_TO_BE_DEPRECATED), ios(2.0, API_TO_BE_DEPRECATED), watchos(2.0, API_TO_BE_DEPRECATED), tvos(9.0, API_TO_BE_DEPRECATED)) |
| CFStreamCreatePairWithSocket | function | CoreFoundation/CFStream.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use nw_connection_t in Network framework instead", macos(10.1, API_TO_BE_DEPRECATED), ios(2.0, API_TO_BE_DEPRECATED), watchos(2.0, API_TO_BE_DEPRECATED), tvos(9.0, API_TO_BE_DEPRECATED)) |
| CFStreamCreatePairWithSocketToHost | function | CoreFoundation/CFStream.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use nw_connection_t in Network framework instead", macos(10.1, API_TO_BE_DEPRECATED), ios(2.0, API_TO_BE_DEPRECATED), watchos(2.0, API_TO_BE_DEPRECATED), tvos(9.0, API_TO_BE_DEPRECATED)) |
| kCFStreamSocketSecurityLevelSSLv2 | constant | CoreFoundation/CFStream.h | Deprecated on macOS; intentionally excluded from coverage targets. | CF_DEPRECATED(10_2, 10_12, 2_0, 10_0) |
| kCFStreamSocketSecurityLevelSSLv3 | constant | CoreFoundation/CFStream.h | Deprecated on macOS; intentionally excluded from coverage targets. | CF_DEPRECATED(10_2, 10_12, 2_0, 10_0) |
| CFURLCopyParameterString | function | CoreFoundation/CFURL.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("The CFURLCopyParameterString function is deprecated. Post deprecation for applications linked with or after the macOS 10.15, and for all iOS, watchOS, and tvOS applications, CFURLCopyParameterString will always return NULL, and the CFURLCopyPath(), CFURLCopyStrictPath(), and CFURLCopyFileSystemPath() functions will return the complete path including the semicolon separator and params component if the URL string contains them.", macosx(10.2,10.15), ios(2.0,13.0), watchos(2.0,6.0), tvos(9.0,13.0)) |
| CFURLCopyQueryString | function | CoreFoundation/CFURL.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("The CFURLCopyParameterString function is deprecated. Post deprecation for applications linked with or after the macOS 10.15, and for all iOS, watchOS, and tvOS applications, CFURLCopyParameterString will always return NULL, and the CFURLCopyPath(), CFURLCopyStrictPath(), and CFURLCopyFileSystemPath() functions will return the complete path including the semicolon separator and params component if the URL string contains them.", macosx(10.2,10.15), ios(2.0,13.0), watchos(2.0,6.0), tvos(9.0,13.0)) |
| CFURLCreateBookmarkDataFromAliasRecord | function | CoreFoundation/CFURL.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("The Carbon Alias Manager is deprecated. This function should only be used to convert Carbon AliasRecords to bookmark data.", macos(10.6,11.0)) API_UNAVAILABLE(ios, watchos, tvos) |
| CFURLCreateFromFSRef | function | CoreFoundation/CFURL.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Not supported", macos(10.0,10.9), ios(2.0,7.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFURLCreateStringByAddingPercentEscapes | function | CoreFoundation/CFURL.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use [NSString stringByAddingPercentEncodingWithAllowedCharacters:] instead, which always uses the recommended UTF-8 encoding, and which encodes for a specific URL component or subcomponent (since each URL component or subcomponent has different rules for what characters are valid).", macos(10.0,10.11), ios(2.0,9.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFURLCreateStringByReplacingPercentEscapesUsingEncoding | function | CoreFoundation/CFURL.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use [NSString stringByRemovingPercentEncoding] or CFURLCreateStringByReplacingPercentEscapes() instead, which always uses the recommended UTF-8 encoding.", macos(10.0,10.11), ios(2.0,9.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFURLGetFSRef | function | CoreFoundation/CFURL.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Not supported", macos(10.0,10.9), ios(2.0,7.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFURLPathStyle | typedef enum | CoreFoundation/CFURL.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Carbon File Manager is deprecated, use kCFURLPOSIXPathStyle where possible", macos(10.0,10.9), ios(2.0,7.0), watchos(2.0,2.0), tvos(9.0,9.0)), |
| kCFURLCustomIconKey | constant | CoreFoundation/CFURL.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use NSURLCustomIconKey", macosx(10.6, 10.12), ios(4.0,10.0), watchos(2.0,3.0), tvos(9.0,10.0)) |
| kCFURLEffectiveIconKey | constant | CoreFoundation/CFURL.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use NSURLEffectiveIconKey", macosx(10.6, 10.12), ios(4.0,10.0), watchos(2.0,3.0), tvos(9.0,10.0)) |
| kCFURLLabelColorKey | constant | CoreFoundation/CFURL.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use NSURLLabelColorKey", macosx(10.6, 10.12), ios(4.0,10.0), watchos(2.0,3.0), tvos(9.0,10.0)) |
| kCFURLTypeIdentifierKey | constant | CoreFoundation/CFURL.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use NSURLContentTypeKey instead", macos(10.6, API_TO_BE_DEPRECATED), ios(4.0, API_TO_BE_DEPRECATED), watchos(2.0, API_TO_BE_DEPRECATED), tvos(9.0, API_TO_BE_DEPRECATED)) |
| kCFURLUbiquitousItemDownloadingStatusKey | constant | CoreFoundation/CFURL.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use kCFURLUbiquitousItemDownloadingStatusKey instead", macos(10.7,10.9), ios(5.0,7.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| kCFURLUbiquitousItemIsDownloadedKey | constant | CoreFoundation/CFURL.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use kCFURLUbiquitousItemDownloadingStatusKey instead", macos(10.7,10.9), ios(5.0,7.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| kCFURLUbiquitousItemPercentDownloadedKey | constant | CoreFoundation/CFURL.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use NSMetadataQuery and NSMetadataUbiquitousItemPercentDownloadedKey on NSMetadataItem instead", macos(10.7,10.8), ios(5.0,6.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| kCFURLUbiquitousItemPercentUploadedKey | constant | CoreFoundation/CFURL.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use NSMetadataQuery and NSMetadataUbiquitousItemPercentUploadedKey on NSMetadataItem instead", macos(10.7,10.8), ios(5.0,6.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFURLCreateDataAndPropertiesFromResource | function | CoreFoundation/CFURLAccess.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("For resource data, use the CFReadStream API. For file resource properties, use CFURLCopyResourcePropertiesForKeys.", macos(10.0,10.9), ios(2.0,7.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFURLCreatePropertyFromResource | function | CoreFoundation/CFURLAccess.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("For file resource properties, use CFURLCopyResourcePropertyForKey.", macos(10.0,10.9), ios(2.0,7.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFURLDestroyResource | function | CoreFoundation/CFURLAccess.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use CFURLGetFileSystemRepresentation and removefile(3) instead.", macos(10.0,10.9), ios(2.0,7.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFURLWriteDataAndPropertiesToResource | function | CoreFoundation/CFURLAccess.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("For resource data, use the CFWriteStream API. For file resource properties, use CFURLSetResourcePropertiesForKeys.", macos(10.0,10.9), ios(2.0,7.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| kCFURLFileDirectoryContents | constant | CoreFoundation/CFURLAccess.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use CFURLResourceIsReachable instead.", macos(10.0,10.9), ios(2.0,7.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| kCFURLFileExists | constant | CoreFoundation/CFURLAccess.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use CFURLResourceIsReachable instead.", macos(10.0,10.9), ios(2.0,7.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| kCFURLFileLastModificationTime | constant | CoreFoundation/CFURLAccess.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use CFURLCopyResourcePropertyForKey with kCFURLFileSizeKey instead.", macos(10.0,10.9), ios(2.0,7.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| kCFURLFileLength | constant | CoreFoundation/CFURLAccess.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use the CFURLEnumerator API instead.", macos(10.0,10.9), ios(2.0,7.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| kCFURLFileOwnerID | constant | CoreFoundation/CFURLAccess.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use CFURLCopyResourcePropertyForKey with kCFURLFileSecurityKey and then the CFFileSecurity API instead.", macos(10.0,10.9), ios(2.0,7.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| kCFURLFilePOSIXMode | constant | CoreFoundation/CFURLAccess.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use CFURLCopyResourcePropertyForKey with kCFURLContentModificationDateKey instead.", macos(10.0,10.9), ios(2.0,7.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| kCFURLHTTPStatusCode | constant | CoreFoundation/CFURLAccess.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use CFURLCopyResourcePropertyForKey with kCFURLFileSecurityKey and then the CFFileSecurity API instead.", macos(10.0,10.9), ios(2.0,7.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| kCFURLHTTPStatusLine | constant | CoreFoundation/CFURLAccess.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use NSHTTPURLResponse methods instead.", macos(10.0,10.9), ios(2.0,7.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFURLEnumeratorGetSourceDidChange | function | CoreFoundation/CFURLEnumerator.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Use File System Events API instead", macos(10.6,10.7), ios(4.0,5.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFXMLNodeCreate | function | CoreFoundation/CFXMLNode.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED(__CFXMLNode_DEPRECATION_MSG, macos(10.0,10.8), ios(2.0,6.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFXMLNodeCreateCopy | function | CoreFoundation/CFXMLNode.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED(__CFXMLNode_DEPRECATION_MSG, macos(10.0,10.8), ios(2.0,6.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFXMLNodeGetInfoPtr | function | CoreFoundation/CFXMLNode.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED(__CFXMLNode_DEPRECATION_MSG, macos(10.0,10.8), ios(2.0,6.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFXMLNodeGetString | function | CoreFoundation/CFXMLNode.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED(__CFXMLNode_DEPRECATION_MSG, macos(10.0,10.8), ios(2.0,6.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFXMLNodeGetTypeCode | function | CoreFoundation/CFXMLNode.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED(__CFXMLNode_DEPRECATION_MSG, macos(10.0,10.8), ios(2.0,6.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFXMLNodeGetTypeID | function | CoreFoundation/CFXMLNode.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED(__CFXMLNode_DEPRECATION_MSG, macos(10.0,10.8), ios(2.0,6.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFXMLNodeGetVersion | function | CoreFoundation/CFXMLNode.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED(__CFXMLNode_DEPRECATION_MSG, macos(10.0,10.8), ios(2.0,6.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFXMLTreeCreateWithNode | function | CoreFoundation/CFXMLNode.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED(__CFXMLNode_DEPRECATION_MSG, macos(10.0,10.8), ios(2.0,6.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFXMLTreeGetNode | function | CoreFoundation/CFXMLNode.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED(__CFXMLNode_DEPRECATION_MSG, macos(10.0,10.8), ios(2.0,6.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFXMLParserAbort | function | CoreFoundation/CFXMLParser.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED(__CFXMLParser_DEPRECATION_MSG, macos(10.0,10.8), ios(2.0,6.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFXMLParserCopyErrorDescription | function | CoreFoundation/CFXMLParser.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED(__CFXMLParser_DEPRECATION_MSG, macos(10.0,10.8), ios(2.0,6.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFXMLParserCreate | function | CoreFoundation/CFXMLParser.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED(__CFXMLParser_DEPRECATION_MSG, macos(10.0,10.8), ios(2.0,6.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFXMLParserCreateWithDataFromURL | function | CoreFoundation/CFXMLParser.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED(__CFXMLParser_DEPRECATION_MSG, macos(10.0,10.8), ios(2.0,6.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFXMLParserGetCallBacks | function | CoreFoundation/CFXMLParser.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED(__CFXMLParser_DEPRECATION_MSG, macos(10.0,10.8), ios(2.0,6.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFXMLParserGetContext | function | CoreFoundation/CFXMLParser.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED(__CFXMLParser_DEPRECATION_MSG, macos(10.0,10.8), ios(2.0,6.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFXMLParserGetDocument | function | CoreFoundation/CFXMLParser.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED(__CFXMLParser_DEPRECATION_MSG, macos(10.0,10.8), ios(2.0,6.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFXMLParserGetLineNumber | function | CoreFoundation/CFXMLParser.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED(__CFXMLParser_DEPRECATION_MSG, macos(10.0,10.8), ios(2.0,6.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFXMLParserGetLocation | function | CoreFoundation/CFXMLParser.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED(__CFXMLParser_DEPRECATION_MSG, macos(10.0,10.8), ios(2.0,6.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFXMLParserGetSourceURL | function | CoreFoundation/CFXMLParser.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED(__CFXMLParser_DEPRECATION_MSG, macos(10.0,10.8), ios(2.0,6.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFXMLParserGetStatusCode | function | CoreFoundation/CFXMLParser.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED(__CFXMLParser_DEPRECATION_MSG, macos(10.0,10.8), ios(2.0,6.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFXMLParserGetTypeID | function | CoreFoundation/CFXMLParser.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED(__CFXMLParser_DEPRECATION_MSG, macos(10.0,10.8), ios(2.0,6.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFXMLParserParse | function | CoreFoundation/CFXMLParser.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED(__CFXMLParser_DEPRECATION_MSG, macos(10.0,10.8), ios(2.0,6.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFXMLTreeCreateFromData | function | CoreFoundation/CFXMLParser.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED(__CFXMLParser_DEPRECATION_MSG, macos(10.0,10.8), ios(2.0,6.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFXMLTreeCreateFromDataWithError | function | CoreFoundation/CFXMLParser.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED(__CFXMLParser_DEPRECATION_MSG, macos(10.0,10.8), ios(2.0,6.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFXMLTreeCreateWithDataFromURL | function | CoreFoundation/CFXMLParser.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED(__CFXMLParser_DEPRECATION_MSG, macos(10.0,10.8), ios(2.0,6.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CFXMLTreeCreateXMLData | function | CoreFoundation/CFXMLParser.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED(__CFXMLParser_DEPRECATION_MSG, macos(10.0,10.8), ios(2.0,6.0), watchos(2.0,2.0), tvos(9.0,9.0)) |
| CMTimebaseCopyMaster | function | CoreMedia/CMSync.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED_WITH_REPLACEMENT("CMTimebaseCopySource", macos(10.11,10.11), ios(9.0,9.0), tvos(9.0,9.0), watchos(6.0,6.0), visionos(1.0,1.0)) |
| CMTimebaseCopyMasterClock | function | CoreMedia/CMSync.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED_WITH_REPLACEMENT("CMTimebaseCopySourceClock", macos(10.11,10.11), ios(9.0,9.0), tvos(9.0,9.0), watchos(6.0,6.0), visionos(1.0,1.0)) |
| CMTimebaseCopyMasterTimebase | function | CoreMedia/CMSync.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED_WITH_REPLACEMENT("CMTimebaseCopySourceTimebase", macos(10.11,10.11), ios(9.0,9.0), tvos(9.0,9.0), watchos(6.0,6.0), visionos(1.0,1.0)) |
| CMTimebaseCopyUltimateMasterClock | function | CoreMedia/CMSync.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED_WITH_REPLACEMENT("CMTimebaseCopyUltimateSourceClock", macos(10.11,10.11), ios(9.0,9.0), tvos(9.0,9.0), watchos(6.0,6.0), visionos(1.0,1.0)) |
| CMTimebaseGetMaster | function | CoreMedia/CMSync.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED_WITH_REPLACEMENT("CMTimebaseCopySource", macos(10.8, 10.11), ios(6.0, 9.0), tvos(9.0, 9.0), visionos(1.0, 1.0)) API_UNAVAILABLE(watchos) |
| CMTimebaseGetMasterClock | function | CoreMedia/CMSync.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED_WITH_REPLACEMENT("CMTimebaseCopySourceClock", macos(10.8, 10.11), ios(6.0, 9.0), tvos(9.0, 9.0), visionos(1.0, 1.0)) API_UNAVAILABLE(watchos) |
| CMTimebaseGetMasterTimebase | function | CoreMedia/CMSync.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED_WITH_REPLACEMENT("CMTimebaseCopySourceTimebase", macos(10.8, 10.11), ios(6.0, 9.0), tvos(9.0, 9.0), visionos(1.0, 1.0)) API_UNAVAILABLE(watchos) |
| CMTimebaseGetUltimateMasterClock | function | CoreMedia/CMSync.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED_WITH_REPLACEMENT("CMTimebaseCopyUltimateSourceClock", macos(10.8, 10.11), ios(6.0, 9.0), tvos(9.0, 9.0), visionos(1.0, 1.0)) API_UNAVAILABLE(watchos) |
| CVBufferGetAttachment | function | CoreVideo/CVBuffer.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED_WITH_REPLACEMENT("CVBufferCopyAttachment", macos(10.4, 12.0), ios(4.0,15.0), tvos(9.0, 15.0), watchos(4.0, 8.0)) |
| CVBufferGetAttachments | function | CoreVideo/CVBuffer.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED_WITH_REPLACEMENT("CVBufferCopyAttachments", macos(10.4, 12.0), ios(4.0,15.0), tvos(9.0, 15.0), watchos(4.0, 8.0)) |
| CVDisplayLinkGetTypeID | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED_BEGIN("use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) ", macos(10.4, 15.0)) |
| CVDisplayLinkRelease | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED_END |
| CVDisplayLinkCreateWithActiveCGDisplays | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED_BEGIN("use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) ", macos(10.4, 15.0)) |
| CVDisplayLinkCreateWithCGDisplay | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED_BEGIN("use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) ", macos(10.4, 15.0)) |
| CVDisplayLinkCreateWithCGDisplays | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED_BEGIN("use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) ", macos(10.4, 15.0)) |
| CVDisplayLinkCreateWithOpenGLDisplayMask | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED_BEGIN("use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) ", macos(10.4, 15.0)) |
| CVDisplayLinkGetActualOutputVideoRefreshPeriod | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED_BEGIN("use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) ", macos(10.4, 15.0)) |
| CVDisplayLinkGetCurrentCGDisplay | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED_BEGIN("use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) ", macos(10.4, 15.0)) |
| CVDisplayLinkGetCurrentTime | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED_BEGIN("use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) ", macos(10.4, 15.0)) |
| CVDisplayLinkGetNominalOutputVideoRefreshPeriod | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED_BEGIN("use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) ", macos(10.4, 15.0)) |
| CVDisplayLinkGetOutputVideoLatency | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED_BEGIN("use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) ", macos(10.4, 15.0)) |
| CVDisplayLinkIsRunning | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED_BEGIN("use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) ", macos(10.4, 15.0)) |
| CVDisplayLinkRef | typedef struct | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED_BEGIN("use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) ", macos(10.4, 15.0)) |
| CVDisplayLinkRetain | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED_BEGIN("use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) ", macos(10.4, 15.0)) |
| CVDisplayLinkSetCurrentCGDisplay | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED_BEGIN("use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) ", macos(10.4, 15.0)) |
| CVDisplayLinkSetCurrentCGDisplayFromOpenGLContext | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED_BEGIN("use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) ", macos(10.4, 15.0)) |
| CVDisplayLinkSetOutputCallback | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED_BEGIN("use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) ", macos(10.4, 15.0)) |
| CVDisplayLinkSetOutputHandler | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED_BEGIN("use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) ", macos(10.4, 15.0)) |
| CVDisplayLinkStart | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED_BEGIN("use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) ", macos(10.4, 15.0)) |
| CVDisplayLinkStop | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED_BEGIN("use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) ", macos(10.4, 15.0)) |
| CVDisplayLinkTranslateTime | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED_BEGIN("use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) ", macos(10.4, 15.0)) |
| CVTimeStamp | typedef struct | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED_BEGIN("use NSView.displayLink(target:selector:), NSWindow.displayLink(target:selector:), or NSScreen.displayLink(target:selector:) ", macos(10.4, 15.0)) |
| kCVImageBufferTransferFunction_EBU_3213 | constant | CoreVideo/CVImageBuffer.h | Deprecated on macOS; intentionally excluded from coverage targets. | AVAILABLE_BUT_DEPRECATED(__MAC_10_5,__MAC_10_6,__IPHONE_NA,__IPHONE_NA) |
| kCVImageBufferTransferFunction_ITU_R_2020 | constant | CoreVideo/CVImageBuffer.h | Deprecated on macOS; intentionally excluded from coverage targets. | AVAILABLE_BUT_DEPRECATED(__MAC_10_5,__MAC_10_6,__IPHONE_NA,__IPHONE_NA) |
| kCVImageBufferTransferFunction_SMPTE_240M_1995 | constant | CoreVideo/CVImageBuffer.h | Deprecated on macOS; intentionally excluded from coverage targets. | AVAILABLE_BUT_DEPRECATED(__MAC_10_5,__MAC_10_6,__IPHONE_NA,__IPHONE_NA) |
| kCVImageBufferTransferFunction_SMPTE_C | constant | CoreVideo/CVImageBuffer.h | Deprecated on macOS; intentionally excluded from coverage targets. | AVAILABLE_BUT_DEPRECATED(__MAC_10_5,__MAC_10_6,__IPHONE_NA,__IPHONE_NA) |
| kCVImageBufferTransferFunction_UseGamma | constant | CoreVideo/CVImageBuffer.h | Deprecated on macOS; intentionally excluded from coverage targets. | AVAILABLE_BUT_DEPRECATED(__MAC_10_5,__MAC_10_6,__IPHONE_NA,__IPHONE_NA) |
| kCVImageBufferTransferFunction_sRGB | constant | CoreVideo/CVImageBuffer.h | Deprecated on macOS; intentionally excluded from coverage targets. | AVAILABLE_BUT_DEPRECATED(__MAC_10_5,__MAC_10_6,__IPHONE_NA,__IPHONE_NA) |
| kCVImageBufferYCbCrMatrix_DCI_P3 | constant | CoreVideo/CVImageBuffer.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("kCVImageBufferYCbCrMatrix_DCI_P3 no longer supported.", macos(10.11, 11.0), ios(9.0, 14.0)) |
| kCVImageBufferYCbCrMatrix_ITU_R_2020 | constant | CoreVideo/CVImageBuffer.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("kCVImageBufferYCbCrMatrix_DCI_P3 no longer supported.", macos(10.11, 11.0), ios(9.0, 14.0)) |
| kCVImageBufferYCbCrMatrix_ITU_R_601_4 | constant | CoreVideo/CVImageBuffer.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("kCVImageBufferYCbCrMatrix_DCI_P3 no longer supported.", macos(10.11, 11.0), ios(9.0, 14.0)) |
| kCVImageBufferYCbCrMatrix_P3_D65 | constant | CoreVideo/CVImageBuffer.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("kCVImageBufferYCbCrMatrix_DCI_P3 no longer supported.", macos(10.11, 11.0), ios(9.0, 14.0)) |
| kCVImageBufferYCbCrMatrix_SMPTE_240M_1995 | constant | CoreVideo/CVImageBuffer.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("kCVImageBufferYCbCrMatrix_DCI_P3 no longer supported.", macos(10.11, 11.0), ios(9.0, 14.0)) |
| kIOSurfaceIsGlobal | constant | IOSurface/IOSurfaceRef.h | Deprecated on macOS; intentionally excluded from coverage targets. | API_DEPRECATED("Global surfaces are insecure",macos(10.6,10.11), ios(11.0,11.0), watchos(4.0,4.0), tvos(11.0,11.0)) |
