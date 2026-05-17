# apple-cf-rs coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 2865
VERIFIED: 2727
GAPS: 0
EXEMPT: 138
COVERAGE_PCT: 95.18%
COVERABLE_SYMBOLS: 2727
COVERABLE_COVERAGE_PCT: 100.00%

## Notes

- Combined audit of CoreFoundation, CoreMedia, CoreVideo, IOSurface, and Dispatch.
- Public declarations were enumerated from MacOSX26.2.sdk via clang AST + header scans.
- Deprecated/unavailable declarations and non-C-ABI Objective-C protocol markers remain tracked as EXEMPT.
- VERIFIED means the declaration is directly referenced by a safe wrapper implementation or exposed by the exhaustive `apple_cf::raw` module added in v0.6.2.
- Rows whose header provenance resolved to `?` are tracked under the `Unscoped` bucket in the framework breakdown.
- v0.6.2 closes every remaining coverable gap; the only exempt rows are deprecated, unavailable, private inline helpers, or Objective-C protocol markers with no standalone C ABI.

## Framework breakdown

| Framework | Verified | Gaps | Exempt |
| --- | ---: | ---: | ---: |
| CoreFoundation | 1283 | 0 | 79 |
| CoreMedia | 776 | 0 | 8 |
| CoreVideo | 284 | 0 | 35 |
| IOSurface | 92 | 0 | 1 |
| Dispatch | 149 | 0 | 1 |
| Unscoped (`?`) | 143 | 0 | 14 |

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| CFAbsoluteTimeGetCurrent | function | CoreFoundation/CFDate.h | cf::CFDate |
| CFAllocatorAllocate | function | CoreFoundation/CFBase.h | raw::CFAllocatorAllocate |
| CFAllocatorAllocateBytes | function | CoreFoundation/CFBase.h | raw::CFAllocatorAllocateBytes |
| CFAllocatorAllocateTyped | function | CoreFoundation/CFBase.h | raw::CFAllocatorAllocateTyped |
| CFAllocatorContext | typedef struct | CoreFoundation/CFBase.h | raw::CFAllocatorContext |
| CFAllocatorCreate | function | CoreFoundation/CFBase.h | raw::CFAllocatorCreate |
| CFAllocatorDeallocate | function | CoreFoundation/CFBase.h | raw::CFAllocatorDeallocate |
| CFAllocatorGetContext | function | CoreFoundation/CFBase.h | raw::CFAllocatorGetContext |
| CFAllocatorGetDefault | function | CoreFoundation/CFBase.h | raw::CFAllocatorGetDefault |
| CFAllocatorGetPreferredSizeForSize | function | CoreFoundation/CFBase.h | raw::CFAllocatorGetPreferredSizeForSize |
| CFAllocatorGetTypeID | function | CoreFoundation/CFBase.h | raw::CFAllocatorGetTypeID |
| CFAllocatorReallocate | function | CoreFoundation/CFBase.h | raw::CFAllocatorReallocate |
| CFAllocatorReallocateBytes | function | CoreFoundation/CFBase.h | raw::CFAllocatorReallocateBytes |
| CFAllocatorReallocateTyped | function | CoreFoundation/CFBase.h | raw::CFAllocatorReallocateTyped |
| CFAllocatorRef | typedef struct | CoreFoundation/CFBase.h | raw::CFAllocatorRef |
| CFAllocatorSetDefault | function | CoreFoundation/CFBase.h | raw::CFAllocatorSetDefault |
| CFArrayAppendArray | function | CoreFoundation/CFArray.h | raw::CFArrayAppendArray |
| CFArrayAppendValue | function | CoreFoundation/CFArray.h | raw::CFArrayAppendValue |
| CFArrayApplyFunction | function | CoreFoundation/CFArray.h | raw::CFArrayApplyFunction |
| CFArrayBSearchValues | function | CoreFoundation/CFArray.h | raw::CFArrayBSearchValues |
| CFArrayCallBacks | typedef struct | CoreFoundation/CFArray.h | raw::CFArrayCallBacks |
| CFArrayContainsValue | function | CoreFoundation/CFArray.h | raw::CFArrayContainsValue |
| CFArrayCreate | function | CoreFoundation/CFArray.h | cf::CFArray |
| CFArrayCreateCopy | function | CoreFoundation/CFArray.h | raw::CFArrayCreateCopy |
| CFArrayCreateMutable | function | CoreFoundation/CFArray.h | raw::CFArrayCreateMutable |
| CFArrayCreateMutableCopy | function | CoreFoundation/CFArray.h | raw::CFArrayCreateMutableCopy |
| CFArrayExchangeValuesAtIndices | function | CoreFoundation/CFArray.h | raw::CFArrayExchangeValuesAtIndices |
| CFArrayGetCount | function | CoreFoundation/CFArray.h | cf::CFArray |
| CFArrayGetCountOfValue | function | CoreFoundation/CFArray.h | raw::CFArrayGetCountOfValue |
| CFArrayGetFirstIndexOfValue | function | CoreFoundation/CFArray.h | raw::CFArrayGetFirstIndexOfValue |
| CFArrayGetLastIndexOfValue | function | CoreFoundation/CFArray.h | raw::CFArrayGetLastIndexOfValue |
| CFArrayGetTypeID | function | CoreFoundation/CFArray.h | cf::CFArray |
| CFArrayGetValueAtIndex | function | CoreFoundation/CFArray.h | cf::CFArray |
| CFArrayGetValues | function | CoreFoundation/CFArray.h | raw::CFArrayGetValues |
| CFArrayInsertValueAtIndex | function | CoreFoundation/CFArray.h | raw::CFArrayInsertValueAtIndex |
| CFArrayRef | typedef struct | CoreFoundation/CFArray.h | cf::CFArray |
| CFArrayRemoveAllValues | function | CoreFoundation/CFArray.h | raw::CFArrayRemoveAllValues |
| CFArrayRemoveValueAtIndex | function | CoreFoundation/CFArray.h | raw::CFArrayRemoveValueAtIndex |
| CFArrayReplaceValues | function | CoreFoundation/CFArray.h | raw::CFArrayReplaceValues |
| CFArraySetValueAtIndex | function | CoreFoundation/CFArray.h | raw::CFArraySetValueAtIndex |
| CFArraySortValues | function | CoreFoundation/CFArray.h | raw::CFArraySortValues |
| CFAttributedStringBeginEditing | function | CoreFoundation/CFAttributedString.h | raw::CFAttributedStringBeginEditing |
| CFAttributedStringCreate | function | CoreFoundation/CFAttributedString.h | cf::CFAttributedString |
| CFAttributedStringCreateCopy | function | CoreFoundation/CFAttributedString.h | raw::CFAttributedStringCreateCopy |
| CFAttributedStringCreateMutable | function | CoreFoundation/CFAttributedString.h | raw::CFAttributedStringCreateMutable |
| CFAttributedStringCreateMutableCopy | function | CoreFoundation/CFAttributedString.h | raw::CFAttributedStringCreateMutableCopy |
| CFAttributedStringCreateWithSubstring | function | CoreFoundation/CFAttributedString.h | raw::CFAttributedStringCreateWithSubstring |
| CFAttributedStringEndEditing | function | CoreFoundation/CFAttributedString.h | raw::CFAttributedStringEndEditing |
| CFAttributedStringGetAttribute | function | CoreFoundation/CFAttributedString.h | raw::CFAttributedStringGetAttribute |
| CFAttributedStringGetAttributeAndLongestEffectiveRange | function | CoreFoundation/CFAttributedString.h | raw::CFAttributedStringGetAttributeAndLongestEffectiveRange |
| CFAttributedStringGetAttributes | function | CoreFoundation/CFAttributedString.h | raw::CFAttributedStringGetAttributes |
| CFAttributedStringGetAttributesAndLongestEffectiveRange | function | CoreFoundation/CFAttributedString.h | raw::CFAttributedStringGetAttributesAndLongestEffectiveRange |
| CFAttributedStringGetBidiLevelsAndResolvedDirections | function | CoreFoundation/CFAttributedString.h | raw::CFAttributedStringGetBidiLevelsAndResolvedDirections |
| CFAttributedStringGetLength | function | CoreFoundation/CFAttributedString.h | cf::CFAttributedString |
| CFAttributedStringGetMutableString | function | CoreFoundation/CFAttributedString.h | raw::CFAttributedStringGetMutableString |
| CFAttributedStringGetStatisticalWritingDirections | function | CoreFoundation/CFAttributedString.h | raw::CFAttributedStringGetStatisticalWritingDirections |
| CFAttributedStringGetString | function | CoreFoundation/CFAttributedString.h | cf::CFAttributedString |
| CFAttributedStringGetTypeID | function | CoreFoundation/CFAttributedString.h | cf::CFAttributedString |
| CFAttributedStringRef | typedef struct | CoreFoundation/CFAttributedString.h | cf::CFAttributedString |
| CFAttributedStringRemoveAttribute | function | CoreFoundation/CFAttributedString.h | raw::CFAttributedStringRemoveAttribute |
| CFAttributedStringReplaceAttributedString | function | CoreFoundation/CFAttributedString.h | raw::CFAttributedStringReplaceAttributedString |
| CFAttributedStringReplaceString | function | CoreFoundation/CFAttributedString.h | raw::CFAttributedStringReplaceString |
| CFAttributedStringSetAttribute | function | CoreFoundation/CFAttributedString.h | raw::CFAttributedStringSetAttribute |
| CFAttributedStringSetAttributes | function | CoreFoundation/CFAttributedString.h | raw::CFAttributedStringSetAttributes |
| CFAutorelease | function | CoreFoundation/CFBase.h | raw::CFAutorelease |
| CFBagAddValue | function | CoreFoundation/CFBag.h | raw::CFBagAddValue |
| CFBagApplyFunction | function | CoreFoundation/CFBag.h | raw::CFBagApplyFunction |
| CFBagCallBacks | typedef struct | CoreFoundation/CFBag.h | raw::CFBagCallBacks |
| CFBagContainsValue | function | CoreFoundation/CFBag.h | cf::CFBag |
| CFBagCreate | function | CoreFoundation/CFBag.h | cf::CFBag |
| CFBagCreateCopy | function | CoreFoundation/CFBag.h | raw::CFBagCreateCopy |
| CFBagCreateMutable | function | CoreFoundation/CFBag.h | raw::CFBagCreateMutable |
| CFBagCreateMutableCopy | function | CoreFoundation/CFBag.h | raw::CFBagCreateMutableCopy |
| CFBagGetCount | function | CoreFoundation/CFBag.h | cf::CFBag |
| CFBagGetCountOfValue | function | CoreFoundation/CFBag.h | cf::CFBag |
| CFBagGetTypeID | function | CoreFoundation/CFBag.h | cf::CFBag |
| CFBagGetValue | function | CoreFoundation/CFBag.h | raw::CFBagGetValue |
| CFBagGetValueIfPresent | function | CoreFoundation/CFBag.h | raw::CFBagGetValueIfPresent |
| CFBagGetValues | function | CoreFoundation/CFBag.h | raw::CFBagGetValues |
| CFBagRef | typedef struct | CoreFoundation/CFBag.h | cf::CFBag |
| CFBagRemoveAllValues | function | CoreFoundation/CFBag.h | raw::CFBagRemoveAllValues |
| CFBagRemoveValue | function | CoreFoundation/CFBag.h | raw::CFBagRemoveValue |
| CFBagReplaceValue | function | CoreFoundation/CFBag.h | raw::CFBagReplaceValue |
| CFBagSetValue | function | CoreFoundation/CFBag.h | raw::CFBagSetValue |
| CFBinaryHeapAddValue | function | CoreFoundation/CFBinaryHeap.h | raw::CFBinaryHeapAddValue |
| CFBinaryHeapApplyFunction | function | CoreFoundation/CFBinaryHeap.h | raw::CFBinaryHeapApplyFunction |
| CFBinaryHeapCallBacks | typedef struct | CoreFoundation/CFBinaryHeap.h | raw::CFBinaryHeapCallBacks |
| CFBinaryHeapCompareContext | typedef struct | CoreFoundation/CFBinaryHeap.h | raw::CFBinaryHeapCompareContext |
| CFBinaryHeapContainsValue | function | CoreFoundation/CFBinaryHeap.h | raw::CFBinaryHeapContainsValue |
| CFBinaryHeapCreate | function | CoreFoundation/CFBinaryHeap.h | raw::CFBinaryHeapCreate |
| CFBinaryHeapCreateCopy | function | CoreFoundation/CFBinaryHeap.h | raw::CFBinaryHeapCreateCopy |
| CFBinaryHeapGetCount | function | CoreFoundation/CFBinaryHeap.h | raw::CFBinaryHeapGetCount |
| CFBinaryHeapGetCountOfValue | function | CoreFoundation/CFBinaryHeap.h | raw::CFBinaryHeapGetCountOfValue |
| CFBinaryHeapGetMinimum | function | CoreFoundation/CFBinaryHeap.h | raw::CFBinaryHeapGetMinimum |
| CFBinaryHeapGetMinimumIfPresent | function | CoreFoundation/CFBinaryHeap.h | raw::CFBinaryHeapGetMinimumIfPresent |
| CFBinaryHeapGetTypeID | function | CoreFoundation/CFBinaryHeap.h | raw::CFBinaryHeapGetTypeID |
| CFBinaryHeapGetValues | function | CoreFoundation/CFBinaryHeap.h | raw::CFBinaryHeapGetValues |
| CFBinaryHeapRef | typedef struct | CoreFoundation/CFBinaryHeap.h | raw::CFBinaryHeapRef |
| CFBinaryHeapRemoveAllValues | function | CoreFoundation/CFBinaryHeap.h | raw::CFBinaryHeapRemoveAllValues |
| CFBinaryHeapRemoveMinimumValue | function | CoreFoundation/CFBinaryHeap.h | raw::CFBinaryHeapRemoveMinimumValue |
| CFBitVectorContainsBit | function | CoreFoundation/CFBitVector.h | raw::CFBitVectorContainsBit |
| CFBitVectorCreate | function | CoreFoundation/CFBitVector.h | raw::CFBitVectorCreate |
| CFBitVectorCreateCopy | function | CoreFoundation/CFBitVector.h | raw::CFBitVectorCreateCopy |
| CFBitVectorCreateMutable | function | CoreFoundation/CFBitVector.h | raw::CFBitVectorCreateMutable |
| CFBitVectorCreateMutableCopy | function | CoreFoundation/CFBitVector.h | raw::CFBitVectorCreateMutableCopy |
| CFBitVectorFlipBitAtIndex | function | CoreFoundation/CFBitVector.h | raw::CFBitVectorFlipBitAtIndex |
| CFBitVectorFlipBits | function | CoreFoundation/CFBitVector.h | raw::CFBitVectorFlipBits |
| CFBitVectorGetBitAtIndex | function | CoreFoundation/CFBitVector.h | raw::CFBitVectorGetBitAtIndex |
| CFBitVectorGetBits | function | CoreFoundation/CFBitVector.h | raw::CFBitVectorGetBits |
| CFBitVectorGetCount | function | CoreFoundation/CFBitVector.h | raw::CFBitVectorGetCount |
| CFBitVectorGetCountOfBit | function | CoreFoundation/CFBitVector.h | raw::CFBitVectorGetCountOfBit |
| CFBitVectorGetFirstIndexOfBit | function | CoreFoundation/CFBitVector.h | raw::CFBitVectorGetFirstIndexOfBit |
| CFBitVectorGetLastIndexOfBit | function | CoreFoundation/CFBitVector.h | raw::CFBitVectorGetLastIndexOfBit |
| CFBitVectorGetTypeID | function | CoreFoundation/CFBitVector.h | raw::CFBitVectorGetTypeID |
| CFBitVectorRef | typedef struct | CoreFoundation/CFBitVector.h | raw::CFBitVectorRef |
| CFBitVectorSetAllBits | function | CoreFoundation/CFBitVector.h | raw::CFBitVectorSetAllBits |
| CFBitVectorSetBitAtIndex | function | CoreFoundation/CFBitVector.h | raw::CFBitVectorSetBitAtIndex |
| CFBitVectorSetBits | function | CoreFoundation/CFBitVector.h | raw::CFBitVectorSetBits |
| CFBitVectorSetCount | function | CoreFoundation/CFBitVector.h | raw::CFBitVectorSetCount |
| CFBooleanGetTypeID | function | CoreFoundation/CFNumber.h | raw::CFBooleanGetTypeID |
| CFBooleanGetValue | function | CoreFoundation/CFNumber.h | raw::CFBooleanGetValue |
| CFBooleanRef | typedef struct | CoreFoundation/CFNumber.h | raw::CFBooleanRef |
| CFBundleCopyAuxiliaryExecutableURL | function | CoreFoundation/CFBundle.h | raw::CFBundleCopyAuxiliaryExecutableURL |
| CFBundleCopyBuiltInPlugInsURL | function | CoreFoundation/CFBundle.h | raw::CFBundleCopyBuiltInPlugInsURL |
| CFBundleCopyBundleLocalizations | function | CoreFoundation/CFBundle.h | raw::CFBundleCopyBundleLocalizations |
| CFBundleCopyBundleURL | function | CoreFoundation/CFBundle.h | cf::CFBundle |
| CFBundleCopyExecutableArchitectures | function | CoreFoundation/CFBundle.h | raw::CFBundleCopyExecutableArchitectures |
| CFBundleCopyExecutableArchitecturesForURL | function | CoreFoundation/CFBundle.h | raw::CFBundleCopyExecutableArchitecturesForURL |
| CFBundleCopyExecutableURL | function | CoreFoundation/CFBundle.h | raw::CFBundleCopyExecutableURL |
| CFBundleCopyInfoDictionaryForURL | function | CoreFoundation/CFBundle.h | raw::CFBundleCopyInfoDictionaryForURL |
| CFBundleCopyInfoDictionaryInDirectory | function | CoreFoundation/CFBundle.h | raw::CFBundleCopyInfoDictionaryInDirectory |
| CFBundleCopyLocalizationsForPreferences | function | CoreFoundation/CFBundle.h | raw::CFBundleCopyLocalizationsForPreferences |
| CFBundleCopyLocalizationsForURL | function | CoreFoundation/CFBundle.h | raw::CFBundleCopyLocalizationsForURL |
| CFBundleCopyLocalizedString | function | CoreFoundation/CFBundle.h | raw::CFBundleCopyLocalizedString |
| CFBundleCopyLocalizedStringForLocalizations | function | CoreFoundation/CFBundle.h | raw::CFBundleCopyLocalizedStringForLocalizations |
| CFBundleCopyPreferredLocalizationsFromArray | function | CoreFoundation/CFBundle.h | raw::CFBundleCopyPreferredLocalizationsFromArray |
| CFBundleCopyPrivateFrameworksURL | function | CoreFoundation/CFBundle.h | raw::CFBundleCopyPrivateFrameworksURL |
| CFBundleCopyResourceURL | function | CoreFoundation/CFBundle.h | cf::CFBundle |
| CFBundleCopyResourceURLForLocalization | function | CoreFoundation/CFBundle.h | raw::CFBundleCopyResourceURLForLocalization |
| CFBundleCopyResourceURLInDirectory | function | CoreFoundation/CFBundle.h | raw::CFBundleCopyResourceURLInDirectory |
| CFBundleCopyResourceURLsOfType | function | CoreFoundation/CFBundle.h | raw::CFBundleCopyResourceURLsOfType |
| CFBundleCopyResourceURLsOfTypeForLocalization | function | CoreFoundation/CFBundle.h | raw::CFBundleCopyResourceURLsOfTypeForLocalization |
| CFBundleCopyResourceURLsOfTypeInDirectory | function | CoreFoundation/CFBundle.h | raw::CFBundleCopyResourceURLsOfTypeInDirectory |
| CFBundleCopyResourcesDirectoryURL | function | CoreFoundation/CFBundle.h | raw::CFBundleCopyResourcesDirectoryURL |
| CFBundleCopySharedFrameworksURL | function | CoreFoundation/CFBundle.h | raw::CFBundleCopySharedFrameworksURL |
| CFBundleCopySharedSupportURL | function | CoreFoundation/CFBundle.h | raw::CFBundleCopySharedSupportURL |
| CFBundleCopySupportFilesDirectoryURL | function | CoreFoundation/CFBundle.h | raw::CFBundleCopySupportFilesDirectoryURL |
| CFBundleCreate | function | CoreFoundation/CFBundle.h | cf::CFBundle |
| CFBundleCreateBundlesFromDirectory | function | CoreFoundation/CFBundle.h | raw::CFBundleCreateBundlesFromDirectory |
| CFBundleGetAllBundles | function | CoreFoundation/CFBundle.h | raw::CFBundleGetAllBundles |
| CFBundleGetBundleWithIdentifier | function | CoreFoundation/CFBundle.h | raw::CFBundleGetBundleWithIdentifier |
| CFBundleGetDataPointerForName | function | CoreFoundation/CFBundle.h | raw::CFBundleGetDataPointerForName |
| CFBundleGetDataPointersForNames | function | CoreFoundation/CFBundle.h | raw::CFBundleGetDataPointersForNames |
| CFBundleGetDevelopmentRegion | function | CoreFoundation/CFBundle.h | raw::CFBundleGetDevelopmentRegion |
| CFBundleGetFunctionPointerForName | function | CoreFoundation/CFBundle.h | raw::CFBundleGetFunctionPointerForName |
| CFBundleGetFunctionPointersForNames | function | CoreFoundation/CFBundle.h | raw::CFBundleGetFunctionPointersForNames |
| CFBundleGetIdentifier | function | CoreFoundation/CFBundle.h | cf::CFBundle |
| CFBundleGetInfoDictionary | function | CoreFoundation/CFBundle.h | raw::CFBundleGetInfoDictionary |
| CFBundleGetLocalInfoDictionary | function | CoreFoundation/CFBundle.h | raw::CFBundleGetLocalInfoDictionary |
| CFBundleGetMainBundle | function | CoreFoundation/CFBundle.h | cf::CFBundle |
| CFBundleGetPackageInfo | function | CoreFoundation/CFBundle.h | raw::CFBundleGetPackageInfo |
| CFBundleGetPackageInfoInDirectory | function | CoreFoundation/CFBundle.h | raw::CFBundleGetPackageInfoInDirectory |
| CFBundleGetPlugIn | function | CoreFoundation/CFBundle.h | raw::CFBundleGetPlugIn |
| CFBundleGetTypeID | function | CoreFoundation/CFBundle.h | cf::CFBundle |
| CFBundleGetValueForInfoDictionaryKey | function | CoreFoundation/CFBundle.h | raw::CFBundleGetValueForInfoDictionaryKey |
| CFBundleGetVersionNumber | function | CoreFoundation/CFBundle.h | raw::CFBundleGetVersionNumber |
| CFBundleIsArchitectureLoadable | function | CoreFoundation/CFBundle.h | raw::CFBundleIsArchitectureLoadable |
| CFBundleIsExecutableLoadable | function | CoreFoundation/CFBundle.h | raw::CFBundleIsExecutableLoadable |
| CFBundleIsExecutableLoadableForURL | function | CoreFoundation/CFBundle.h | raw::CFBundleIsExecutableLoadableForURL |
| CFBundleIsExecutableLoaded | function | CoreFoundation/CFBundle.h | raw::CFBundleIsExecutableLoaded |
| CFBundleLoadExecutable | function | CoreFoundation/CFBundle.h | raw::CFBundleLoadExecutable |
| CFBundleLoadExecutableAndReturnError | function | CoreFoundation/CFBundle.h | raw::CFBundleLoadExecutableAndReturnError |
| CFBundlePreflightExecutable | function | CoreFoundation/CFBundle.h | raw::CFBundlePreflightExecutable |
| CFBundleRef | typedef struct | CoreFoundation/CFBundle.h | cf::CFBundle |
| CFBundleUnloadExecutable | function | CoreFoundation/CFBundle.h | raw::CFBundleUnloadExecutable |
| CFByteOrderGetCurrent | function | CoreFoundation/CFByteOrder.h | raw::CFByteOrderGetCurrent |
| CFCalendarAddComponents | function | CoreFoundation/CFCalendar.h | raw::CFCalendarAddComponents |
| CFCalendarComposeAbsoluteTime | function | CoreFoundation/CFCalendar.h | raw::CFCalendarComposeAbsoluteTime |
| CFCalendarCopyCurrent | function | CoreFoundation/CFCalendar.h | cf::CFCalendar |
| CFCalendarCopyLocale | function | CoreFoundation/CFCalendar.h | raw::CFCalendarCopyLocale |
| CFCalendarCopyTimeZone | function | CoreFoundation/CFCalendar.h | cf::CFCalendar |
| CFCalendarCreateWithIdentifier | function | CoreFoundation/CFCalendar.h | cf::CFCalendar |
| CFCalendarDecomposeAbsoluteTime | function | CoreFoundation/CFCalendar.h | raw::CFCalendarDecomposeAbsoluteTime |
| CFCalendarGetComponentDifference | function | CoreFoundation/CFCalendar.h | raw::CFCalendarGetComponentDifference |
| CFCalendarGetFirstWeekday | function | CoreFoundation/CFCalendar.h | raw::CFCalendarGetFirstWeekday |
| CFCalendarGetIdentifier | function | CoreFoundation/CFCalendar.h | cf::CFCalendar |
| CFCalendarGetMaximumRangeOfUnit | function | CoreFoundation/CFCalendar.h | raw::CFCalendarGetMaximumRangeOfUnit |
| CFCalendarGetMinimumDaysInFirstWeek | function | CoreFoundation/CFCalendar.h | raw::CFCalendarGetMinimumDaysInFirstWeek |
| CFCalendarGetMinimumRangeOfUnit | function | CoreFoundation/CFCalendar.h | raw::CFCalendarGetMinimumRangeOfUnit |
| CFCalendarGetOrdinalityOfUnit | function | CoreFoundation/CFCalendar.h | raw::CFCalendarGetOrdinalityOfUnit |
| CFCalendarGetRangeOfUnit | function | CoreFoundation/CFCalendar.h | raw::CFCalendarGetRangeOfUnit |
| CFCalendarGetTimeRangeOfUnit | function | CoreFoundation/CFCalendar.h | raw::CFCalendarGetTimeRangeOfUnit |
| CFCalendarGetTypeID | function | CoreFoundation/CFCalendar.h | cf::CFCalendar |
| CFCalendarRef | typedef struct | CoreFoundation/CFCalendar.h | cf::CFCalendar |
| CFCalendarSetFirstWeekday | function | CoreFoundation/CFCalendar.h | raw::CFCalendarSetFirstWeekday |
| CFCalendarSetLocale | function | CoreFoundation/CFCalendar.h | raw::CFCalendarSetLocale |
| CFCalendarSetMinimumDaysInFirstWeek | function | CoreFoundation/CFCalendar.h | raw::CFCalendarSetMinimumDaysInFirstWeek |
| CFCalendarSetTimeZone | function | CoreFoundation/CFCalendar.h | cf::CFCalendar |
| CFCalendarUnit | typedef enum | CoreFoundation/CFCalendar.h | raw::CFCalendarUnit |
| CFCharacterSetAddCharactersInRange | function | CoreFoundation/CFCharacterSet.h | raw::CFCharacterSetAddCharactersInRange |
| CFCharacterSetAddCharactersInString | function | CoreFoundation/CFCharacterSet.h | raw::CFCharacterSetAddCharactersInString |
| CFCharacterSetCreateBitmapRepresentation | function | CoreFoundation/CFCharacterSet.h | raw::CFCharacterSetCreateBitmapRepresentation |
| CFCharacterSetCreateCopy | function | CoreFoundation/CFCharacterSet.h | raw::CFCharacterSetCreateCopy |
| CFCharacterSetCreateInvertedSet | function | CoreFoundation/CFCharacterSet.h | cf::CFCharacterSet |
| CFCharacterSetCreateMutable | function | CoreFoundation/CFCharacterSet.h | raw::CFCharacterSetCreateMutable |
| CFCharacterSetCreateMutableCopy | function | CoreFoundation/CFCharacterSet.h | raw::CFCharacterSetCreateMutableCopy |
| CFCharacterSetCreateWithBitmapRepresentation | function | CoreFoundation/CFCharacterSet.h | raw::CFCharacterSetCreateWithBitmapRepresentation |
| CFCharacterSetCreateWithCharactersInRange | function | CoreFoundation/CFCharacterSet.h | raw::CFCharacterSetCreateWithCharactersInRange |
| CFCharacterSetCreateWithCharactersInString | function | CoreFoundation/CFCharacterSet.h | cf::CFCharacterSet |
| CFCharacterSetGetPredefined | function | CoreFoundation/CFCharacterSet.h | raw::CFCharacterSetGetPredefined |
| CFCharacterSetGetTypeID | function | CoreFoundation/CFCharacterSet.h | cf::CFCharacterSet |
| CFCharacterSetHasMemberInPlane | function | CoreFoundation/CFCharacterSet.h | raw::CFCharacterSetHasMemberInPlane |
| CFCharacterSetIntersect | function | CoreFoundation/CFCharacterSet.h | raw::CFCharacterSetIntersect |
| CFCharacterSetInvert | function | CoreFoundation/CFCharacterSet.h | raw::CFCharacterSetInvert |
| CFCharacterSetIsCharacterMember | function | CoreFoundation/CFCharacterSet.h | cf::CFCharacterSet |
| CFCharacterSetIsLongCharacterMember | function | CoreFoundation/CFCharacterSet.h | raw::CFCharacterSetIsLongCharacterMember |
| CFCharacterSetIsSupersetOfSet | function | CoreFoundation/CFCharacterSet.h | raw::CFCharacterSetIsSupersetOfSet |
| CFCharacterSetPredefinedSet | typedef enum | CoreFoundation/CFCharacterSet.h | raw::CFCharacterSetPredefinedSet |
| CFCharacterSetRef | typedef struct | CoreFoundation/CFCharacterSet.h | cf::CFCharacterSet |
| CFCharacterSetRemoveCharactersInRange | function | CoreFoundation/CFCharacterSet.h | raw::CFCharacterSetRemoveCharactersInRange |
| CFCharacterSetRemoveCharactersInString | function | CoreFoundation/CFCharacterSet.h | raw::CFCharacterSetRemoveCharactersInString |
| CFCharacterSetUnion | function | CoreFoundation/CFCharacterSet.h | raw::CFCharacterSetUnion |
| CFComparisonResult | typedef enum | CoreFoundation/CFBase.h | raw::CFComparisonResult |
| CFConvertDoubleHostToSwapped | function | CoreFoundation/CFByteOrder.h | raw::CFConvertDoubleHostToSwapped |
| CFConvertDoubleSwappedToHost | function | CoreFoundation/CFByteOrder.h | raw::CFConvertDoubleSwappedToHost |
| CFConvertFloat32HostToSwapped | function | CoreFoundation/CFByteOrder.h | raw::CFConvertFloat32HostToSwapped |
| CFConvertFloat32SwappedToHost | function | CoreFoundation/CFByteOrder.h | raw::CFConvertFloat32SwappedToHost |
| CFConvertFloat64HostToSwapped | function | CoreFoundation/CFByteOrder.h | raw::CFConvertFloat64HostToSwapped |
| CFConvertFloat64SwappedToHost | function | CoreFoundation/CFByteOrder.h | raw::CFConvertFloat64SwappedToHost |
| CFConvertFloatHostToSwapped | function | CoreFoundation/CFByteOrder.h | raw::CFConvertFloatHostToSwapped |
| CFConvertFloatSwappedToHost | function | CoreFoundation/CFByteOrder.h | raw::CFConvertFloatSwappedToHost |
| CFCopyDescription | function | CoreFoundation/CFBase.h | cf::CFType |
| CFCopyTypeIDDescription | function | CoreFoundation/CFBase.h | raw::CFCopyTypeIDDescription |
| CFDataAppendBytes | function | CoreFoundation/CFData.h | raw::CFDataAppendBytes |
| CFDataCreate | function | CoreFoundation/CFData.h | cf::CFData |
| CFDataCreateCopy | function | CoreFoundation/CFData.h | raw::CFDataCreateCopy |
| CFDataCreateMutable | function | CoreFoundation/CFData.h | raw::CFDataCreateMutable |
| CFDataCreateMutableCopy | function | CoreFoundation/CFData.h | raw::CFDataCreateMutableCopy |
| CFDataCreateWithBytesNoCopy | function | CoreFoundation/CFData.h | raw::CFDataCreateWithBytesNoCopy |
| CFDataDeleteBytes | function | CoreFoundation/CFData.h | raw::CFDataDeleteBytes |
| CFDataFind | function | CoreFoundation/CFData.h | raw::CFDataFind |
| CFDataGetBytePtr | function | CoreFoundation/CFData.h | cf::CFData |
| CFDataGetBytes | function | CoreFoundation/CFData.h | raw::CFDataGetBytes |
| CFDataGetLength | function | CoreFoundation/CFData.h | cf::CFData |
| CFDataGetMutableBytePtr | function | CoreFoundation/CFData.h | raw::CFDataGetMutableBytePtr |
| CFDataGetTypeID | function | CoreFoundation/CFData.h | cf::CFData |
| CFDataIncreaseLength | function | CoreFoundation/CFData.h | raw::CFDataIncreaseLength |
| CFDataRef | typedef struct | CoreFoundation/CFData.h | cf::CFData |
| CFDataReplaceBytes | function | CoreFoundation/CFData.h | raw::CFDataReplaceBytes |
| CFDataSearchFlags | typedef enum | CoreFoundation/CFData.h | raw::CFDataSearchFlags |
| CFDataSetLength | function | CoreFoundation/CFData.h | raw::CFDataSetLength |
| CFDateCompare | function | CoreFoundation/CFDate.h | raw::CFDateCompare |
| CFDateCreate | function | CoreFoundation/CFDate.h | cf::CFDate |
| CFDateFormatterCopyProperty | function | CoreFoundation/CFDateFormatter.h | raw::CFDateFormatterCopyProperty |
| CFDateFormatterCreate | function | CoreFoundation/CFDateFormatter.h | cf::CFDateFormatter |
| CFDateFormatterCreateDateFormatFromTemplate | function | CoreFoundation/CFDateFormatter.h | raw::CFDateFormatterCreateDateFormatFromTemplate |
| CFDateFormatterCreateDateFromString | function | CoreFoundation/CFDateFormatter.h | raw::CFDateFormatterCreateDateFromString |
| CFDateFormatterCreateISO8601Formatter | function | CoreFoundation/CFDateFormatter.h | raw::CFDateFormatterCreateISO8601Formatter |
| CFDateFormatterCreateStringWithAbsoluteTime | function | CoreFoundation/CFDateFormatter.h | raw::CFDateFormatterCreateStringWithAbsoluteTime |
| CFDateFormatterCreateStringWithDate | function | CoreFoundation/CFDateFormatter.h | cf::CFDateFormatter |
| CFDateFormatterGetAbsoluteTimeFromString | function | CoreFoundation/CFDateFormatter.h | raw::CFDateFormatterGetAbsoluteTimeFromString |
| CFDateFormatterGetDateStyle | function | CoreFoundation/CFDateFormatter.h | raw::CFDateFormatterGetDateStyle |
| CFDateFormatterGetFormat | function | CoreFoundation/CFDateFormatter.h | raw::CFDateFormatterGetFormat |
| CFDateFormatterGetLocale | function | CoreFoundation/CFDateFormatter.h | raw::CFDateFormatterGetLocale |
| CFDateFormatterGetTimeStyle | function | CoreFoundation/CFDateFormatter.h | raw::CFDateFormatterGetTimeStyle |
| CFDateFormatterGetTypeID | function | CoreFoundation/CFDateFormatter.h | cf::CFDateFormatter |
| CFDateFormatterRef | typedef struct | CoreFoundation/CFDateFormatter.h | cf::CFDateFormatter |
| CFDateFormatterSetFormat | function | CoreFoundation/CFDateFormatter.h | raw::CFDateFormatterSetFormat |
| CFDateFormatterSetProperty | function | CoreFoundation/CFDateFormatter.h | raw::CFDateFormatterSetProperty |
| CFDateFormatterStyle | typedef enum | CoreFoundation/CFDateFormatter.h | cf::CFDateFormatter |
| CFDateGetAbsoluteTime | function | CoreFoundation/CFDate.h | cf::CFDate |
| CFDateGetTimeIntervalSinceDate | function | CoreFoundation/CFDate.h | raw::CFDateGetTimeIntervalSinceDate |
| CFDateGetTypeID | function | CoreFoundation/CFDate.h | cf::CFDate |
| CFDateRef | typedef struct | CoreFoundation/CFDate.h | cf::CFDate |
| CFDictionaryAddValue | function | CoreFoundation/CFDictionary.h | raw::CFDictionaryAddValue |
| CFDictionaryApplyFunction | function | CoreFoundation/CFDictionary.h | raw::CFDictionaryApplyFunction |
| CFDictionaryContainsKey | function | CoreFoundation/CFDictionary.h | cf::CFDictionary |
| CFDictionaryContainsValue | function | CoreFoundation/CFDictionary.h | raw::CFDictionaryContainsValue |
| CFDictionaryCreate | function | CoreFoundation/CFDictionary.h | cf::CFDictionary |
| CFDictionaryCreateCopy | function | CoreFoundation/CFDictionary.h | raw::CFDictionaryCreateCopy |
| CFDictionaryCreateMutable | function | CoreFoundation/CFDictionary.h | raw::CFDictionaryCreateMutable |
| CFDictionaryCreateMutableCopy | function | CoreFoundation/CFDictionary.h | raw::CFDictionaryCreateMutableCopy |
| CFDictionaryGetCount | function | CoreFoundation/CFDictionary.h | cf::CFDictionary |
| CFDictionaryGetCountOfKey | function | CoreFoundation/CFDictionary.h | raw::CFDictionaryGetCountOfKey |
| CFDictionaryGetCountOfValue | function | CoreFoundation/CFDictionary.h | raw::CFDictionaryGetCountOfValue |
| CFDictionaryGetKeysAndValues | function | CoreFoundation/CFDictionary.h | cf::CFDictionary |
| CFDictionaryGetTypeID | function | CoreFoundation/CFDictionary.h | cf::CFDictionary |
| CFDictionaryGetValue | function | CoreFoundation/CFDictionary.h | cf::CFDictionary |
| CFDictionaryGetValueIfPresent | function | CoreFoundation/CFDictionary.h | raw::CFDictionaryGetValueIfPresent |
| CFDictionaryKeyCallBacks | typedef struct | CoreFoundation/CFDictionary.h | raw::CFDictionaryKeyCallBacks |
| CFDictionaryRef | typedef struct | CoreFoundation/CFDictionary.h | cf::CFDictionary |
| CFDictionaryRemoveAllValues | function | CoreFoundation/CFDictionary.h | raw::CFDictionaryRemoveAllValues |
| CFDictionaryRemoveValue | function | CoreFoundation/CFDictionary.h | raw::CFDictionaryRemoveValue |
| CFDictionaryReplaceValue | function | CoreFoundation/CFDictionary.h | raw::CFDictionaryReplaceValue |
| CFDictionarySetValue | function | CoreFoundation/CFDictionary.h | raw::CFDictionarySetValue |
| CFDictionaryValueCallBacks | typedef struct | CoreFoundation/CFDictionary.h | raw::CFDictionaryValueCallBacks |
| CFEqual | function | CoreFoundation/CFBase.h | cf::CFType |
| CFErrorCopyDescription | function | CoreFoundation/CFError.h | cf::CFError |
| CFErrorCopyFailureReason | function | CoreFoundation/CFError.h | cf::CFError |
| CFErrorCopyRecoverySuggestion | function | CoreFoundation/CFError.h | raw::CFErrorCopyRecoverySuggestion |
| CFErrorCopyUserInfo | function | CoreFoundation/CFError.h | raw::CFErrorCopyUserInfo |
| CFErrorCreate | function | CoreFoundation/CFError.h | cf::CFError |
| CFErrorCreateWithUserInfoKeysAndValues | function | CoreFoundation/CFError.h | raw::CFErrorCreateWithUserInfoKeysAndValues |
| CFErrorGetCode | function | CoreFoundation/CFError.h | cf::CFError |
| CFErrorGetDomain | function | CoreFoundation/CFError.h | cf::CFError |
| CFErrorGetTypeID | function | CoreFoundation/CFError.h | cf::CFError |
| CFErrorRef | typedef struct | CoreFoundation/CFError.h | cf::CFError |
| CFFileDescriptorContext | typedef struct | CoreFoundation/CFFileDescriptor.h | raw::CFFileDescriptorContext |
| CFFileDescriptorCreate | function | CoreFoundation/CFFileDescriptor.h | cf::CFFileDescriptor |
| CFFileDescriptorCreateRunLoopSource | function | CoreFoundation/CFFileDescriptor.h | raw::CFFileDescriptorCreateRunLoopSource |
| CFFileDescriptorDisableCallBacks | function | CoreFoundation/CFFileDescriptor.h | raw::CFFileDescriptorDisableCallBacks |
| CFFileDescriptorEnableCallBacks | function | CoreFoundation/CFFileDescriptor.h | raw::CFFileDescriptorEnableCallBacks |
| CFFileDescriptorGetContext | function | CoreFoundation/CFFileDescriptor.h | raw::CFFileDescriptorGetContext |
| CFFileDescriptorGetNativeDescriptor | function | CoreFoundation/CFFileDescriptor.h | cf::CFFileDescriptor |
| CFFileDescriptorGetTypeID | function | CoreFoundation/CFFileDescriptor.h | cf::CFFileDescriptor |
| CFFileDescriptorInvalidate | function | CoreFoundation/CFFileDescriptor.h | cf::CFFileDescriptor |
| CFFileDescriptorIsValid | function | CoreFoundation/CFFileDescriptor.h | raw::CFFileDescriptorIsValid |
| CFFileDescriptorRef | typedef struct | CoreFoundation/CFFileDescriptor.h | cf::CFFileDescriptor |
| CFFileSecurityClearOptions | typedef enum | CoreFoundation/CFFileSecurity.h | raw::CFFileSecurityClearOptions |
| CFFileSecurityClearProperties | function | CoreFoundation/CFFileSecurity.h | raw::CFFileSecurityClearProperties |
| CFFileSecurityCopyAccessControlList | function | CoreFoundation/CFFileSecurity.h | raw::CFFileSecurityCopyAccessControlList |
| CFFileSecurityCopyGroupUUID | function | CoreFoundation/CFFileSecurity.h | raw::CFFileSecurityCopyGroupUUID |
| CFFileSecurityCopyOwnerUUID | function | CoreFoundation/CFFileSecurity.h | cf::CFFileSecurity |
| CFFileSecurityCreate | function | CoreFoundation/CFFileSecurity.h | cf::CFFileSecurity |
| CFFileSecurityCreateCopy | function | CoreFoundation/CFFileSecurity.h | raw::CFFileSecurityCreateCopy |
| CFFileSecurityGetGroup | function | CoreFoundation/CFFileSecurity.h | raw::CFFileSecurityGetGroup |
| CFFileSecurityGetMode | function | CoreFoundation/CFFileSecurity.h | cf::CFFileSecurity |
| CFFileSecurityGetOwner | function | CoreFoundation/CFFileSecurity.h | raw::CFFileSecurityGetOwner |
| CFFileSecurityGetTypeID | function | CoreFoundation/CFFileSecurity.h | cf::CFFileSecurity |
| CFFileSecurityRef | typedef struct | CoreFoundation/CFFileSecurity.h | cf::CFFileSecurity |
| CFFileSecuritySetAccessControlList | function | CoreFoundation/CFFileSecurity.h | raw::CFFileSecuritySetAccessControlList |
| CFFileSecuritySetGroup | function | CoreFoundation/CFFileSecurity.h | raw::CFFileSecuritySetGroup |
| CFFileSecuritySetGroupUUID | function | CoreFoundation/CFFileSecurity.h | raw::CFFileSecuritySetGroupUUID |
| CFFileSecuritySetMode | function | CoreFoundation/CFFileSecurity.h | cf::CFFileSecurity |
| CFFileSecuritySetOwner | function | CoreFoundation/CFFileSecurity.h | raw::CFFileSecuritySetOwner |
| CFFileSecuritySetOwnerUUID | function | CoreFoundation/CFFileSecurity.h | cf::CFFileSecurity |
| CFGetAllocator | function | CoreFoundation/CFBase.h | raw::CFGetAllocator |
| CFGetRetainCount | function | CoreFoundation/CFBase.h | raw::CFGetRetainCount |
| CFGetTypeID | function | CoreFoundation/CFBase.h | cf::CFType |
| CFHash | function | CoreFoundation/CFBase.h | cf::CFType |
| CFISO8601DateFormatOptions | typedef enum | CoreFoundation/CFDateFormatter.h | raw::CFISO8601DateFormatOptions |
| CFLocaleCopyAvailableLocaleIdentifiers | function | CoreFoundation/CFLocale.h | raw::CFLocaleCopyAvailableLocaleIdentifiers |
| CFLocaleCopyCommonISOCurrencyCodes | function | CoreFoundation/CFLocale.h | raw::CFLocaleCopyCommonISOCurrencyCodes |
| CFLocaleCopyCurrent | function | CoreFoundation/CFLocale.h | cf::CFLocale |
| CFLocaleCopyDisplayNameForPropertyValue | function | CoreFoundation/CFLocale.h | raw::CFLocaleCopyDisplayNameForPropertyValue |
| CFLocaleCopyISOCountryCodes | function | CoreFoundation/CFLocale.h | raw::CFLocaleCopyISOCountryCodes |
| CFLocaleCopyISOCurrencyCodes | function | CoreFoundation/CFLocale.h | raw::CFLocaleCopyISOCurrencyCodes |
| CFLocaleCopyISOLanguageCodes | function | CoreFoundation/CFLocale.h | raw::CFLocaleCopyISOLanguageCodes |
| CFLocaleCopyPreferredLanguages | function | CoreFoundation/CFLocale.h | raw::CFLocaleCopyPreferredLanguages |
| CFLocaleCreate | function | CoreFoundation/CFLocale.h | cf::CFLocale |
| CFLocaleCreateCanonicalLanguageIdentifierFromString | function | CoreFoundation/CFLocale.h | raw::CFLocaleCreateCanonicalLanguageIdentifierFromString |
| CFLocaleCreateCanonicalLocaleIdentifierFromScriptManagerCodes | function | CoreFoundation/CFLocale.h | raw::CFLocaleCreateCanonicalLocaleIdentifierFromScriptManagerCodes |
| CFLocaleCreateCanonicalLocaleIdentifierFromString | function | CoreFoundation/CFLocale.h | raw::CFLocaleCreateCanonicalLocaleIdentifierFromString |
| CFLocaleCreateComponentsFromLocaleIdentifier | function | CoreFoundation/CFLocale.h | raw::CFLocaleCreateComponentsFromLocaleIdentifier |
| CFLocaleCreateCopy | function | CoreFoundation/CFLocale.h | raw::CFLocaleCreateCopy |
| CFLocaleCreateLocaleIdentifierFromComponents | function | CoreFoundation/CFLocale.h | raw::CFLocaleCreateLocaleIdentifierFromComponents |
| CFLocaleCreateLocaleIdentifierFromWindowsLocaleCode | function | CoreFoundation/CFLocale.h | raw::CFLocaleCreateLocaleIdentifierFromWindowsLocaleCode |
| CFLocaleGetIdentifier | function | CoreFoundation/CFLocale.h | cf::CFLocale |
| CFLocaleGetLanguageCharacterDirection | function | CoreFoundation/CFLocale.h | raw::CFLocaleGetLanguageCharacterDirection |
| CFLocaleGetLanguageLineDirection | function | CoreFoundation/CFLocale.h | raw::CFLocaleGetLanguageLineDirection |
| CFLocaleGetSystem | function | CoreFoundation/CFLocale.h | raw::CFLocaleGetSystem |
| CFLocaleGetTypeID | function | CoreFoundation/CFLocale.h | cf::CFLocale |
| CFLocaleGetValue | function | CoreFoundation/CFLocale.h | raw::CFLocaleGetValue |
| CFLocaleGetWindowsLocaleCodeFromLocaleIdentifier | function | CoreFoundation/CFLocale.h | raw::CFLocaleGetWindowsLocaleCodeFromLocaleIdentifier |
| CFLocaleLanguageDirection | typedef enum | CoreFoundation/CFLocale.h | raw::CFLocaleLanguageDirection |
| CFLocaleRef | typedef struct | CoreFoundation/CFLocale.h | cf::CFLocale |
| CFMachPortContext | typedef struct | CoreFoundation/CFMachPort.h | raw::CFMachPortContext |
| CFMachPortCreate | function | CoreFoundation/CFMachPort.h | raw::CFMachPortCreate |
| CFMachPortCreateRunLoopSource | function | CoreFoundation/CFMachPort.h | raw::CFMachPortCreateRunLoopSource |
| CFMachPortCreateWithPort | function | CoreFoundation/CFMachPort.h | raw::CFMachPortCreateWithPort |
| CFMachPortGetContext | function | CoreFoundation/CFMachPort.h | raw::CFMachPortGetContext |
| CFMachPortGetInvalidationCallBack | function | CoreFoundation/CFMachPort.h | raw::CFMachPortGetInvalidationCallBack |
| CFMachPortGetPort | function | CoreFoundation/CFMachPort.h | raw::CFMachPortGetPort |
| CFMachPortGetTypeID | function | CoreFoundation/CFMachPort.h | raw::CFMachPortGetTypeID |
| CFMachPortInvalidate | function | CoreFoundation/CFMachPort.h | raw::CFMachPortInvalidate |
| CFMachPortIsValid | function | CoreFoundation/CFMachPort.h | raw::CFMachPortIsValid |
| CFMachPortRef | typedef struct | CoreFoundation/CFMachPort.h | raw::CFMachPortRef |
| CFMachPortSetInvalidationCallBack | function | CoreFoundation/CFMachPort.h | raw::CFMachPortSetInvalidationCallBack |
| CFMakeCollectable | function | CoreFoundation/CFBase.h | raw::CFMakeCollectable |
| CFMessagePortContext | typedef struct | CoreFoundation/CFMessagePort.h | raw::CFMessagePortContext |
| CFMessagePortCreateLocal | function | CoreFoundation/CFMessagePort.h | cf::CFMessagePort |
| CFMessagePortCreateRemote | function | CoreFoundation/CFMessagePort.h | cf::CFMessagePort |
| CFMessagePortCreateRunLoopSource | function | CoreFoundation/CFMessagePort.h | cf::CFMessagePort |
| CFMessagePortGetContext | function | CoreFoundation/CFMessagePort.h | raw::CFMessagePortGetContext |
| CFMessagePortGetInvalidationCallBack | function | CoreFoundation/CFMessagePort.h | raw::CFMessagePortGetInvalidationCallBack |
| CFMessagePortGetName | function | CoreFoundation/CFMessagePort.h | raw::CFMessagePortGetName |
| CFMessagePortGetTypeID | function | CoreFoundation/CFMessagePort.h | cf::CFMessagePort |
| CFMessagePortInvalidate | function | CoreFoundation/CFMessagePort.h | cf::CFMessagePort |
| CFMessagePortIsRemote | function | CoreFoundation/CFMessagePort.h | raw::CFMessagePortIsRemote |
| CFMessagePortIsValid | function | CoreFoundation/CFMessagePort.h | raw::CFMessagePortIsValid |
| CFMessagePortRef | typedef struct | CoreFoundation/CFMessagePort.h | cf::CFMessagePort |
| CFMessagePortSendRequest | function | CoreFoundation/CFMessagePort.h | cf::CFMessagePort |
| CFMessagePortSetDispatchQueue | function | CoreFoundation/CFMessagePort.h | raw::CFMessagePortSetDispatchQueue |
| CFMessagePortSetInvalidationCallBack | function | CoreFoundation/CFMessagePort.h | raw::CFMessagePortSetInvalidationCallBack |
| CFMessagePortSetName | function | CoreFoundation/CFMessagePort.h | raw::CFMessagePortSetName |
| CFMutableArrayRef | typedef struct | CoreFoundation/CFArray.h | raw::CFMutableArrayRef |
| CFMutableAttributedStringRef | typedef struct | CoreFoundation/CFAttributedString.h | raw::CFMutableAttributedStringRef |
| CFMutableBagRef | typedef struct | CoreFoundation/CFBag.h | raw::CFMutableBagRef |
| CFMutableBitVectorRef | typedef struct | CoreFoundation/CFBitVector.h | raw::CFMutableBitVectorRef |
| CFMutableCharacterSetRef | typedef struct | CoreFoundation/CFCharacterSet.h | raw::CFMutableCharacterSetRef |
| CFMutableDataRef | typedef struct | CoreFoundation/CFData.h | raw::CFMutableDataRef |
| CFMutableDictionaryRef | typedef struct | CoreFoundation/CFDictionary.h | raw::CFMutableDictionaryRef |
| CFMutableSetRef | typedef struct | CoreFoundation/CFSet.h | cf::CFMutableSet |
| CFMutableStringRef | typedef struct | CoreFoundation/CFBase.h | raw::CFMutableStringRef |
| CFNotificationCenterAddObserver | function | CoreFoundation/CFNotificationCenter.h | raw::CFNotificationCenterAddObserver |
| CFNotificationCenterGetDarwinNotifyCenter | function | CoreFoundation/CFNotificationCenter.h | cf::CFNotificationCenter |
| CFNotificationCenterGetDistributedCenter | function | CoreFoundation/CFNotificationCenter.h | cf::CFNotificationCenter |
| CFNotificationCenterGetLocalCenter | function | CoreFoundation/CFNotificationCenter.h | cf::CFNotificationCenter |
| CFNotificationCenterGetTypeID | function | CoreFoundation/CFNotificationCenter.h | cf::CFNotificationCenter |
| CFNotificationCenterPostNotification | function | CoreFoundation/CFNotificationCenter.h | cf::CFNotificationCenter |
| CFNotificationCenterPostNotificationWithOptions | function | CoreFoundation/CFNotificationCenter.h | raw::CFNotificationCenterPostNotificationWithOptions |
| CFNotificationCenterRef | typedef struct | CoreFoundation/CFNotificationCenter.h | cf::CFNotificationCenter |
| CFNotificationCenterRemoveEveryObserver | function | CoreFoundation/CFNotificationCenter.h | raw::CFNotificationCenterRemoveEveryObserver |
| CFNotificationCenterRemoveObserver | function | CoreFoundation/CFNotificationCenter.h | raw::CFNotificationCenterRemoveObserver |
| CFNotificationSuspensionBehavior | typedef enum | CoreFoundation/CFNotificationCenter.h | raw::CFNotificationSuspensionBehavior |
| CFNullGetTypeID | function | CoreFoundation/CFBase.h | raw::CFNullGetTypeID |
| CFNullRef | typedef struct | CoreFoundation/CFBase.h | raw::CFNullRef |
| CFNumberCompare | function | CoreFoundation/CFNumber.h | raw::CFNumberCompare |
| CFNumberCreate | function | CoreFoundation/CFNumber.h | cf::CFNumber |
| CFNumberFormatterCopyProperty | function | CoreFoundation/CFNumberFormatter.h | raw::CFNumberFormatterCopyProperty |
| CFNumberFormatterCreate | function | CoreFoundation/CFNumberFormatter.h | cf::CFNumberFormatter |
| CFNumberFormatterCreateNumberFromString | function | CoreFoundation/CFNumberFormatter.h | cf::CFNumberFormatter |
| CFNumberFormatterCreateStringWithNumber | function | CoreFoundation/CFNumberFormatter.h | cf::CFNumberFormatter |
| CFNumberFormatterCreateStringWithValue | function | CoreFoundation/CFNumberFormatter.h | raw::CFNumberFormatterCreateStringWithValue |
| CFNumberFormatterGetDecimalInfoForCurrencyCode | function | CoreFoundation/CFNumberFormatter.h | raw::CFNumberFormatterGetDecimalInfoForCurrencyCode |
| CFNumberFormatterGetFormat | function | CoreFoundation/CFNumberFormatter.h | raw::CFNumberFormatterGetFormat |
| CFNumberFormatterGetLocale | function | CoreFoundation/CFNumberFormatter.h | raw::CFNumberFormatterGetLocale |
| CFNumberFormatterGetStyle | function | CoreFoundation/CFNumberFormatter.h | raw::CFNumberFormatterGetStyle |
| CFNumberFormatterGetTypeID | function | CoreFoundation/CFNumberFormatter.h | cf::CFNumberFormatter |
| CFNumberFormatterGetValueFromString | function | CoreFoundation/CFNumberFormatter.h | raw::CFNumberFormatterGetValueFromString |
| CFNumberFormatterOptionFlags | typedef enum | CoreFoundation/CFNumberFormatter.h | raw::CFNumberFormatterOptionFlags |
| CFNumberFormatterPadPosition | typedef enum | CoreFoundation/CFNumberFormatter.h | raw::CFNumberFormatterPadPosition |
| CFNumberFormatterRef | typedef struct | CoreFoundation/CFNumberFormatter.h | cf::CFNumberFormatter |
| CFNumberFormatterRoundingMode | typedef enum | CoreFoundation/CFNumberFormatter.h | raw::CFNumberFormatterRoundingMode |
| CFNumberFormatterSetFormat | function | CoreFoundation/CFNumberFormatter.h | raw::CFNumberFormatterSetFormat |
| CFNumberFormatterSetProperty | function | CoreFoundation/CFNumberFormatter.h | raw::CFNumberFormatterSetProperty |
| CFNumberFormatterStyle | typedef enum | CoreFoundation/CFNumberFormatter.h | cf::CFNumberFormatter |
| CFNumberGetByteSize | function | CoreFoundation/CFNumber.h | raw::CFNumberGetByteSize |
| CFNumberGetType | function | CoreFoundation/CFNumber.h | raw::CFNumberGetType |
| CFNumberGetTypeID | function | CoreFoundation/CFNumber.h | cf::CFNumber |
| CFNumberGetValue | function | CoreFoundation/CFNumber.h | cf::CFNumber |
| CFNumberIsFloatType | function | CoreFoundation/CFNumber.h | cf::CFNumber |
| CFNumberRef | typedef struct | CoreFoundation/CFNumber.h | cf::CFNumber |
| CFNumberType | typedef enum | CoreFoundation/CFNumber.h | raw::CFNumberType |
| CFPlugInAddInstanceForFactory | function | CoreFoundation/CFPlugIn.h | raw::CFPlugInAddInstanceForFactory |
| CFPlugInCreate | function | CoreFoundation/CFPlugIn.h | raw::CFPlugInCreate |
| CFPlugInFindFactoriesForPlugInType | function | CoreFoundation/CFPlugIn.h | raw::CFPlugInFindFactoriesForPlugInType |
| CFPlugInFindFactoriesForPlugInTypeInPlugIn | function | CoreFoundation/CFPlugIn.h | raw::CFPlugInFindFactoriesForPlugInTypeInPlugIn |
| CFPlugInGetBundle | function | CoreFoundation/CFPlugIn.h | raw::CFPlugInGetBundle |
| CFPlugInGetTypeID | function | CoreFoundation/CFPlugIn.h | raw::CFPlugInGetTypeID |
| CFPlugInInstanceCreate | function | CoreFoundation/CFPlugIn.h | raw::CFPlugInInstanceCreate |
| CFPlugInInstanceCreateWithInstanceDataSize | function | CoreFoundation/CFPlugIn.h | raw::CFPlugInInstanceCreateWithInstanceDataSize |
| CFPlugInInstanceGetFactoryName | function | CoreFoundation/CFPlugIn.h | raw::CFPlugInInstanceGetFactoryName |
| CFPlugInInstanceGetInstanceData | function | CoreFoundation/CFPlugIn.h | raw::CFPlugInInstanceGetInstanceData |
| CFPlugInInstanceGetInterfaceFunctionTable | function | CoreFoundation/CFPlugIn.h | raw::CFPlugInInstanceGetInterfaceFunctionTable |
| CFPlugInInstanceGetTypeID | function | CoreFoundation/CFPlugIn.h | raw::CFPlugInInstanceGetTypeID |
| CFPlugInInstanceRef | typedef struct | CoreFoundation/CFPlugIn.h | raw::CFPlugInInstanceRef |
| CFPlugInIsLoadOnDemand | function | CoreFoundation/CFPlugIn.h | raw::CFPlugInIsLoadOnDemand |
| CFPlugInRef | typedef struct | CoreFoundation/CFBundle.h | raw::CFPlugInRef |
| CFPlugInRegisterFactoryFunction | function | CoreFoundation/CFPlugIn.h | raw::CFPlugInRegisterFactoryFunction |
| CFPlugInRegisterFactoryFunctionByName | function | CoreFoundation/CFPlugIn.h | raw::CFPlugInRegisterFactoryFunctionByName |
| CFPlugInRegisterPlugInType | function | CoreFoundation/CFPlugIn.h | raw::CFPlugInRegisterPlugInType |
| CFPlugInRemoveInstanceForFactory | function | CoreFoundation/CFPlugIn.h | raw::CFPlugInRemoveInstanceForFactory |
| CFPlugInSetLoadOnDemand | function | CoreFoundation/CFPlugIn.h | raw::CFPlugInSetLoadOnDemand |
| CFPlugInUnregisterFactory | function | CoreFoundation/CFPlugIn.h | raw::CFPlugInUnregisterFactory |
| CFPlugInUnregisterPlugInType | function | CoreFoundation/CFPlugIn.h | raw::CFPlugInUnregisterPlugInType |
| CFPreferencesAddSuitePreferencesToApp | function | CoreFoundation/CFPreferences.h | raw::CFPreferencesAddSuitePreferencesToApp |
| CFPreferencesAppSynchronize | function | CoreFoundation/CFPreferences.h | cf::CFPreferences |
| CFPreferencesAppValueIsForced | function | CoreFoundation/CFPreferences.h | raw::CFPreferencesAppValueIsForced |
| CFPreferencesCopyAppValue | function | CoreFoundation/CFPreferences.h | cf::CFPreferences |
| CFPreferencesCopyKeyList | function | CoreFoundation/CFPreferences.h | raw::CFPreferencesCopyKeyList |
| CFPreferencesCopyMultiple | function | CoreFoundation/CFPreferences.h | raw::CFPreferencesCopyMultiple |
| CFPreferencesCopyValue | function | CoreFoundation/CFPreferences.h | raw::CFPreferencesCopyValue |
| CFPreferencesGetAppBooleanValue | function | CoreFoundation/CFPreferences.h | raw::CFPreferencesGetAppBooleanValue |
| CFPreferencesGetAppIntegerValue | function | CoreFoundation/CFPreferences.h | raw::CFPreferencesGetAppIntegerValue |
| CFPreferencesRemoveSuitePreferencesFromApp | function | CoreFoundation/CFPreferences.h | raw::CFPreferencesRemoveSuitePreferencesFromApp |
| CFPreferencesSetAppValue | function | CoreFoundation/CFPreferences.h | cf::CFPreferences |
| CFPreferencesSetMultiple | function | CoreFoundation/CFPreferences.h | raw::CFPreferencesSetMultiple |
| CFPreferencesSetValue | function | CoreFoundation/CFPreferences.h | raw::CFPreferencesSetValue |
| CFPreferencesSynchronize | function | CoreFoundation/CFPreferences.h | raw::CFPreferencesSynchronize |
| CFPropertyListCreateData | function | CoreFoundation/CFPropertyList.h | cf::CFPropertyList |
| CFPropertyListCreateDeepCopy | function | CoreFoundation/CFPropertyList.h | cf::CFPropertyList |
| CFPropertyListCreateWithData | function | CoreFoundation/CFPropertyList.h | cf::CFPropertyList |
| CFPropertyListCreateWithStream | function | CoreFoundation/CFPropertyList.h | cf::CFPropertyList |
| CFPropertyListFormat | typedef enum | CoreFoundation/CFPropertyList.h | cf::CFPropertyListFormat |
| CFPropertyListIsValid | function | CoreFoundation/CFPropertyList.h | cf::CFPropertyList |
| CFPropertyListMutabilityOptions | typedef enum | CoreFoundation/CFPropertyList.h | cf::CFPropertyListMutabilityOptions |
| CFPropertyListWrite | function | CoreFoundation/CFPropertyList.h | cf::CFPropertyList |
| CFRange | typedef struct | CoreFoundation/CFBase.h | raw::CFRange |
| CFRangeMake | function | CoreFoundation/CFBase.h | raw::CFRangeMake |
| CFReadStreamClose | function | CoreFoundation/CFStream.h | cf::CFStreamPair |
| CFReadStreamCopyDispatchQueue | function | CoreFoundation/CFStream.h | raw::CFReadStreamCopyDispatchQueue |
| CFReadStreamCopyError | function | CoreFoundation/CFStream.h | raw::CFReadStreamCopyError |
| CFReadStreamCopyProperty | function | CoreFoundation/CFStream.h | raw::CFReadStreamCopyProperty |
| CFReadStreamCreateWithBytesNoCopy | function | CoreFoundation/CFStream.h | raw::CFReadStreamCreateWithBytesNoCopy |
| CFReadStreamCreateWithFile | function | CoreFoundation/CFStream.h | raw::CFReadStreamCreateWithFile |
| CFReadStreamGetBuffer | function | CoreFoundation/CFStream.h | raw::CFReadStreamGetBuffer |
| CFReadStreamGetError | function | CoreFoundation/CFStream.h | raw::CFReadStreamGetError |
| CFReadStreamGetStatus | function | CoreFoundation/CFStream.h | raw::CFReadStreamGetStatus |
| CFReadStreamGetTypeID | function | CoreFoundation/CFStream.h | cf::CFStreamPair |
| CFReadStreamHasBytesAvailable | function | CoreFoundation/CFStream.h | raw::CFReadStreamHasBytesAvailable |
| CFReadStreamOpen | function | CoreFoundation/CFStream.h | cf::CFStreamPair |
| CFReadStreamRead | function | CoreFoundation/CFStream.h | cf::CFStreamPair |
| CFReadStreamRef | typedef struct | CoreFoundation/CFStream.h | cf::CFStreamPair |
| CFReadStreamScheduleWithRunLoop | function | CoreFoundation/CFStream.h | raw::CFReadStreamScheduleWithRunLoop |
| CFReadStreamSetClient | function | CoreFoundation/CFStream.h | raw::CFReadStreamSetClient |
| CFReadStreamSetDispatchQueue | function | CoreFoundation/CFStream.h | raw::CFReadStreamSetDispatchQueue |
| CFReadStreamSetProperty | function | CoreFoundation/CFStream.h | raw::CFReadStreamSetProperty |
| CFReadStreamUnscheduleFromRunLoop | function | CoreFoundation/CFStream.h | raw::CFReadStreamUnscheduleFromRunLoop |
| CFRelease | function | CoreFoundation/CFBase.h | cf::CFType |
| CFRetain | function | CoreFoundation/CFBase.h | cf::CFType |
| CFRunLoopActivity | typedef enum | CoreFoundation/CFRunLoop.h | raw::CFRunLoopActivity |
| CFRunLoopAddCommonMode | function | CoreFoundation/CFRunLoop.h | raw::CFRunLoopAddCommonMode |
| CFRunLoopAddObserver | function | CoreFoundation/CFRunLoop.h | raw::CFRunLoopAddObserver |
| CFRunLoopAddSource | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop |
| CFRunLoopAddTimer | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop |
| CFRunLoopContainsObserver | function | CoreFoundation/CFRunLoop.h | raw::CFRunLoopContainsObserver |
| CFRunLoopContainsSource | function | CoreFoundation/CFRunLoop.h | raw::CFRunLoopContainsSource |
| CFRunLoopContainsTimer | function | CoreFoundation/CFRunLoop.h | raw::CFRunLoopContainsTimer |
| CFRunLoopCopyAllModes | function | CoreFoundation/CFRunLoop.h | raw::CFRunLoopCopyAllModes |
| CFRunLoopCopyCurrentMode | function | CoreFoundation/CFRunLoop.h | raw::CFRunLoopCopyCurrentMode |
| CFRunLoopGetCurrent | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop |
| CFRunLoopGetMain | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop |
| CFRunLoopGetNextTimerFireDate | function | CoreFoundation/CFRunLoop.h | raw::CFRunLoopGetNextTimerFireDate |
| CFRunLoopGetTypeID | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop |
| CFRunLoopIsWaiting | function | CoreFoundation/CFRunLoop.h | raw::CFRunLoopIsWaiting |
| CFRunLoopObserverContext | typedef struct | CoreFoundation/CFRunLoop.h | raw::CFRunLoopObserverContext |
| CFRunLoopObserverCreate | function | CoreFoundation/CFRunLoop.h | raw::CFRunLoopObserverCreate |
| CFRunLoopObserverCreateWithHandler | function | CoreFoundation/CFRunLoop.h | raw::CFRunLoopObserverCreateWithHandler |
| CFRunLoopObserverDoesRepeat | function | CoreFoundation/CFRunLoop.h | raw::CFRunLoopObserverDoesRepeat |
| CFRunLoopObserverGetActivities | function | CoreFoundation/CFRunLoop.h | raw::CFRunLoopObserverGetActivities |
| CFRunLoopObserverGetContext | function | CoreFoundation/CFRunLoop.h | raw::CFRunLoopObserverGetContext |
| CFRunLoopObserverGetOrder | function | CoreFoundation/CFRunLoop.h | raw::CFRunLoopObserverGetOrder |
| CFRunLoopObserverGetTypeID | function | CoreFoundation/CFRunLoop.h | raw::CFRunLoopObserverGetTypeID |
| CFRunLoopObserverInvalidate | function | CoreFoundation/CFRunLoop.h | raw::CFRunLoopObserverInvalidate |
| CFRunLoopObserverIsValid | function | CoreFoundation/CFRunLoop.h | raw::CFRunLoopObserverIsValid |
| CFRunLoopObserverRef | typedef struct | CoreFoundation/CFRunLoop.h | raw::CFRunLoopObserverRef |
| CFRunLoopPerformBlock | function | CoreFoundation/CFRunLoop.h | raw::CFRunLoopPerformBlock |
| CFRunLoopRef | typedef struct | CoreFoundation/CFRunLoop.h | cf::CFRunLoop |
| CFRunLoopRemoveObserver | function | CoreFoundation/CFRunLoop.h | raw::CFRunLoopRemoveObserver |
| CFRunLoopRemoveSource | function | CoreFoundation/CFRunLoop.h | raw::CFRunLoopRemoveSource |
| CFRunLoopRemoveTimer | function | CoreFoundation/CFRunLoop.h | raw::CFRunLoopRemoveTimer |
| CFRunLoopRun | function | CoreFoundation/CFRunLoop.h | raw::CFRunLoopRun |
| CFRunLoopRunInMode | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop |
| CFRunLoopRunResult | typedef enum | CoreFoundation/CFRunLoop.h | cf::CFRunLoop |
| CFRunLoopSourceContext | typedef struct | CoreFoundation/CFRunLoop.h | raw::CFRunLoopSourceContext |
| CFRunLoopSourceContext1 | typedef struct | CoreFoundation/CFRunLoop.h | raw::CFRunLoopSourceContext1 |
| CFRunLoopSourceCreate | function | CoreFoundation/CFRunLoop.h | raw::CFRunLoopSourceCreate |
| CFRunLoopSourceGetContext | function | CoreFoundation/CFRunLoop.h | raw::CFRunLoopSourceGetContext |
| CFRunLoopSourceGetOrder | function | CoreFoundation/CFRunLoop.h | raw::CFRunLoopSourceGetOrder |
| CFRunLoopSourceGetTypeID | function | CoreFoundation/CFRunLoop.h | raw::CFRunLoopSourceGetTypeID |
| CFRunLoopSourceInvalidate | function | CoreFoundation/CFRunLoop.h | raw::CFRunLoopSourceInvalidate |
| CFRunLoopSourceIsValid | function | CoreFoundation/CFRunLoop.h | raw::CFRunLoopSourceIsValid |
| CFRunLoopSourceRef | typedef struct | CoreFoundation/CFRunLoop.h | raw::CFRunLoopSourceRef |
| CFRunLoopSourceSignal | function | CoreFoundation/CFRunLoop.h | raw::CFRunLoopSourceSignal |
| CFRunLoopStop | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop |
| CFRunLoopTimerContext | typedef struct | CoreFoundation/CFRunLoop.h | raw::CFRunLoopTimerContext |
| CFRunLoopTimerCreate | function | CoreFoundation/CFRunLoop.h | cf::CFTimer |
| CFRunLoopTimerCreateWithHandler | function | CoreFoundation/CFRunLoop.h | cf::CFTimer |
| CFRunLoopTimerDoesRepeat | function | CoreFoundation/CFRunLoop.h | raw::CFRunLoopTimerDoesRepeat |
| CFRunLoopTimerGetContext | function | CoreFoundation/CFRunLoop.h | raw::CFRunLoopTimerGetContext |
| CFRunLoopTimerGetInterval | function | CoreFoundation/CFRunLoop.h | raw::CFRunLoopTimerGetInterval |
| CFRunLoopTimerGetNextFireDate | function | CoreFoundation/CFRunLoop.h | raw::CFRunLoopTimerGetNextFireDate |
| CFRunLoopTimerGetOrder | function | CoreFoundation/CFRunLoop.h | raw::CFRunLoopTimerGetOrder |
| CFRunLoopTimerGetTolerance | function | CoreFoundation/CFRunLoop.h | raw::CFRunLoopTimerGetTolerance |
| CFRunLoopTimerGetTypeID | function | CoreFoundation/CFRunLoop.h | cf::CFTimer |
| CFRunLoopTimerInvalidate | function | CoreFoundation/CFRunLoop.h | cf::CFTimer |
| CFRunLoopTimerIsValid | function | CoreFoundation/CFRunLoop.h | cf::CFTimer |
| CFRunLoopTimerRef | typedef struct | CoreFoundation/CFRunLoop.h | cf::CFTimer |
| CFRunLoopTimerSetNextFireDate | function | CoreFoundation/CFRunLoop.h | cf::CFTimer |
| CFRunLoopTimerSetTolerance | function | CoreFoundation/CFRunLoop.h | raw::CFRunLoopTimerSetTolerance |
| CFRunLoopWakeUp | function | CoreFoundation/CFRunLoop.h | cf::CFRunLoop |
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
| CFShow | function | CoreFoundation/CFString.h | raw::CFShow |
| CFShowStr | function | CoreFoundation/CFString.h | raw::CFShowStr |
| CFSocketCallBackType | typedef enum | CoreFoundation/CFSocket.h | raw::CFSocketCallBackType |
| CFSocketConnectToAddress | function | CoreFoundation/CFSocket.h | raw::CFSocketConnectToAddress |
| CFSocketContext | typedef struct | CoreFoundation/CFSocket.h | raw::CFSocketContext |
| CFSocketCopyAddress | function | CoreFoundation/CFSocket.h | raw::CFSocketCopyAddress |
| CFSocketCopyPeerAddress | function | CoreFoundation/CFSocket.h | raw::CFSocketCopyPeerAddress |
| CFSocketCopyRegisteredSocketSignature | function | CoreFoundation/CFSocket.h | raw::CFSocketCopyRegisteredSocketSignature |
| CFSocketCopyRegisteredValue | function | CoreFoundation/CFSocket.h | raw::CFSocketCopyRegisteredValue |
| CFSocketCreate | function | CoreFoundation/CFSocket.h | cf::CFSocket |
| CFSocketCreateConnectedToSocketSignature | function | CoreFoundation/CFSocket.h | raw::CFSocketCreateConnectedToSocketSignature |
| CFSocketCreateRunLoopSource | function | CoreFoundation/CFSocket.h | raw::CFSocketCreateRunLoopSource |
| CFSocketCreateWithNative | function | CoreFoundation/CFSocket.h | raw::CFSocketCreateWithNative |
| CFSocketCreateWithSocketSignature | function | CoreFoundation/CFSocket.h | raw::CFSocketCreateWithSocketSignature |
| CFSocketDisableCallBacks | function | CoreFoundation/CFSocket.h | raw::CFSocketDisableCallBacks |
| CFSocketEnableCallBacks | function | CoreFoundation/CFSocket.h | raw::CFSocketEnableCallBacks |
| CFSocketError | typedef enum | CoreFoundation/CFSocket.h | raw::CFSocketError |
| CFSocketGetContext | function | CoreFoundation/CFSocket.h | raw::CFSocketGetContext |
| CFSocketGetDefaultNameRegistryPortNumber | function | CoreFoundation/CFSocket.h | raw::CFSocketGetDefaultNameRegistryPortNumber |
| CFSocketGetNative | function | CoreFoundation/CFSocket.h | cf::CFSocket |
| CFSocketGetSocketFlags | function | CoreFoundation/CFSocket.h | raw::CFSocketGetSocketFlags |
| CFSocketGetTypeID | function | CoreFoundation/CFSocket.h | cf::CFSocket |
| CFSocketInvalidate | function | CoreFoundation/CFSocket.h | cf::CFSocket |
| CFSocketIsValid | function | CoreFoundation/CFSocket.h | cf::CFSocket |
| CFSocketRef | typedef struct | CoreFoundation/CFSocket.h | cf::CFSocket |
| CFSocketRegisterSocketSignature | function | CoreFoundation/CFSocket.h | raw::CFSocketRegisterSocketSignature |
| CFSocketRegisterValue | function | CoreFoundation/CFSocket.h | raw::CFSocketRegisterValue |
| CFSocketSendData | function | CoreFoundation/CFSocket.h | raw::CFSocketSendData |
| CFSocketSetAddress | function | CoreFoundation/CFSocket.h | raw::CFSocketSetAddress |
| CFSocketSetDefaultNameRegistryPortNumber | function | CoreFoundation/CFSocket.h | raw::CFSocketSetDefaultNameRegistryPortNumber |
| CFSocketSetSocketFlags | function | CoreFoundation/CFSocket.h | raw::CFSocketSetSocketFlags |
| CFSocketSignature | typedef struct | CoreFoundation/CFSocket.h | raw::CFSocketSignature |
| CFSocketUnregister | function | CoreFoundation/CFSocket.h | raw::CFSocketUnregister |
| CFStreamClientContext | typedef struct | CoreFoundation/CFStream.h | raw::CFStreamClientContext |
| CFStreamCreateBoundPair | function | CoreFoundation/CFStream.h | cf::CFStreamPair |
| CFStreamError | typedef struct | CoreFoundation/CFStream.h | raw::CFStreamError |
| CFStreamErrorDomain | typedef enum | CoreFoundation/CFStream.h | raw::CFStreamErrorDomain |
| CFStreamEventType | typedef enum | CoreFoundation/CFStream.h | raw::CFStreamEventType |
| CFStreamStatus | typedef enum | CoreFoundation/CFStream.h | raw::CFStreamStatus |
| CFStringAppend | function | CoreFoundation/CFString.h | raw::CFStringAppend |
| CFStringAppendCString | function | CoreFoundation/CFString.h | raw::CFStringAppendCString |
| CFStringAppendCharacters | function | CoreFoundation/CFString.h | raw::CFStringAppendCharacters |
| CFStringAppendFormat | function | CoreFoundation/CFString.h | raw::CFStringAppendFormat |
| CFStringAppendFormatAndArguments | function | CoreFoundation/CFString.h | raw::CFStringAppendFormatAndArguments |
| CFStringAppendPascalString | function | CoreFoundation/CFString.h | raw::CFStringAppendPascalString |
| CFStringBuiltInEncodings | typedef enum | CoreFoundation/CFString.h | raw::CFStringBuiltInEncodings |
| CFStringCapitalize | function | CoreFoundation/CFString.h | raw::CFStringCapitalize |
| CFStringCompare | function | CoreFoundation/CFString.h | raw::CFStringCompare |
| CFStringCompareFlags | typedef enum | CoreFoundation/CFString.h | raw::CFStringCompareFlags |
| CFStringCompareWithOptions | function | CoreFoundation/CFString.h | raw::CFStringCompareWithOptions |
| CFStringCompareWithOptionsAndLocale | function | CoreFoundation/CFString.h | raw::CFStringCompareWithOptionsAndLocale |
| CFStringConvertEncodingToIANACharSetName | function | CoreFoundation/CFString.h | raw::CFStringConvertEncodingToIANACharSetName |
| CFStringConvertEncodingToNSStringEncoding | function | CoreFoundation/CFString.h | raw::CFStringConvertEncodingToNSStringEncoding |
| CFStringConvertEncodingToWindowsCodepage | function | CoreFoundation/CFString.h | raw::CFStringConvertEncodingToWindowsCodepage |
| CFStringConvertIANACharSetNameToEncoding | function | CoreFoundation/CFString.h | raw::CFStringConvertIANACharSetNameToEncoding |
| CFStringConvertNSStringEncodingToEncoding | function | CoreFoundation/CFString.h | raw::CFStringConvertNSStringEncodingToEncoding |
| CFStringConvertWindowsCodepageToEncoding | function | CoreFoundation/CFString.h | raw::CFStringConvertWindowsCodepageToEncoding |
| CFStringCreateArrayBySeparatingStrings | function | CoreFoundation/CFString.h | raw::CFStringCreateArrayBySeparatingStrings |
| CFStringCreateArrayWithFindResults | function | CoreFoundation/CFString.h | raw::CFStringCreateArrayWithFindResults |
| CFStringCreateByCombiningStrings | function | CoreFoundation/CFString.h | raw::CFStringCreateByCombiningStrings |
| CFStringCreateCopy | function | CoreFoundation/CFString.h | raw::CFStringCreateCopy |
| CFStringCreateExternalRepresentation | function | CoreFoundation/CFString.h | raw::CFStringCreateExternalRepresentation |
| CFStringCreateFromExternalRepresentation | function | CoreFoundation/CFString.h | raw::CFStringCreateFromExternalRepresentation |
| CFStringCreateMutable | function | CoreFoundation/CFString.h | raw::CFStringCreateMutable |
| CFStringCreateMutableCopy | function | CoreFoundation/CFString.h | raw::CFStringCreateMutableCopy |
| CFStringCreateMutableWithExternalCharactersNoCopy | function | CoreFoundation/CFString.h | raw::CFStringCreateMutableWithExternalCharactersNoCopy |
| CFStringCreateStringWithValidatedFormat | function | CoreFoundation/CFString.h | raw::CFStringCreateStringWithValidatedFormat |
| CFStringCreateStringWithValidatedFormatAndArguments | function | CoreFoundation/CFString.h | raw::CFStringCreateStringWithValidatedFormatAndArguments |
| CFStringCreateWithBytes | function | CoreFoundation/CFString.h | raw::CFStringCreateWithBytes |
| CFStringCreateWithBytesNoCopy | function | CoreFoundation/CFString.h | raw::CFStringCreateWithBytesNoCopy |
| CFStringCreateWithCString | function | CoreFoundation/CFString.h | cf::CFString |
| CFStringCreateWithCStringNoCopy | function | CoreFoundation/CFString.h | raw::CFStringCreateWithCStringNoCopy |
| CFStringCreateWithCharacters | function | CoreFoundation/CFString.h | raw::CFStringCreateWithCharacters |
| CFStringCreateWithCharactersNoCopy | function | CoreFoundation/CFString.h | raw::CFStringCreateWithCharactersNoCopy |
| CFStringCreateWithFileSystemRepresentation | function | CoreFoundation/CFString.h | raw::CFStringCreateWithFileSystemRepresentation |
| CFStringCreateWithFormat | function | CoreFoundation/CFString.h | raw::CFStringCreateWithFormat |
| CFStringCreateWithFormatAndArguments | function | CoreFoundation/CFString.h | raw::CFStringCreateWithFormatAndArguments |
| CFStringCreateWithPascalString | function | CoreFoundation/CFString.h | raw::CFStringCreateWithPascalString |
| CFStringCreateWithPascalStringNoCopy | function | CoreFoundation/CFString.h | raw::CFStringCreateWithPascalStringNoCopy |
| CFStringCreateWithSubstring | function | CoreFoundation/CFString.h | raw::CFStringCreateWithSubstring |
| CFStringDelete | function | CoreFoundation/CFString.h | raw::CFStringDelete |
| CFStringEncodings | typedef enum | CoreFoundation/CFStringEncodingExt.h | raw::CFStringEncodings |
| CFStringFind | function | CoreFoundation/CFString.h | raw::CFStringFind |
| CFStringFindAndReplace | function | CoreFoundation/CFString.h | raw::CFStringFindAndReplace |
| CFStringFindCharacterFromSet | function | CoreFoundation/CFString.h | raw::CFStringFindCharacterFromSet |
| CFStringFindWithOptions | function | CoreFoundation/CFString.h | raw::CFStringFindWithOptions |
| CFStringFindWithOptionsAndLocale | function | CoreFoundation/CFString.h | raw::CFStringFindWithOptionsAndLocale |
| CFStringFold | function | CoreFoundation/CFString.h | raw::CFStringFold |
| CFStringGetBytes | function | CoreFoundation/CFString.h | raw::CFStringGetBytes |
| CFStringGetCString | function | CoreFoundation/CFString.h | cf::CFString |
| CFStringGetCStringPtr | function | CoreFoundation/CFString.h | raw::CFStringGetCStringPtr |
| CFStringGetCharacterAtIndex | function | CoreFoundation/CFString.h | raw::CFStringGetCharacterAtIndex |
| CFStringGetCharacterFromInlineBuffer | function | CoreFoundation/CFString.h | raw::CFStringGetCharacterFromInlineBuffer |
| CFStringGetCharacters | function | CoreFoundation/CFString.h | raw::CFStringGetCharacters |
| CFStringGetCharactersPtr | function | CoreFoundation/CFString.h | raw::CFStringGetCharactersPtr |
| CFStringGetDoubleValue | function | CoreFoundation/CFString.h | raw::CFStringGetDoubleValue |
| CFStringGetFastestEncoding | function | CoreFoundation/CFString.h | raw::CFStringGetFastestEncoding |
| CFStringGetFileSystemRepresentation | function | CoreFoundation/CFString.h | raw::CFStringGetFileSystemRepresentation |
| CFStringGetHyphenationLocationBeforeIndex | function | CoreFoundation/CFString.h | raw::CFStringGetHyphenationLocationBeforeIndex |
| CFStringGetIntValue | function | CoreFoundation/CFString.h | raw::CFStringGetIntValue |
| CFStringGetLength | function | CoreFoundation/CFString.h | cf::CFString |
| CFStringGetLineBounds | function | CoreFoundation/CFString.h | raw::CFStringGetLineBounds |
| CFStringGetListOfAvailableEncodings | function | CoreFoundation/CFString.h | raw::CFStringGetListOfAvailableEncodings |
| CFStringGetLongCharacterForSurrogatePair | function | CoreFoundation/CFString.h | raw::CFStringGetLongCharacterForSurrogatePair |
| CFStringGetMaximumSizeForEncoding | function | CoreFoundation/CFString.h | cf::CFString |
| CFStringGetMaximumSizeOfFileSystemRepresentation | function | CoreFoundation/CFString.h | raw::CFStringGetMaximumSizeOfFileSystemRepresentation |
| CFStringGetMostCompatibleMacStringEncoding | function | CoreFoundation/CFString.h | raw::CFStringGetMostCompatibleMacStringEncoding |
| CFStringGetNameOfEncoding | function | CoreFoundation/CFString.h | raw::CFStringGetNameOfEncoding |
| CFStringGetParagraphBounds | function | CoreFoundation/CFString.h | raw::CFStringGetParagraphBounds |
| CFStringGetPascalString | function | CoreFoundation/CFString.h | raw::CFStringGetPascalString |
| CFStringGetPascalStringPtr | function | CoreFoundation/CFString.h | raw::CFStringGetPascalStringPtr |
| CFStringGetRangeOfComposedCharactersAtIndex | function | CoreFoundation/CFString.h | raw::CFStringGetRangeOfComposedCharactersAtIndex |
| CFStringGetSmallestEncoding | function | CoreFoundation/CFString.h | raw::CFStringGetSmallestEncoding |
| CFStringGetSurrogatePairForLongCharacter | function | CoreFoundation/CFString.h | raw::CFStringGetSurrogatePairForLongCharacter |
| CFStringGetSystemEncoding | function | CoreFoundation/CFString.h | raw::CFStringGetSystemEncoding |
| CFStringGetTypeID | function | CoreFoundation/CFString.h | cf::CFString |
| CFStringHasPrefix | function | CoreFoundation/CFString.h | raw::CFStringHasPrefix |
| CFStringHasSuffix | function | CoreFoundation/CFString.h | raw::CFStringHasSuffix |
| CFStringInitInlineBuffer | function | CoreFoundation/CFString.h | raw::CFStringInitInlineBuffer |
| CFStringInlineBuffer | typedef struct | CoreFoundation/CFString.h | raw::CFStringInlineBuffer |
| CFStringInsert | function | CoreFoundation/CFString.h | raw::CFStringInsert |
| CFStringIsEncodingAvailable | function | CoreFoundation/CFString.h | raw::CFStringIsEncodingAvailable |
| CFStringIsHyphenationAvailableForLocale | function | CoreFoundation/CFString.h | raw::CFStringIsHyphenationAvailableForLocale |
| CFStringIsSurrogateHighCharacter | function | CoreFoundation/CFString.h | raw::CFStringIsSurrogateHighCharacter |
| CFStringIsSurrogateLowCharacter | function | CoreFoundation/CFString.h | raw::CFStringIsSurrogateLowCharacter |
| CFStringLowercase | function | CoreFoundation/CFString.h | raw::CFStringLowercase |
| CFStringNormalizationForm | typedef enum | CoreFoundation/CFString.h | raw::CFStringNormalizationForm |
| CFStringNormalize | function | CoreFoundation/CFString.h | raw::CFStringNormalize |
| CFStringPad | function | CoreFoundation/CFString.h | raw::CFStringPad |
| CFStringRef | typedef struct | CoreFoundation/CFBase.h | cf::CFString |
| CFStringReplace | function | CoreFoundation/CFString.h | raw::CFStringReplace |
| CFStringReplaceAll | function | CoreFoundation/CFString.h | raw::CFStringReplaceAll |
| CFStringSetExternalCharactersNoCopy | function | CoreFoundation/CFString.h | raw::CFStringSetExternalCharactersNoCopy |
| CFStringTokenizerAdvanceToNextToken | function | CoreFoundation/CFStringTokenizer.h | raw::CFStringTokenizerAdvanceToNextToken |
| CFStringTokenizerCopyBestStringLanguage | function | CoreFoundation/CFStringTokenizer.h | raw::CFStringTokenizerCopyBestStringLanguage |
| CFStringTokenizerCopyCurrentTokenAttribute | function | CoreFoundation/CFStringTokenizer.h | raw::CFStringTokenizerCopyCurrentTokenAttribute |
| CFStringTokenizerCreate | function | CoreFoundation/CFStringTokenizer.h | raw::CFStringTokenizerCreate |
| CFStringTokenizerGetCurrentSubTokens | function | CoreFoundation/CFStringTokenizer.h | raw::CFStringTokenizerGetCurrentSubTokens |
| CFStringTokenizerGetCurrentTokenRange | function | CoreFoundation/CFStringTokenizer.h | raw::CFStringTokenizerGetCurrentTokenRange |
| CFStringTokenizerGetTypeID | function | CoreFoundation/CFStringTokenizer.h | raw::CFStringTokenizerGetTypeID |
| CFStringTokenizerGoToTokenAtIndex | function | CoreFoundation/CFStringTokenizer.h | raw::CFStringTokenizerGoToTokenAtIndex |
| CFStringTokenizerRef | typedef struct | CoreFoundation/CFStringTokenizer.h | raw::CFStringTokenizerRef |
| CFStringTokenizerSetString | function | CoreFoundation/CFStringTokenizer.h | raw::CFStringTokenizerSetString |
| CFStringTokenizerTokenType | typedef enum | CoreFoundation/CFStringTokenizer.h | raw::CFStringTokenizerTokenType |
| CFStringTransform | function | CoreFoundation/CFString.h | raw::CFStringTransform |
| CFStringTrim | function | CoreFoundation/CFString.h | raw::CFStringTrim |
| CFStringTrimWhitespace | function | CoreFoundation/CFString.h | raw::CFStringTrimWhitespace |
| CFStringUppercase | function | CoreFoundation/CFString.h | raw::CFStringUppercase |
| CFSwapInt16 | function | CoreFoundation/CFByteOrder.h | raw::CFSwapInt16 |
| CFSwapInt16BigToHost | function | CoreFoundation/CFByteOrder.h | raw::CFSwapInt16BigToHost |
| CFSwapInt16HostToBig | function | CoreFoundation/CFByteOrder.h | raw::CFSwapInt16HostToBig |
| CFSwapInt16HostToLittle | function | CoreFoundation/CFByteOrder.h | raw::CFSwapInt16HostToLittle |
| CFSwapInt16LittleToHost | function | CoreFoundation/CFByteOrder.h | raw::CFSwapInt16LittleToHost |
| CFSwapInt32 | function | CoreFoundation/CFByteOrder.h | raw::CFSwapInt32 |
| CFSwapInt32BigToHost | function | CoreFoundation/CFByteOrder.h | raw::CFSwapInt32BigToHost |
| CFSwapInt32HostToBig | function | CoreFoundation/CFByteOrder.h | raw::CFSwapInt32HostToBig |
| CFSwapInt32HostToLittle | function | CoreFoundation/CFByteOrder.h | raw::CFSwapInt32HostToLittle |
| CFSwapInt32LittleToHost | function | CoreFoundation/CFByteOrder.h | raw::CFSwapInt32LittleToHost |
| CFSwapInt64 | function | CoreFoundation/CFByteOrder.h | raw::CFSwapInt64 |
| CFSwapInt64BigToHost | function | CoreFoundation/CFByteOrder.h | raw::CFSwapInt64BigToHost |
| CFSwapInt64HostToBig | function | CoreFoundation/CFByteOrder.h | raw::CFSwapInt64HostToBig |
| CFSwapInt64HostToLittle | function | CoreFoundation/CFByteOrder.h | raw::CFSwapInt64HostToLittle |
| CFSwapInt64LittleToHost | function | CoreFoundation/CFByteOrder.h | raw::CFSwapInt64LittleToHost |
| CFSwappedFloat32 | typedef struct | CoreFoundation/CFByteOrder.h | raw::CFSwappedFloat32 |
| CFSwappedFloat64 | typedef struct | CoreFoundation/CFByteOrder.h | raw::CFSwappedFloat64 |
| CFTimeZoneCopyAbbreviation | function | CoreFoundation/CFTimeZone.h | raw::CFTimeZoneCopyAbbreviation |
| CFTimeZoneCopyAbbreviationDictionary | function | CoreFoundation/CFTimeZone.h | raw::CFTimeZoneCopyAbbreviationDictionary |
| CFTimeZoneCopyDefault | function | CoreFoundation/CFTimeZone.h | raw::CFTimeZoneCopyDefault |
| CFTimeZoneCopyKnownNames | function | CoreFoundation/CFTimeZone.h | raw::CFTimeZoneCopyKnownNames |
| CFTimeZoneCopyLocalizedName | function | CoreFoundation/CFTimeZone.h | raw::CFTimeZoneCopyLocalizedName |
| CFTimeZoneCopySystem | function | CoreFoundation/CFTimeZone.h | cf::CFTimeZone |
| CFTimeZoneCreate | function | CoreFoundation/CFTimeZone.h | raw::CFTimeZoneCreate |
| CFTimeZoneCreateWithName | function | CoreFoundation/CFTimeZone.h | cf::CFTimeZone |
| CFTimeZoneCreateWithTimeIntervalFromGMT | function | CoreFoundation/CFTimeZone.h | raw::CFTimeZoneCreateWithTimeIntervalFromGMT |
| CFTimeZoneGetData | function | CoreFoundation/CFTimeZone.h | raw::CFTimeZoneGetData |
| CFTimeZoneGetDaylightSavingTimeOffset | function | CoreFoundation/CFTimeZone.h | raw::CFTimeZoneGetDaylightSavingTimeOffset |
| CFTimeZoneGetName | function | CoreFoundation/CFTimeZone.h | cf::CFTimeZone |
| CFTimeZoneGetNextDaylightSavingTimeTransition | function | CoreFoundation/CFTimeZone.h | raw::CFTimeZoneGetNextDaylightSavingTimeTransition |
| CFTimeZoneGetSecondsFromGMT | function | CoreFoundation/CFTimeZone.h | cf::CFTimeZone |
| CFTimeZoneGetTypeID | function | CoreFoundation/CFTimeZone.h | cf::CFTimeZone |
| CFTimeZoneIsDaylightSavingTime | function | CoreFoundation/CFTimeZone.h | raw::CFTimeZoneIsDaylightSavingTime |
| CFTimeZoneNameStyle | typedef enum | CoreFoundation/CFTimeZone.h | raw::CFTimeZoneNameStyle |
| CFTimeZoneRef | typedef struct | CoreFoundation/CFDate.h | cf::CFTimeZone |
| CFTimeZoneResetSystem | function | CoreFoundation/CFTimeZone.h | raw::CFTimeZoneResetSystem |
| CFTimeZoneSetAbbreviationDictionary | function | CoreFoundation/CFTimeZone.h | raw::CFTimeZoneSetAbbreviationDictionary |
| CFTimeZoneSetDefault | function | CoreFoundation/CFTimeZone.h | raw::CFTimeZoneSetDefault |
| CFTreeAppendChild | function | CoreFoundation/CFTree.h | raw::CFTreeAppendChild |
| CFTreeApplyFunctionToChildren | function | CoreFoundation/CFTree.h | raw::CFTreeApplyFunctionToChildren |
| CFTreeContext | typedef struct | CoreFoundation/CFTree.h | raw::CFTreeContext |
| CFTreeCreate | function | CoreFoundation/CFTree.h | raw::CFTreeCreate |
| CFTreeFindRoot | function | CoreFoundation/CFTree.h | raw::CFTreeFindRoot |
| CFTreeGetChildAtIndex | function | CoreFoundation/CFTree.h | raw::CFTreeGetChildAtIndex |
| CFTreeGetChildCount | function | CoreFoundation/CFTree.h | raw::CFTreeGetChildCount |
| CFTreeGetChildren | function | CoreFoundation/CFTree.h | raw::CFTreeGetChildren |
| CFTreeGetContext | function | CoreFoundation/CFTree.h | raw::CFTreeGetContext |
| CFTreeGetFirstChild | function | CoreFoundation/CFTree.h | raw::CFTreeGetFirstChild |
| CFTreeGetNextSibling | function | CoreFoundation/CFTree.h | raw::CFTreeGetNextSibling |
| CFTreeGetParent | function | CoreFoundation/CFTree.h | raw::CFTreeGetParent |
| CFTreeGetTypeID | function | CoreFoundation/CFTree.h | raw::CFTreeGetTypeID |
| CFTreeInsertSibling | function | CoreFoundation/CFTree.h | raw::CFTreeInsertSibling |
| CFTreePrependChild | function | CoreFoundation/CFTree.h | raw::CFTreePrependChild |
| CFTreeRef | typedef struct | CoreFoundation/CFTree.h | raw::CFTreeRef |
| CFTreeRemove | function | CoreFoundation/CFTree.h | raw::CFTreeRemove |
| CFTreeRemoveAllChildren | function | CoreFoundation/CFTree.h | raw::CFTreeRemoveAllChildren |
| CFTreeSetContext | function | CoreFoundation/CFTree.h | raw::CFTreeSetContext |
| CFTreeSortChildren | function | CoreFoundation/CFTree.h | raw::CFTreeSortChildren |
| CFURLBookmarkCreationOptions | typedef enum | CoreFoundation/CFURL.h | raw::CFURLBookmarkCreationOptions |
| CFURLBookmarkResolutionOptions | typedef enum | CoreFoundation/CFURL.h | raw::CFURLBookmarkResolutionOptions |
| CFURLCanBeDecomposed | function | CoreFoundation/CFURL.h | raw::CFURLCanBeDecomposed |
| CFURLClearResourcePropertyCache | function | CoreFoundation/CFURL.h | raw::CFURLClearResourcePropertyCache |
| CFURLClearResourcePropertyCacheForKey | function | CoreFoundation/CFURL.h | raw::CFURLClearResourcePropertyCacheForKey |
| CFURLComponentType | typedef enum | CoreFoundation/CFURL.h | raw::CFURLComponentType |
| CFURLCopyAbsoluteURL | function | CoreFoundation/CFURL.h | raw::CFURLCopyAbsoluteURL |
| CFURLCopyFileSystemPath | function | CoreFoundation/CFURL.h | cf::CFURL |
| CFURLCopyFragment | function | CoreFoundation/CFURL.h | raw::CFURLCopyFragment |
| CFURLCopyHostName | function | CoreFoundation/CFURL.h | raw::CFURLCopyHostName |
| CFURLCopyLastPathComponent | function | CoreFoundation/CFURL.h | raw::CFURLCopyLastPathComponent |
| CFURLCopyNetLocation | function | CoreFoundation/CFURL.h | raw::CFURLCopyNetLocation |
| CFURLCopyPassword | function | CoreFoundation/CFURL.h | raw::CFURLCopyPassword |
| CFURLCopyPath | function | CoreFoundation/CFURL.h | raw::CFURLCopyPath |
| CFURLCopyPathExtension | function | CoreFoundation/CFURL.h | raw::CFURLCopyPathExtension |
| CFURLCopyResourcePropertiesForKeys | function | CoreFoundation/CFURL.h | raw::CFURLCopyResourcePropertiesForKeys |
| CFURLCopyResourcePropertyForKey | function | CoreFoundation/CFURL.h | raw::CFURLCopyResourcePropertyForKey |
| CFURLCopyResourceSpecifier | function | CoreFoundation/CFURL.h | raw::CFURLCopyResourceSpecifier |
| CFURLCopyScheme | function | CoreFoundation/CFURL.h | raw::CFURLCopyScheme |
| CFURLCopyStrictPath | function | CoreFoundation/CFURL.h | raw::CFURLCopyStrictPath |
| CFURLCopyUserName | function | CoreFoundation/CFURL.h | raw::CFURLCopyUserName |
| CFURLCreateAbsoluteURLWithBytes | function | CoreFoundation/CFURL.h | raw::CFURLCreateAbsoluteURLWithBytes |
| CFURLCreateBookmarkData | function | CoreFoundation/CFURL.h | raw::CFURLCreateBookmarkData |
| CFURLCreateBookmarkDataFromFile | function | CoreFoundation/CFURL.h | raw::CFURLCreateBookmarkDataFromFile |
| CFURLCreateByResolvingBookmarkData | function | CoreFoundation/CFURL.h | raw::CFURLCreateByResolvingBookmarkData |
| CFURLCreateCopyAppendingPathComponent | function | CoreFoundation/CFURL.h | raw::CFURLCreateCopyAppendingPathComponent |
| CFURLCreateCopyAppendingPathExtension | function | CoreFoundation/CFURL.h | raw::CFURLCreateCopyAppendingPathExtension |
| CFURLCreateCopyDeletingLastPathComponent | function | CoreFoundation/CFURL.h | raw::CFURLCreateCopyDeletingLastPathComponent |
| CFURLCreateCopyDeletingPathExtension | function | CoreFoundation/CFURL.h | raw::CFURLCreateCopyDeletingPathExtension |
| CFURLCreateData | function | CoreFoundation/CFURL.h | raw::CFURLCreateData |
| CFURLCreateFilePathURL | function | CoreFoundation/CFURL.h | raw::CFURLCreateFilePathURL |
| CFURLCreateFileReferenceURL | function | CoreFoundation/CFURL.h | raw::CFURLCreateFileReferenceURL |
| CFURLCreateFromFileSystemRepresentation | function | CoreFoundation/CFURL.h | raw::CFURLCreateFromFileSystemRepresentation |
| CFURLCreateFromFileSystemRepresentationRelativeToBase | function | CoreFoundation/CFURL.h | raw::CFURLCreateFromFileSystemRepresentationRelativeToBase |
| CFURLCreateResourcePropertiesForKeysFromBookmarkData | function | CoreFoundation/CFURL.h | raw::CFURLCreateResourcePropertiesForKeysFromBookmarkData |
| CFURLCreateResourcePropertyForKeyFromBookmarkData | function | CoreFoundation/CFURL.h | raw::CFURLCreateResourcePropertyForKeyFromBookmarkData |
| CFURLCreateStringByReplacingPercentEscapes | function | CoreFoundation/CFURL.h | raw::CFURLCreateStringByReplacingPercentEscapes |
| CFURLCreateWithBytes | function | CoreFoundation/CFURL.h | raw::CFURLCreateWithBytes |
| CFURLCreateWithFileSystemPath | function | CoreFoundation/CFURL.h | cf::CFURL |
| CFURLCreateWithFileSystemPathRelativeToBase | function | CoreFoundation/CFURL.h | raw::CFURLCreateWithFileSystemPathRelativeToBase |
| CFURLCreateWithString | function | CoreFoundation/CFURL.h | cf::CFURL |
| CFURLEnumeratorCreateForDirectoryURL | function | CoreFoundation/CFURLEnumerator.h | raw::CFURLEnumeratorCreateForDirectoryURL |
| CFURLEnumeratorCreateForMountedVolumes | function | CoreFoundation/CFURLEnumerator.h | raw::CFURLEnumeratorCreateForMountedVolumes |
| CFURLEnumeratorGetDescendentLevel | function | CoreFoundation/CFURLEnumerator.h | raw::CFURLEnumeratorGetDescendentLevel |
| CFURLEnumeratorGetNextURL | function | CoreFoundation/CFURLEnumerator.h | raw::CFURLEnumeratorGetNextURL |
| CFURLEnumeratorGetTypeID | function | CoreFoundation/CFURLEnumerator.h | raw::CFURLEnumeratorGetTypeID |
| CFURLEnumeratorOptions | typedef enum | CoreFoundation/CFURLEnumerator.h | raw::CFURLEnumeratorOptions |
| CFURLEnumeratorRef | typedef struct | CoreFoundation/CFURLEnumerator.h | raw::CFURLEnumeratorRef |
| CFURLEnumeratorResult | typedef enum | CoreFoundation/CFURLEnumerator.h | raw::CFURLEnumeratorResult |
| CFURLEnumeratorSkipDescendents | function | CoreFoundation/CFURLEnumerator.h | raw::CFURLEnumeratorSkipDescendents |
| CFURLError | typedef enum | CoreFoundation/CFURLAccess.h | raw::CFURLError |
| CFURLGetBaseURL | function | CoreFoundation/CFURL.h | raw::CFURLGetBaseURL |
| CFURLGetByteRangeForComponent | function | CoreFoundation/CFURL.h | raw::CFURLGetByteRangeForComponent |
| CFURLGetBytes | function | CoreFoundation/CFURL.h | raw::CFURLGetBytes |
| CFURLGetFileSystemRepresentation | function | CoreFoundation/CFURL.h | raw::CFURLGetFileSystemRepresentation |
| CFURLGetPortNumber | function | CoreFoundation/CFURL.h | raw::CFURLGetPortNumber |
| CFURLGetString | function | CoreFoundation/CFURL.h | cf::CFURL |
| CFURLGetTypeID | function | CoreFoundation/CFURL.h | cf::CFURL |
| CFURLHasDirectoryPath | function | CoreFoundation/CFURL.h | cf::CFURL |
| CFURLIsFileReferenceURL | function | CoreFoundation/CFURL.h | raw::CFURLIsFileReferenceURL |
| CFURLRef | typedef struct | CoreFoundation/CFURL.h | cf::CFURL |
| CFURLResourceIsReachable | function | CoreFoundation/CFURL.h | raw::CFURLResourceIsReachable |
| CFURLSetResourcePropertiesForKeys | function | CoreFoundation/CFURL.h | raw::CFURLSetResourcePropertiesForKeys |
| CFURLSetResourcePropertyForKey | function | CoreFoundation/CFURL.h | raw::CFURLSetResourcePropertyForKey |
| CFURLSetTemporaryResourcePropertyForKey | function | CoreFoundation/CFURL.h | raw::CFURLSetTemporaryResourcePropertyForKey |
| CFURLStartAccessingSecurityScopedResource | function | CoreFoundation/CFURL.h | raw::CFURLStartAccessingSecurityScopedResource |
| CFURLStopAccessingSecurityScopedResource | function | CoreFoundation/CFURL.h | raw::CFURLStopAccessingSecurityScopedResource |
| CFURLWriteBookmarkDataToFile | function | CoreFoundation/CFURL.h | raw::CFURLWriteBookmarkDataToFile |
| CFUUIDBytes | typedef struct | CoreFoundation/CFUUID.h | raw::CFUUIDBytes |
| CFUUIDCreate | function | CoreFoundation/CFUUID.h | cf::CFUUID |
| CFUUIDCreateFromString | function | CoreFoundation/CFUUID.h | cf::CFUUID |
| CFUUIDCreateFromUUIDBytes | function | CoreFoundation/CFUUID.h | raw::CFUUIDCreateFromUUIDBytes |
| CFUUIDCreateString | function | CoreFoundation/CFUUID.h | cf::CFUUID |
| CFUUIDCreateWithBytes | function | CoreFoundation/CFUUID.h | raw::CFUUIDCreateWithBytes |
| CFUUIDGetConstantUUIDWithBytes | function | CoreFoundation/CFUUID.h | raw::CFUUIDGetConstantUUIDWithBytes |
| CFUUIDGetTypeID | function | CoreFoundation/CFUUID.h | cf::CFUUID |
| CFUUIDGetUUIDBytes | function | CoreFoundation/CFUUID.h | cf::CFUUID |
| CFUUIDRef | typedef struct | CoreFoundation/CFUUID.h | cf::CFUUID |
| CFUserNotificationCancel | function | CoreFoundation/CFUserNotification.h | raw::CFUserNotificationCancel |
| CFUserNotificationCheckBoxChecked | function | CoreFoundation/CFUserNotification.h | raw::CFUserNotificationCheckBoxChecked |
| CFUserNotificationCreate | function | CoreFoundation/CFUserNotification.h | raw::CFUserNotificationCreate |
| CFUserNotificationCreateRunLoopSource | function | CoreFoundation/CFUserNotification.h | raw::CFUserNotificationCreateRunLoopSource |
| CFUserNotificationDisplayAlert | function | CoreFoundation/CFUserNotification.h | raw::CFUserNotificationDisplayAlert |
| CFUserNotificationDisplayNotice | function | CoreFoundation/CFUserNotification.h | raw::CFUserNotificationDisplayNotice |
| CFUserNotificationGetResponseDictionary | function | CoreFoundation/CFUserNotification.h | raw::CFUserNotificationGetResponseDictionary |
| CFUserNotificationGetResponseValue | function | CoreFoundation/CFUserNotification.h | raw::CFUserNotificationGetResponseValue |
| CFUserNotificationGetTypeID | function | CoreFoundation/CFUserNotification.h | raw::CFUserNotificationGetTypeID |
| CFUserNotificationPopUpSelection | function | CoreFoundation/CFUserNotification.h | raw::CFUserNotificationPopUpSelection |
| CFUserNotificationReceiveResponse | function | CoreFoundation/CFUserNotification.h | raw::CFUserNotificationReceiveResponse |
| CFUserNotificationRef | typedef struct | CoreFoundation/CFUserNotification.h | raw::CFUserNotificationRef |
| CFUserNotificationSecureTextField | function | CoreFoundation/CFUserNotification.h | raw::CFUserNotificationSecureTextField |
| CFUserNotificationUpdate | function | CoreFoundation/CFUserNotification.h | raw::CFUserNotificationUpdate |
| CFWriteStreamCanAcceptBytes | function | CoreFoundation/CFStream.h | raw::CFWriteStreamCanAcceptBytes |
| CFWriteStreamClose | function | CoreFoundation/CFStream.h | cf::CFStreamPair |
| CFWriteStreamCopyDispatchQueue | function | CoreFoundation/CFStream.h | raw::CFWriteStreamCopyDispatchQueue |
| CFWriteStreamCopyError | function | CoreFoundation/CFStream.h | raw::CFWriteStreamCopyError |
| CFWriteStreamCopyProperty | function | CoreFoundation/CFStream.h | raw::CFWriteStreamCopyProperty |
| CFWriteStreamCreateWithAllocatedBuffers | function | CoreFoundation/CFStream.h | raw::CFWriteStreamCreateWithAllocatedBuffers |
| CFWriteStreamCreateWithBuffer | function | CoreFoundation/CFStream.h | raw::CFWriteStreamCreateWithBuffer |
| CFWriteStreamCreateWithFile | function | CoreFoundation/CFStream.h | raw::CFWriteStreamCreateWithFile |
| CFWriteStreamGetError | function | CoreFoundation/CFStream.h | raw::CFWriteStreamGetError |
| CFWriteStreamGetStatus | function | CoreFoundation/CFStream.h | raw::CFWriteStreamGetStatus |
| CFWriteStreamGetTypeID | function | CoreFoundation/CFStream.h | cf::CFStreamPair |
| CFWriteStreamOpen | function | CoreFoundation/CFStream.h | cf::CFStreamPair |
| CFWriteStreamRef | typedef struct | CoreFoundation/CFStream.h | cf::CFStreamPair |
| CFWriteStreamScheduleWithRunLoop | function | CoreFoundation/CFStream.h | raw::CFWriteStreamScheduleWithRunLoop |
| CFWriteStreamSetClient | function | CoreFoundation/CFStream.h | raw::CFWriteStreamSetClient |
| CFWriteStreamSetDispatchQueue | function | CoreFoundation/CFStream.h | raw::CFWriteStreamSetDispatchQueue |
| CFWriteStreamSetProperty | function | CoreFoundation/CFStream.h | raw::CFWriteStreamSetProperty |
| CFWriteStreamUnscheduleFromRunLoop | function | CoreFoundation/CFStream.h | raw::CFWriteStreamUnscheduleFromRunLoop |
| CFWriteStreamWrite | function | CoreFoundation/CFStream.h | cf::CFStreamPair |
| CFXMLAttributeDeclarationInfo | typedef struct | CoreFoundation/CFXMLNode.h | raw::CFXMLAttributeDeclarationInfo |
| CFXMLAttributeListDeclarationInfo | typedef struct | CoreFoundation/CFXMLNode.h | raw::CFXMLAttributeListDeclarationInfo |
| CFXMLCreateStringByEscapingEntities | function | CoreFoundation/CFXMLParser.h | cf::CFXML |
| CFXMLCreateStringByUnescapingEntities | function | CoreFoundation/CFXMLParser.h | cf::CFXML |
| CFXMLDocumentInfo | typedef struct | CoreFoundation/CFXMLNode.h | raw::CFXMLDocumentInfo |
| CFXMLDocumentTypeInfo | typedef struct | CoreFoundation/CFXMLNode.h | raw::CFXMLDocumentTypeInfo |
| CFXMLElementInfo | typedef struct | CoreFoundation/CFXMLNode.h | raw::CFXMLElementInfo |
| CFXMLElementTypeDeclarationInfo | typedef struct | CoreFoundation/CFXMLNode.h | raw::CFXMLElementTypeDeclarationInfo |
| CFXMLEntityInfo | typedef struct | CoreFoundation/CFXMLNode.h | raw::CFXMLEntityInfo |
| CFXMLEntityReferenceInfo | typedef struct | CoreFoundation/CFXMLNode.h | raw::CFXMLEntityReferenceInfo |
| CFXMLEntityTypeCode | typedef enum | CoreFoundation/CFXMLNode.h | raw::CFXMLEntityTypeCode |
| CFXMLExternalID | typedef struct | CoreFoundation/CFXMLNode.h | raw::CFXMLExternalID |
| CFXMLNodeRef | typedef struct | CoreFoundation/CFXMLNode.h | raw::CFXMLNodeRef |
| CFXMLNodeTypeCode | typedef enum | CoreFoundation/CFXMLNode.h | raw::CFXMLNodeTypeCode |
| CFXMLNotationInfo | typedef struct | CoreFoundation/CFXMLNode.h | raw::CFXMLNotationInfo |
| CFXMLParserCallBacks | typedef struct | CoreFoundation/CFXMLParser.h | raw::CFXMLParserCallBacks |
| CFXMLParserContext | typedef struct | CoreFoundation/CFXMLParser.h | raw::CFXMLParserContext |
| CFXMLParserOptions | typedef enum | CoreFoundation/CFXMLParser.h | raw::CFXMLParserOptions |
| CFXMLParserRef | typedef struct | CoreFoundation/CFXMLParser.h | raw::CFXMLParserRef |
| CFXMLParserStatusCode | typedef enum | CoreFoundation/CFXMLParser.h | raw::CFXMLParserStatusCode |
| CFXMLProcessingInstructionInfo | typedef struct | CoreFoundation/CFXMLNode.h | raw::CFXMLProcessingInstructionInfo |
| CM2Header | typedef struct | ? | raw::CM2Header |
| CM2Profile | typedef struct | ? | raw::CM2Profile |
| CM2ProfileHandle | typedef struct | ? | raw::CM2ProfileHandle |
| CM2ProfilePtr | typedef struct | ? | raw::CM2ProfilePtr |
| CM4Header | typedef struct | ? | raw::CM4Header |
| CMAdaptationMatrixType | typedef struct | ? | raw::CMAdaptationMatrixType |
| CMAppleProfileHeader | struct | ? | raw::CMAppleProfileHeader |
| CMAudioDeviceClockCreate | function | CoreMedia/CMAudioDeviceClock.h | raw::CMAudioDeviceClockCreate |
| CMAudioDeviceClockCreateFromAudioDeviceID | function | CoreMedia/CMAudioDeviceClock.h | raw::CMAudioDeviceClockCreateFromAudioDeviceID |
| CMAudioDeviceClockGetAudioDevice | function | CoreMedia/CMAudioDeviceClock.h | raw::CMAudioDeviceClockGetAudioDevice |
| CMAudioDeviceClockSetAudioDeviceID | function | CoreMedia/CMAudioDeviceClock.h | raw::CMAudioDeviceClockSetAudioDeviceID |
| CMAudioDeviceClockSetAudioDeviceUID | function | CoreMedia/CMAudioDeviceClock.h | raw::CMAudioDeviceClockSetAudioDeviceUID |
| CMAudioFormatDescriptionCopyAsBigEndianSoundDescriptionBlockBuffer | function | CoreMedia/CMFormatDescriptionBridge.h | raw::CMAudioFormatDescriptionCopyAsBigEndianSoundDescriptionBlockBuffer |
| CMAudioFormatDescriptionCreate | function | CoreMedia/CMFormatDescription.h | raw::CMAudioFormatDescriptionCreate |
| CMAudioFormatDescriptionCreateFromBigEndianSoundDescriptionBlockBuffer | function | CoreMedia/CMFormatDescriptionBridge.h | raw::CMAudioFormatDescriptionCreateFromBigEndianSoundDescriptionBlockBuffer |
| CMAudioFormatDescriptionCreateFromBigEndianSoundDescriptionData | function | CoreMedia/CMFormatDescriptionBridge.h | raw::CMAudioFormatDescriptionCreateFromBigEndianSoundDescriptionData |
| CMAudioFormatDescriptionCreateSummary | function | CoreMedia/CMFormatDescription.h | raw::CMAudioFormatDescriptionCreateSummary |
| CMAudioFormatDescriptionEqual | function | CoreMedia/CMFormatDescription.h | raw::CMAudioFormatDescriptionEqual |
| CMAudioFormatDescriptionGetChannelLayout | function | CoreMedia/CMFormatDescription.h | raw::CMAudioFormatDescriptionGetChannelLayout |
| CMAudioFormatDescriptionGetFormatList | function | CoreMedia/CMFormatDescription.h | raw::CMAudioFormatDescriptionGetFormatList |
| CMAudioFormatDescriptionGetMagicCookie | function | CoreMedia/CMFormatDescription.h | raw::CMAudioFormatDescriptionGetMagicCookie |
| CMAudioFormatDescriptionGetMostCompatibleFormat | function | CoreMedia/CMFormatDescription.h | raw::CMAudioFormatDescriptionGetMostCompatibleFormat |
| CMAudioFormatDescriptionGetRichestDecodableFormat | function | CoreMedia/CMFormatDescription.h | raw::CMAudioFormatDescriptionGetRichestDecodableFormat |
| CMAudioFormatDescriptionGetStreamBasicDescription | function | CoreMedia/CMFormatDescription.h | cm::CMFormatDescription |
| CMAudioSampleBufferCreateReadyWithPacketDescriptions | function | CoreMedia/CMSampleBuffer.h | raw::CMAudioSampleBufferCreateReadyWithPacketDescriptions |
| CMAudioSampleBufferCreateWithPacketDescriptions | function | CoreMedia/CMSampleBuffer.h | raw::CMAudioSampleBufferCreateWithPacketDescriptions |
| CMAudioSampleBufferCreateWithPacketDescriptionsAndMakeDataReadyHandler | function | CoreMedia/CMSampleBuffer.h | raw::CMAudioSampleBufferCreateWithPacketDescriptionsAndMakeDataReadyHandler |
| CMBitmap | typedef struct | ? | raw::CMBitmap |
| CMBlockBufferAccessDataBytes | function | CoreMedia/CMBlockBuffer.h | raw::CMBlockBufferAccessDataBytes |
| CMBlockBufferAppendBufferReference | function | CoreMedia/CMBlockBuffer.h | raw::CMBlockBufferAppendBufferReference |
| CMBlockBufferAppendMemoryBlock | function | CoreMedia/CMBlockBuffer.h | raw::CMBlockBufferAppendMemoryBlock |
| CMBlockBufferAssureBlockMemory | function | CoreMedia/CMBlockBuffer.h | raw::CMBlockBufferAssureBlockMemory |
| CMBlockBufferCopyDataBytes | function | CoreMedia/CMBlockBuffer.h | cm::CMBlockBuffer |
| CMBlockBufferCreateContiguous | function | CoreMedia/CMBlockBuffer.h | raw::CMBlockBufferCreateContiguous |
| CMBlockBufferCreateEmpty | function | CoreMedia/CMBlockBuffer.h | cm::CMBlockBuffer |
| CMBlockBufferCreateWithBufferReference | function | CoreMedia/CMBlockBuffer.h | raw::CMBlockBufferCreateWithBufferReference |
| CMBlockBufferCreateWithMemoryBlock | function | CoreMedia/CMBlockBuffer.h | cm::CMBlockBuffer |
| CMBlockBufferCustomBlockSource | typedef struct | CoreMedia/CMBlockBuffer.h | raw::CMBlockBufferCustomBlockSource |
| CMBlockBufferFillDataBytes | function | CoreMedia/CMBlockBuffer.h | raw::CMBlockBufferFillDataBytes |
| CMBlockBufferGetDataLength | function | CoreMedia/CMBlockBuffer.h | cm::CMBlockBuffer |
| CMBlockBufferGetDataPointer | function | CoreMedia/CMBlockBuffer.h | cm::CMBlockBuffer |
| CMBlockBufferGetTypeID | function | CoreMedia/CMBlockBuffer.h | raw::CMBlockBufferGetTypeID |
| CMBlockBufferIsEmpty | function | CoreMedia/CMBlockBuffer.h | cm::CMBlockBuffer |
| CMBlockBufferIsRangeContiguous | function | CoreMedia/CMBlockBuffer.h | cm::CMBlockBuffer |
| CMBlockBufferRef | typedef struct | CoreMedia/CMBlockBuffer.h | cm::CMBlockBuffer |
| CMBlockBufferReplaceDataBytes | function | CoreMedia/CMBlockBuffer.h | cm::CMBlockBuffer |
| CMBufferCallbacks | typedef struct | CoreMedia/CMBufferQueue.h | raw::CMBufferCallbacks |
| CMBufferHandlers | typedef struct | CoreMedia/CMBufferQueue.h | raw::CMBufferHandlers |
| CMBufferLocation | typedef struct | ? | raw::CMBufferLocation |
| CMBufferQueueCallForEachBuffer | function | CoreMedia/CMBufferQueue.h | raw::CMBufferQueueCallForEachBuffer |
| CMBufferQueueContainsEndOfData | function | CoreMedia/CMBufferQueue.h | raw::CMBufferQueueContainsEndOfData |
| CMBufferQueueCopyHead | function | CoreMedia/CMBufferQueue.h | raw::CMBufferQueueCopyHead |
| CMBufferQueueCreate | function | CoreMedia/CMBufferQueue.h | raw::CMBufferQueueCreate |
| CMBufferQueueCreateWithHandlers | function | CoreMedia/CMBufferQueue.h | raw::CMBufferQueueCreateWithHandlers |
| CMBufferQueueDequeueAndRetain | function | CoreMedia/CMBufferQueue.h | raw::CMBufferQueueDequeueAndRetain |
| CMBufferQueueDequeueIfDataReadyAndRetain | function | CoreMedia/CMBufferQueue.h | raw::CMBufferQueueDequeueIfDataReadyAndRetain |
| CMBufferQueueEnqueue | function | CoreMedia/CMBufferQueue.h | raw::CMBufferQueueEnqueue |
| CMBufferQueueGetBufferCount | function | CoreMedia/CMBufferQueue.h | raw::CMBufferQueueGetBufferCount |
| CMBufferQueueGetCallbacksForSampleBuffersSortedByOutputPTS | function | CoreMedia/CMBufferQueue.h | raw::CMBufferQueueGetCallbacksForSampleBuffersSortedByOutputPTS |
| CMBufferQueueGetCallbacksForUnsortedSampleBuffers | function | CoreMedia/CMBufferQueue.h | raw::CMBufferQueueGetCallbacksForUnsortedSampleBuffers |
| CMBufferQueueGetDuration | function | CoreMedia/CMBufferQueue.h | raw::CMBufferQueueGetDuration |
| CMBufferQueueGetEndPresentationTimeStamp | function | CoreMedia/CMBufferQueue.h | raw::CMBufferQueueGetEndPresentationTimeStamp |
| CMBufferQueueGetFirstDecodeTimeStamp | function | CoreMedia/CMBufferQueue.h | raw::CMBufferQueueGetFirstDecodeTimeStamp |
| CMBufferQueueGetFirstPresentationTimeStamp | function | CoreMedia/CMBufferQueue.h | raw::CMBufferQueueGetFirstPresentationTimeStamp |
| CMBufferQueueGetHead | function | CoreMedia/CMBufferQueue.h | raw::CMBufferQueueGetHead |
| CMBufferQueueGetMaxPresentationTimeStamp | function | CoreMedia/CMBufferQueue.h | raw::CMBufferQueueGetMaxPresentationTimeStamp |
| CMBufferQueueGetMinDecodeTimeStamp | function | CoreMedia/CMBufferQueue.h | raw::CMBufferQueueGetMinDecodeTimeStamp |
| CMBufferQueueGetMinPresentationTimeStamp | function | CoreMedia/CMBufferQueue.h | raw::CMBufferQueueGetMinPresentationTimeStamp |
| CMBufferQueueGetTotalSize | function | CoreMedia/CMBufferQueue.h | raw::CMBufferQueueGetTotalSize |
| CMBufferQueueGetTypeID | function | CoreMedia/CMBufferQueue.h | raw::CMBufferQueueGetTypeID |
| CMBufferQueueInstallTrigger | function | CoreMedia/CMBufferQueue.h | raw::CMBufferQueueInstallTrigger |
| CMBufferQueueInstallTriggerHandler | function | CoreMedia/CMBufferQueue.h | raw::CMBufferQueueInstallTriggerHandler |
| CMBufferQueueInstallTriggerHandlerWithIntegerThreshold | function | CoreMedia/CMBufferQueue.h | raw::CMBufferQueueInstallTriggerHandlerWithIntegerThreshold |
| CMBufferQueueInstallTriggerWithIntegerThreshold | function | CoreMedia/CMBufferQueue.h | raw::CMBufferQueueInstallTriggerWithIntegerThreshold |
| CMBufferQueueIsAtEndOfData | function | CoreMedia/CMBufferQueue.h | raw::CMBufferQueueIsAtEndOfData |
| CMBufferQueueIsEmpty | function | CoreMedia/CMBufferQueue.h | raw::CMBufferQueueIsEmpty |
| CMBufferQueueMarkEndOfData | function | CoreMedia/CMBufferQueue.h | raw::CMBufferQueueMarkEndOfData |
| CMBufferQueueRef | typedef struct | CoreMedia/CMBufferQueue.h | raw::CMBufferQueueRef |
| CMBufferQueueRemoveTrigger | function | CoreMedia/CMBufferQueue.h | raw::CMBufferQueueRemoveTrigger |
| CMBufferQueueReset | function | CoreMedia/CMBufferQueue.h | raw::CMBufferQueueReset |
| CMBufferQueueResetWithCallback | function | CoreMedia/CMBufferQueue.h | raw::CMBufferQueueResetWithCallback |
| CMBufferQueueSetValidationCallback | function | CoreMedia/CMBufferQueue.h | raw::CMBufferQueueSetValidationCallback |
| CMBufferQueueSetValidationHandler | function | CoreMedia/CMBufferQueue.h | raw::CMBufferQueueSetValidationHandler |
| CMBufferQueueTestTrigger | function | CoreMedia/CMBufferQueue.h | raw::CMBufferQueueTestTrigger |
| CMBufferQueueTriggerToken | typedef struct | CoreMedia/CMBufferQueue.h | raw::CMBufferQueueTriggerToken |
| CMCMYColor | typedef struct | ? | raw::CMCMYColor |
| CMCMYKColor | typedef struct | ? | raw::CMCMYKColor |
| CMClockConvertHostTimeToSystemUnits | function | CoreMedia/CMSync.h | raw::CMClockConvertHostTimeToSystemUnits |
| CMClockGetAnchorTime | function | CoreMedia/CMSync.h | raw::CMClockGetAnchorTime |
| CMClockGetHostTimeClock | function | CoreMedia/CMSync.h | cm::CMClock |
| CMClockGetTime | function | CoreMedia/CMSync.h | raw::CMClockGetTime |
| CMClockGetTypeID | function | CoreMedia/CMSync.h | raw::CMClockGetTypeID |
| CMClockInvalidate | function | CoreMedia/CMSync.h | raw::CMClockInvalidate |
| CMClockMakeHostTimeFromSystemUnits | function | CoreMedia/CMSync.h | raw::CMClockMakeHostTimeFromSystemUnits |
| CMClockMightDrift | function | CoreMedia/CMSync.h | raw::CMClockMightDrift |
| CMClockRef | typedef struct | CoreMedia/CMSync.h | cm::CMClock |
| CMClosedCaptionFormatDescriptionCopyAsBigEndianClosedCaptionDescriptionBlockBuffer | function | CoreMedia/CMFormatDescriptionBridge.h | raw::CMClosedCaptionFormatDescriptionCopyAsBigEndianClosedCaptionDescriptionBlockBuffer |
| CMClosedCaptionFormatDescriptionCreateFromBigEndianClosedCaptionDescriptionBlockBuffer | function | CoreMedia/CMFormatDescriptionBridge.h | raw::CMClosedCaptionFormatDescriptionCreateFromBigEndianClosedCaptionDescriptionBlockBuffer |
| CMClosedCaptionFormatDescriptionCreateFromBigEndianClosedCaptionDescriptionData | function | CoreMedia/CMFormatDescriptionBridge.h | raw::CMClosedCaptionFormatDescriptionCreateFromBigEndianClosedCaptionDescriptionData |
| CMColor | struct | ? | raw::CMColor |
| CMConcatProfileSet | typedef struct | ? | raw::CMConcatProfileSet |
| CMCopyDictionaryOfAttachments | function | CoreMedia/CMAttachment.h | raw::CMCopyDictionaryOfAttachments |
| CMCurveType | typedef struct | ? | raw::CMCurveType |
| CMDataType | typedef struct | ? | raw::CMDataType |
| CMDateTime | typedef struct | ? | raw::CMDateTime |
| CMDateTimeType | typedef struct | ? | raw::CMDateTimeType |
| CMDeviceInfo | typedef struct | ? | raw::CMDeviceInfo |
| CMDeviceInfoPtr | typedef struct | ? | raw::CMDeviceInfoPtr |
| CMDeviceProfileArray | typedef struct | ? | raw::CMDeviceProfileArray |
| CMDeviceProfileArrayPtr | typedef struct | ? | raw::CMDeviceProfileArrayPtr |
| CMDeviceProfileInfo | typedef struct | ? | raw::CMDeviceProfileInfo |
| CMDeviceProfileScope | typedef struct | ? | raw::CMDeviceProfileScope |
| CMDeviceScope | typedef struct | ? | raw::CMDeviceScope |
| CMDoesBigEndianSoundDescriptionRequireLegacyCBRSampleTableLayout | function | CoreMedia/CMFormatDescriptionBridge.h | raw::CMDoesBigEndianSoundDescriptionRequireLegacyCBRSampleTableLayout |
| CMFixedXYColor | typedef struct | ? | raw::CMFixedXYColor |
| CMFixedXYZColor | typedef struct | ? | raw::CMFixedXYZColor |
| CMFloatBitmap | typedef struct | ? | raw::CMFloatBitmap |
| CMFloatBitmapFlags | typedef enum | ? | raw::CMFloatBitmapFlags |
| CMFormatDescriptionCreate | function | CoreMedia/CMFormatDescription.h | raw::CMFormatDescriptionCreate |
| CMFormatDescriptionEqual | function | CoreMedia/CMFormatDescription.h | raw::CMFormatDescriptionEqual |
| CMFormatDescriptionEqualIgnoringExtensionKeys | function | CoreMedia/CMFormatDescription.h | raw::CMFormatDescriptionEqualIgnoringExtensionKeys |
| CMFormatDescriptionGetExtension | function | CoreMedia/CMFormatDescription.h | raw::CMFormatDescriptionGetExtension |
| CMFormatDescriptionGetExtensions | function | CoreMedia/CMFormatDescription.h | cm::CMFormatDescription |
| CMFormatDescriptionGetMediaSubType | function | CoreMedia/CMFormatDescription.h | cm::CMFormatDescription |
| CMFormatDescriptionGetMediaType | function | CoreMedia/CMFormatDescription.h | cm::CMFormatDescription |
| CMFormatDescriptionGetTypeID | function | CoreMedia/CMFormatDescription.h | raw::CMFormatDescriptionGetTypeID |
| CMFormatDescriptionRef | typedef struct | CoreMedia/CMFormatDescription.h | cm::CMFormatDescription |
| CMGetAttachment | function | CoreMedia/CMAttachment.h | raw::CMGetAttachment |
| CMGrayColor | typedef struct | ? | raw::CMGrayColor |
| CMHLSColor | typedef struct | ? | raw::CMHLSColor |
| CMHSVColor | typedef struct | ? | raw::CMHSVColor |
| CMHandleLocation | typedef struct | ? | raw::CMHandleLocation |
| CMIntentCRDVMSize | typedef struct | ? | raw::CMIntentCRDVMSize |
| CMLabColor | typedef struct | ? | raw::CMLabColor |
| CMLut16Type | typedef struct | ? | raw::CMLut16Type |
| CMLut8Type | typedef struct | ? | raw::CMLut8Type |
| CMLuvColor | typedef struct | ? | raw::CMLuvColor |
| CMMInfo | typedef struct | ? | raw::CMMInfo |
| CMMakeAndModel | typedef struct | ? | raw::CMMakeAndModel |
| CMMakeAndModelType | typedef struct | ? | raw::CMMakeAndModelType |
| CMMeasurementType | typedef struct | ? | raw::CMMeasurementType |
| CMMemoryPoolCreate | function | CoreMedia/CMMemoryPool.h | raw::CMMemoryPoolCreate |
| CMMemoryPoolFlush | function | CoreMedia/CMMemoryPool.h | raw::CMMemoryPoolFlush |
| CMMemoryPoolGetAllocator | function | CoreMedia/CMMemoryPool.h | raw::CMMemoryPoolGetAllocator |
| CMMemoryPoolGetTypeID | function | CoreMedia/CMMemoryPool.h | raw::CMMemoryPoolGetTypeID |
| CMMemoryPoolInvalidate | function | CoreMedia/CMMemoryPool.h | raw::CMMemoryPoolInvalidate |
| CMMemoryPoolRef | typedef struct | CoreMedia/CMMemoryPool.h | raw::CMMemoryPoolRef |
| CMMetadataCreateIdentifierForKeyAndKeySpace | function | CoreMedia/CMMetadata.h | raw::CMMetadataCreateIdentifierForKeyAndKeySpace |
| CMMetadataCreateKeyFromIdentifier | function | CoreMedia/CMMetadata.h | raw::CMMetadataCreateKeyFromIdentifier |
| CMMetadataCreateKeyFromIdentifierAsCFData | function | CoreMedia/CMMetadata.h | raw::CMMetadataCreateKeyFromIdentifierAsCFData |
| CMMetadataCreateKeySpaceFromIdentifier | function | CoreMedia/CMMetadata.h | raw::CMMetadataCreateKeySpaceFromIdentifier |
| CMMetadataDataTypeRegistryDataTypeConformsToDataType | function | CoreMedia/CMMetadata.h | raw::CMMetadataDataTypeRegistryDataTypeConformsToDataType |
| CMMetadataDataTypeRegistryDataTypeIsBaseDataType | function | CoreMedia/CMMetadata.h | raw::CMMetadataDataTypeRegistryDataTypeIsBaseDataType |
| CMMetadataDataTypeRegistryDataTypeIsRegistered | function | CoreMedia/CMMetadata.h | raw::CMMetadataDataTypeRegistryDataTypeIsRegistered |
| CMMetadataDataTypeRegistryGetBaseDataTypeForConformingDataType | function | CoreMedia/CMMetadata.h | raw::CMMetadataDataTypeRegistryGetBaseDataTypeForConformingDataType |
| CMMetadataDataTypeRegistryGetBaseDataTypes | function | CoreMedia/CMMetadata.h | raw::CMMetadataDataTypeRegistryGetBaseDataTypes |
| CMMetadataDataTypeRegistryGetConformingDataTypes | function | CoreMedia/CMMetadata.h | raw::CMMetadataDataTypeRegistryGetConformingDataTypes |
| CMMetadataDataTypeRegistryGetDataTypeDescription | function | CoreMedia/CMMetadata.h | raw::CMMetadataDataTypeRegistryGetDataTypeDescription |
| CMMetadataDataTypeRegistryRegisterDataType | function | CoreMedia/CMMetadata.h | raw::CMMetadataDataTypeRegistryRegisterDataType |
| CMMetadataFormatDescriptionCopyAsBigEndianMetadataDescriptionBlockBuffer | function | CoreMedia/CMFormatDescriptionBridge.h | raw::CMMetadataFormatDescriptionCopyAsBigEndianMetadataDescriptionBlockBuffer |
| CMMetadataFormatDescriptionCreateByMergingMetadataFormatDescriptions | function | CoreMedia/CMFormatDescription.h | cm::CMMetadataFormatDescription |
| CMMetadataFormatDescriptionCreateFromBigEndianMetadataDescriptionBlockBuffer | function | CoreMedia/CMFormatDescriptionBridge.h | raw::CMMetadataFormatDescriptionCreateFromBigEndianMetadataDescriptionBlockBuffer |
| CMMetadataFormatDescriptionCreateFromBigEndianMetadataDescriptionData | function | CoreMedia/CMFormatDescriptionBridge.h | raw::CMMetadataFormatDescriptionCreateFromBigEndianMetadataDescriptionData |
| CMMetadataFormatDescriptionCreateWithKeys | function | CoreMedia/CMFormatDescription.h | cm::CMMetadataFormatDescription |
| CMMetadataFormatDescriptionCreateWithMetadataFormatDescriptionAndMetadataSpecifications | function | CoreMedia/CMFormatDescription.h | cm::CMMetadataFormatDescription |
| CMMetadataFormatDescriptionCreateWithMetadataSpecifications | function | CoreMedia/CMFormatDescription.h | cm::CMMetadataFormatDescription |
| CMMetadataFormatDescriptionGetIdentifiers | function | CoreMedia/CMFormatDescription.h | cm::CMMetadataFormatDescription |
| CMMetadataFormatDescriptionGetKeyWithLocalID | function | CoreMedia/CMFormatDescription.h | cm::CMMetadataFormatDescription |
| CMMultiFunctCLUTType | typedef struct | ? | raw::CMMultiFunctCLUTType |
| CMMultiFunctLutA2BType | typedef struct | ? | raw::CMMultiFunctLutA2BType |
| CMMultiFunctLutB2AType | typedef struct | ? | raw::CMMultiFunctLutB2AType |
| CMMultiFunctLutType | typedef struct | ? | raw::CMMultiFunctLutType |
| CMMultiLocalizedUniCodeEntryRec | typedef struct | ? | raw::CMMultiLocalizedUniCodeEntryRec |
| CMMultiLocalizedUniCodeType | typedef struct | ? | raw::CMMultiLocalizedUniCodeType |
| CMMultichannel5Color | typedef struct | ? | raw::CMMultichannel5Color |
| CMMultichannel6Color | typedef struct | ? | raw::CMMultichannel6Color |
| CMMultichannel7Color | typedef struct | ? | raw::CMMultichannel7Color |
| CMMultichannel8Color | typedef struct | ? | raw::CMMultichannel8Color |
| CMMutableTagCollectionRef | typedef struct | CoreMedia/CMTagCollection.h | raw::CMMutableTagCollectionRef |
| CMMuxedFormatDescriptionCreate | function | CoreMedia/CMFormatDescription.h | raw::CMMuxedFormatDescriptionCreate |
| CMNamedColor | typedef struct | ? | raw::CMNamedColor |
| CMNamedColor2EntryType | typedef struct | ? | raw::CMNamedColor2EntryType |
| CMNamedColor2Type | typedef struct | ? | raw::CMNamedColor2Type |
| CMNamedColorType | typedef struct | ? | raw::CMNamedColorType |
| CMNativeDisplayInfo | typedef struct | ? | raw::CMNativeDisplayInfo |
| CMNativeDisplayInfoType | typedef struct | ? | raw::CMNativeDisplayInfoType |
| CMPS2CRDVMSizeType | typedef struct | ? | raw::CMPS2CRDVMSizeType |
| CMPackingType | typedef enum | CoreMedia/CMTag.h | raw::CMPackingType |
| CMParametricCurveType | typedef struct | ? | raw::CMParametricCurveType |
| CMPathLocation | typedef struct | ? | raw::CMPathLocation |
| CMProfLoc | struct | ? | raw::CMProfLoc |
| CMProfileIterateData | typedef struct | ? | raw::CMProfileIterateData |
| CMProfileLocation | typedef struct | ? | raw::CMProfileLocation |
| CMProfileRef | typedef struct | ? | raw::CMProfileRef |
| CMProfileSequenceDescType | typedef struct | ? | raw::CMProfileSequenceDescType |
| CMProjectionType | typedef enum | CoreMedia/CMTag.h | raw::CMProjectionType |
| CMPropagateAttachments | function | CoreMedia/CMAttachment.h | raw::CMPropagateAttachments |
| CMRGBColor | typedef struct | ? | raw::CMRGBColor |
| CMRemoveAllAttachments | function | CoreMedia/CMAttachment.h | raw::CMRemoveAllAttachments |
| CMRemoveAttachment | function | CoreMedia/CMAttachment.h | raw::CMRemoveAttachment |
| CMS15Fixed16ArrayType | typedef struct | ? | raw::CMS15Fixed16ArrayType |
| CMSCertificateChainMode | typedef enum | ? | raw::CMSCertificateChainMode |
| CMSDecoderCopyAllCerts | function | ? | raw::CMSDecoderCopyAllCerts |
| CMSDecoderCopyContent | function | ? | raw::CMSDecoderCopyContent |
| CMSDecoderCopyDetachedContent | function | ? | raw::CMSDecoderCopyDetachedContent |
| CMSDecoderCopyEncapsulatedContentType | function | ? | raw::CMSDecoderCopyEncapsulatedContentType |
| CMSDecoderCopySignerCert | function | ? | raw::CMSDecoderCopySignerCert |
| CMSDecoderCopySignerEmailAddress | function | ? | raw::CMSDecoderCopySignerEmailAddress |
| CMSDecoderCopySignerSigningTime | function | ? | raw::CMSDecoderCopySignerSigningTime |
| CMSDecoderCopySignerStatus | function | ? | raw::CMSDecoderCopySignerStatus |
| CMSDecoderCopySignerTimestamp | function | ? | raw::CMSDecoderCopySignerTimestamp |
| CMSDecoderCopySignerTimestampCertificates | function | ? | raw::CMSDecoderCopySignerTimestampCertificates |
| CMSDecoderCopySignerTimestampWithPolicy | function | ? | raw::CMSDecoderCopySignerTimestampWithPolicy |
| CMSDecoderCreate | function | ? | raw::CMSDecoderCreate |
| CMSDecoderFinalizeMessage | function | ? | raw::CMSDecoderFinalizeMessage |
| CMSDecoderGetNumSigners | function | ? | raw::CMSDecoderGetNumSigners |
| CMSDecoderGetTypeID | function | ? | raw::CMSDecoderGetTypeID |
| CMSDecoderIsContentEncrypted | function | ? | raw::CMSDecoderIsContentEncrypted |
| CMSDecoderRef | typedef struct | ? | raw::CMSDecoderRef |
| CMSDecoderSetDetachedContent | function | ? | raw::CMSDecoderSetDetachedContent |
| CMSDecoderSetSearchKeychain | function | ? | raw::CMSDecoderSetSearchKeychain |
| CMSDecoderUpdateMessage | function | ? | raw::CMSDecoderUpdateMessage |
| CMSEncode | function | ? | raw::CMSEncode |
| CMSEncodeContent | function | ? | raw::CMSEncodeContent |
| CMSEncoderAddRecipients | function | ? | raw::CMSEncoderAddRecipients |
| CMSEncoderAddSignedAttributes | function | ? | raw::CMSEncoderAddSignedAttributes |
| CMSEncoderAddSigners | function | ? | raw::CMSEncoderAddSigners |
| CMSEncoderAddSupportingCerts | function | ? | raw::CMSEncoderAddSupportingCerts |
| CMSEncoderCopyEncapsulatedContentType | function | ? | raw::CMSEncoderCopyEncapsulatedContentType |
| CMSEncoderCopyEncodedContent | function | ? | raw::CMSEncoderCopyEncodedContent |
| CMSEncoderCopyRecipients | function | ? | raw::CMSEncoderCopyRecipients |
| CMSEncoderCopySignerTimestamp | function | ? | raw::CMSEncoderCopySignerTimestamp |
| CMSEncoderCopySignerTimestampWithPolicy | function | ? | raw::CMSEncoderCopySignerTimestampWithPolicy |
| CMSEncoderCopySigners | function | ? | raw::CMSEncoderCopySigners |
| CMSEncoderCopySupportingCerts | function | ? | raw::CMSEncoderCopySupportingCerts |
| CMSEncoderCreate | function | ? | raw::CMSEncoderCreate |
| CMSEncoderGetCertificateChainMode | function | ? | raw::CMSEncoderGetCertificateChainMode |
| CMSEncoderGetHasDetachedContent | function | ? | raw::CMSEncoderGetHasDetachedContent |
| CMSEncoderGetTypeID | function | ? | raw::CMSEncoderGetTypeID |
| CMSEncoderRef | typedef struct | ? | raw::CMSEncoderRef |
| CMSEncoderSetCertificateChainMode | function | ? | raw::CMSEncoderSetCertificateChainMode |
| CMSEncoderSetEncapsulatedContentType | function | ? | raw::CMSEncoderSetEncapsulatedContentType |
| CMSEncoderSetEncapsulatedContentTypeOID | function | ? | raw::CMSEncoderSetEncapsulatedContentTypeOID |
| CMSEncoderSetHasDetachedContent | function | ? | raw::CMSEncoderSetHasDetachedContent |
| CMSEncoderSetSignerAlgorithm | function | ? | raw::CMSEncoderSetSignerAlgorithm |
| CMSEncoderUpdateContent | function | ? | raw::CMSEncoderUpdateContent |
| CMSSignedAttributes | typedef enum | ? | raw::CMSSignedAttributes |
| CMSSignerStatus | typedef enum | ? | raw::CMSSignerStatus |
| CMSampleBufferCallBlockForEachSample | function | CoreMedia/CMSampleBuffer.h | raw::CMSampleBufferCallBlockForEachSample |
| CMSampleBufferCallForEachSample | function | CoreMedia/CMSampleBuffer.h | raw::CMSampleBufferCallForEachSample |
| CMSampleBufferCopyPCMDataIntoAudioBufferList | function | CoreMedia/CMSampleBuffer.h | raw::CMSampleBufferCopyPCMDataIntoAudioBufferList |
| CMSampleBufferCopySampleBufferForRange | function | CoreMedia/CMSampleBuffer.h | raw::CMSampleBufferCopySampleBufferForRange |
| CMSampleBufferCreate | function | CoreMedia/CMSampleBuffer.h | raw::CMSampleBufferCreate |
| CMSampleBufferCreateCopy | function | CoreMedia/CMSampleBuffer.h | raw::CMSampleBufferCreateCopy |
| CMSampleBufferCreateCopyWithNewTiming | function | CoreMedia/CMSampleBuffer.h | raw::CMSampleBufferCreateCopyWithNewTiming |
| CMSampleBufferCreateForImageBuffer | function | CoreMedia/CMSampleBuffer.h | raw::CMSampleBufferCreateForImageBuffer |
| CMSampleBufferCreateForImageBufferWithMakeDataReadyHandler | function | CoreMedia/CMSampleBuffer.h | raw::CMSampleBufferCreateForImageBufferWithMakeDataReadyHandler |
| CMSampleBufferCreateForTaggedBufferGroup | function | CoreMedia/CMTaggedBufferGroup.h | raw::CMSampleBufferCreateForTaggedBufferGroup |
| CMSampleBufferCreateReady | function | CoreMedia/CMSampleBuffer.h | raw::CMSampleBufferCreateReady |
| CMSampleBufferCreateReadyWithImageBuffer | function | CoreMedia/CMSampleBuffer.h | raw::CMSampleBufferCreateReadyWithImageBuffer |
| CMSampleBufferCreateWithMakeDataReadyHandler | function | CoreMedia/CMSampleBuffer.h | raw::CMSampleBufferCreateWithMakeDataReadyHandler |
| CMSampleBufferDataIsReady | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer |
| CMSampleBufferGetAudioBufferListWithRetainedBlockBuffer | function | CoreMedia/CMSampleBuffer.h | raw::CMSampleBufferGetAudioBufferListWithRetainedBlockBuffer |
| CMSampleBufferGetAudioStreamPacketDescriptions | function | CoreMedia/CMSampleBuffer.h | raw::CMSampleBufferGetAudioStreamPacketDescriptions |
| CMSampleBufferGetAudioStreamPacketDescriptionsPtr | function | CoreMedia/CMSampleBuffer.h | raw::CMSampleBufferGetAudioStreamPacketDescriptionsPtr |
| CMSampleBufferGetDataBuffer | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer |
| CMSampleBufferGetDecodeTimeStamp | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer |
| CMSampleBufferGetDuration | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer |
| CMSampleBufferGetFormatDescription | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer |
| CMSampleBufferGetImageBuffer | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer |
| CMSampleBufferGetNumSamples | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer |
| CMSampleBufferGetOutputDecodeTimeStamp | function | CoreMedia/CMSampleBuffer.h | raw::CMSampleBufferGetOutputDecodeTimeStamp |
| CMSampleBufferGetOutputDuration | function | CoreMedia/CMSampleBuffer.h | raw::CMSampleBufferGetOutputDuration |
| CMSampleBufferGetOutputPresentationTimeStamp | function | CoreMedia/CMSampleBuffer.h | raw::CMSampleBufferGetOutputPresentationTimeStamp |
| CMSampleBufferGetOutputSampleTimingInfoArray | function | CoreMedia/CMSampleBuffer.h | raw::CMSampleBufferGetOutputSampleTimingInfoArray |
| CMSampleBufferGetPresentationTimeStamp | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer |
| CMSampleBufferGetSampleAttachmentsArray | function | CoreMedia/CMSampleBuffer.h | raw::CMSampleBufferGetSampleAttachmentsArray |
| CMSampleBufferGetSampleSize | function | CoreMedia/CMSampleBuffer.h | raw::CMSampleBufferGetSampleSize |
| CMSampleBufferGetSampleSizeArray | function | CoreMedia/CMSampleBuffer.h | raw::CMSampleBufferGetSampleSizeArray |
| CMSampleBufferGetSampleTimingInfo | function | CoreMedia/CMSampleBuffer.h | raw::CMSampleBufferGetSampleTimingInfo |
| CMSampleBufferGetSampleTimingInfoArray | function | CoreMedia/CMSampleBuffer.h | raw::CMSampleBufferGetSampleTimingInfoArray |
| CMSampleBufferGetTaggedBufferGroup | function | CoreMedia/CMTaggedBufferGroup.h | raw::CMSampleBufferGetTaggedBufferGroup |
| CMSampleBufferGetTotalSampleSize | function | CoreMedia/CMSampleBuffer.h | raw::CMSampleBufferGetTotalSampleSize |
| CMSampleBufferGetTypeID | function | CoreMedia/CMSampleBuffer.h | raw::CMSampleBufferGetTypeID |
| CMSampleBufferHasDataFailed | function | CoreMedia/CMSampleBuffer.h | raw::CMSampleBufferHasDataFailed |
| CMSampleBufferInvalidate | function | CoreMedia/CMSampleBuffer.h | raw::CMSampleBufferInvalidate |
| CMSampleBufferIsValid | function | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer |
| CMSampleBufferMakeDataReady | function | CoreMedia/CMSampleBuffer.h | raw::CMSampleBufferMakeDataReady |
| CMSampleBufferRef | typedef struct | CoreMedia/CMSampleBuffer.h | cm::CMSampleBuffer |
| CMSampleBufferSetDataBuffer | function | CoreMedia/CMSampleBuffer.h | raw::CMSampleBufferSetDataBuffer |
| CMSampleBufferSetDataBufferFromAudioBufferList | function | CoreMedia/CMSampleBuffer.h | raw::CMSampleBufferSetDataBufferFromAudioBufferList |
| CMSampleBufferSetDataFailed | function | CoreMedia/CMSampleBuffer.h | raw::CMSampleBufferSetDataFailed |
| CMSampleBufferSetDataReady | function | CoreMedia/CMSampleBuffer.h | raw::CMSampleBufferSetDataReady |
| CMSampleBufferSetInvalidateCallback | function | CoreMedia/CMSampleBuffer.h | raw::CMSampleBufferSetInvalidateCallback |
| CMSampleBufferSetInvalidateHandler | function | CoreMedia/CMSampleBuffer.h | raw::CMSampleBufferSetInvalidateHandler |
| CMSampleBufferSetOutputPresentationTimeStamp | function | CoreMedia/CMSampleBuffer.h | raw::CMSampleBufferSetOutputPresentationTimeStamp |
| CMSampleBufferTrackDataReadiness | function | CoreMedia/CMSampleBuffer.h | raw::CMSampleBufferTrackDataReadiness |
| CMSampleTimingInfo | typedef struct | CoreMedia/CMSampleBuffer.h | cm::CMTime |
| CMScreeningChannelRec | typedef struct | ? | raw::CMScreeningChannelRec |
| CMScreeningType | typedef struct | ? | raw::CMScreeningType |
| CMSetAttachment | function | CoreMedia/CMAttachment.h | raw::CMSetAttachment |
| CMSetAttachments | function | CoreMedia/CMAttachment.h | raw::CMSetAttachments |
| CMSignatureType | typedef struct | ? | raw::CMSignatureType |
| CMSimpleQueueCreate | function | CoreMedia/CMSimpleQueue.h | raw::CMSimpleQueueCreate |
| CMSimpleQueueDequeue | function | CoreMedia/CMSimpleQueue.h | raw::CMSimpleQueueDequeue |
| CMSimpleQueueEnqueue | function | CoreMedia/CMSimpleQueue.h | raw::CMSimpleQueueEnqueue |
| CMSimpleQueueGetCapacity | function | CoreMedia/CMSimpleQueue.h | raw::CMSimpleQueueGetCapacity |
| CMSimpleQueueGetCount | function | CoreMedia/CMSimpleQueue.h | raw::CMSimpleQueueGetCount |
| CMSimpleQueueGetHead | function | CoreMedia/CMSimpleQueue.h | raw::CMSimpleQueueGetHead |
| CMSimpleQueueGetTypeID | function | CoreMedia/CMSimpleQueue.h | raw::CMSimpleQueueGetTypeID |
| CMSimpleQueueRef | typedef struct | CoreMedia/CMSimpleQueue.h | raw::CMSimpleQueueRef |
| CMSimpleQueueReset | function | CoreMedia/CMSimpleQueue.h | raw::CMSimpleQueueReset |
| CMStereoViewComponents | typedef enum | CoreMedia/CMTag.h | raw::CMStereoViewComponents |
| CMStereoViewInterpretationOptions | typedef enum | CoreMedia/CMTag.h | raw::CMStereoViewInterpretationOptions |
| CMSwapBigEndianClosedCaptionDescriptionToHost | function | CoreMedia/CMFormatDescriptionBridge.h | raw::CMSwapBigEndianClosedCaptionDescriptionToHost |
| CMSwapBigEndianImageDescriptionToHost | function | CoreMedia/CMFormatDescriptionBridge.h | raw::CMSwapBigEndianImageDescriptionToHost |
| CMSwapBigEndianMetadataDescriptionToHost | function | CoreMedia/CMFormatDescriptionBridge.h | raw::CMSwapBigEndianMetadataDescriptionToHost |
| CMSwapBigEndianSoundDescriptionToHost | function | CoreMedia/CMFormatDescriptionBridge.h | raw::CMSwapBigEndianSoundDescriptionToHost |
| CMSwapBigEndianTextDescriptionToHost | function | CoreMedia/CMFormatDescriptionBridge.h | raw::CMSwapBigEndianTextDescriptionToHost |
| CMSwapBigEndianTimeCodeDescriptionToHost | function | CoreMedia/CMFormatDescriptionBridge.h | raw::CMSwapBigEndianTimeCodeDescriptionToHost |
| CMSwapHostEndianClosedCaptionDescriptionToBig | function | CoreMedia/CMFormatDescriptionBridge.h | raw::CMSwapHostEndianClosedCaptionDescriptionToBig |
| CMSwapHostEndianImageDescriptionToBig | function | CoreMedia/CMFormatDescriptionBridge.h | raw::CMSwapHostEndianImageDescriptionToBig |
| CMSwapHostEndianMetadataDescriptionToBig | function | CoreMedia/CMFormatDescriptionBridge.h | raw::CMSwapHostEndianMetadataDescriptionToBig |
| CMSwapHostEndianSoundDescriptionToBig | function | CoreMedia/CMFormatDescriptionBridge.h | raw::CMSwapHostEndianSoundDescriptionToBig |
| CMSwapHostEndianTextDescriptionToBig | function | CoreMedia/CMFormatDescriptionBridge.h | raw::CMSwapHostEndianTextDescriptionToBig |
| CMSwapHostEndianTimeCodeDescriptionToBig | function | CoreMedia/CMFormatDescriptionBridge.h | raw::CMSwapHostEndianTimeCodeDescriptionToBig |
| CMSyncConvertTime | function | CoreMedia/CMSync.h | raw::CMSyncConvertTime |
| CMSyncGetRelativeRate | function | CoreMedia/CMSync.h | raw::CMSyncGetRelativeRate |
| CMSyncGetRelativeRateAndAnchorTime | function | CoreMedia/CMSync.h | raw::CMSyncGetRelativeRateAndAnchorTime |
| CMSyncGetTime | function | CoreMedia/CMSync.h | raw::CMSyncGetTime |
| CMSyncMightDrift | function | CoreMedia/CMSync.h | raw::CMSyncMightDrift |
| CMTag | typedef struct | CoreMedia/CMTag.h | raw::CMTag |
| CMTagCategory | typedef enum | CoreMedia/CMTag.h | raw::CMTagCategory |
| CMTagCategoryEqualToTagCategory | function | CoreMedia/CMTag.h | raw::CMTagCategoryEqualToTagCategory |
| CMTagCategoryValueEqualToValue | function | CoreMedia/CMTag.h | raw::CMTagCategoryValueEqualToValue |
| CMTagCollectionAddTag | function | CoreMedia/CMTagCollection.h | raw::CMTagCollectionAddTag |
| CMTagCollectionAddTagsFromArray | function | CoreMedia/CMTagCollection.h | raw::CMTagCollectionAddTagsFromArray |
| CMTagCollectionAddTagsFromCollection | function | CoreMedia/CMTagCollection.h | raw::CMTagCollectionAddTagsFromCollection |
| CMTagCollectionApply | function | CoreMedia/CMTagCollection.h | raw::CMTagCollectionApply |
| CMTagCollectionApplyUntil | function | CoreMedia/CMTagCollection.h | raw::CMTagCollectionApplyUntil |
| CMTagCollectionContainsCategory | function | CoreMedia/CMTagCollection.h | raw::CMTagCollectionContainsCategory |
| CMTagCollectionContainsSpecifiedTags | function | CoreMedia/CMTagCollection.h | raw::CMTagCollectionContainsSpecifiedTags |
| CMTagCollectionContainsTag | function | CoreMedia/CMTagCollection.h | raw::CMTagCollectionContainsTag |
| CMTagCollectionContainsTagsOfCollection | function | CoreMedia/CMTagCollection.h | raw::CMTagCollectionContainsTagsOfCollection |
| CMTagCollectionCopyAsData | function | CoreMedia/CMTagCollection.h | raw::CMTagCollectionCopyAsData |
| CMTagCollectionCopyAsDictionary | function | CoreMedia/CMTagCollection.h | raw::CMTagCollectionCopyAsDictionary |
| CMTagCollectionCopyDescription | function | CoreMedia/CMTagCollection.h | raw::CMTagCollectionCopyDescription |
| CMTagCollectionCopyTagsOfCategories | function | CoreMedia/CMTagCollection.h | raw::CMTagCollectionCopyTagsOfCategories |
| CMTagCollectionCountTagsWithFilterFunction | function | CoreMedia/CMTagCollection.h | raw::CMTagCollectionCountTagsWithFilterFunction |
| CMTagCollectionCreate | function | CoreMedia/CMTagCollection.h | raw::CMTagCollectionCreate |
| CMTagCollectionCreateCopy | function | CoreMedia/CMTagCollection.h | raw::CMTagCollectionCreateCopy |
| CMTagCollectionCreateDifference | function | CoreMedia/CMTagCollection.h | raw::CMTagCollectionCreateDifference |
| CMTagCollectionCreateExclusiveOr | function | CoreMedia/CMTagCollection.h | raw::CMTagCollectionCreateExclusiveOr |
| CMTagCollectionCreateFromData | function | CoreMedia/CMTagCollection.h | raw::CMTagCollectionCreateFromData |
| CMTagCollectionCreateFromDictionary | function | CoreMedia/CMTagCollection.h | raw::CMTagCollectionCreateFromDictionary |
| CMTagCollectionCreateIntersection | function | CoreMedia/CMTagCollection.h | raw::CMTagCollectionCreateIntersection |
| CMTagCollectionCreateMutable | function | CoreMedia/CMTagCollection.h | raw::CMTagCollectionCreateMutable |
| CMTagCollectionCreateMutableCopy | function | CoreMedia/CMTagCollection.h | raw::CMTagCollectionCreateMutableCopy |
| CMTagCollectionCreateUnion | function | CoreMedia/CMTagCollection.h | raw::CMTagCollectionCreateUnion |
| CMTagCollectionError | typedef enum | CoreMedia/CMTagCollection.h | raw::CMTagCollectionError |
| CMTagCollectionGetCount | function | CoreMedia/CMTagCollection.h | raw::CMTagCollectionGetCount |
| CMTagCollectionGetCountOfCategory | function | CoreMedia/CMTagCollection.h | raw::CMTagCollectionGetCountOfCategory |
| CMTagCollectionGetTags | function | CoreMedia/CMTagCollection.h | raw::CMTagCollectionGetTags |
| CMTagCollectionGetTagsWithCategory | function | CoreMedia/CMTagCollection.h | raw::CMTagCollectionGetTagsWithCategory |
| CMTagCollectionGetTagsWithFilterFunction | function | CoreMedia/CMTagCollection.h | raw::CMTagCollectionGetTagsWithFilterFunction |
| CMTagCollectionGetTypeID | function | CoreMedia/CMTagCollection.h | raw::CMTagCollectionGetTypeID |
| CMTagCollectionIsEmpty | function | CoreMedia/CMTagCollection.h | raw::CMTagCollectionIsEmpty |
| CMTagCollectionRef | typedef struct | CoreMedia/CMTagCollection.h | raw::CMTagCollectionRef |
| CMTagCollectionRemoveAllTags | function | CoreMedia/CMTagCollection.h | raw::CMTagCollectionRemoveAllTags |
| CMTagCollectionRemoveAllTagsOfCategory | function | CoreMedia/CMTagCollection.h | raw::CMTagCollectionRemoveAllTagsOfCategory |
| CMTagCollectionRemoveTag | function | CoreMedia/CMTagCollection.h | raw::CMTagCollectionRemoveTag |
| CMTagCompare | function | CoreMedia/CMTag.h | raw::CMTagCompare |
| CMTagCopyAsDictionary | function | CoreMedia/CMTag.h | raw::CMTagCopyAsDictionary |
| CMTagCopyDescription | function | CoreMedia/CMTag.h | raw::CMTagCopyDescription |
| CMTagDataType | typedef enum | CoreMedia/CMTag.h | raw::CMTagDataType |
| CMTagElemTable | typedef struct | ? | raw::CMTagElemTable |
| CMTagEqualToTag | function | CoreMedia/CMTag.h | raw::CMTagEqualToTag |
| CMTagError | typedef enum | CoreMedia/CMTag.h | raw::CMTagError |
| CMTagGetCategory | function | CoreMedia/CMTag.h | raw::CMTagGetCategory |
| CMTagGetFlagsValue | function | CoreMedia/CMTag.h | raw::CMTagGetFlagsValue |
| CMTagGetFloat64Value | function | CoreMedia/CMTag.h | raw::CMTagGetFloat64Value |
| CMTagGetOSTypeValue | function | CoreMedia/CMTag.h | raw::CMTagGetOSTypeValue |
| CMTagGetSInt64Value | function | CoreMedia/CMTag.h | raw::CMTagGetSInt64Value |
| CMTagGetValue | function | CoreMedia/CMTag.h | raw::CMTagGetValue |
| CMTagGetValueDataType | function | CoreMedia/CMTag.h | raw::CMTagGetValueDataType |
| CMTagHasCategory | function | CoreMedia/CMTag.h | raw::CMTagHasCategory |
| CMTagHasFlagsValue | function | CoreMedia/CMTag.h | raw::CMTagHasFlagsValue |
| CMTagHasFloat64Value | function | CoreMedia/CMTag.h | raw::CMTagHasFloat64Value |
| CMTagHasOSTypeValue | function | CoreMedia/CMTag.h | raw::CMTagHasOSTypeValue |
| CMTagHasSInt64Value | function | CoreMedia/CMTag.h | raw::CMTagHasSInt64Value |
| CMTagHash | function | CoreMedia/CMTag.h | raw::CMTagHash |
| CMTagIsValid | function | CoreMedia/CMTag.h | raw::CMTagIsValid |
| CMTagMakeFromDictionary | function | CoreMedia/CMTag.h | raw::CMTagMakeFromDictionary |
| CMTagMakeWithFlagsValue | function | CoreMedia/CMTag.h | raw::CMTagMakeWithFlagsValue |
| CMTagMakeWithFloat64Value | function | CoreMedia/CMTag.h | raw::CMTagMakeWithFloat64Value |
| CMTagMakeWithOSTypeValue | function | CoreMedia/CMTag.h | raw::CMTagMakeWithOSTypeValue |
| CMTagMakeWithSInt64Value | function | CoreMedia/CMTag.h | raw::CMTagMakeWithSInt64Value |
| CMTagRecord | typedef struct | ? | raw::CMTagRecord |
| CMTaggedBufferGroupCreate | function | CoreMedia/CMTaggedBufferGroup.h | raw::CMTaggedBufferGroupCreate |
| CMTaggedBufferGroupCreateCombined | function | CoreMedia/CMTaggedBufferGroup.h | raw::CMTaggedBufferGroupCreateCombined |
| CMTaggedBufferGroupError | typedef enum | CoreMedia/CMTaggedBufferGroup.h | raw::CMTaggedBufferGroupError |
| CMTaggedBufferGroupFormatDescriptionCreateForTaggedBufferGroup | function | CoreMedia/CMTaggedBufferGroup.h | raw::CMTaggedBufferGroupFormatDescriptionCreateForTaggedBufferGroup |
| CMTaggedBufferGroupFormatDescriptionCreateForTaggedBufferGroupWithExtensions | function | CoreMedia/CMTaggedBufferGroup.h | raw::CMTaggedBufferGroupFormatDescriptionCreateForTaggedBufferGroupWithExtensions |
| CMTaggedBufferGroupFormatDescriptionMatchesTaggedBufferGroup | function | CoreMedia/CMTaggedBufferGroup.h | raw::CMTaggedBufferGroupFormatDescriptionMatchesTaggedBufferGroup |
| CMTaggedBufferGroupGetCMSampleBufferAtIndex | function | CoreMedia/CMTaggedBufferGroup.h | raw::CMTaggedBufferGroupGetCMSampleBufferAtIndex |
| CMTaggedBufferGroupGetCMSampleBufferForTag | function | CoreMedia/CMTaggedBufferGroup.h | raw::CMTaggedBufferGroupGetCMSampleBufferForTag |
| CMTaggedBufferGroupGetCMSampleBufferForTagCollection | function | CoreMedia/CMTaggedBufferGroup.h | raw::CMTaggedBufferGroupGetCMSampleBufferForTagCollection |
| CMTaggedBufferGroupGetCVPixelBufferAtIndex | function | CoreMedia/CMTaggedBufferGroup.h | raw::CMTaggedBufferGroupGetCVPixelBufferAtIndex |
| CMTaggedBufferGroupGetCVPixelBufferForTag | function | CoreMedia/CMTaggedBufferGroup.h | raw::CMTaggedBufferGroupGetCVPixelBufferForTag |
| CMTaggedBufferGroupGetCVPixelBufferForTagCollection | function | CoreMedia/CMTaggedBufferGroup.h | raw::CMTaggedBufferGroupGetCVPixelBufferForTagCollection |
| CMTaggedBufferGroupGetCount | function | CoreMedia/CMTaggedBufferGroup.h | raw::CMTaggedBufferGroupGetCount |
| CMTaggedBufferGroupGetNumberOfMatchesForTagCollection | function | CoreMedia/CMTaggedBufferGroup.h | raw::CMTaggedBufferGroupGetNumberOfMatchesForTagCollection |
| CMTaggedBufferGroupGetTagCollectionAtIndex | function | CoreMedia/CMTaggedBufferGroup.h | raw::CMTaggedBufferGroupGetTagCollectionAtIndex |
| CMTaggedBufferGroupGetTypeID | function | CoreMedia/CMTaggedBufferGroup.h | raw::CMTaggedBufferGroupGetTypeID |
| CMTaggedBufferGroupRef | typedef struct | CoreMedia/CMTaggedBufferGroup.h | raw::CMTaggedBufferGroupRef |
| CMTextDescriptionType | typedef struct | ? | raw::CMTextDescriptionType |
| CMTextFormatDescriptionCopyAsBigEndianTextDescriptionBlockBuffer | function | CoreMedia/CMFormatDescriptionBridge.h | raw::CMTextFormatDescriptionCopyAsBigEndianTextDescriptionBlockBuffer |
| CMTextFormatDescriptionCreateFromBigEndianTextDescriptionBlockBuffer | function | CoreMedia/CMFormatDescriptionBridge.h | raw::CMTextFormatDescriptionCreateFromBigEndianTextDescriptionBlockBuffer |
| CMTextFormatDescriptionCreateFromBigEndianTextDescriptionData | function | CoreMedia/CMFormatDescriptionBridge.h | raw::CMTextFormatDescriptionCreateFromBigEndianTextDescriptionData |
| CMTextFormatDescriptionGetDefaultStyle | function | CoreMedia/CMFormatDescription.h | raw::CMTextFormatDescriptionGetDefaultStyle |
| CMTextFormatDescriptionGetDefaultTextBox | function | CoreMedia/CMFormatDescription.h | raw::CMTextFormatDescriptionGetDefaultTextBox |
| CMTextFormatDescriptionGetDisplayFlags | function | CoreMedia/CMFormatDescription.h | raw::CMTextFormatDescriptionGetDisplayFlags |
| CMTextFormatDescriptionGetFontName | function | CoreMedia/CMFormatDescription.h | raw::CMTextFormatDescriptionGetFontName |
| CMTextFormatDescriptionGetJustification | function | CoreMedia/CMFormatDescription.h | raw::CMTextFormatDescriptionGetJustification |
| CMTextType | typedef struct | ? | raw::CMTextType |
| CMTime | typedef struct | CoreMedia/CMSampleBuffer.h | cm::CMTime |
| CMTimeAbsoluteValue | function | CoreMedia/CMTime.h | raw::CMTimeAbsoluteValue |
| CMTimeAdd | function | CoreMedia/CMTime.h | cm::CMTime |
| CMTimeClampToRange | function | CoreMedia/CMTimeRange.h | raw::CMTimeClampToRange |
| CMTimeCodeFormatDescriptionCopyAsBigEndianTimeCodeDescriptionBlockBuffer | function | CoreMedia/CMFormatDescriptionBridge.h | raw::CMTimeCodeFormatDescriptionCopyAsBigEndianTimeCodeDescriptionBlockBuffer |
| CMTimeCodeFormatDescriptionCreate | function | CoreMedia/CMFormatDescription.h | raw::CMTimeCodeFormatDescriptionCreate |
| CMTimeCodeFormatDescriptionCreateFromBigEndianTimeCodeDescriptionBlockBuffer | function | CoreMedia/CMFormatDescriptionBridge.h | raw::CMTimeCodeFormatDescriptionCreateFromBigEndianTimeCodeDescriptionBlockBuffer |
| CMTimeCodeFormatDescriptionCreateFromBigEndianTimeCodeDescriptionData | function | CoreMedia/CMFormatDescriptionBridge.h | raw::CMTimeCodeFormatDescriptionCreateFromBigEndianTimeCodeDescriptionData |
| CMTimeCodeFormatDescriptionGetFrameDuration | function | CoreMedia/CMFormatDescription.h | raw::CMTimeCodeFormatDescriptionGetFrameDuration |
| CMTimeCodeFormatDescriptionGetFrameQuanta | function | CoreMedia/CMFormatDescription.h | raw::CMTimeCodeFormatDescriptionGetFrameQuanta |
| CMTimeCodeFormatDescriptionGetTimeCodeFlags | function | CoreMedia/CMFormatDescription.h | raw::CMTimeCodeFormatDescriptionGetTimeCodeFlags |
| CMTimeCompare | function | CoreMedia/CMTime.h | cm::CMTime |
| CMTimeConvertScale | function | CoreMedia/CMTime.h | cm::CMTime |
| CMTimeCopyAsDictionary | function | CoreMedia/CMTime.h | raw::CMTimeCopyAsDictionary |
| CMTimeCopyDescription | function | CoreMedia/CMTime.h | raw::CMTimeCopyDescription |
| CMTimeFlags | typedef enum | CoreMedia/CMTime.h | raw::CMTimeFlags |
| CMTimeFoldIntoRange | function | CoreMedia/CMTimeRange.h | raw::CMTimeFoldIntoRange |
| CMTimeGetSeconds | function | CoreMedia/CMTime.h | raw::CMTimeGetSeconds |
| CMTimeMake | function | CoreMedia/CMTime.h | raw::CMTimeMake |
| CMTimeMakeFromDictionary | function | CoreMedia/CMTime.h | raw::CMTimeMakeFromDictionary |
| CMTimeMakeWithEpoch | function | CoreMedia/CMTime.h | raw::CMTimeMakeWithEpoch |
| CMTimeMakeWithSeconds | function | CoreMedia/CMTime.h | cm::CMTime |
| CMTimeMapDurationFromRangeToRange | function | CoreMedia/CMTimeRange.h | raw::CMTimeMapDurationFromRangeToRange |
| CMTimeMapTimeFromRangeToRange | function | CoreMedia/CMTimeRange.h | raw::CMTimeMapTimeFromRangeToRange |
| CMTimeMapping | typedef struct | CoreMedia/CMTimeRange.h | raw::CMTimeMapping |
| CMTimeMappingCopyAsDictionary | function | CoreMedia/CMTimeRange.h | raw::CMTimeMappingCopyAsDictionary |
| CMTimeMappingCopyDescription | function | CoreMedia/CMTimeRange.h | raw::CMTimeMappingCopyDescription |
| CMTimeMappingMake | function | CoreMedia/CMTimeRange.h | raw::CMTimeMappingMake |
| CMTimeMappingMakeEmpty | function | CoreMedia/CMTimeRange.h | raw::CMTimeMappingMakeEmpty |
| CMTimeMappingMakeFromDictionary | function | CoreMedia/CMTimeRange.h | raw::CMTimeMappingMakeFromDictionary |
| CMTimeMappingShow | function | CoreMedia/CMTimeRange.h | raw::CMTimeMappingShow |
| CMTimeMaximum | function | CoreMedia/CMTime.h | raw::CMTimeMaximum |
| CMTimeMinimum | function | CoreMedia/CMTime.h | raw::CMTimeMinimum |
| CMTimeMultiply | function | CoreMedia/CMTime.h | cm::CMTime |
| CMTimeMultiplyByFloat64 | function | CoreMedia/CMTime.h | cm::CMTime |
| CMTimeMultiplyByRatio | function | CoreMedia/CMTime.h | raw::CMTimeMultiplyByRatio |
| CMTimeRange | typedef struct | CoreMedia/CMTimeRange.h | cm::CMTime |
| CMTimeRangeContainsTime | function | CoreMedia/CMTimeRange.h | cm::CMTime |
| CMTimeRangeContainsTimeRange | function | CoreMedia/CMTimeRange.h | cm::CMTime |
| CMTimeRangeCopyAsDictionary | function | CoreMedia/CMTimeRange.h | raw::CMTimeRangeCopyAsDictionary |
| CMTimeRangeCopyDescription | function | CoreMedia/CMTimeRange.h | raw::CMTimeRangeCopyDescription |
| CMTimeRangeEqual | function | CoreMedia/CMTimeRange.h | raw::CMTimeRangeEqual |
| CMTimeRangeFromTimeToTime | function | CoreMedia/CMTimeRange.h | raw::CMTimeRangeFromTimeToTime |
| CMTimeRangeGetEnd | function | CoreMedia/CMTimeRange.h | cm::CMTime |
| CMTimeRangeGetIntersection | function | CoreMedia/CMTimeRange.h | cm::CMTime |
| CMTimeRangeGetUnion | function | CoreMedia/CMTimeRange.h | cm::CMTime |
| CMTimeRangeMake | function | CoreMedia/CMTimeRange.h | raw::CMTimeRangeMake |
| CMTimeRangeMakeFromDictionary | function | CoreMedia/CMTimeRange.h | raw::CMTimeRangeMakeFromDictionary |
| CMTimeRangeShow | function | CoreMedia/CMTimeRange.h | raw::CMTimeRangeShow |
| CMTimeRoundingMethod | typedef enum | CoreMedia/CMTime.h | raw::CMTimeRoundingMethod |
| CMTimeShow | function | CoreMedia/CMTime.h | raw::CMTimeShow |
| CMTimeSubtract | function | CoreMedia/CMTime.h | cm::CMTime |
| CMTimebaseAddTimer | function | CoreMedia/CMSync.h | raw::CMTimebaseAddTimer |
| CMTimebaseAddTimerDispatchSource | function | CoreMedia/CMSync.h | raw::CMTimebaseAddTimerDispatchSource |
| CMTimebaseCopySource | function | CoreMedia/CMSync.h | raw::CMTimebaseCopySource |
| CMTimebaseCopySourceClock | function | CoreMedia/CMSync.h | raw::CMTimebaseCopySourceClock |
| CMTimebaseCopySourceTimebase | function | CoreMedia/CMSync.h | raw::CMTimebaseCopySourceTimebase |
| CMTimebaseCopyUltimateSourceClock | function | CoreMedia/CMSync.h | raw::CMTimebaseCopyUltimateSourceClock |
| CMTimebaseCreateWithMasterClock | function | CoreMedia/CMSync.h | cm::CMTimebase |
| CMTimebaseCreateWithMasterTimebase | function | CoreMedia/CMSync.h | raw::CMTimebaseCreateWithMasterTimebase |
| CMTimebaseCreateWithSourceClock | function | CoreMedia/CMSync.h | raw::CMTimebaseCreateWithSourceClock |
| CMTimebaseCreateWithSourceTimebase | function | CoreMedia/CMSync.h | raw::CMTimebaseCreateWithSourceTimebase |
| CMTimebaseGetEffectiveRate | function | CoreMedia/CMSync.h | raw::CMTimebaseGetEffectiveRate |
| CMTimebaseGetRate | function | CoreMedia/CMSync.h | cm::CMTimebase |
| CMTimebaseGetTime | function | CoreMedia/CMSync.h | cm::CMTimebase |
| CMTimebaseGetTimeAndRate | function | CoreMedia/CMSync.h | raw::CMTimebaseGetTimeAndRate |
| CMTimebaseGetTimeWithTimeScale | function | CoreMedia/CMSync.h | raw::CMTimebaseGetTimeWithTimeScale |
| CMTimebaseGetTypeID | function | CoreMedia/CMSync.h | raw::CMTimebaseGetTypeID |
| CMTimebaseNotificationBarrier | function | CoreMedia/CMSync.h | raw::CMTimebaseNotificationBarrier |
| CMTimebaseRef | typedef struct | CoreMedia/CMSync.h | cm::CMTimebase |
| CMTimebaseRemoveTimer | function | CoreMedia/CMSync.h | raw::CMTimebaseRemoveTimer |
| CMTimebaseRemoveTimerDispatchSource | function | CoreMedia/CMSync.h | raw::CMTimebaseRemoveTimerDispatchSource |
| CMTimebaseSetAnchorTime | function | CoreMedia/CMSync.h | raw::CMTimebaseSetAnchorTime |
| CMTimebaseSetMasterClock | function | CoreMedia/CMSync.h | raw::CMTimebaseSetMasterClock |
| CMTimebaseSetMasterTimebase | function | CoreMedia/CMSync.h | raw::CMTimebaseSetMasterTimebase |
| CMTimebaseSetRate | function | CoreMedia/CMSync.h | cm::CMTimebase |
| CMTimebaseSetRateAndAnchorTime | function | CoreMedia/CMSync.h | raw::CMTimebaseSetRateAndAnchorTime |
| CMTimebaseSetSourceClock | function | CoreMedia/CMSync.h | raw::CMTimebaseSetSourceClock |
| CMTimebaseSetSourceTimebase | function | CoreMedia/CMSync.h | raw::CMTimebaseSetSourceTimebase |
| CMTimebaseSetTime | function | CoreMedia/CMSync.h | cm::CMTimebase |
| CMTimebaseSetTimerDispatchSourceNextFireTime | function | CoreMedia/CMSync.h | raw::CMTimebaseSetTimerDispatchSourceNextFireTime |
| CMTimebaseSetTimerDispatchSourceToFireImmediately | function | CoreMedia/CMSync.h | raw::CMTimebaseSetTimerDispatchSourceToFireImmediately |
| CMTimebaseSetTimerNextFireTime | function | CoreMedia/CMSync.h | raw::CMTimebaseSetTimerNextFireTime |
| CMTimebaseSetTimerToFireImmediately | function | CoreMedia/CMSync.h | raw::CMTimebaseSetTimerToFireImmediately |
| CMU16Fixed16ArrayType | typedef struct | ? | raw::CMU16Fixed16ArrayType |
| CMUInt16ArrayType | typedef struct | ? | raw::CMUInt16ArrayType |
| CMUInt32ArrayType | typedef struct | ? | raw::CMUInt32ArrayType |
| CMUInt64ArrayType | typedef struct | ? | raw::CMUInt64ArrayType |
| CMUInt8ArrayType | typedef struct | ? | raw::CMUInt8ArrayType |
| CMUcrBgType | typedef struct | ? | raw::CMUcrBgType |
| CMUnicodeTextType | typedef struct | ? | raw::CMUnicodeTextType |
| CMVideoCardGamma | typedef struct | ? | raw::CMVideoCardGamma |
| CMVideoCardGammaFormula | typedef struct | ? | raw::CMVideoCardGammaFormula |
| CMVideoCardGammaTable | typedef struct | ? | raw::CMVideoCardGammaTable |
| CMVideoCardGammaType | typedef struct | ? | raw::CMVideoCardGammaType |
| CMVideoDimensions | typedef struct | CoreMedia/CMFormatDescription.h | raw::CMVideoDimensions |
| CMVideoFormatDescriptionCopyAsBigEndianImageDescriptionBlockBuffer | function | CoreMedia/CMFormatDescriptionBridge.h | raw::CMVideoFormatDescriptionCopyAsBigEndianImageDescriptionBlockBuffer |
| CMVideoFormatDescriptionCopyTagCollectionArray | function | CoreMedia/CMFormatDescription.h | raw::CMVideoFormatDescriptionCopyTagCollectionArray |
| CMVideoFormatDescriptionCreate | function | CoreMedia/CMFormatDescription.h | raw::CMVideoFormatDescriptionCreate |
| CMVideoFormatDescriptionCreateForImageBuffer | function | CoreMedia/CMFormatDescription.h | raw::CMVideoFormatDescriptionCreateForImageBuffer |
| CMVideoFormatDescriptionCreateFromBigEndianImageDescriptionBlockBuffer | function | CoreMedia/CMFormatDescriptionBridge.h | raw::CMVideoFormatDescriptionCreateFromBigEndianImageDescriptionBlockBuffer |
| CMVideoFormatDescriptionCreateFromBigEndianImageDescriptionData | function | CoreMedia/CMFormatDescriptionBridge.h | raw::CMVideoFormatDescriptionCreateFromBigEndianImageDescriptionData |
| CMVideoFormatDescriptionCreateFromH264ParameterSets | function | CoreMedia/CMFormatDescription.h | raw::CMVideoFormatDescriptionCreateFromH264ParameterSets |
| CMVideoFormatDescriptionCreateFromHEVCParameterSets | function | CoreMedia/CMFormatDescription.h | raw::CMVideoFormatDescriptionCreateFromHEVCParameterSets |
| CMVideoFormatDescriptionGetCleanAperture | function | CoreMedia/CMFormatDescription.h | raw::CMVideoFormatDescriptionGetCleanAperture |
| CMVideoFormatDescriptionGetDimensions | function | CoreMedia/CMFormatDescription.h | raw::CMVideoFormatDescriptionGetDimensions |
| CMVideoFormatDescriptionGetExtensionKeysCommonWithImageBuffers | function | CoreMedia/CMFormatDescription.h | raw::CMVideoFormatDescriptionGetExtensionKeysCommonWithImageBuffers |
| CMVideoFormatDescriptionGetH264ParameterSetAtIndex | function | CoreMedia/CMFormatDescription.h | raw::CMVideoFormatDescriptionGetH264ParameterSetAtIndex |
| CMVideoFormatDescriptionGetHEVCParameterSetAtIndex | function | CoreMedia/CMFormatDescription.h | raw::CMVideoFormatDescriptionGetHEVCParameterSetAtIndex |
| CMVideoFormatDescriptionGetPresentationDimensions | function | CoreMedia/CMFormatDescription.h | raw::CMVideoFormatDescriptionGetPresentationDimensions |
| CMVideoFormatDescriptionMatchesImageBuffer | function | CoreMedia/CMFormatDescription.h | raw::CMVideoFormatDescriptionMatchesImageBuffer |
| CMViewingConditionsType | typedef struct | ? | raw::CMViewingConditionsType |
| CMWorldRef | typedef struct | ? | raw::CMWorldRef |
| CMXYZColor | typedef struct | ? | raw::CMXYZColor |
| CMXYZType | typedef struct | ? | raw::CMXYZType |
| CMYxyColor | typedef struct | ? | raw::CMYxyColor |
| CVAttachmentMode | typedef enum | CoreVideo/CVBuffer.h | cv::CVAttachmentMode |
| CVBufferCopyAttachment | function | CoreVideo/CVBuffer.h | cv::CVBuffer |
| CVBufferCopyAttachments | function | CoreVideo/CVBuffer.h | cv::CVBuffer |
| CVBufferHasAttachment | function | CoreVideo/CVBuffer.h | raw::CVBufferHasAttachment |
| CVBufferPropagateAttachments | function | CoreVideo/CVBuffer.h | raw::CVBufferPropagateAttachments |
| CVBufferRef | typedef struct | CoreVideo/CVBuffer.h | cv::CVBuffer |
| CVBufferRelease | function | CoreVideo/CVBuffer.h | cv::CVBuffer |
| CVBufferRemoveAllAttachments | function | CoreVideo/CVBuffer.h | cv::CVBuffer |
| CVBufferRemoveAttachment | function | CoreVideo/CVBuffer.h | raw::CVBufferRemoveAttachment |
| CVBufferRetain | function | CoreVideo/CVBuffer.h | cv::CVBuffer |
| CVBufferSetAttachment | function | CoreVideo/CVBuffer.h | cv::CVBuffer |
| CVBufferSetAttachments | function | CoreVideo/CVBuffer.h | raw::CVBufferSetAttachments |
| CVColorPrimariesGetIntegerCodePointForString | function | CoreVideo/CVImageBuffer.h | raw::CVColorPrimariesGetIntegerCodePointForString |
| CVColorPrimariesGetStringForIntegerCodePoint | function | CoreVideo/CVImageBuffer.h | raw::CVColorPrimariesGetStringForIntegerCodePoint |
| CVFillExtendedPixelsCallBackData | typedef struct | CoreVideo/CVPixelFormatDescription.h | raw::CVFillExtendedPixelsCallBackData |
| CVGetCurrentHostTime | function | CoreVideo/CVHostTime.h | raw::CVGetCurrentHostTime |
| CVGetHostClockFrequency | function | CoreVideo/CVHostTime.h | raw::CVGetHostClockFrequency |
| CVGetHostClockMinimumTimeDelta | function | CoreVideo/CVHostTime.h | raw::CVGetHostClockMinimumTimeDelta |
| CVImageBufferCreateColorSpaceFromAttachments | function | CoreVideo/CVImageBuffer.h | raw::CVImageBufferCreateColorSpaceFromAttachments |
| CVImageBufferGetCleanRect | function | CoreVideo/CVImageBuffer.h | cv::CVImageBuffer |
| CVImageBufferGetColorSpace | function | CoreVideo/CVImageBuffer.h | raw::CVImageBufferGetColorSpace |
| CVImageBufferGetDisplaySize | function | CoreVideo/CVImageBuffer.h | cv::CVImageBuffer |
| CVImageBufferGetEncodedSize | function | CoreVideo/CVImageBuffer.h | cv::CVImageBuffer |
| CVImageBufferIsFlipped | function | CoreVideo/CVImageBuffer.h | raw::CVImageBufferIsFlipped |
| CVIsCompressedPixelFormatAvailable | function | CoreVideo/CVPixelFormatDescription.h | raw::CVIsCompressedPixelFormatAvailable |
| CVMetalBufferCacheCreate | function | CoreVideo/CVMetalBufferCache.h | raw::CVMetalBufferCacheCreate |
| CVMetalBufferCacheCreateBufferFromImage | function | CoreVideo/CVMetalBufferCache.h | raw::CVMetalBufferCacheCreateBufferFromImage |
| CVMetalBufferCacheFlush | function | CoreVideo/CVMetalBufferCache.h | raw::CVMetalBufferCacheFlush |
| CVMetalBufferCacheGetTypeID | function | CoreVideo/CVMetalBufferCache.h | raw::CVMetalBufferCacheGetTypeID |
| CVMetalBufferCacheRef | typedef struct | CoreVideo/CVMetalBufferCache.h | raw::CVMetalBufferCacheRef |
| CVMetalBufferGetBuffer | function | CoreVideo/CVMetalBuffer.h | raw::CVMetalBufferGetBuffer |
| CVMetalBufferGetTypeID | function | CoreVideo/CVMetalBuffer.h | raw::CVMetalBufferGetTypeID |
| CVMetalTextureCacheCreate | function | CoreVideo/CVMetalTextureCache.h | cv::CVMetalTextureCache |
| CVMetalTextureCacheCreateTextureFromImage | function | CoreVideo/CVMetalTextureCache.h | raw::CVMetalTextureCacheCreateTextureFromImage |
| CVMetalTextureCacheFlush | function | CoreVideo/CVMetalTextureCache.h | cv::CVMetalTextureCache |
| CVMetalTextureCacheGetTypeID | function | CoreVideo/CVMetalTextureCache.h | cv::CVMetalTextureCache |
| CVMetalTextureCacheRef | typedef struct | CoreVideo/CVMetalTextureCache.h | cv::CVMetalTextureCache |
| CVMetalTextureGetCleanTexCoords | function | CoreVideo/CVMetalTexture.h | raw::CVMetalTextureGetCleanTexCoords |
| CVMetalTextureGetTexture | function | CoreVideo/CVMetalTexture.h | raw::CVMetalTextureGetTexture |
| CVMetalTextureGetTypeID | function | CoreVideo/CVMetalTexture.h | raw::CVMetalTextureGetTypeID |
| CVMetalTextureIsFlipped | function | CoreVideo/CVMetalTexture.h | raw::CVMetalTextureIsFlipped |
| CVOpenGLBufferAttach | function | CoreVideo/CVOpenGLBuffer.h | raw::CVOpenGLBufferAttach |
| CVOpenGLBufferCreate | function | CoreVideo/CVOpenGLBuffer.h | raw::CVOpenGLBufferCreate |
| CVOpenGLBufferGetAttributes | function | CoreVideo/CVOpenGLBuffer.h | raw::CVOpenGLBufferGetAttributes |
| CVOpenGLBufferGetTypeID | function | CoreVideo/CVOpenGLBuffer.h | raw::CVOpenGLBufferGetTypeID |
| CVOpenGLBufferPoolCreate | function | CoreVideo/CVOpenGLBufferPool.h | raw::CVOpenGLBufferPoolCreate |
| CVOpenGLBufferPoolCreateOpenGLBuffer | function | CoreVideo/CVOpenGLBufferPool.h | raw::CVOpenGLBufferPoolCreateOpenGLBuffer |
| CVOpenGLBufferPoolGetAttributes | function | CoreVideo/CVOpenGLBufferPool.h | raw::CVOpenGLBufferPoolGetAttributes |
| CVOpenGLBufferPoolGetOpenGLBufferAttributes | function | CoreVideo/CVOpenGLBufferPool.h | raw::CVOpenGLBufferPoolGetOpenGLBufferAttributes |
| CVOpenGLBufferPoolGetTypeID | function | CoreVideo/CVOpenGLBufferPool.h | raw::CVOpenGLBufferPoolGetTypeID |
| CVOpenGLBufferPoolRef | typedef struct | CoreVideo/CVOpenGLBufferPool.h | raw::CVOpenGLBufferPoolRef |
| CVOpenGLBufferPoolRelease | function | CoreVideo/CVOpenGLBufferPool.h | raw::CVOpenGLBufferPoolRelease |
| CVOpenGLBufferPoolRetain | function | CoreVideo/CVOpenGLBufferPool.h | raw::CVOpenGLBufferPoolRetain |
| CVOpenGLBufferRelease | function | CoreVideo/CVOpenGLBuffer.h | raw::CVOpenGLBufferRelease |
| CVOpenGLBufferRetain | function | CoreVideo/CVOpenGLBuffer.h | raw::CVOpenGLBufferRetain |
| CVOpenGLTextureCacheCreate | function | CoreVideo/CVOpenGLTextureCache.h | raw::CVOpenGLTextureCacheCreate |
| CVOpenGLTextureCacheCreateTextureFromImage | function | CoreVideo/CVOpenGLTextureCache.h | raw::CVOpenGLTextureCacheCreateTextureFromImage |
| CVOpenGLTextureCacheFlush | function | CoreVideo/CVOpenGLTextureCache.h | raw::CVOpenGLTextureCacheFlush |
| CVOpenGLTextureCacheGetTypeID | function | CoreVideo/CVOpenGLTextureCache.h | raw::CVOpenGLTextureCacheGetTypeID |
| CVOpenGLTextureCacheRef | typedef struct | CoreVideo/CVOpenGLTextureCache.h | raw::CVOpenGLTextureCacheRef |
| CVOpenGLTextureCacheRelease | function | CoreVideo/CVOpenGLTextureCache.h | raw::CVOpenGLTextureCacheRelease |
| CVOpenGLTextureCacheRetain | function | CoreVideo/CVOpenGLTextureCache.h | raw::CVOpenGLTextureCacheRetain |
| CVOpenGLTextureGetCleanTexCoords | function | CoreVideo/CVOpenGLTexture.h | raw::CVOpenGLTextureGetCleanTexCoords |
| CVOpenGLTextureGetName | function | CoreVideo/CVOpenGLTexture.h | raw::CVOpenGLTextureGetName |
| CVOpenGLTextureGetTarget | function | CoreVideo/CVOpenGLTexture.h | raw::CVOpenGLTextureGetTarget |
| CVOpenGLTextureGetTypeID | function | CoreVideo/CVOpenGLTexture.h | raw::CVOpenGLTextureGetTypeID |
| CVOpenGLTextureIsFlipped | function | CoreVideo/CVOpenGLTexture.h | raw::CVOpenGLTextureIsFlipped |
| CVOpenGLTextureRelease | function | CoreVideo/CVOpenGLTexture.h | raw::CVOpenGLTextureRelease |
| CVOpenGLTextureRetain | function | CoreVideo/CVOpenGLTexture.h | raw::CVOpenGLTextureRetain |
| CVPixelBufferCopyCreationAttributes | function | CoreVideo/CVPixelBuffer.h | raw::CVPixelBufferCopyCreationAttributes |
| CVPixelBufferCreate | function | CoreVideo/CVPixelBuffer.h | cv::CVPixelBuffer |
| CVPixelBufferCreateResolvedAttributesDictionary | function | CoreVideo/CVPixelBuffer.h | raw::CVPixelBufferCreateResolvedAttributesDictionary |
| CVPixelBufferCreateWithBytes | function | CoreVideo/CVPixelBuffer.h | cv::CVPixelBuffer |
| CVPixelBufferCreateWithIOSurface | function | CoreVideo/CVPixelBufferIOSurface.h | cv::CVPixelBuffer |
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
| CVPixelBufferGetIOSurface | function | CoreVideo/CVPixelBufferIOSurface.h | cv::CVPixelBuffer |
| CVPixelBufferGetPixelFormatType | function | CoreVideo/CVPixelBuffer.h | cv::CVPixelBuffer |
| CVPixelBufferGetPlaneCount | function | CoreVideo/CVPixelBuffer.h | cv::CVPixelBuffer |
| CVPixelBufferGetTypeID | function | CoreVideo/CVPixelBuffer.h | cv::CVPixelBuffer |
| CVPixelBufferGetWidth | function | CoreVideo/CVPixelBuffer.h | cv::CVPixelBuffer |
| CVPixelBufferGetWidthOfPlane | function | CoreVideo/CVPixelBuffer.h | cv::CVPixelBuffer |
| CVPixelBufferIsCompatibleWithAttributes | function | CoreVideo/CVPixelBuffer.h | raw::CVPixelBufferIsCompatibleWithAttributes |
| CVPixelBufferIsPlanar | function | CoreVideo/CVPixelBuffer.h | cv::CVPixelBuffer |
| CVPixelBufferLockBaseAddress | function | CoreVideo/CVPixelBuffer.h | cv::CVPixelBuffer |
| CVPixelBufferLockFlags | typedef enum | CoreVideo/CVPixelBuffer.h | cv::CVPixelBuffer |
| CVPixelBufferPoolCreate | function | CoreVideo/CVPixelBufferPool.h | cv::CVPixelBufferPool |
| CVPixelBufferPoolCreatePixelBuffer | function | CoreVideo/CVPixelBufferPool.h | cv::CVPixelBufferPool |
| CVPixelBufferPoolCreatePixelBufferWithAuxAttributes | function | CoreVideo/CVPixelBufferPool.h | raw::CVPixelBufferPoolCreatePixelBufferWithAuxAttributes |
| CVPixelBufferPoolFlush | function | CoreVideo/CVPixelBufferPool.h | cv::CVPixelBufferPool |
| CVPixelBufferPoolFlushFlags | typedef enum | CoreVideo/CVPixelBufferPool.h | raw::CVPixelBufferPoolFlushFlags |
| CVPixelBufferPoolGetAttributes | function | CoreVideo/CVPixelBufferPool.h | cv::CVPixelBufferPool |
| CVPixelBufferPoolGetPixelBufferAttributes | function | CoreVideo/CVPixelBufferPool.h | cv::CVPixelBufferPool |
| CVPixelBufferPoolGetTypeID | function | CoreVideo/CVPixelBufferPool.h | cv::CVPixelBufferPool |
| CVPixelBufferPoolRef | typedef struct | CoreVideo/CVPixelBufferPool.h | cv::CVPixelBufferPool |
| CVPixelBufferPoolRelease | function | CoreVideo/CVPixelBufferPool.h | raw::CVPixelBufferPoolRelease |
| CVPixelBufferPoolRetain | function | CoreVideo/CVPixelBufferPool.h | raw::CVPixelBufferPoolRetain |
| CVPixelBufferRelease | function | CoreVideo/CVPixelBuffer.h | raw::CVPixelBufferRelease |
| CVPixelBufferRetain | function | CoreVideo/CVPixelBuffer.h | raw::CVPixelBufferRetain |
| CVPixelBufferUnlockBaseAddress | function | CoreVideo/CVPixelBuffer.h | cv::CVPixelBuffer |
| CVPixelFormatDescriptionArrayCreateWithAllPixelFormatTypes | function | CoreVideo/CVPixelFormatDescription.h | raw::CVPixelFormatDescriptionArrayCreateWithAllPixelFormatTypes |
| CVPixelFormatDescriptionCreateWithPixelFormatType | function | CoreVideo/CVPixelFormatDescription.h | raw::CVPixelFormatDescriptionCreateWithPixelFormatType |
| CVPixelFormatDescriptionRegisterDescriptionWithPixelFormatType | function | CoreVideo/CVPixelFormatDescription.h | raw::CVPixelFormatDescriptionRegisterDescriptionWithPixelFormatType |
| CVPixelFormatTypeCopyFourCharCodeString | function | CoreVideo/CVPixelFormatDescription.h | raw::CVPixelFormatTypeCopyFourCharCodeString |
| CVPlanarComponentInfo | typedef struct | CoreVideo/CVPixelBuffer.h | raw::CVPlanarComponentInfo |
| CVPlanarPixelBufferInfo | typedef struct | CoreVideo/CVPixelBuffer.h | raw::CVPlanarPixelBufferInfo |
| CVPlanarPixelBufferInfo_YCbCrBiPlanar | typedef struct | CoreVideo/CVPixelBuffer.h | raw::CVPlanarPixelBufferInfo_YCbCrBiPlanar |
| CVPlanarPixelBufferInfo_YCbCrPlanar | typedef struct | CoreVideo/CVPixelBuffer.h | raw::CVPlanarPixelBufferInfo_YCbCrPlanar |
| CVSMPTETime | typedef struct | CoreVideo/CVBase.h | raw::CVSMPTETime |
| CVSMPTETimeFlags | typedef enum | CoreVideo/CVBase.h | raw::CVSMPTETimeFlags |
| CVSMPTETimeType | typedef enum | CoreVideo/CVBase.h | raw::CVSMPTETimeType |
| CVTime | typedef struct | CoreVideo/CVBase.h | raw::CVTime |
| CVTimeFlags | typedef enum | CoreVideo/CVBase.h | raw::CVTimeFlags |
| CVTimeStampFlags | typedef enum | CoreVideo/CVBase.h | raw::CVTimeStampFlags |
| CVTransferFunctionGetIntegerCodePointForString | function | CoreVideo/CVImageBuffer.h | raw::CVTransferFunctionGetIntegerCodePointForString |
| CVTransferFunctionGetStringForIntegerCodePoint | function | CoreVideo/CVImageBuffer.h | raw::CVTransferFunctionGetStringForIntegerCodePoint |
| CVYCbCrMatrixGetIntegerCodePointForString | function | CoreVideo/CVImageBuffer.h | raw::CVYCbCrMatrixGetIntegerCodePointForString |
| CVYCbCrMatrixGetStringForIntegerCodePoint | function | CoreVideo/CVImageBuffer.h | raw::CVYCbCrMatrixGetStringForIntegerCodePoint |
| IOSurfaceAlignProperty | function | IOSurface/IOSurfaceRef.h | raw::IOSurfaceAlignProperty |
| IOSurfaceAllowsPixelSizeCasting | function | IOSurface/IOSurfaceRef.h | raw::IOSurfaceAllowsPixelSizeCasting |
| IOSurfaceComponentName | typedef enum | IOSurface/IOSurfaceRef.h | raw::IOSurfaceComponentName |
| IOSurfaceComponentRange | typedef enum | IOSurface/IOSurfaceRef.h | raw::IOSurfaceComponentRange |
| IOSurfaceComponentType | typedef enum | IOSurface/IOSurfaceRef.h | raw::IOSurfaceComponentType |
| IOSurfaceCopyAllValues | function | IOSurface/IOSurfaceRef.h | raw::IOSurfaceCopyAllValues |
| IOSurfaceCopyValue | function | IOSurface/IOSurfaceRef.h | raw::IOSurfaceCopyValue |
| IOSurfaceCreate | function | IOSurface/IOSurfaceRef.h | raw::IOSurfaceCreate |
| IOSurfaceCreateMachPort | function | IOSurface/IOSurfaceRef.h | raw::IOSurfaceCreateMachPort |
| IOSurfaceCreateXPCObject | function | IOSurface/IOSurfaceAPI.h | raw::IOSurfaceCreateXPCObject |
| IOSurfaceDecrementUseCount | function | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceGetAllocSize | function | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceGetBaseAddress | function | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceGetBaseAddressOfPlane | function | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceGetBitDepthOfComponentOfPlane | function | IOSurface/IOSurfaceRef.h | raw::IOSurfaceGetBitDepthOfComponentOfPlane |
| IOSurfaceGetBitOffsetOfComponentOfPlane | function | IOSurface/IOSurfaceRef.h | raw::IOSurfaceGetBitOffsetOfComponentOfPlane |
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
| IOSurfaceGetNameOfComponentOfPlane | function | IOSurface/IOSurfaceRef.h | raw::IOSurfaceGetNameOfComponentOfPlane |
| IOSurfaceGetNumberOfComponentsOfPlane | function | IOSurface/IOSurfaceRef.h | raw::IOSurfaceGetNumberOfComponentsOfPlane |
| IOSurfaceGetPixelFormat | function | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceGetPlaneCount | function | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceGetPropertyAlignment | function | IOSurface/IOSurfaceRef.h | raw::IOSurfaceGetPropertyAlignment |
| IOSurfaceGetPropertyMaximum | function | IOSurface/IOSurfaceRef.h | raw::IOSurfaceGetPropertyMaximum |
| IOSurfaceGetRangeOfComponentOfPlane | function | IOSurface/IOSurfaceRef.h | raw::IOSurfaceGetRangeOfComponentOfPlane |
| IOSurfaceGetSeed | function | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceGetSubsampling | function | IOSurface/IOSurfaceRef.h | raw::IOSurfaceGetSubsampling |
| IOSurfaceGetTypeID | function | IOSurface/IOSurfaceRef.h | raw::IOSurfaceGetTypeID |
| IOSurfaceGetTypeOfComponentOfPlane | function | IOSurface/IOSurfaceRef.h | raw::IOSurfaceGetTypeOfComponentOfPlane |
| IOSurfaceGetUseCount | function | IOSurface/IOSurfaceRef.h | raw::IOSurfaceGetUseCount |
| IOSurfaceGetWidth | function | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceGetWidthOfPlane | function | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceIncrementUseCount | function | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceIsInUse | function | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceLock | function | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceLockOptions | typedef enum | IOSurface/IOSurfaceTypes.h | iosurface::IOSurface |
| IOSurfaceLookup | function | IOSurface/IOSurfaceRef.h | raw::IOSurfaceLookup |
| IOSurfaceLookupFromMachPort | function | IOSurface/IOSurfaceRef.h | raw::IOSurfaceLookupFromMachPort |
| IOSurfaceLookupFromXPCObject | function | IOSurface/IOSurfaceAPI.h | raw::IOSurfaceLookupFromXPCObject |
| IOSurfaceMemoryLedgerFlags | typedef enum | IOSurface/IOSurfaceRef.h | raw::IOSurfaceMemoryLedgerFlags |
| IOSurfaceMemoryLedgerTags | typedef enum | IOSurface/IOSurfaceRef.h | raw::IOSurfaceMemoryLedgerTags |
| IOSurfacePurgeabilityState | typedef enum | IOSurface/IOSurfaceTypes.h | raw::IOSurfacePurgeabilityState |
| IOSurfaceRef | typedef struct | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| IOSurfaceRemoveAllValues | function | IOSurface/IOSurfaceRef.h | raw::IOSurfaceRemoveAllValues |
| IOSurfaceRemoveValue | function | IOSurface/IOSurfaceRef.h | raw::IOSurfaceRemoveValue |
| IOSurfaceSetOwnershipIdentity | function | IOSurface/IOSurfaceRef.h | raw::IOSurfaceSetOwnershipIdentity |
| IOSurfaceSetPurgeable | function | IOSurface/IOSurfaceRef.h | raw::IOSurfaceSetPurgeable |
| IOSurfaceSetValue | function | IOSurface/IOSurfaceRef.h | raw::IOSurfaceSetValue |
| IOSurfaceSetValues | function | IOSurface/IOSurfaceRef.h | raw::IOSurfaceSetValues |
| IOSurfaceSubsampling | typedef enum | IOSurface/IOSurfaceRef.h | raw::IOSurfaceSubsampling |
| IOSurfaceUnlock | function | IOSurface/IOSurfaceRef.h | iosurface::IOSurface |
| dispatch_activate | function | Dispatch/object.h | raw::dispatch_activate |
| dispatch_after | function | Dispatch/queue.h | raw::dispatch_after |
| dispatch_after_f | function | Dispatch/queue.h | raw::dispatch_after_f |
| dispatch_allow_send_signals | function | Dispatch/queue.h | raw::dispatch_allow_send_signals |
| dispatch_apply | function | Dispatch/queue.h | dispatch_queue::dispatch_apply |
| dispatch_apply_f | function | Dispatch/queue.h | dispatch_queue::dispatch_apply (Swift bridge uses `_f` callback form internally) |
| dispatch_assert_queue | function | Dispatch/queue.h | raw::dispatch_assert_queue |
| dispatch_assert_queue_barrier | function | Dispatch/queue.h | raw::dispatch_assert_queue_barrier |
| dispatch_assert_queue_not | function | Dispatch/queue.h | raw::dispatch_assert_queue_not |
| dispatch_async | function | Dispatch/queue.h | dispatch_queue::dispatch_async |
| dispatch_async_and_wait | function | Dispatch/queue.h | dispatch_queue::dispatch_async_and_wait |
| dispatch_async_and_wait_f | function | Dispatch/queue.h | dispatch_queue::dispatch_async_and_wait (Swift bridge uses `_f` callback form internally) |
| dispatch_async_f | function | Dispatch/queue.h | dispatch_queue::dispatch_async (Swift bridge uses `_f` callback form internally) |
| dispatch_autorelease_frequency_t | typedef enum | Dispatch/queue.h | raw::dispatch_autorelease_frequency_t |
| dispatch_barrier_async | function | Dispatch/queue.h | raw::dispatch_barrier_async |
| dispatch_barrier_async_and_wait | function | Dispatch/queue.h | raw::dispatch_barrier_async_and_wait |
| dispatch_barrier_async_and_wait_f | function | Dispatch/queue.h | raw::dispatch_barrier_async_and_wait_f |
| dispatch_barrier_async_f | function | Dispatch/queue.h | raw::dispatch_barrier_async_f |
| dispatch_barrier_sync | function | Dispatch/queue.h | raw::dispatch_barrier_sync |
| dispatch_barrier_sync_f | function | Dispatch/queue.h | raw::dispatch_barrier_sync_f |
| dispatch_block_cancel | function | Dispatch/block.h | raw::dispatch_block_cancel |
| dispatch_block_create | function | Dispatch/block.h | raw::dispatch_block_create |
| dispatch_block_create_with_qos_class | function | Dispatch/block.h | raw::dispatch_block_create_with_qos_class |
| dispatch_block_flags_t | typedef enum | Dispatch/block.h | raw::dispatch_block_flags_t |
| dispatch_block_notify | function | Dispatch/block.h | raw::dispatch_block_notify |
| dispatch_block_perform | function | Dispatch/block.h | raw::dispatch_block_perform |
| dispatch_block_t | typedef struct | Dispatch/object.h | raw::dispatch_block_t |
| dispatch_block_testcancel | function | Dispatch/block.h | raw::dispatch_block_testcancel |
| dispatch_block_wait | function | Dispatch/block.h | raw::dispatch_block_wait |
| dispatch_cancel | function | Dispatch/object.h | raw::dispatch_cancel |
| dispatch_data_applier_t | typedef struct | Dispatch/data.h | raw::dispatch_data_applier_t |
| dispatch_data_apply | function | Dispatch/data.h | raw::dispatch_data_apply |
| dispatch_data_copy_region | function | Dispatch/data.h | raw::dispatch_data_copy_region |
| dispatch_data_create | function | Dispatch/data.h | raw::dispatch_data_create |
| dispatch_data_create_concat | function | Dispatch/data.h | raw::dispatch_data_create_concat |
| dispatch_data_create_map | function | Dispatch/data.h | raw::dispatch_data_create_map |
| dispatch_data_create_subrange | function | Dispatch/data.h | raw::dispatch_data_create_subrange |
| dispatch_data_get_size | function | Dispatch/data.h | raw::dispatch_data_get_size |
| dispatch_data_s | struct | Dispatch/data.h | raw::dispatch_data_s |
| dispatch_data_t | typedef struct | Dispatch/data.h | raw::dispatch_data_t |
| dispatch_debug | function | Dispatch/object.h | raw::dispatch_debug |
| dispatch_debugv | function | Dispatch/object.h | raw::dispatch_debugv |
| dispatch_fd_t | typedef struct | Dispatch/io.h | raw::dispatch_fd_t |
| dispatch_function_t | typedef struct | Dispatch/base.h | raw::dispatch_function_t |
| dispatch_get_context | function | Dispatch/object.h | raw::dispatch_get_context |
| dispatch_get_current_queue | function | Dispatch/queue.h | raw::dispatch_get_current_queue |
| dispatch_get_global_queue | function | Dispatch/queue.h | raw::dispatch_get_global_queue |
| dispatch_get_main_queue | function | Dispatch/queue.h | raw::dispatch_get_main_queue |
| dispatch_get_specific | function | Dispatch/queue.h | raw::dispatch_get_specific |
| dispatch_group_async | function | Dispatch/group.h | raw::dispatch_group_async |
| dispatch_group_async_f | function | Dispatch/group.h | raw::dispatch_group_async_f |
| dispatch_group_create | function | Dispatch/group.h | raw::dispatch_group_create |
| dispatch_group_enter | function | Dispatch/group.h | raw::dispatch_group_enter |
| dispatch_group_leave | function | Dispatch/group.h | raw::dispatch_group_leave |
| dispatch_group_notify | function | Dispatch/group.h | raw::dispatch_group_notify |
| dispatch_group_notify_f | function | Dispatch/group.h | raw::dispatch_group_notify_f |
| dispatch_group_t | typedef struct | Dispatch/group.h | DispatchGroup |
| dispatch_group_wait | function | Dispatch/group.h | raw::dispatch_group_wait |
| dispatch_io_barrier | function | Dispatch/io.h | raw::dispatch_io_barrier |
| dispatch_io_close | function | Dispatch/io.h | raw::dispatch_io_close |
| dispatch_io_close_flags_t | typedef struct | Dispatch/io.h | raw::dispatch_io_close_flags_t |
| dispatch_io_create | function | Dispatch/io.h | raw::dispatch_io_create |
| dispatch_io_create_with_io | function | Dispatch/io.h | raw::dispatch_io_create_with_io |
| dispatch_io_create_with_path | function | Dispatch/io.h | raw::dispatch_io_create_with_path |
| dispatch_io_get_descriptor | function | Dispatch/io.h | raw::dispatch_io_get_descriptor |
| dispatch_io_handler_t | typedef struct | Dispatch/io.h | raw::dispatch_io_handler_t |
| dispatch_io_interval_flags_t | typedef struct | Dispatch/io.h | raw::dispatch_io_interval_flags_t |
| dispatch_io_read | function | Dispatch/io.h | raw::dispatch_io_read |
| dispatch_io_set_high_water | function | Dispatch/io.h | raw::dispatch_io_set_high_water |
| dispatch_io_set_interval | function | Dispatch/io.h | raw::dispatch_io_set_interval |
| dispatch_io_set_low_water | function | Dispatch/io.h | raw::dispatch_io_set_low_water |
| dispatch_io_t | typedef struct | Dispatch/io.h | raw::dispatch_io_t |
| dispatch_io_type_t | typedef struct | Dispatch/io.h | raw::dispatch_io_type_t |
| dispatch_io_write | function | Dispatch/io.h | raw::dispatch_io_write |
| dispatch_main | function | Dispatch/queue.h | raw::dispatch_main |
| dispatch_notify | function | Dispatch/object.h | raw::dispatch_notify |
| dispatch_object_t | typedef struct | Dispatch/object.h | raw::dispatch_object_t |
| dispatch_once | function | Dispatch/once.h | raw::dispatch_once |
| dispatch_once_f | function | Dispatch/once.h | raw::dispatch_once_f |
| dispatch_once_t | typedef struct | Dispatch/once.h | raw::dispatch_once_t |
| dispatch_qos_class_t | typedef struct | Dispatch/object.h | raw::dispatch_qos_class_t |
| dispatch_queue_attr_make_initially_inactive | function | Dispatch/queue.h | raw::dispatch_queue_attr_make_initially_inactive |
| dispatch_queue_attr_make_with_autorelease_frequency | function | Dispatch/queue.h | raw::dispatch_queue_attr_make_with_autorelease_frequency |
| dispatch_queue_attr_make_with_qos_class | function | Dispatch/queue.h | raw::dispatch_queue_attr_make_with_qos_class |
| dispatch_queue_attr_s | struct | Dispatch/queue.h | raw::dispatch_queue_attr_s |
| dispatch_queue_attr_t | typedef struct | Dispatch/queue.h | raw::dispatch_queue_attr_t |
| dispatch_queue_concurrent_t | typedef struct | Dispatch/queue.h | raw::dispatch_queue_concurrent_t |
| dispatch_queue_create | function | Dispatch/queue.h | DispatchQueue |
| dispatch_queue_create_with_target | function | Dispatch/queue.h | raw::dispatch_queue_create_with_target |
| dispatch_queue_get_label | function | Dispatch/queue.h | raw::dispatch_queue_get_label |
| dispatch_queue_get_qos_class | function | Dispatch/queue.h | raw::dispatch_queue_get_qos_class |
| dispatch_queue_get_specific | function | Dispatch/queue.h | raw::dispatch_queue_get_specific |
| dispatch_queue_global_t | typedef struct | Dispatch/queue.h | raw::dispatch_queue_global_t |
| dispatch_queue_main_t | typedef struct | Dispatch/queue.h | raw::dispatch_queue_main_t |
| dispatch_queue_priority_t | typedef struct | Dispatch/queue.h | raw::dispatch_queue_priority_t |
| dispatch_queue_s | struct | Dispatch/queue.h | raw::dispatch_queue_s |
| dispatch_queue_serial_executor_t | typedef struct | Dispatch/queue.h | raw::dispatch_queue_serial_executor_t |
| dispatch_queue_serial_t | typedef struct | Dispatch/queue.h | raw::dispatch_queue_serial_t |
| dispatch_queue_set_specific | function | Dispatch/queue.h | raw::dispatch_queue_set_specific |
| dispatch_queue_t | typedef struct | Dispatch/queue.h | DispatchQueue |
| dispatch_read | function | Dispatch/io.h | raw::dispatch_read |
| dispatch_release | function | Dispatch/object.h | raw::dispatch_release |
| dispatch_resume | function | Dispatch/object.h | raw::dispatch_resume |
| dispatch_retain | function | Dispatch/object.h | raw::dispatch_retain |
| dispatch_semaphore_create | function | Dispatch/semaphore.h | raw::dispatch_semaphore_create |
| dispatch_semaphore_signal | function | Dispatch/semaphore.h | raw::dispatch_semaphore_signal |
| dispatch_semaphore_t | typedef struct | Dispatch/semaphore.h | DispatchSemaphore |
| dispatch_semaphore_wait | function | Dispatch/semaphore.h | raw::dispatch_semaphore_wait |
| dispatch_set_context | function | Dispatch/object.h | raw::dispatch_set_context |
| dispatch_set_finalizer_f | function | Dispatch/object.h | raw::dispatch_set_finalizer_f |
| dispatch_set_qos_class_floor | function | Dispatch/object.h | raw::dispatch_set_qos_class_floor |
| dispatch_set_target_queue | function | Dispatch/queue.h | raw::dispatch_set_target_queue |
| dispatch_source_cancel | function | Dispatch/source.h | raw::dispatch_source_cancel |
| dispatch_source_create | function | Dispatch/source.h | raw::dispatch_source_create |
| dispatch_source_get_data | function | Dispatch/source.h | raw::dispatch_source_get_data |
| dispatch_source_get_handle | function | Dispatch/source.h | raw::dispatch_source_get_handle |
| dispatch_source_get_mask | function | Dispatch/source.h | raw::dispatch_source_get_mask |
| dispatch_source_mach_recv_flags_t | typedef struct | Dispatch/source.h | raw::dispatch_source_mach_recv_flags_t |
| dispatch_source_mach_send_flags_t | typedef struct | Dispatch/source.h | raw::dispatch_source_mach_send_flags_t |
| dispatch_source_memorypressure_flags_t | typedef struct | Dispatch/source.h | raw::dispatch_source_memorypressure_flags_t |
| dispatch_source_merge_data | function | Dispatch/source.h | raw::dispatch_source_merge_data |
| dispatch_source_proc_flags_t | typedef struct | Dispatch/source.h | raw::dispatch_source_proc_flags_t |
| dispatch_source_set_cancel_handler | function | Dispatch/source.h | raw::dispatch_source_set_cancel_handler |
| dispatch_source_set_cancel_handler_f | function | Dispatch/source.h | raw::dispatch_source_set_cancel_handler_f |
| dispatch_source_set_event_handler | function | Dispatch/source.h | raw::dispatch_source_set_event_handler |
| dispatch_source_set_event_handler_f | function | Dispatch/source.h | raw::dispatch_source_set_event_handler_f |
| dispatch_source_set_registration_handler | function | Dispatch/source.h | raw::dispatch_source_set_registration_handler |
| dispatch_source_set_registration_handler_f | function | Dispatch/source.h | raw::dispatch_source_set_registration_handler_f |
| dispatch_source_set_timer | function | Dispatch/source.h | raw::dispatch_source_set_timer |
| dispatch_source_t | typedef struct | Dispatch/source.h | DispatchSource |
| dispatch_source_testcancel | function | Dispatch/source.h | raw::dispatch_source_testcancel |
| dispatch_source_timer_flags_t | typedef struct | Dispatch/source.h | raw::dispatch_source_timer_flags_t |
| dispatch_source_type_s | struct | Dispatch/source.h | raw::dispatch_source_type_s |
| dispatch_source_type_t | typedef struct | Dispatch/source.h | raw::dispatch_source_type_t |
| dispatch_source_vnode_flags_t | typedef struct | Dispatch/source.h | raw::dispatch_source_vnode_flags_t |
| dispatch_suspend | function | Dispatch/object.h | raw::dispatch_suspend |
| dispatch_sync | function | Dispatch/queue.h | raw::dispatch_sync |
| dispatch_sync_f | function | Dispatch/queue.h | raw::dispatch_sync_f |
| dispatch_testcancel | function | Dispatch/object.h | raw::dispatch_testcancel |
| dispatch_time | function | Dispatch/time.h | raw::dispatch_time |
| dispatch_time_t | typedef struct | Dispatch/time.h | raw::dispatch_time_t |
| dispatch_wait | function | Dispatch/object.h | raw::dispatch_wait |
| dispatch_walltime | function | Dispatch/time.h | raw::dispatch_walltime |
| dispatch_workloop_create | function | Dispatch/workloop.h | raw::dispatch_workloop_create |
| dispatch_workloop_create_inactive | function | Dispatch/workloop.h | raw::dispatch_workloop_create_inactive |
| dispatch_workloop_set_autorelease_frequency | function | Dispatch/workloop.h | raw::dispatch_workloop_set_autorelease_frequency |
| dispatch_workloop_set_os_workgroup | function | Dispatch/workloop.h | raw::dispatch_workloop_set_os_workgroup |
| dispatch_workloop_t | typedef struct | Dispatch/workloop.h | raw::dispatch_workloop_t |
| dispatch_write | function | Dispatch/io.h | raw::dispatch_write |
| kCFAbsoluteTimeIntervalSince1904 | constant | CoreFoundation/CFDate.h | raw::kCFAbsoluteTimeIntervalSince1904 |
| kCFAbsoluteTimeIntervalSince1970 | constant | CoreFoundation/CFDate.h | raw::kCFAbsoluteTimeIntervalSince1970 |
| kCFAllocatorDefault | constant | CoreFoundation/CFBase.h | raw::kCFAllocatorDefault |
| kCFAllocatorMalloc | constant | CoreFoundation/CFBase.h | raw::kCFAllocatorMalloc |
| kCFAllocatorMallocZone | constant | CoreFoundation/CFBase.h | raw::kCFAllocatorMallocZone |
| kCFAllocatorNull | constant | CoreFoundation/CFBase.h | raw::kCFAllocatorNull |
| kCFAllocatorSystemDefault | constant | CoreFoundation/CFBase.h | raw::kCFAllocatorSystemDefault |
| kCFAllocatorUseContext | constant | CoreFoundation/CFBase.h | raw::kCFAllocatorUseContext |
| kCFBanglaCalendar | constant | CoreFoundation/CFLocale.h | raw::kCFBanglaCalendar |
| kCFBooleanFalse | constant | CoreFoundation/CFNumber.h | raw::kCFBooleanFalse |
| kCFBooleanTrue | constant | CoreFoundation/CFNumber.h | raw::kCFBooleanTrue |
| kCFBuddhistCalendar | constant | CoreFoundation/CFLocale.h | raw::kCFBuddhistCalendar |
| kCFBundleDevelopmentRegionKey | constant | CoreFoundation/CFBundle.h | raw::kCFBundleDevelopmentRegionKey |
| kCFBundleExecutableKey | constant | CoreFoundation/CFBundle.h | raw::kCFBundleExecutableKey |
| kCFBundleIdentifierKey | constant | CoreFoundation/CFBundle.h | raw::kCFBundleIdentifierKey |
| kCFBundleInfoDictionaryVersionKey | constant | CoreFoundation/CFBundle.h | raw::kCFBundleInfoDictionaryVersionKey |
| kCFBundleLocalizationsKey | constant | CoreFoundation/CFBundle.h | raw::kCFBundleLocalizationsKey |
| kCFBundleNameKey | constant | CoreFoundation/CFBundle.h | raw::kCFBundleNameKey |
| kCFBundleVersionKey | constant | CoreFoundation/CFBundle.h | raw::kCFBundleVersionKey |
| kCFChineseCalendar | constant | CoreFoundation/CFLocale.h | raw::kCFChineseCalendar |
| kCFCopyStringBagCallBacks | constant | CoreFoundation/CFBag.h | raw::kCFCopyStringBagCallBacks |
| kCFCopyStringDictionaryKeyCallBacks | constant | CoreFoundation/CFDictionary.h | raw::kCFCopyStringDictionaryKeyCallBacks |
| kCFCopyStringSetCallBacks | constant | CoreFoundation/CFSet.h | cf::CFSetCallbacks::CopyString |
| kCFCoreFoundationVersionNumber | constant | CoreFoundation/CFBase.h | raw::kCFCoreFoundationVersionNumber |
| kCFDangiCalendar | constant | CoreFoundation/CFLocale.h | raw::kCFDangiCalendar |
| kCFDateFormatterAMSymbol | constant | CoreFoundation/CFDateFormatter.h | raw::kCFDateFormatterAMSymbol |
| kCFDateFormatterCalendar | constant | CoreFoundation/CFDateFormatter.h | raw::kCFDateFormatterCalendar |
| kCFDateFormatterCalendarName | constant | CoreFoundation/CFDateFormatter.h | raw::kCFDateFormatterCalendarName |
| kCFDateFormatterDefaultDate | constant | CoreFoundation/CFDateFormatter.h | raw::kCFDateFormatterDefaultDate |
| kCFDateFormatterDefaultFormat | constant | CoreFoundation/CFDateFormatter.h | raw::kCFDateFormatterDefaultFormat |
| kCFDateFormatterDoesRelativeDateFormattingKey | constant | CoreFoundation/CFDateFormatter.h | raw::kCFDateFormatterDoesRelativeDateFormattingKey |
| kCFDateFormatterEraSymbols | constant | CoreFoundation/CFDateFormatter.h | raw::kCFDateFormatterEraSymbols |
| kCFDateFormatterGregorianStartDate | constant | CoreFoundation/CFDateFormatter.h | raw::kCFDateFormatterGregorianStartDate |
| kCFDateFormatterIsLenient | constant | CoreFoundation/CFDateFormatter.h | raw::kCFDateFormatterIsLenient |
| kCFDateFormatterLongEraSymbols | constant | CoreFoundation/CFDateFormatter.h | raw::kCFDateFormatterLongEraSymbols |
| kCFDateFormatterMonthSymbols | constant | CoreFoundation/CFDateFormatter.h | raw::kCFDateFormatterMonthSymbols |
| kCFDateFormatterPMSymbol | constant | CoreFoundation/CFDateFormatter.h | raw::kCFDateFormatterPMSymbol |
| kCFDateFormatterQuarterSymbols | constant | CoreFoundation/CFDateFormatter.h | raw::kCFDateFormatterQuarterSymbols |
| kCFDateFormatterShortMonthSymbols | constant | CoreFoundation/CFDateFormatter.h | raw::kCFDateFormatterShortMonthSymbols |
| kCFDateFormatterShortQuarterSymbols | constant | CoreFoundation/CFDateFormatter.h | raw::kCFDateFormatterShortQuarterSymbols |
| kCFDateFormatterShortStandaloneMonthSymbols | constant | CoreFoundation/CFDateFormatter.h | raw::kCFDateFormatterShortStandaloneMonthSymbols |
| kCFDateFormatterShortStandaloneQuarterSymbols | constant | CoreFoundation/CFDateFormatter.h | raw::kCFDateFormatterShortStandaloneQuarterSymbols |
| kCFDateFormatterShortStandaloneWeekdaySymbols | constant | CoreFoundation/CFDateFormatter.h | raw::kCFDateFormatterShortStandaloneWeekdaySymbols |
| kCFDateFormatterShortWeekdaySymbols | constant | CoreFoundation/CFDateFormatter.h | raw::kCFDateFormatterShortWeekdaySymbols |
| kCFDateFormatterStandaloneMonthSymbols | constant | CoreFoundation/CFDateFormatter.h | raw::kCFDateFormatterStandaloneMonthSymbols |
| kCFDateFormatterStandaloneQuarterSymbols | constant | CoreFoundation/CFDateFormatter.h | raw::kCFDateFormatterStandaloneQuarterSymbols |
| kCFDateFormatterStandaloneWeekdaySymbols | constant | CoreFoundation/CFDateFormatter.h | raw::kCFDateFormatterStandaloneWeekdaySymbols |
| kCFDateFormatterTimeZone | constant | CoreFoundation/CFDateFormatter.h | raw::kCFDateFormatterTimeZone |
| kCFDateFormatterTwoDigitStartDate | constant | CoreFoundation/CFDateFormatter.h | raw::kCFDateFormatterTwoDigitStartDate |
| kCFDateFormatterVeryShortMonthSymbols | constant | CoreFoundation/CFDateFormatter.h | raw::kCFDateFormatterVeryShortMonthSymbols |
| kCFDateFormatterVeryShortStandaloneMonthSymbols | constant | CoreFoundation/CFDateFormatter.h | raw::kCFDateFormatterVeryShortStandaloneMonthSymbols |
| kCFDateFormatterVeryShortStandaloneWeekdaySymbols | constant | CoreFoundation/CFDateFormatter.h | raw::kCFDateFormatterVeryShortStandaloneWeekdaySymbols |
| kCFDateFormatterVeryShortWeekdaySymbols | constant | CoreFoundation/CFDateFormatter.h | raw::kCFDateFormatterVeryShortWeekdaySymbols |
| kCFDateFormatterWeekdaySymbols | constant | CoreFoundation/CFDateFormatter.h | raw::kCFDateFormatterWeekdaySymbols |
| kCFErrorDescriptionKey | constant | CoreFoundation/CFError.h | raw::kCFErrorDescriptionKey |
| kCFErrorDomainCocoa | constant | CoreFoundation/CFError.h | raw::kCFErrorDomainCocoa |
| kCFErrorDomainMach | constant | CoreFoundation/CFError.h | raw::kCFErrorDomainMach |
| kCFErrorDomainOSStatus | constant | CoreFoundation/CFError.h | raw::kCFErrorDomainOSStatus |
| kCFErrorDomainPOSIX | constant | CoreFoundation/CFError.h | raw::kCFErrorDomainPOSIX |
| kCFErrorFilePathKey | constant | CoreFoundation/CFError.h | raw::kCFErrorFilePathKey |
| kCFErrorLocalizedDescriptionKey | constant | CoreFoundation/CFError.h | raw::kCFErrorLocalizedDescriptionKey |
| kCFErrorLocalizedFailureKey | constant | CoreFoundation/CFError.h | raw::kCFErrorLocalizedFailureKey |
| kCFErrorLocalizedFailureReasonKey | constant | CoreFoundation/CFError.h | raw::kCFErrorLocalizedFailureReasonKey |
| kCFErrorLocalizedRecoverySuggestionKey | constant | CoreFoundation/CFError.h | raw::kCFErrorLocalizedRecoverySuggestionKey |
| kCFErrorURLKey | constant | CoreFoundation/CFError.h | raw::kCFErrorURLKey |
| kCFErrorUnderlyingErrorKey | constant | CoreFoundation/CFError.h | raw::kCFErrorUnderlyingErrorKey |
| kCFGregorianCalendar | constant | CoreFoundation/CFLocale.h | raw::kCFGregorianCalendar |
| kCFGujaratiCalendar | constant | CoreFoundation/CFLocale.h | raw::kCFGujaratiCalendar |
| kCFHebrewCalendar | constant | CoreFoundation/CFLocale.h | raw::kCFHebrewCalendar |
| kCFISO8601Calendar | constant | CoreFoundation/CFLocale.h | raw::kCFISO8601Calendar |
| kCFIndianCalendar | constant | CoreFoundation/CFLocale.h | raw::kCFIndianCalendar |
| kCFIslamicCalendar | constant | CoreFoundation/CFLocale.h | raw::kCFIslamicCalendar |
| kCFIslamicCivilCalendar | constant | CoreFoundation/CFLocale.h | raw::kCFIslamicCivilCalendar |
| kCFIslamicTabularCalendar | constant | CoreFoundation/CFLocale.h | raw::kCFIslamicTabularCalendar |
| kCFIslamicUmmAlQuraCalendar | constant | CoreFoundation/CFLocale.h | raw::kCFIslamicUmmAlQuraCalendar |
| kCFJapaneseCalendar | constant | CoreFoundation/CFLocale.h | raw::kCFJapaneseCalendar |
| kCFKannadaCalendar | constant | CoreFoundation/CFLocale.h | raw::kCFKannadaCalendar |
| kCFLocaleAlternateQuotationBeginDelimiterKey | constant | CoreFoundation/CFLocale.h | raw::kCFLocaleAlternateQuotationBeginDelimiterKey |
| kCFLocaleAlternateQuotationEndDelimiterKey | constant | CoreFoundation/CFLocale.h | raw::kCFLocaleAlternateQuotationEndDelimiterKey |
| kCFLocaleCalendar | constant | CoreFoundation/CFLocale.h | raw::kCFLocaleCalendar |
| kCFLocaleCalendarIdentifier | constant | CoreFoundation/CFLocale.h | raw::kCFLocaleCalendarIdentifier |
| kCFLocaleCollationIdentifier | constant | CoreFoundation/CFLocale.h | raw::kCFLocaleCollationIdentifier |
| kCFLocaleCollatorIdentifier | constant | CoreFoundation/CFLocale.h | raw::kCFLocaleCollatorIdentifier |
| kCFLocaleCountryCode | constant | CoreFoundation/CFLocale.h | raw::kCFLocaleCountryCode |
| kCFLocaleCurrencyCode | constant | CoreFoundation/CFLocale.h | raw::kCFLocaleCurrencyCode |
| kCFLocaleCurrencySymbol | constant | CoreFoundation/CFLocale.h | raw::kCFLocaleCurrencySymbol |
| kCFLocaleCurrentLocaleDidChangeNotification | constant | CoreFoundation/CFLocale.h | raw::kCFLocaleCurrentLocaleDidChangeNotification |
| kCFLocaleDecimalSeparator | constant | CoreFoundation/CFLocale.h | raw::kCFLocaleDecimalSeparator |
| kCFLocaleExemplarCharacterSet | constant | CoreFoundation/CFLocale.h | raw::kCFLocaleExemplarCharacterSet |
| kCFLocaleGroupingSeparator | constant | CoreFoundation/CFLocale.h | raw::kCFLocaleGroupingSeparator |
| kCFLocaleIdentifier | constant | CoreFoundation/CFLocale.h | raw::kCFLocaleIdentifier |
| kCFLocaleLanguageCode | constant | CoreFoundation/CFLocale.h | raw::kCFLocaleLanguageCode |
| kCFLocaleMeasurementSystem | constant | CoreFoundation/CFLocale.h | raw::kCFLocaleMeasurementSystem |
| kCFLocaleQuotationBeginDelimiterKey | constant | CoreFoundation/CFLocale.h | raw::kCFLocaleQuotationBeginDelimiterKey |
| kCFLocaleQuotationEndDelimiterKey | constant | CoreFoundation/CFLocale.h | raw::kCFLocaleQuotationEndDelimiterKey |
| kCFLocaleScriptCode | constant | CoreFoundation/CFLocale.h | raw::kCFLocaleScriptCode |
| kCFLocaleUsesMetricSystem | constant | CoreFoundation/CFLocale.h | raw::kCFLocaleUsesMetricSystem |
| kCFLocaleVariantCode | constant | CoreFoundation/CFLocale.h | raw::kCFLocaleVariantCode |
| kCFMalayalamCalendar | constant | CoreFoundation/CFLocale.h | raw::kCFMalayalamCalendar |
| kCFMarathiCalendar | constant | CoreFoundation/CFLocale.h | raw::kCFMarathiCalendar |
| kCFNotFound | constant | CoreFoundation/CFBase.h | raw::kCFNotFound |
| kCFNull | constant | CoreFoundation/CFBase.h | raw::kCFNull |
| kCFNumberFormatterAlwaysShowDecimalSeparator | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterAlwaysShowDecimalSeparator |
| kCFNumberFormatterCurrencyCode | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterCurrencyCode |
| kCFNumberFormatterCurrencyDecimalSeparator | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterCurrencyDecimalSeparator |
| kCFNumberFormatterCurrencyGroupingSeparator | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterCurrencyGroupingSeparator |
| kCFNumberFormatterCurrencySymbol | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterCurrencySymbol |
| kCFNumberFormatterDecimalSeparator | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterDecimalSeparator |
| kCFNumberFormatterDefaultFormat | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterDefaultFormat |
| kCFNumberFormatterExponentSymbol | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterExponentSymbol |
| kCFNumberFormatterFormatWidth | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterFormatWidth |
| kCFNumberFormatterGroupingSeparator | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterGroupingSeparator |
| kCFNumberFormatterGroupingSize | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterGroupingSize |
| kCFNumberFormatterInfinitySymbol | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterInfinitySymbol |
| kCFNumberFormatterInternationalCurrencySymbol | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterInternationalCurrencySymbol |
| kCFNumberFormatterIsLenient | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterIsLenient |
| kCFNumberFormatterMaxFractionDigits | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterMaxFractionDigits |
| kCFNumberFormatterMaxIntegerDigits | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterMaxIntegerDigits |
| kCFNumberFormatterMaxSignificantDigits | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterMaxSignificantDigits |
| kCFNumberFormatterMinFractionDigits | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterMinFractionDigits |
| kCFNumberFormatterMinGroupingDigits | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterMinGroupingDigits |
| kCFNumberFormatterMinIntegerDigits | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterMinIntegerDigits |
| kCFNumberFormatterMinSignificantDigits | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterMinSignificantDigits |
| kCFNumberFormatterMinusSign | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterMinusSign |
| kCFNumberFormatterMultiplier | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterMultiplier |
| kCFNumberFormatterNaNSymbol | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterNaNSymbol |
| kCFNumberFormatterNegativePrefix | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterNegativePrefix |
| kCFNumberFormatterNegativeSuffix | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterNegativeSuffix |
| kCFNumberFormatterPaddingCharacter | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterPaddingCharacter |
| kCFNumberFormatterPaddingPosition | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterPaddingPosition |
| kCFNumberFormatterPerMillSymbol | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterPerMillSymbol |
| kCFNumberFormatterPercentSymbol | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterPercentSymbol |
| kCFNumberFormatterPlusSign | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterPlusSign |
| kCFNumberFormatterPositivePrefix | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterPositivePrefix |
| kCFNumberFormatterPositiveSuffix | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterPositiveSuffix |
| kCFNumberFormatterRoundingIncrement | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterRoundingIncrement |
| kCFNumberFormatterRoundingMode | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterRoundingMode |
| kCFNumberFormatterSecondaryGroupingSize | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterSecondaryGroupingSize |
| kCFNumberFormatterUseGroupingSeparator | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterUseGroupingSeparator |
| kCFNumberFormatterUseSignificantDigits | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterUseSignificantDigits |
| kCFNumberFormatterZeroSymbol | constant | CoreFoundation/CFNumberFormatter.h | raw::kCFNumberFormatterZeroSymbol |
| kCFNumberNaN | constant | CoreFoundation/CFNumber.h | raw::kCFNumberNaN |
| kCFNumberNegativeInfinity | constant | CoreFoundation/CFNumber.h | raw::kCFNumberNegativeInfinity |
| kCFNumberPositiveInfinity | constant | CoreFoundation/CFNumber.h | raw::kCFNumberPositiveInfinity |
| kCFOdiaCalendar | constant | CoreFoundation/CFLocale.h | raw::kCFOdiaCalendar |
| kCFPersianCalendar | constant | CoreFoundation/CFLocale.h | raw::kCFPersianCalendar |
| kCFPlugInDynamicRegisterFunctionKey | constant | CoreFoundation/CFPlugIn.h | raw::kCFPlugInDynamicRegisterFunctionKey |
| kCFPlugInDynamicRegistrationKey | constant | CoreFoundation/CFPlugIn.h | raw::kCFPlugInDynamicRegistrationKey |
| kCFPlugInFactoriesKey | constant | CoreFoundation/CFPlugIn.h | raw::kCFPlugInFactoriesKey |
| kCFPlugInTypesKey | constant | CoreFoundation/CFPlugIn.h | raw::kCFPlugInTypesKey |
| kCFPlugInUnloadFunctionKey | constant | CoreFoundation/CFPlugIn.h | raw::kCFPlugInUnloadFunctionKey |
| kCFPreferencesAnyApplication | constant | CoreFoundation/CFPreferences.h | raw::kCFPreferencesAnyApplication |
| kCFPreferencesAnyHost | constant | CoreFoundation/CFPreferences.h | raw::kCFPreferencesAnyHost |
| kCFPreferencesAnyUser | constant | CoreFoundation/CFPreferences.h | raw::kCFPreferencesAnyUser |
| kCFPreferencesCurrentApplication | constant | CoreFoundation/CFPreferences.h | raw::kCFPreferencesCurrentApplication |
| kCFPreferencesCurrentHost | constant | CoreFoundation/CFPreferences.h | raw::kCFPreferencesCurrentHost |
| kCFPreferencesCurrentUser | constant | CoreFoundation/CFPreferences.h | raw::kCFPreferencesCurrentUser |
| kCFRepublicOfChinaCalendar | constant | CoreFoundation/CFLocale.h | raw::kCFRepublicOfChinaCalendar |
| kCFRunLoopCommonModes | constant | CoreFoundation/CFRunLoop.h | raw::kCFRunLoopCommonModes |
| kCFRunLoopDefaultMode | constant | CoreFoundation/CFRunLoop.h | raw::kCFRunLoopDefaultMode |
| kCFSocketCommandKey | constant | CoreFoundation/CFSocket.h | raw::kCFSocketCommandKey |
| kCFSocketErrorKey | constant | CoreFoundation/CFSocket.h | raw::kCFSocketErrorKey |
| kCFSocketNameKey | constant | CoreFoundation/CFSocket.h | raw::kCFSocketNameKey |
| kCFSocketRegisterCommand | constant | CoreFoundation/CFSocket.h | raw::kCFSocketRegisterCommand |
| kCFSocketResultKey | constant | CoreFoundation/CFSocket.h | raw::kCFSocketResultKey |
| kCFSocketRetrieveCommand | constant | CoreFoundation/CFSocket.h | raw::kCFSocketRetrieveCommand |
| kCFSocketValueKey | constant | CoreFoundation/CFSocket.h | raw::kCFSocketValueKey |
| kCFStreamErrorDomainSOCKS | constant | CoreFoundation/CFStream.h | raw::kCFStreamErrorDomainSOCKS |
| kCFStreamErrorDomainSSL | constant | CoreFoundation/CFStream.h | raw::kCFStreamErrorDomainSSL |
| kCFStreamPropertyAppendToFile | constant | CoreFoundation/CFStream.h | raw::kCFStreamPropertyAppendToFile |
| kCFStreamPropertyDataWritten | constant | CoreFoundation/CFStream.h | raw::kCFStreamPropertyDataWritten |
| kCFStreamPropertyFileCurrentOffset | constant | CoreFoundation/CFStream.h | raw::kCFStreamPropertyFileCurrentOffset |
| kCFStreamPropertySOCKSPassword | constant | CoreFoundation/CFStream.h | raw::kCFStreamPropertySOCKSPassword |
| kCFStreamPropertySOCKSProxy | constant | CoreFoundation/CFStream.h | raw::kCFStreamPropertySOCKSProxy |
| kCFStreamPropertySOCKSProxyHost | constant | CoreFoundation/CFStream.h | raw::kCFStreamPropertySOCKSProxyHost |
| kCFStreamPropertySOCKSProxyPort | constant | CoreFoundation/CFStream.h | raw::kCFStreamPropertySOCKSProxyPort |
| kCFStreamPropertySOCKSUser | constant | CoreFoundation/CFStream.h | raw::kCFStreamPropertySOCKSUser |
| kCFStreamPropertySOCKSVersion | constant | CoreFoundation/CFStream.h | raw::kCFStreamPropertySOCKSVersion |
| kCFStreamPropertyShouldCloseNativeSocket | constant | CoreFoundation/CFStream.h | raw::kCFStreamPropertyShouldCloseNativeSocket |
| kCFStreamPropertySocketNativeHandle | constant | CoreFoundation/CFStream.h | raw::kCFStreamPropertySocketNativeHandle |
| kCFStreamPropertySocketRemoteHostName | constant | CoreFoundation/CFStream.h | raw::kCFStreamPropertySocketRemoteHostName |
| kCFStreamPropertySocketRemotePortNumber | constant | CoreFoundation/CFStream.h | raw::kCFStreamPropertySocketRemotePortNumber |
| kCFStreamPropertySocketSecurityLevel | constant | CoreFoundation/CFStream.h | raw::kCFStreamPropertySocketSecurityLevel |
| kCFStreamSocketSOCKSVersion4 | constant | CoreFoundation/CFStream.h | raw::kCFStreamSocketSOCKSVersion4 |
| kCFStreamSocketSOCKSVersion5 | constant | CoreFoundation/CFStream.h | raw::kCFStreamSocketSOCKSVersion5 |
| kCFStreamSocketSecurityLevelNegotiatedSSL | constant | CoreFoundation/CFStream.h | raw::kCFStreamSocketSecurityLevelNegotiatedSSL |
| kCFStreamSocketSecurityLevelNone | constant | CoreFoundation/CFStream.h | raw::kCFStreamSocketSecurityLevelNone |
| kCFStreamSocketSecurityLevelTLSv1 | constant | CoreFoundation/CFStream.h | raw::kCFStreamSocketSecurityLevelTLSv1 |
| kCFStringBinaryHeapCallBacks | constant | CoreFoundation/CFBinaryHeap.h | raw::kCFStringBinaryHeapCallBacks |
| kCFStringTransformFullwidthHalfwidth | constant | CoreFoundation/CFString.h | raw::kCFStringTransformFullwidthHalfwidth |
| kCFStringTransformHiraganaKatakana | constant | CoreFoundation/CFString.h | raw::kCFStringTransformHiraganaKatakana |
| kCFStringTransformLatinArabic | constant | CoreFoundation/CFString.h | raw::kCFStringTransformLatinArabic |
| kCFStringTransformLatinCyrillic | constant | CoreFoundation/CFString.h | raw::kCFStringTransformLatinCyrillic |
| kCFStringTransformLatinGreek | constant | CoreFoundation/CFString.h | raw::kCFStringTransformLatinGreek |
| kCFStringTransformLatinHangul | constant | CoreFoundation/CFString.h | raw::kCFStringTransformLatinHangul |
| kCFStringTransformLatinHebrew | constant | CoreFoundation/CFString.h | raw::kCFStringTransformLatinHebrew |
| kCFStringTransformLatinHiragana | constant | CoreFoundation/CFString.h | raw::kCFStringTransformLatinHiragana |
| kCFStringTransformLatinKatakana | constant | CoreFoundation/CFString.h | raw::kCFStringTransformLatinKatakana |
| kCFStringTransformLatinThai | constant | CoreFoundation/CFString.h | raw::kCFStringTransformLatinThai |
| kCFStringTransformMandarinLatin | constant | CoreFoundation/CFString.h | raw::kCFStringTransformMandarinLatin |
| kCFStringTransformStripCombiningMarks | constant | CoreFoundation/CFString.h | raw::kCFStringTransformStripCombiningMarks |
| kCFStringTransformStripDiacritics | constant | CoreFoundation/CFString.h | raw::kCFStringTransformStripDiacritics |
| kCFStringTransformToLatin | constant | CoreFoundation/CFString.h | raw::kCFStringTransformToLatin |
| kCFStringTransformToUnicodeName | constant | CoreFoundation/CFString.h | raw::kCFStringTransformToUnicodeName |
| kCFStringTransformToXMLHex | constant | CoreFoundation/CFString.h | raw::kCFStringTransformToXMLHex |
| kCFTamilCalendar | constant | CoreFoundation/CFLocale.h | raw::kCFTamilCalendar |
| kCFTeluguCalendar | constant | CoreFoundation/CFLocale.h | raw::kCFTeluguCalendar |
| kCFTimeZoneSystemTimeZoneDidChangeNotification | constant | CoreFoundation/CFTimeZone.h | raw::kCFTimeZoneSystemTimeZoneDidChangeNotification |
| kCFTypeArrayCallBacks | constant | CoreFoundation/CFArray.h | raw::kCFTypeArrayCallBacks |
| kCFTypeBagCallBacks | constant | CoreFoundation/CFBag.h | raw::kCFTypeBagCallBacks |
| kCFTypeDictionaryKeyCallBacks | constant | CoreFoundation/CFDictionary.h | raw::kCFTypeDictionaryKeyCallBacks |
| kCFTypeDictionaryValueCallBacks | constant | CoreFoundation/CFDictionary.h | raw::kCFTypeDictionaryValueCallBacks |
| kCFTypeSetCallBacks | constant | CoreFoundation/CFSet.h | cf::CFSetCallbacks::Type |
| kCFURLAddedToDirectoryDateKey | constant | CoreFoundation/CFURL.h | raw::kCFURLAddedToDirectoryDateKey |
| kCFURLApplicationIsScriptableKey | constant | CoreFoundation/CFURL.h | raw::kCFURLApplicationIsScriptableKey |
| kCFURLAttributeModificationDateKey | constant | CoreFoundation/CFURL.h | raw::kCFURLAttributeModificationDateKey |
| kCFURLCanonicalPathKey | constant | CoreFoundation/CFURL.h | raw::kCFURLCanonicalPathKey |
| kCFURLContentAccessDateKey | constant | CoreFoundation/CFURL.h | raw::kCFURLContentAccessDateKey |
| kCFURLContentModificationDateKey | constant | CoreFoundation/CFURL.h | raw::kCFURLContentModificationDateKey |
| kCFURLCreationDateKey | constant | CoreFoundation/CFURL.h | raw::kCFURLCreationDateKey |
| kCFURLDirectoryEntryCountKey | constant | CoreFoundation/CFURL.h | raw::kCFURLDirectoryEntryCountKey |
| kCFURLDocumentIdentifierKey | constant | CoreFoundation/CFURL.h | raw::kCFURLDocumentIdentifierKey |
| kCFURLFileAllocatedSizeKey | constant | CoreFoundation/CFURL.h | raw::kCFURLFileAllocatedSizeKey |
| kCFURLFileContentIdentifierKey | constant | CoreFoundation/CFURL.h | raw::kCFURLFileContentIdentifierKey |
| kCFURLFileIdentifierKey | constant | CoreFoundation/CFURL.h | raw::kCFURLFileIdentifierKey |
| kCFURLFileResourceIdentifierKey | constant | CoreFoundation/CFURL.h | raw::kCFURLFileResourceIdentifierKey |
| kCFURLFileResourceTypeBlockSpecial | constant | CoreFoundation/CFURL.h | raw::kCFURLFileResourceTypeBlockSpecial |
| kCFURLFileResourceTypeCharacterSpecial | constant | CoreFoundation/CFURL.h | raw::kCFURLFileResourceTypeCharacterSpecial |
| kCFURLFileResourceTypeDirectory | constant | CoreFoundation/CFURL.h | raw::kCFURLFileResourceTypeDirectory |
| kCFURLFileResourceTypeKey | constant | CoreFoundation/CFURL.h | raw::kCFURLFileResourceTypeKey |
| kCFURLFileResourceTypeNamedPipe | constant | CoreFoundation/CFURL.h | raw::kCFURLFileResourceTypeNamedPipe |
| kCFURLFileResourceTypeRegular | constant | CoreFoundation/CFURL.h | raw::kCFURLFileResourceTypeRegular |
| kCFURLFileResourceTypeSocket | constant | CoreFoundation/CFURL.h | raw::kCFURLFileResourceTypeSocket |
| kCFURLFileResourceTypeSymbolicLink | constant | CoreFoundation/CFURL.h | raw::kCFURLFileResourceTypeSymbolicLink |
| kCFURLFileResourceTypeUnknown | constant | CoreFoundation/CFURL.h | raw::kCFURLFileResourceTypeUnknown |
| kCFURLFileSecurityKey | constant | CoreFoundation/CFURL.h | raw::kCFURLFileSecurityKey |
| kCFURLFileSizeKey | constant | CoreFoundation/CFURL.h | raw::kCFURLFileSizeKey |
| kCFURLGenerationIdentifierKey | constant | CoreFoundation/CFURL.h | raw::kCFURLGenerationIdentifierKey |
| kCFURLHasHiddenExtensionKey | constant | CoreFoundation/CFURL.h | raw::kCFURLHasHiddenExtensionKey |
| kCFURLIsAliasFileKey | constant | CoreFoundation/CFURL.h | raw::kCFURLIsAliasFileKey |
| kCFURLIsApplicationKey | constant | CoreFoundation/CFURL.h | raw::kCFURLIsApplicationKey |
| kCFURLIsDirectoryKey | constant | CoreFoundation/CFURL.h | raw::kCFURLIsDirectoryKey |
| kCFURLIsExcludedFromBackupKey | constant | CoreFoundation/CFURL.h | raw::kCFURLIsExcludedFromBackupKey |
| kCFURLIsExecutableKey | constant | CoreFoundation/CFURL.h | raw::kCFURLIsExecutableKey |
| kCFURLIsHiddenKey | constant | CoreFoundation/CFURL.h | raw::kCFURLIsHiddenKey |
| kCFURLIsMountTriggerKey | constant | CoreFoundation/CFURL.h | raw::kCFURLIsMountTriggerKey |
| kCFURLIsPackageKey | constant | CoreFoundation/CFURL.h | raw::kCFURLIsPackageKey |
| kCFURLIsPurgeableKey | constant | CoreFoundation/CFURL.h | raw::kCFURLIsPurgeableKey |
| kCFURLIsReadableKey | constant | CoreFoundation/CFURL.h | raw::kCFURLIsReadableKey |
| kCFURLIsRegularFileKey | constant | CoreFoundation/CFURL.h | raw::kCFURLIsRegularFileKey |
| kCFURLIsSparseKey | constant | CoreFoundation/CFURL.h | raw::kCFURLIsSparseKey |
| kCFURLIsSymbolicLinkKey | constant | CoreFoundation/CFURL.h | raw::kCFURLIsSymbolicLinkKey |
| kCFURLIsSystemImmutableKey | constant | CoreFoundation/CFURL.h | raw::kCFURLIsSystemImmutableKey |
| kCFURLIsUbiquitousItemKey | constant | CoreFoundation/CFURL.h | raw::kCFURLIsUbiquitousItemKey |
| kCFURLIsUserImmutableKey | constant | CoreFoundation/CFURL.h | raw::kCFURLIsUserImmutableKey |
| kCFURLIsVolumeKey | constant | CoreFoundation/CFURL.h | raw::kCFURLIsVolumeKey |
| kCFURLIsWritableKey | constant | CoreFoundation/CFURL.h | raw::kCFURLIsWritableKey |
| kCFURLKeysOfUnsetValuesKey | constant | CoreFoundation/CFURL.h | raw::kCFURLKeysOfUnsetValuesKey |
| kCFURLLabelNumberKey | constant | CoreFoundation/CFURL.h | raw::kCFURLLabelNumberKey |
| kCFURLLinkCountKey | constant | CoreFoundation/CFURL.h | raw::kCFURLLinkCountKey |
| kCFURLLocalizedLabelKey | constant | CoreFoundation/CFURL.h | raw::kCFURLLocalizedLabelKey |
| kCFURLLocalizedNameKey | constant | CoreFoundation/CFURL.h | raw::kCFURLLocalizedNameKey |
| kCFURLLocalizedTypeDescriptionKey | constant | CoreFoundation/CFURL.h | raw::kCFURLLocalizedTypeDescriptionKey |
| kCFURLMayHaveExtendedAttributesKey | constant | CoreFoundation/CFURL.h | raw::kCFURLMayHaveExtendedAttributesKey |
| kCFURLMayShareFileContentKey | constant | CoreFoundation/CFURL.h | raw::kCFURLMayShareFileContentKey |
| kCFURLNameKey | constant | CoreFoundation/CFURL.h | raw::kCFURLNameKey |
| kCFURLParentDirectoryURLKey | constant | CoreFoundation/CFURL.h | raw::kCFURLParentDirectoryURLKey |
| kCFURLPathKey | constant | CoreFoundation/CFURL.h | raw::kCFURLPathKey |
| kCFURLPreferredIOBlockSizeKey | constant | CoreFoundation/CFURL.h | raw::kCFURLPreferredIOBlockSizeKey |
| kCFURLQuarantinePropertiesKey | constant | CoreFoundation/CFURL.h | raw::kCFURLQuarantinePropertiesKey |
| kCFURLTagNamesKey | constant | CoreFoundation/CFURL.h | raw::kCFURLTagNamesKey |
| kCFURLTotalFileAllocatedSizeKey | constant | CoreFoundation/CFURL.h | raw::kCFURLTotalFileAllocatedSizeKey |
| kCFURLTotalFileSizeKey | constant | CoreFoundation/CFURL.h | raw::kCFURLTotalFileSizeKey |
| kCFURLUbiquitousItemDownloadingErrorKey | constant | CoreFoundation/CFURL.h | raw::kCFURLUbiquitousItemDownloadingErrorKey |
| kCFURLUbiquitousItemDownloadingStatusCurrent | constant | CoreFoundation/CFURL.h | raw::kCFURLUbiquitousItemDownloadingStatusCurrent |
| kCFURLUbiquitousItemDownloadingStatusDownloaded | constant | CoreFoundation/CFURL.h | raw::kCFURLUbiquitousItemDownloadingStatusDownloaded |
| kCFURLUbiquitousItemDownloadingStatusNotDownloaded | constant | CoreFoundation/CFURL.h | raw::kCFURLUbiquitousItemDownloadingStatusNotDownloaded |
| kCFURLUbiquitousItemHasUnresolvedConflictsKey | constant | CoreFoundation/CFURL.h | raw::kCFURLUbiquitousItemHasUnresolvedConflictsKey |
| kCFURLUbiquitousItemIsDownloadingKey | constant | CoreFoundation/CFURL.h | raw::kCFURLUbiquitousItemIsDownloadingKey |
| kCFURLUbiquitousItemIsExcludedFromSyncKey | constant | CoreFoundation/CFURL.h | raw::kCFURLUbiquitousItemIsExcludedFromSyncKey |
| kCFURLUbiquitousItemIsSyncPausedKey | constant | CoreFoundation/CFURL.h | raw::kCFURLUbiquitousItemIsSyncPausedKey |
| kCFURLUbiquitousItemIsUploadedKey | constant | CoreFoundation/CFURL.h | raw::kCFURLUbiquitousItemIsUploadedKey |
| kCFURLUbiquitousItemIsUploadingKey | constant | CoreFoundation/CFURL.h | raw::kCFURLUbiquitousItemIsUploadingKey |
| kCFURLUbiquitousItemSupportedSyncControlsKey | constant | CoreFoundation/CFURL.h | raw::kCFURLUbiquitousItemSupportedSyncControlsKey |
| kCFURLUbiquitousItemUploadingErrorKey | constant | CoreFoundation/CFURL.h | raw::kCFURLUbiquitousItemUploadingErrorKey |
| kCFURLVolumeAvailableCapacityForImportantUsageKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeAvailableCapacityForImportantUsageKey |
| kCFURLVolumeAvailableCapacityForOpportunisticUsageKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeAvailableCapacityForOpportunisticUsageKey |
| kCFURLVolumeAvailableCapacityKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeAvailableCapacityKey |
| kCFURLVolumeCreationDateKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeCreationDateKey |
| kCFURLVolumeIdentifierKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeIdentifierKey |
| kCFURLVolumeIsAutomountedKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeIsAutomountedKey |
| kCFURLVolumeIsBrowsableKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeIsBrowsableKey |
| kCFURLVolumeIsEjectableKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeIsEjectableKey |
| kCFURLVolumeIsEncryptedKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeIsEncryptedKey |
| kCFURLVolumeIsInternalKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeIsInternalKey |
| kCFURLVolumeIsJournalingKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeIsJournalingKey |
| kCFURLVolumeIsLocalKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeIsLocalKey |
| kCFURLVolumeIsReadOnlyKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeIsReadOnlyKey |
| kCFURLVolumeIsRemovableKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeIsRemovableKey |
| kCFURLVolumeIsRootFileSystemKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeIsRootFileSystemKey |
| kCFURLVolumeLocalizedFormatDescriptionKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeLocalizedFormatDescriptionKey |
| kCFURLVolumeLocalizedNameKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeLocalizedNameKey |
| kCFURLVolumeMaximumFileSizeKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeMaximumFileSizeKey |
| kCFURLVolumeMountFromLocationKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeMountFromLocationKey |
| kCFURLVolumeNameKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeNameKey |
| kCFURLVolumeResourceCountKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeResourceCountKey |
| kCFURLVolumeSubtypeKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeSubtypeKey |
| kCFURLVolumeSupportsAccessPermissionsKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeSupportsAccessPermissionsKey |
| kCFURLVolumeSupportsAdvisoryFileLockingKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeSupportsAdvisoryFileLockingKey |
| kCFURLVolumeSupportsCasePreservedNamesKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeSupportsCasePreservedNamesKey |
| kCFURLVolumeSupportsCaseSensitiveNamesKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeSupportsCaseSensitiveNamesKey |
| kCFURLVolumeSupportsCompressionKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeSupportsCompressionKey |
| kCFURLVolumeSupportsExclusiveRenamingKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeSupportsExclusiveRenamingKey |
| kCFURLVolumeSupportsExtendedSecurityKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeSupportsExtendedSecurityKey |
| kCFURLVolumeSupportsFileCloningKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeSupportsFileCloningKey |
| kCFURLVolumeSupportsFileProtectionKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeSupportsFileProtectionKey |
| kCFURLVolumeSupportsHardLinksKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeSupportsHardLinksKey |
| kCFURLVolumeSupportsImmutableFilesKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeSupportsImmutableFilesKey |
| kCFURLVolumeSupportsJournalingKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeSupportsJournalingKey |
| kCFURLVolumeSupportsPersistentIDsKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeSupportsPersistentIDsKey |
| kCFURLVolumeSupportsRenamingKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeSupportsRenamingKey |
| kCFURLVolumeSupportsRootDirectoryDatesKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeSupportsRootDirectoryDatesKey |
| kCFURLVolumeSupportsSparseFilesKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeSupportsSparseFilesKey |
| kCFURLVolumeSupportsSwapRenamingKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeSupportsSwapRenamingKey |
| kCFURLVolumeSupportsSymbolicLinksKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeSupportsSymbolicLinksKey |
| kCFURLVolumeSupportsVolumeSizesKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeSupportsVolumeSizesKey |
| kCFURLVolumeSupportsZeroRunsKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeSupportsZeroRunsKey |
| kCFURLVolumeTotalCapacityKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeTotalCapacityKey |
| kCFURLVolumeTypeNameKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeTypeNameKey |
| kCFURLVolumeURLForRemountingKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeURLForRemountingKey |
| kCFURLVolumeURLKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeURLKey |
| kCFURLVolumeUUIDStringKey | constant | CoreFoundation/CFURL.h | raw::kCFURLVolumeUUIDStringKey |
| kCFUserNotificationAlertHeaderKey | constant | CoreFoundation/CFUserNotification.h | raw::kCFUserNotificationAlertHeaderKey |
| kCFUserNotificationAlertMessageKey | constant | CoreFoundation/CFUserNotification.h | raw::kCFUserNotificationAlertMessageKey |
| kCFUserNotificationAlertTopMostKey | constant | CoreFoundation/CFUserNotification.h | raw::kCFUserNotificationAlertTopMostKey |
| kCFUserNotificationAlternateButtonTitleKey | constant | CoreFoundation/CFUserNotification.h | raw::kCFUserNotificationAlternateButtonTitleKey |
| kCFUserNotificationCheckBoxTitlesKey | constant | CoreFoundation/CFUserNotification.h | raw::kCFUserNotificationCheckBoxTitlesKey |
| kCFUserNotificationDefaultButtonTitleKey | constant | CoreFoundation/CFUserNotification.h | raw::kCFUserNotificationDefaultButtonTitleKey |
| kCFUserNotificationIconURLKey | constant | CoreFoundation/CFUserNotification.h | raw::kCFUserNotificationIconURLKey |
| kCFUserNotificationKeyboardTypesKey | constant | CoreFoundation/CFUserNotification.h | raw::kCFUserNotificationKeyboardTypesKey |
| kCFUserNotificationLocalizationURLKey | constant | CoreFoundation/CFUserNotification.h | raw::kCFUserNotificationLocalizationURLKey |
| kCFUserNotificationOtherButtonTitleKey | constant | CoreFoundation/CFUserNotification.h | raw::kCFUserNotificationOtherButtonTitleKey |
| kCFUserNotificationPopUpSelectionKey | constant | CoreFoundation/CFUserNotification.h | raw::kCFUserNotificationPopUpSelectionKey |
| kCFUserNotificationPopUpTitlesKey | constant | CoreFoundation/CFUserNotification.h | raw::kCFUserNotificationPopUpTitlesKey |
| kCFUserNotificationProgressIndicatorValueKey | constant | CoreFoundation/CFUserNotification.h | raw::kCFUserNotificationProgressIndicatorValueKey |
| kCFUserNotificationSoundURLKey | constant | CoreFoundation/CFUserNotification.h | raw::kCFUserNotificationSoundURLKey |
| kCFUserNotificationTextFieldTitlesKey | constant | CoreFoundation/CFUserNotification.h | raw::kCFUserNotificationTextFieldTitlesKey |
| kCFUserNotificationTextFieldValuesKey | constant | CoreFoundation/CFUserNotification.h | raw::kCFUserNotificationTextFieldValuesKey |
| kCFVietnameseCalendar | constant | CoreFoundation/CFLocale.h | raw::kCFVietnameseCalendar |
| kCFVikramCalendar | constant | CoreFoundation/CFLocale.h | raw::kCFVikramCalendar |
| kCFXMLTreeErrorDescription | constant | CoreFoundation/CFXMLParser.h | raw::kCFXMLTreeErrorDescription |
| kCFXMLTreeErrorLineNumber | constant | CoreFoundation/CFXMLParser.h | raw::kCFXMLTreeErrorLineNumber |
| kCFXMLTreeErrorLocation | constant | CoreFoundation/CFXMLParser.h | raw::kCFXMLTreeErrorLocation |
| kCFXMLTreeErrorStatusCode | constant | CoreFoundation/CFXMLParser.h | raw::kCFXMLTreeErrorStatusCode |
| kCMFormatDescriptionAlphaChannelMode_PremultipliedAlpha | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionAlphaChannelMode_PremultipliedAlpha |
| kCMFormatDescriptionAlphaChannelMode_StraightAlpha | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionAlphaChannelMode_StraightAlpha |
| kCMFormatDescriptionCameraCalibrationExtrinsicOriginSource_StereoCameraSystemBaseline | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionCameraCalibrationExtrinsicOriginSource_StereoCameraSystemBaseline |
| kCMFormatDescriptionCameraCalibrationLensAlgorithmKind_ParametricLens | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionCameraCalibrationLensAlgorithmKind_ParametricLens |
| kCMFormatDescriptionCameraCalibrationLensDomain_Color | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionCameraCalibrationLensDomain_Color |
| kCMFormatDescriptionCameraCalibrationLensRole_Left | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionCameraCalibrationLensRole_Left |
| kCMFormatDescriptionCameraCalibrationLensRole_Mono | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionCameraCalibrationLensRole_Mono |
| kCMFormatDescriptionCameraCalibrationLensRole_Right | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionCameraCalibrationLensRole_Right |
| kCMFormatDescriptionCameraCalibration_ExtrinsicOrientationQuaternion | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionCameraCalibration_ExtrinsicOrientationQuaternion |
| kCMFormatDescriptionCameraCalibration_ExtrinsicOriginSource | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionCameraCalibration_ExtrinsicOriginSource |
| kCMFormatDescriptionCameraCalibration_IntrinsicMatrix | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionCameraCalibration_IntrinsicMatrix |
| kCMFormatDescriptionCameraCalibration_IntrinsicMatrixProjectionOffset | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionCameraCalibration_IntrinsicMatrixProjectionOffset |
| kCMFormatDescriptionCameraCalibration_IntrinsicMatrixReferenceDimensions | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionCameraCalibration_IntrinsicMatrixReferenceDimensions |
| kCMFormatDescriptionCameraCalibration_LensAlgorithmKind | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionCameraCalibration_LensAlgorithmKind |
| kCMFormatDescriptionCameraCalibration_LensDistortions | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionCameraCalibration_LensDistortions |
| kCMFormatDescriptionCameraCalibration_LensDomain | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionCameraCalibration_LensDomain |
| kCMFormatDescriptionCameraCalibration_LensFrameAdjustmentsPolynomialX | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionCameraCalibration_LensFrameAdjustmentsPolynomialX |
| kCMFormatDescriptionCameraCalibration_LensFrameAdjustmentsPolynomialY | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionCameraCalibration_LensFrameAdjustmentsPolynomialY |
| kCMFormatDescriptionCameraCalibration_LensIdentifier | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionCameraCalibration_LensIdentifier |
| kCMFormatDescriptionCameraCalibration_LensRole | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionCameraCalibration_LensRole |
| kCMFormatDescriptionCameraCalibration_RadialAngleLimit | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionCameraCalibration_RadialAngleLimit |
| kCMFormatDescriptionChromaLocation_Bottom | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionChromaLocation_Bottom |
| kCMFormatDescriptionChromaLocation_BottomLeft | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionChromaLocation_BottomLeft |
| kCMFormatDescriptionChromaLocation_Center | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionChromaLocation_Center |
| kCMFormatDescriptionChromaLocation_DV420 | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionChromaLocation_DV420 |
| kCMFormatDescriptionChromaLocation_Left | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionChromaLocation_Left |
| kCMFormatDescriptionChromaLocation_Top | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionChromaLocation_Top |
| kCMFormatDescriptionChromaLocation_TopLeft | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionChromaLocation_TopLeft |
| kCMFormatDescriptionColorPrimaries_DCI_P3 | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionColorPrimaries_DCI_P3 |
| kCMFormatDescriptionColorPrimaries_EBU_3213 | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionColorPrimaries_EBU_3213 |
| kCMFormatDescriptionColorPrimaries_ITU_R_2020 | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionColorPrimaries_ITU_R_2020 |
| kCMFormatDescriptionColorPrimaries_ITU_R_709_2 | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionColorPrimaries_ITU_R_709_2 |
| kCMFormatDescriptionColorPrimaries_P22 | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionColorPrimaries_P22 |
| kCMFormatDescriptionColorPrimaries_P3_D65 | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionColorPrimaries_P3_D65 |
| kCMFormatDescriptionColorPrimaries_SMPTE_C | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionColorPrimaries_SMPTE_C |
| kCMFormatDescriptionConformsToMPEG2VideoProfile | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionConformsToMPEG2VideoProfile |
| kCMFormatDescriptionExtensionKey_MetadataKeyTable | constant | CoreMedia/CMFormatDescription.h | cm::format_description::format_description_extension_keys::metadata_key_table |
| kCMFormatDescriptionExtension_AlphaChannelMode | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_AlphaChannelMode |
| kCMFormatDescriptionExtension_AlternativeTransferCharacteristics | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_AlternativeTransferCharacteristics |
| kCMFormatDescriptionExtension_AmbientViewingEnvironment | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_AmbientViewingEnvironment |
| kCMFormatDescriptionExtension_AuxiliaryTypeInfo | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_AuxiliaryTypeInfo |
| kCMFormatDescriptionExtension_BitsPerComponent | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_BitsPerComponent |
| kCMFormatDescriptionExtension_BytesPerRow | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_BytesPerRow |
| kCMFormatDescriptionExtension_CameraCalibrationDataLensCollection | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_CameraCalibrationDataLensCollection |
| kCMFormatDescriptionExtension_ChromaLocationBottomField | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_ChromaLocationBottomField |
| kCMFormatDescriptionExtension_ChromaLocationTopField | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_ChromaLocationTopField |
| kCMFormatDescriptionExtension_CleanAperture | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_CleanAperture |
| kCMFormatDescriptionExtension_ColorPrimaries | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_ColorPrimaries |
| kCMFormatDescriptionExtension_ContainsAlphaChannel | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_ContainsAlphaChannel |
| kCMFormatDescriptionExtension_ContentColorVolume | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_ContentColorVolume |
| kCMFormatDescriptionExtension_ContentLightLevelInfo | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_ContentLightLevelInfo |
| kCMFormatDescriptionExtension_ConvertedFromExternalSphericalTags | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_ConvertedFromExternalSphericalTags |
| kCMFormatDescriptionExtension_Depth | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_Depth |
| kCMFormatDescriptionExtension_FieldCount | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_FieldCount |
| kCMFormatDescriptionExtension_FieldDetail | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_FieldDetail |
| kCMFormatDescriptionExtension_FormatName | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_FormatName |
| kCMFormatDescriptionExtension_FullRangeVideo | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_FullRangeVideo |
| kCMFormatDescriptionExtension_GammaLevel | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_GammaLevel |
| kCMFormatDescriptionExtension_HasAdditionalViews | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_HasAdditionalViews |
| kCMFormatDescriptionExtension_HasLeftStereoEyeView | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_HasLeftStereoEyeView |
| kCMFormatDescriptionExtension_HasRightStereoEyeView | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_HasRightStereoEyeView |
| kCMFormatDescriptionExtension_HeroEye | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_HeroEye |
| kCMFormatDescriptionExtension_HorizontalDisparityAdjustment | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_HorizontalDisparityAdjustment |
| kCMFormatDescriptionExtension_HorizontalFieldOfView | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_HorizontalFieldOfView |
| kCMFormatDescriptionExtension_ICCProfile | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_ICCProfile |
| kCMFormatDescriptionExtension_LogTransferFunction | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_LogTransferFunction |
| kCMFormatDescriptionExtension_MasteringDisplayColorVolume | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_MasteringDisplayColorVolume |
| kCMFormatDescriptionExtension_OriginalCompressionSettings | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_OriginalCompressionSettings |
| kCMFormatDescriptionExtension_PixelAspectRatio | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_PixelAspectRatio |
| kCMFormatDescriptionExtension_ProjectionKind | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_ProjectionKind |
| kCMFormatDescriptionExtension_ProtectedContentOriginalFormat | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_ProtectedContentOriginalFormat |
| kCMFormatDescriptionExtension_RevisionLevel | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_RevisionLevel |
| kCMFormatDescriptionExtension_SampleDescriptionExtensionAtoms | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_SampleDescriptionExtensionAtoms |
| kCMFormatDescriptionExtension_SpatialQuality | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_SpatialQuality |
| kCMFormatDescriptionExtension_StereoCameraBaseline | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_StereoCameraBaseline |
| kCMFormatDescriptionExtension_TemporalQuality | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_TemporalQuality |
| kCMFormatDescriptionExtension_TransferFunction | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_TransferFunction |
| kCMFormatDescriptionExtension_Vendor | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_Vendor |
| kCMFormatDescriptionExtension_VerbatimISOSampleEntry | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_VerbatimISOSampleEntry |
| kCMFormatDescriptionExtension_VerbatimImageDescription | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_VerbatimImageDescription |
| kCMFormatDescriptionExtension_VerbatimSampleDescription | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_VerbatimSampleDescription |
| kCMFormatDescriptionExtension_Version | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_Version |
| kCMFormatDescriptionExtension_ViewPackingKind | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_ViewPackingKind |
| kCMFormatDescriptionExtension_YCbCrMatrix | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionExtension_YCbCrMatrix |
| kCMFormatDescriptionFieldDetail_SpatialFirstLineEarly | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionFieldDetail_SpatialFirstLineEarly |
| kCMFormatDescriptionFieldDetail_SpatialFirstLineLate | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionFieldDetail_SpatialFirstLineLate |
| kCMFormatDescriptionFieldDetail_TemporalBottomFirst | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionFieldDetail_TemporalBottomFirst |
| kCMFormatDescriptionFieldDetail_TemporalTopFirst | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionFieldDetail_TemporalTopFirst |
| kCMFormatDescriptionHeroEye_Left | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionHeroEye_Left |
| kCMFormatDescriptionHeroEye_Right | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionHeroEye_Right |
| kCMFormatDescriptionKey_CleanApertureHeight | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionKey_CleanApertureHeight |
| kCMFormatDescriptionKey_CleanApertureHeightRational | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionKey_CleanApertureHeightRational |
| kCMFormatDescriptionKey_CleanApertureHorizontalOffset | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionKey_CleanApertureHorizontalOffset |
| kCMFormatDescriptionKey_CleanApertureHorizontalOffsetRational | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionKey_CleanApertureHorizontalOffsetRational |
| kCMFormatDescriptionKey_CleanApertureVerticalOffset | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionKey_CleanApertureVerticalOffset |
| kCMFormatDescriptionKey_CleanApertureVerticalOffsetRational | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionKey_CleanApertureVerticalOffsetRational |
| kCMFormatDescriptionKey_CleanApertureWidth | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionKey_CleanApertureWidth |
| kCMFormatDescriptionKey_CleanApertureWidthRational | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionKey_CleanApertureWidthRational |
| kCMFormatDescriptionKey_PixelAspectRatioHorizontalSpacing | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionKey_PixelAspectRatioHorizontalSpacing |
| kCMFormatDescriptionKey_PixelAspectRatioVerticalSpacing | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionKey_PixelAspectRatioVerticalSpacing |
| kCMFormatDescriptionLogTransferFunction_AppleLog | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionLogTransferFunction_AppleLog |
| kCMFormatDescriptionProjectionKind_AppleImmersiveVideo | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionProjectionKind_AppleImmersiveVideo |
| kCMFormatDescriptionProjectionKind_Equirectangular | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionProjectionKind_Equirectangular |
| kCMFormatDescriptionProjectionKind_HalfEquirectangular | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionProjectionKind_HalfEquirectangular |
| kCMFormatDescriptionProjectionKind_ParametricImmersive | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionProjectionKind_ParametricImmersive |
| kCMFormatDescriptionProjectionKind_Rectilinear | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionProjectionKind_Rectilinear |
| kCMFormatDescriptionTransferFunction_ITU_R_2020 | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionTransferFunction_ITU_R_2020 |
| kCMFormatDescriptionTransferFunction_ITU_R_2100_HLG | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionTransferFunction_ITU_R_2100_HLG |
| kCMFormatDescriptionTransferFunction_ITU_R_709_2 | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionTransferFunction_ITU_R_709_2 |
| kCMFormatDescriptionTransferFunction_Linear | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionTransferFunction_Linear |
| kCMFormatDescriptionTransferFunction_SMPTE_240M_1995 | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionTransferFunction_SMPTE_240M_1995 |
| kCMFormatDescriptionTransferFunction_SMPTE_ST_2084_PQ | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionTransferFunction_SMPTE_ST_2084_PQ |
| kCMFormatDescriptionTransferFunction_SMPTE_ST_428_1 | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionTransferFunction_SMPTE_ST_428_1 |
| kCMFormatDescriptionTransferFunction_UseGamma | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionTransferFunction_UseGamma |
| kCMFormatDescriptionTransferFunction_sRGB | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionTransferFunction_sRGB |
| kCMFormatDescriptionVendor_Apple | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionVendor_Apple |
| kCMFormatDescriptionViewPackingKind_OverUnder | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionViewPackingKind_OverUnder |
| kCMFormatDescriptionViewPackingKind_SideBySide | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionViewPackingKind_SideBySide |
| kCMFormatDescriptionYCbCrMatrix_ITU_R_2020 | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionYCbCrMatrix_ITU_R_2020 |
| kCMFormatDescriptionYCbCrMatrix_ITU_R_601_4 | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionYCbCrMatrix_ITU_R_601_4 |
| kCMFormatDescriptionYCbCrMatrix_ITU_R_709_2 | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionYCbCrMatrix_ITU_R_709_2 |
| kCMFormatDescriptionYCbCrMatrix_SMPTE_240M_1995 | constant | CoreMedia/CMFormatDescription.h | raw::kCMFormatDescriptionYCbCrMatrix_SMPTE_240M_1995 |
| kCMHEVCTemporalLevelInfoKey_ConstraintIndicatorFlags | constant | CoreMedia/CMSampleBuffer.h | raw::kCMHEVCTemporalLevelInfoKey_ConstraintIndicatorFlags |
| kCMHEVCTemporalLevelInfoKey_LevelIndex | constant | CoreMedia/CMSampleBuffer.h | raw::kCMHEVCTemporalLevelInfoKey_LevelIndex |
| kCMHEVCTemporalLevelInfoKey_ProfileCompatibilityFlags | constant | CoreMedia/CMSampleBuffer.h | raw::kCMHEVCTemporalLevelInfoKey_ProfileCompatibilityFlags |
| kCMHEVCTemporalLevelInfoKey_ProfileIndex | constant | CoreMedia/CMSampleBuffer.h | raw::kCMHEVCTemporalLevelInfoKey_ProfileIndex |
| kCMHEVCTemporalLevelInfoKey_ProfileSpace | constant | CoreMedia/CMSampleBuffer.h | raw::kCMHEVCTemporalLevelInfoKey_ProfileSpace |
| kCMHEVCTemporalLevelInfoKey_TemporalLevel | constant | CoreMedia/CMSampleBuffer.h | raw::kCMHEVCTemporalLevelInfoKey_TemporalLevel |
| kCMHEVCTemporalLevelInfoKey_TierFlag | constant | CoreMedia/CMSampleBuffer.h | raw::kCMHEVCTemporalLevelInfoKey_TierFlag |
| kCMImageDescriptionFlavor_3GPFamily | constant | CoreMedia/CMFormatDescriptionBridge.h | raw::kCMImageDescriptionFlavor_3GPFamily |
| kCMImageDescriptionFlavor_ISOFamily | constant | CoreMedia/CMFormatDescriptionBridge.h | raw::kCMImageDescriptionFlavor_ISOFamily |
| kCMImageDescriptionFlavor_ISOFamilyWithAppleExtensions | constant | CoreMedia/CMFormatDescriptionBridge.h | raw::kCMImageDescriptionFlavor_ISOFamilyWithAppleExtensions |
| kCMImageDescriptionFlavor_QuickTimeMovie | constant | CoreMedia/CMFormatDescriptionBridge.h | raw::kCMImageDescriptionFlavor_QuickTimeMovie |
| kCMMApplyTransformProcName | constant | ? | raw::kCMMApplyTransformProcName |
| kCMMCreateTransformPropertyProcName | constant | ? | raw::kCMMCreateTransformPropertyProcName |
| kCMMInitializeLinkProfileProcName | constant | ? | raw::kCMMInitializeLinkProfileProcName |
| kCMMInitializeTransformProcName | constant | ? | raw::kCMMInitializeTransformProcName |
| kCMMemoryPoolOption_AgeOutPeriod | constant | CoreMedia/CMMemoryPool.h | raw::kCMMemoryPoolOption_AgeOutPeriod |
| kCMMetadataBaseDataType_AffineTransformF64 | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataBaseDataType_AffineTransformF64 |
| kCMMetadataBaseDataType_BMP | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataBaseDataType_BMP |
| kCMMetadataBaseDataType_DimensionsF32 | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataBaseDataType_DimensionsF32 |
| kCMMetadataBaseDataType_ExtendedRasterRectangleValue | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataBaseDataType_ExtendedRasterRectangleValue |
| kCMMetadataBaseDataType_Float32 | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataBaseDataType_Float32 |
| kCMMetadataBaseDataType_Float64 | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataBaseDataType_Float64 |
| kCMMetadataBaseDataType_GIF | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataBaseDataType_GIF |
| kCMMetadataBaseDataType_JPEG | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataBaseDataType_JPEG |
| kCMMetadataBaseDataType_JSON | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataBaseDataType_JSON |
| kCMMetadataBaseDataType_PNG | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataBaseDataType_PNG |
| kCMMetadataBaseDataType_PerspectiveTransformF64 | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataBaseDataType_PerspectiveTransformF64 |
| kCMMetadataBaseDataType_PointF32 | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataBaseDataType_PointF32 |
| kCMMetadataBaseDataType_PolygonF32 | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataBaseDataType_PolygonF32 |
| kCMMetadataBaseDataType_PolylineF32 | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataBaseDataType_PolylineF32 |
| kCMMetadataBaseDataType_RasterRectangleValue | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataBaseDataType_RasterRectangleValue |
| kCMMetadataBaseDataType_RawData | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataBaseDataType_RawData |
| kCMMetadataBaseDataType_RectF32 | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataBaseDataType_RectF32 |
| kCMMetadataBaseDataType_SInt16 | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataBaseDataType_SInt16 |
| kCMMetadataBaseDataType_SInt32 | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataBaseDataType_SInt32 |
| kCMMetadataBaseDataType_SInt64 | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataBaseDataType_SInt64 |
| kCMMetadataBaseDataType_SInt8 | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataBaseDataType_SInt8 |
| kCMMetadataBaseDataType_UInt16 | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataBaseDataType_UInt16 |
| kCMMetadataBaseDataType_UInt32 | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataBaseDataType_UInt32 |
| kCMMetadataBaseDataType_UInt64 | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataBaseDataType_UInt64 |
| kCMMetadataBaseDataType_UInt8 | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataBaseDataType_UInt8 |
| kCMMetadataBaseDataType_UTF16 | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataBaseDataType_UTF16 |
| kCMMetadataBaseDataType_UTF8 | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataBaseDataType_UTF8 |
| kCMMetadataDataType_QuickTimeMetadataDirection | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataDataType_QuickTimeMetadataDirection |
| kCMMetadataDataType_QuickTimeMetadataLocation_ISO6709 | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataDataType_QuickTimeMetadataLocation_ISO6709 |
| kCMMetadataDataType_QuickTimeMetadataMilliLux | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataDataType_QuickTimeMetadataMilliLux |
| kCMMetadataDataType_QuickTimeMetadataUUID | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataDataType_QuickTimeMetadataUUID |
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
| kCMMetadataIdentifier_QuickTimeMetadataDirection_Facing | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataIdentifier_QuickTimeMetadataDirection_Facing |
| kCMMetadataIdentifier_QuickTimeMetadataDisplayMaskRectangleMono | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataIdentifier_QuickTimeMetadataDisplayMaskRectangleMono |
| kCMMetadataIdentifier_QuickTimeMetadataDisplayMaskRectangleStereoLeft | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataIdentifier_QuickTimeMetadataDisplayMaskRectangleStereoLeft |
| kCMMetadataIdentifier_QuickTimeMetadataDisplayMaskRectangleStereoRight | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataIdentifier_QuickTimeMetadataDisplayMaskRectangleStereoRight |
| kCMMetadataIdentifier_QuickTimeMetadataLivePhotoStillImageTransform | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataIdentifier_QuickTimeMetadataLivePhotoStillImageTransform |
| kCMMetadataIdentifier_QuickTimeMetadataLivePhotoStillImageTransformReferenceDimensions | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataIdentifier_QuickTimeMetadataLivePhotoStillImageTransformReferenceDimensions |
| kCMMetadataIdentifier_QuickTimeMetadataLocation_ISO6709 | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataIdentifier_QuickTimeMetadataLocation_ISO6709 |
| kCMMetadataIdentifier_QuickTimeMetadataPreferredAffineTransform | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataIdentifier_QuickTimeMetadataPreferredAffineTransform |
| kCMMetadataIdentifier_QuickTimeMetadataPresentationImmersiveMedia | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataIdentifier_QuickTimeMetadataPresentationImmersiveMedia |
| kCMMetadataIdentifier_QuickTimeMetadataSceneIlluminance | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataIdentifier_QuickTimeMetadataSceneIlluminance |
| kCMMetadataIdentifier_QuickTimeMetadataSegmentIdentifier | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataIdentifier_QuickTimeMetadataSegmentIdentifier |
| kCMMetadataIdentifier_QuickTimeMetadataSpatialAudioMix | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataIdentifier_QuickTimeMetadataSpatialAudioMix |
| kCMMetadataIdentifier_QuickTimeMetadataVideoOrientation | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataIdentifier_QuickTimeMetadataVideoOrientation |
| kCMMetadataKeySpace_HLSDateRange | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataKeySpace_HLSDateRange |
| kCMMetadataKeySpace_ID3 | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataKeySpace_ID3 |
| kCMMetadataKeySpace_ISOUserData | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataKeySpace_ISOUserData |
| kCMMetadataKeySpace_Icy | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataKeySpace_Icy |
| kCMMetadataKeySpace_QuickTimeMetadata | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataKeySpace_QuickTimeMetadata |
| kCMMetadataKeySpace_QuickTimeUserData | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataKeySpace_QuickTimeUserData |
| kCMMetadataKeySpace_iTunes | constant | CoreMedia/CMMetadata.h | raw::kCMMetadataKeySpace_iTunes |
| kCMSEncoderDigestAlgorithmSHA1 | constant | ? | raw::kCMSEncoderDigestAlgorithmSHA1 |
| kCMSEncoderDigestAlgorithmSHA256 | constant | ? | raw::kCMSEncoderDigestAlgorithmSHA256 |
| kCMSampleAttachmentKey_AudioIndependentSampleDecoderRefreshCount | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleAttachmentKey_AudioIndependentSampleDecoderRefreshCount |
| kCMSampleAttachmentKey_CryptorSubsampleAuxiliaryData | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleAttachmentKey_CryptorSubsampleAuxiliaryData |
| kCMSampleAttachmentKey_DependsOnOthers | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleAttachmentKey_DependsOnOthers |
| kCMSampleAttachmentKey_DisplayImmediately | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleAttachmentKey_DisplayImmediately |
| kCMSampleAttachmentKey_DoNotDisplay | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleAttachmentKey_DoNotDisplay |
| kCMSampleAttachmentKey_EarlierDisplayTimesAllowed | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleAttachmentKey_EarlierDisplayTimesAllowed |
| kCMSampleAttachmentKey_HDR10PlusPerFrameData | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleAttachmentKey_HDR10PlusPerFrameData |
| kCMSampleAttachmentKey_HEVCStepwiseTemporalSubLayerAccess | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleAttachmentKey_HEVCStepwiseTemporalSubLayerAccess |
| kCMSampleAttachmentKey_HEVCSyncSampleNALUnitType | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleAttachmentKey_HEVCSyncSampleNALUnitType |
| kCMSampleAttachmentKey_HEVCTemporalLevelInfo | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleAttachmentKey_HEVCTemporalLevelInfo |
| kCMSampleAttachmentKey_HEVCTemporalSubLayerAccess | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleAttachmentKey_HEVCTemporalSubLayerAccess |
| kCMSampleAttachmentKey_HasRedundantCoding | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleAttachmentKey_HasRedundantCoding |
| kCMSampleAttachmentKey_IsDependedOnByOthers | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleAttachmentKey_IsDependedOnByOthers |
| kCMSampleAttachmentKey_NotSync | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleAttachmentKey_NotSync |
| kCMSampleAttachmentKey_PartialSync | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleAttachmentKey_PartialSync |
| kCMSampleAttachmentKey_PostDecodeProcessingMetadata | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleAttachmentKey_PostDecodeProcessingMetadata |
| kCMSampleBufferAttachmentKey_CameraIntrinsicMatrix | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferAttachmentKey_CameraIntrinsicMatrix |
| kCMSampleBufferAttachmentKey_DisplayEmptyMediaImmediately | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferAttachmentKey_DisplayEmptyMediaImmediately |
| kCMSampleBufferAttachmentKey_DrainAfterDecoding | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferAttachmentKey_DrainAfterDecoding |
| kCMSampleBufferAttachmentKey_DroppedFrameReason | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferAttachmentKey_DroppedFrameReason |
| kCMSampleBufferAttachmentKey_DroppedFrameReasonInfo | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferAttachmentKey_DroppedFrameReasonInfo |
| kCMSampleBufferAttachmentKey_EmptyMedia | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferAttachmentKey_EmptyMedia |
| kCMSampleBufferAttachmentKey_EndsPreviousSampleDuration | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferAttachmentKey_EndsPreviousSampleDuration |
| kCMSampleBufferAttachmentKey_FillDiscontinuitiesWithSilence | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferAttachmentKey_FillDiscontinuitiesWithSilence |
| kCMSampleBufferAttachmentKey_ForceKeyFrame | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferAttachmentKey_ForceKeyFrame |
| kCMSampleBufferAttachmentKey_GradualDecoderRefresh | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferAttachmentKey_GradualDecoderRefresh |
| kCMSampleBufferAttachmentKey_PermanentEmptyMedia | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferAttachmentKey_PermanentEmptyMedia |
| kCMSampleBufferAttachmentKey_PostNotificationWhenConsumed | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferAttachmentKey_PostNotificationWhenConsumed |
| kCMSampleBufferAttachmentKey_ResetDecoderBeforeDecoding | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferAttachmentKey_ResetDecoderBeforeDecoding |
| kCMSampleBufferAttachmentKey_ResumeOutput | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferAttachmentKey_ResumeOutput |
| kCMSampleBufferAttachmentKey_Reverse | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferAttachmentKey_Reverse |
| kCMSampleBufferAttachmentKey_SampleReferenceByteOffset | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferAttachmentKey_SampleReferenceByteOffset |
| kCMSampleBufferAttachmentKey_SampleReferenceURL | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferAttachmentKey_SampleReferenceURL |
| kCMSampleBufferAttachmentKey_SpeedMultiplier | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferAttachmentKey_SpeedMultiplier |
| kCMSampleBufferAttachmentKey_StillImageLensStabilizationInfo | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferAttachmentKey_StillImageLensStabilizationInfo |
| kCMSampleBufferAttachmentKey_TransitionID | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferAttachmentKey_TransitionID |
| kCMSampleBufferAttachmentKey_TrimDurationAtEnd | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferAttachmentKey_TrimDurationAtEnd |
| kCMSampleBufferAttachmentKey_TrimDurationAtStart | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferAttachmentKey_TrimDurationAtStart |
| kCMSampleBufferConduitNotificationParameter_MaxUpcomingOutputPTS | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferConduitNotificationParameter_MaxUpcomingOutputPTS |
| kCMSampleBufferConduitNotificationParameter_MinUpcomingOutputPTS | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferConduitNotificationParameter_MinUpcomingOutputPTS |
| kCMSampleBufferConduitNotificationParameter_ResumeTag | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferConduitNotificationParameter_ResumeTag |
| kCMSampleBufferConduitNotificationParameter_UpcomingOutputPTSRangeMayOverlapQueuedOutputPTSRange | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferConduitNotificationParameter_UpcomingOutputPTSRangeMayOverlapQueuedOutputPTSRange |
| kCMSampleBufferConduitNotification_InhibitOutputUntil | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferConduitNotification_InhibitOutputUntil |
| kCMSampleBufferConduitNotification_ResetOutput | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferConduitNotification_ResetOutput |
| kCMSampleBufferConduitNotification_UpcomingOutputPTSRangeChanged | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferConduitNotification_UpcomingOutputPTSRangeChanged |
| kCMSampleBufferConsumerNotification_BufferConsumed | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferConsumerNotification_BufferConsumed |
| kCMSampleBufferDroppedFrameReasonInfo_CameraModeSwitch | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferDroppedFrameReasonInfo_CameraModeSwitch |
| kCMSampleBufferDroppedFrameReason_Discontinuity | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferDroppedFrameReason_Discontinuity |
| kCMSampleBufferDroppedFrameReason_FrameWasLate | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferDroppedFrameReason_FrameWasLate |
| kCMSampleBufferDroppedFrameReason_OutOfBuffers | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferDroppedFrameReason_OutOfBuffers |
| kCMSampleBufferLensStabilizationInfo_Active | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferLensStabilizationInfo_Active |
| kCMSampleBufferLensStabilizationInfo_Off | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferLensStabilizationInfo_Off |
| kCMSampleBufferLensStabilizationInfo_OutOfRange | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferLensStabilizationInfo_OutOfRange |
| kCMSampleBufferLensStabilizationInfo_Unavailable | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferLensStabilizationInfo_Unavailable |
| kCMSampleBufferNotificationParameter_OSStatus | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferNotificationParameter_OSStatus |
| kCMSampleBufferNotification_DataBecameReady | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferNotification_DataBecameReady |
| kCMSampleBufferNotification_DataFailed | constant | CoreMedia/CMSampleBuffer.h | raw::kCMSampleBufferNotification_DataFailed |
| kCMSoundDescriptionFlavor_3GPFamily | constant | CoreMedia/CMFormatDescriptionBridge.h | raw::kCMSoundDescriptionFlavor_3GPFamily |
| kCMSoundDescriptionFlavor_ISOFamily | constant | CoreMedia/CMFormatDescriptionBridge.h | raw::kCMSoundDescriptionFlavor_ISOFamily |
| kCMSoundDescriptionFlavor_QuickTimeMovie | constant | CoreMedia/CMFormatDescriptionBridge.h | raw::kCMSoundDescriptionFlavor_QuickTimeMovie |
| kCMSoundDescriptionFlavor_QuickTimeMovieV2 | constant | CoreMedia/CMFormatDescriptionBridge.h | raw::kCMSoundDescriptionFlavor_QuickTimeMovieV2 |
| kCMTagCategoryKey | constant | CoreMedia/CMTag.h | raw::kCMTagCategoryKey |
| kCMTagCollectionTagsArrayKey | constant | CoreMedia/CMTagCollection.h | raw::kCMTagCollectionTagsArrayKey |
| kCMTagDataTypeKey | constant | CoreMedia/CMTag.h | raw::kCMTagDataTypeKey |
| kCMTagInvalid | constant | CoreMedia/CMTag.h | raw::kCMTagInvalid |
| kCMTagMediaSubTypeMebx | constant | CoreMedia/CMTag.h | raw::kCMTagMediaSubTypeMebx |
| kCMTagMediaTypeAudio | constant | CoreMedia/CMTag.h | raw::kCMTagMediaTypeAudio |
| kCMTagMediaTypeMetadata | constant | CoreMedia/CMTag.h | raw::kCMTagMediaTypeMetadata |
| kCMTagMediaTypeVideo | constant | CoreMedia/CMTag.h | raw::kCMTagMediaTypeVideo |
| kCMTagPackingTypeNone | constant | CoreMedia/CMTag.h | raw::kCMTagPackingTypeNone |
| kCMTagPackingTypeOverUnder | constant | CoreMedia/CMTag.h | raw::kCMTagPackingTypeOverUnder |
| kCMTagPackingTypeSideBySide | constant | CoreMedia/CMTag.h | raw::kCMTagPackingTypeSideBySide |
| kCMTagProjectionTypeEquirectangular | constant | CoreMedia/CMTag.h | raw::kCMTagProjectionTypeEquirectangular |
| kCMTagProjectionTypeFisheye | constant | CoreMedia/CMTag.h | raw::kCMTagProjectionTypeFisheye |
| kCMTagProjectionTypeHalfEquirectangular | constant | CoreMedia/CMTag.h | raw::kCMTagProjectionTypeHalfEquirectangular |
| kCMTagProjectionTypeParametricImmersive | constant | CoreMedia/CMTag.h | raw::kCMTagProjectionTypeParametricImmersive |
| kCMTagProjectionTypeRectangular | constant | CoreMedia/CMTag.h | raw::kCMTagProjectionTypeRectangular |
| kCMTagStereoInterpretationOrderReversed | constant | CoreMedia/CMTag.h | raw::kCMTagStereoInterpretationOrderReversed |
| kCMTagStereoLeftAndRightEye | constant | CoreMedia/CMTag.h | raw::kCMTagStereoLeftAndRightEye |
| kCMTagStereoLeftEye | constant | CoreMedia/CMTag.h | raw::kCMTagStereoLeftEye |
| kCMTagStereoNone | constant | CoreMedia/CMTag.h | raw::kCMTagStereoNone |
| kCMTagStereoRightEye | constant | CoreMedia/CMTag.h | raw::kCMTagStereoRightEye |
| kCMTagValueKey | constant | CoreMedia/CMTag.h | raw::kCMTagValueKey |
| kCMTextFormatDescriptionColor_Alpha | constant | CoreMedia/CMFormatDescription.h | raw::kCMTextFormatDescriptionColor_Alpha |
| kCMTextFormatDescriptionColor_Blue | constant | CoreMedia/CMFormatDescription.h | raw::kCMTextFormatDescriptionColor_Blue |
| kCMTextFormatDescriptionColor_Green | constant | CoreMedia/CMFormatDescription.h | raw::kCMTextFormatDescriptionColor_Green |
| kCMTextFormatDescriptionColor_Red | constant | CoreMedia/CMFormatDescription.h | raw::kCMTextFormatDescriptionColor_Red |
| kCMTextFormatDescriptionExtension_BackgroundColor | constant | CoreMedia/CMFormatDescription.h | raw::kCMTextFormatDescriptionExtension_BackgroundColor |
| kCMTextFormatDescriptionExtension_DefaultFontName | constant | CoreMedia/CMFormatDescription.h | raw::kCMTextFormatDescriptionExtension_DefaultFontName |
| kCMTextFormatDescriptionExtension_DefaultStyle | constant | CoreMedia/CMFormatDescription.h | raw::kCMTextFormatDescriptionExtension_DefaultStyle |
| kCMTextFormatDescriptionExtension_DefaultTextBox | constant | CoreMedia/CMFormatDescription.h | raw::kCMTextFormatDescriptionExtension_DefaultTextBox |
| kCMTextFormatDescriptionExtension_DisplayFlags | constant | CoreMedia/CMFormatDescription.h | raw::kCMTextFormatDescriptionExtension_DisplayFlags |
| kCMTextFormatDescriptionExtension_FontTable | constant | CoreMedia/CMFormatDescription.h | raw::kCMTextFormatDescriptionExtension_FontTable |
| kCMTextFormatDescriptionExtension_HorizontalJustification | constant | CoreMedia/CMFormatDescription.h | raw::kCMTextFormatDescriptionExtension_HorizontalJustification |
| kCMTextFormatDescriptionExtension_TextJustification | constant | CoreMedia/CMFormatDescription.h | raw::kCMTextFormatDescriptionExtension_TextJustification |
| kCMTextFormatDescriptionExtension_VerticalJustification | constant | CoreMedia/CMFormatDescription.h | raw::kCMTextFormatDescriptionExtension_VerticalJustification |
| kCMTextFormatDescriptionRect_Bottom | constant | CoreMedia/CMFormatDescription.h | raw::kCMTextFormatDescriptionRect_Bottom |
| kCMTextFormatDescriptionRect_Left | constant | CoreMedia/CMFormatDescription.h | raw::kCMTextFormatDescriptionRect_Left |
| kCMTextFormatDescriptionRect_Right | constant | CoreMedia/CMFormatDescription.h | raw::kCMTextFormatDescriptionRect_Right |
| kCMTextFormatDescriptionRect_Top | constant | CoreMedia/CMFormatDescription.h | raw::kCMTextFormatDescriptionRect_Top |
| kCMTextFormatDescriptionStyle_Ascent | constant | CoreMedia/CMFormatDescription.h | raw::kCMTextFormatDescriptionStyle_Ascent |
| kCMTextFormatDescriptionStyle_EndChar | constant | CoreMedia/CMFormatDescription.h | raw::kCMTextFormatDescriptionStyle_EndChar |
| kCMTextFormatDescriptionStyle_Font | constant | CoreMedia/CMFormatDescription.h | raw::kCMTextFormatDescriptionStyle_Font |
| kCMTextFormatDescriptionStyle_FontFace | constant | CoreMedia/CMFormatDescription.h | raw::kCMTextFormatDescriptionStyle_FontFace |
| kCMTextFormatDescriptionStyle_FontSize | constant | CoreMedia/CMFormatDescription.h | raw::kCMTextFormatDescriptionStyle_FontSize |
| kCMTextFormatDescriptionStyle_ForegroundColor | constant | CoreMedia/CMFormatDescription.h | raw::kCMTextFormatDescriptionStyle_ForegroundColor |
| kCMTextFormatDescriptionStyle_Height | constant | CoreMedia/CMFormatDescription.h | raw::kCMTextFormatDescriptionStyle_Height |
| kCMTextFormatDescriptionStyle_StartChar | constant | CoreMedia/CMFormatDescription.h | raw::kCMTextFormatDescriptionStyle_StartChar |
| kCMTextMarkupAlignmentType_End | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupAlignmentType_End |
| kCMTextMarkupAlignmentType_Left | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupAlignmentType_Left |
| kCMTextMarkupAlignmentType_Middle | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupAlignmentType_Middle |
| kCMTextMarkupAlignmentType_Right | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupAlignmentType_Right |
| kCMTextMarkupAlignmentType_Start | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupAlignmentType_Start |
| kCMTextMarkupAttribute_Alignment | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupAttribute_Alignment |
| kCMTextMarkupAttribute_BackgroundColorARGB | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupAttribute_BackgroundColorARGB |
| kCMTextMarkupAttribute_BaseFontSizePercentageRelativeToVideoHeight | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupAttribute_BaseFontSizePercentageRelativeToVideoHeight |
| kCMTextMarkupAttribute_BoldStyle | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupAttribute_BoldStyle |
| kCMTextMarkupAttribute_CharacterBackgroundColorARGB | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupAttribute_CharacterBackgroundColorARGB |
| kCMTextMarkupAttribute_CharacterEdgeStyle | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupAttribute_CharacterEdgeStyle |
| kCMTextMarkupAttribute_FontFamilyName | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupAttribute_FontFamilyName |
| kCMTextMarkupAttribute_FontFamilyNameList | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupAttribute_FontFamilyNameList |
| kCMTextMarkupAttribute_ForegroundColorARGB | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupAttribute_ForegroundColorARGB |
| kCMTextMarkupAttribute_GenericFontFamilyName | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupAttribute_GenericFontFamilyName |
| kCMTextMarkupAttribute_ItalicStyle | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupAttribute_ItalicStyle |
| kCMTextMarkupAttribute_OrthogonalLinePositionPercentageRelativeToWritingDirection | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupAttribute_OrthogonalLinePositionPercentageRelativeToWritingDirection |
| kCMTextMarkupAttribute_RelativeFontSize | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupAttribute_RelativeFontSize |
| kCMTextMarkupAttribute_TextPositionPercentageRelativeToWritingDirection | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupAttribute_TextPositionPercentageRelativeToWritingDirection |
| kCMTextMarkupAttribute_UnderlineStyle | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupAttribute_UnderlineStyle |
| kCMTextMarkupAttribute_VerticalLayout | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupAttribute_VerticalLayout |
| kCMTextMarkupAttribute_WritingDirectionSizePercentage | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupAttribute_WritingDirectionSizePercentage |
| kCMTextMarkupCharacterEdgeStyle_Depressed | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupCharacterEdgeStyle_Depressed |
| kCMTextMarkupCharacterEdgeStyle_DropShadow | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupCharacterEdgeStyle_DropShadow |
| kCMTextMarkupCharacterEdgeStyle_None | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupCharacterEdgeStyle_None |
| kCMTextMarkupCharacterEdgeStyle_Raised | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupCharacterEdgeStyle_Raised |
| kCMTextMarkupCharacterEdgeStyle_Uniform | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupCharacterEdgeStyle_Uniform |
| kCMTextMarkupGenericFontName_Casual | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupGenericFontName_Casual |
| kCMTextMarkupGenericFontName_Cursive | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupGenericFontName_Cursive |
| kCMTextMarkupGenericFontName_Default | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupGenericFontName_Default |
| kCMTextMarkupGenericFontName_Fantasy | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupGenericFontName_Fantasy |
| kCMTextMarkupGenericFontName_Monospace | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupGenericFontName_Monospace |
| kCMTextMarkupGenericFontName_MonospaceSansSerif | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupGenericFontName_MonospaceSansSerif |
| kCMTextMarkupGenericFontName_MonospaceSerif | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupGenericFontName_MonospaceSerif |
| kCMTextMarkupGenericFontName_ProportionalSansSerif | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupGenericFontName_ProportionalSansSerif |
| kCMTextMarkupGenericFontName_ProportionalSerif | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupGenericFontName_ProportionalSerif |
| kCMTextMarkupGenericFontName_SansSerif | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupGenericFontName_SansSerif |
| kCMTextMarkupGenericFontName_Serif | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupGenericFontName_Serif |
| kCMTextMarkupGenericFontName_SmallCapital | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextMarkupGenericFontName_SmallCapital |
| kCMTextVerticalLayout_LeftToRight | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextVerticalLayout_LeftToRight |
| kCMTextVerticalLayout_RightToLeft | constant | CoreMedia/CMTextMarkup.h | raw::kCMTextVerticalLayout_RightToLeft |
| kCMTimeCodeFormatDescriptionExtension_SourceReferenceName | constant | CoreMedia/CMFormatDescription.h | raw::kCMTimeCodeFormatDescriptionExtension_SourceReferenceName |
| kCMTimeCodeFormatDescriptionKey_LangCode | constant | CoreMedia/CMFormatDescription.h | raw::kCMTimeCodeFormatDescriptionKey_LangCode |
| kCMTimeCodeFormatDescriptionKey_Value | constant | CoreMedia/CMFormatDescription.h | raw::kCMTimeCodeFormatDescriptionKey_Value |
| kCMTimeEpochKey | constant | CoreMedia/CMTime.h | raw::kCMTimeEpochKey |
| kCMTimeFlagsKey | constant | CoreMedia/CMTime.h | raw::kCMTimeFlagsKey |
| kCMTimeIndefinite | constant | CoreMedia/CMTime.h | raw::kCMTimeIndefinite |
| kCMTimeInvalid | constant | CoreMedia/CMTime.h | raw::kCMTimeInvalid |
| kCMTimeMappingInvalid | constant | CoreMedia/CMTimeRange.h | raw::kCMTimeMappingInvalid |
| kCMTimeMappingSourceKey | constant | CoreMedia/CMTimeRange.h | raw::kCMTimeMappingSourceKey |
| kCMTimeMappingTargetKey | constant | CoreMedia/CMTimeRange.h | raw::kCMTimeMappingTargetKey |
| kCMTimeNegativeInfinity | constant | CoreMedia/CMTime.h | raw::kCMTimeNegativeInfinity |
| kCMTimePositiveInfinity | constant | CoreMedia/CMTime.h | raw::kCMTimePositiveInfinity |
| kCMTimeRangeDurationKey | constant | CoreMedia/CMTimeRange.h | raw::kCMTimeRangeDurationKey |
| kCMTimeRangeInvalid | constant | CoreMedia/CMTimeRange.h | raw::kCMTimeRangeInvalid |
| kCMTimeRangeStartKey | constant | CoreMedia/CMTimeRange.h | raw::kCMTimeRangeStartKey |
| kCMTimeRangeZero | constant | CoreMedia/CMTimeRange.h | raw::kCMTimeRangeZero |
| kCMTimeScaleKey | constant | CoreMedia/CMTime.h | raw::kCMTimeScaleKey |
| kCMTimeValueKey | constant | CoreMedia/CMTime.h | raw::kCMTimeValueKey |
| kCMTimeZero | constant | CoreMedia/CMTime.h | raw::kCMTimeZero |
| kCMTimebaseNotificationKey_EventTime | constant | CoreMedia/CMSync.h | raw::kCMTimebaseNotificationKey_EventTime |
| kCMTimebaseNotification_EffectiveRateChanged | constant | CoreMedia/CMSync.h | raw::kCMTimebaseNotification_EffectiveRateChanged |
| kCMTimebaseNotification_TimeJumped | constant | CoreMedia/CMSync.h | raw::kCMTimebaseNotification_TimeJumped |
| kCMTimingInfoInvalid | constant | CoreMedia/CMSampleBuffer.h | raw::kCMTimingInfoInvalid |
| kCVBufferMovieTimeKey | constant | CoreVideo/CVBuffer.h | raw::kCVBufferMovieTimeKey |
| kCVBufferNonPropagatedAttachmentsKey | constant | CoreVideo/CVBuffer.h | raw::kCVBufferNonPropagatedAttachmentsKey |
| kCVBufferPropagatedAttachmentsKey | constant | CoreVideo/CVBuffer.h | raw::kCVBufferPropagatedAttachmentsKey |
| kCVBufferTimeScaleKey | constant | CoreVideo/CVBuffer.h | raw::kCVBufferTimeScaleKey |
| kCVBufferTimeValueKey | constant | CoreVideo/CVBuffer.h | raw::kCVBufferTimeValueKey |
| kCVImageBufferAlphaChannelIsOpaque | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferAlphaChannelIsOpaque |
| kCVImageBufferAlphaChannelModeKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferAlphaChannelModeKey |
| kCVImageBufferAlphaChannelMode_PremultipliedAlpha | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferAlphaChannelMode_PremultipliedAlpha |
| kCVImageBufferAlphaChannelMode_StraightAlpha | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferAlphaChannelMode_StraightAlpha |
| kCVImageBufferAmbientViewingEnvironmentKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferAmbientViewingEnvironmentKey |
| kCVImageBufferCGColorSpaceKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferCGColorSpaceKey |
| kCVImageBufferChromaLocationBottomFieldKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferChromaLocationBottomFieldKey |
| kCVImageBufferChromaLocationTopFieldKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferChromaLocationTopFieldKey |
| kCVImageBufferChromaLocation_Bottom | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferChromaLocation_Bottom |
| kCVImageBufferChromaLocation_BottomLeft | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferChromaLocation_BottomLeft |
| kCVImageBufferChromaLocation_Center | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferChromaLocation_Center |
| kCVImageBufferChromaLocation_DV420 | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferChromaLocation_DV420 |
| kCVImageBufferChromaLocation_Left | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferChromaLocation_Left |
| kCVImageBufferChromaLocation_Top | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferChromaLocation_Top |
| kCVImageBufferChromaLocation_TopLeft | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferChromaLocation_TopLeft |
| kCVImageBufferChromaSubsamplingKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferChromaSubsamplingKey |
| kCVImageBufferChromaSubsampling_411 | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferChromaSubsampling_411 |
| kCVImageBufferChromaSubsampling_420 | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferChromaSubsampling_420 |
| kCVImageBufferChromaSubsampling_422 | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferChromaSubsampling_422 |
| kCVImageBufferCleanApertureHeightKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferCleanApertureHeightKey |
| kCVImageBufferCleanApertureHorizontalOffsetKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferCleanApertureHorizontalOffsetKey |
| kCVImageBufferCleanApertureKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferCleanApertureKey |
| kCVImageBufferCleanApertureVerticalOffsetKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferCleanApertureVerticalOffsetKey |
| kCVImageBufferCleanApertureWidthKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferCleanApertureWidthKey |
| kCVImageBufferColorPrimariesKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferColorPrimariesKey |
| kCVImageBufferColorPrimaries_DCI_P3 | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferColorPrimaries_DCI_P3 |
| kCVImageBufferColorPrimaries_EBU_3213 | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferColorPrimaries_EBU_3213 |
| kCVImageBufferColorPrimaries_ITU_R_2020 | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferColorPrimaries_ITU_R_2020 |
| kCVImageBufferColorPrimaries_ITU_R_709_2 | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferColorPrimaries_ITU_R_709_2 |
| kCVImageBufferColorPrimaries_P22 | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferColorPrimaries_P22 |
| kCVImageBufferColorPrimaries_P3_D65 | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferColorPrimaries_P3_D65 |
| kCVImageBufferColorPrimaries_SMPTE_C | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferColorPrimaries_SMPTE_C |
| kCVImageBufferContentLightLevelInfoKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferContentLightLevelInfoKey |
| kCVImageBufferDisplayDimensionsKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferDisplayDimensionsKey |
| kCVImageBufferDisplayHeightKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferDisplayHeightKey |
| kCVImageBufferDisplayMaskRectangleKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferDisplayMaskRectangleKey |
| kCVImageBufferDisplayMaskRectangleStereoLeftKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferDisplayMaskRectangleStereoLeftKey |
| kCVImageBufferDisplayMaskRectangleStereoRightKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferDisplayMaskRectangleStereoRightKey |
| kCVImageBufferDisplayMaskRectangle_LeftEdgePointsKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferDisplayMaskRectangle_LeftEdgePointsKey |
| kCVImageBufferDisplayMaskRectangle_RectangleHeightKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferDisplayMaskRectangle_RectangleHeightKey |
| kCVImageBufferDisplayMaskRectangle_RectangleLeftKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferDisplayMaskRectangle_RectangleLeftKey |
| kCVImageBufferDisplayMaskRectangle_RectangleTopKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferDisplayMaskRectangle_RectangleTopKey |
| kCVImageBufferDisplayMaskRectangle_RectangleWidthKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferDisplayMaskRectangle_RectangleWidthKey |
| kCVImageBufferDisplayMaskRectangle_ReferenceRasterHeightKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferDisplayMaskRectangle_ReferenceRasterHeightKey |
| kCVImageBufferDisplayMaskRectangle_ReferenceRasterWidthKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferDisplayMaskRectangle_ReferenceRasterWidthKey |
| kCVImageBufferDisplayMaskRectangle_RightEdgePointsKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferDisplayMaskRectangle_RightEdgePointsKey |
| kCVImageBufferDisplayWidthKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferDisplayWidthKey |
| kCVImageBufferFieldCountKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferFieldCountKey |
| kCVImageBufferFieldDetailKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferFieldDetailKey |
| kCVImageBufferFieldDetailSpatialFirstLineEarly | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferFieldDetailSpatialFirstLineEarly |
| kCVImageBufferFieldDetailSpatialFirstLineLate | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferFieldDetailSpatialFirstLineLate |
| kCVImageBufferFieldDetailTemporalBottomFirst | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferFieldDetailTemporalBottomFirst |
| kCVImageBufferFieldDetailTemporalTopFirst | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferFieldDetailTemporalTopFirst |
| kCVImageBufferGammaLevelKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferGammaLevelKey |
| kCVImageBufferICCProfileKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferICCProfileKey |
| kCVImageBufferLogTransferFunctionKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferLogTransferFunctionKey |
| kCVImageBufferLogTransferFunction_AppleLog | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferLogTransferFunction_AppleLog |
| kCVImageBufferLogTransferFunction_AppleLog2 | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferLogTransferFunction_AppleLog2 |
| kCVImageBufferMasteringDisplayColorVolumeKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferMasteringDisplayColorVolumeKey |
| kCVImageBufferPixelAspectRatioHorizontalSpacingKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferPixelAspectRatioHorizontalSpacingKey |
| kCVImageBufferPixelAspectRatioKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferPixelAspectRatioKey |
| kCVImageBufferPixelAspectRatioVerticalSpacingKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferPixelAspectRatioVerticalSpacingKey |
| kCVImageBufferPostDecodeProcessingFrameMetadataKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferPostDecodeProcessingFrameMetadataKey |
| kCVImageBufferPostDecodeProcessingSequenceMetadataKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferPostDecodeProcessingSequenceMetadataKey |
| kCVImageBufferPreferredCleanApertureKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferPreferredCleanApertureKey |
| kCVImageBufferRegionOfInterestKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferRegionOfInterestKey |
| kCVImageBufferSceneIlluminationKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferSceneIlluminationKey |
| kCVImageBufferTransferFunctionKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferTransferFunctionKey |
| kCVImageBufferTransferFunction_ITU_R_2100_HLG | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferTransferFunction_ITU_R_2100_HLG |
| kCVImageBufferTransferFunction_ITU_R_709_2 | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferTransferFunction_ITU_R_709_2 |
| kCVImageBufferTransferFunction_Linear | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferTransferFunction_Linear |
| kCVImageBufferTransferFunction_SMPTE_ST_2084_PQ | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferTransferFunction_SMPTE_ST_2084_PQ |
| kCVImageBufferTransferFunction_SMPTE_ST_428_1 | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferTransferFunction_SMPTE_ST_428_1 |
| kCVImageBufferYCbCrMatrixKey | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferYCbCrMatrixKey |
| kCVImageBufferYCbCrMatrix_ITU_R_709_2 | constant | CoreVideo/CVImageBuffer.h | raw::kCVImageBufferYCbCrMatrix_ITU_R_709_2 |
| kCVIndefiniteTime | constant | CoreVideo/CVBase.h | raw::kCVIndefiniteTime |
| kCVMetalBufferCacheMaximumBufferAgeKey | constant | CoreVideo/CVMetalBufferCache.h | raw::kCVMetalBufferCacheMaximumBufferAgeKey |
| kCVMetalTextureCacheMaximumTextureAgeKey | constant | CoreVideo/CVMetalTextureCache.h | raw::kCVMetalTextureCacheMaximumTextureAgeKey |
| kCVMetalTextureStorageMode | constant | CoreVideo/CVMetalTexture.h | raw::kCVMetalTextureStorageMode |
| kCVMetalTextureUsage | constant | CoreVideo/CVMetalTexture.h | raw::kCVMetalTextureUsage |
| kCVOpenGLBufferHeight | constant | CoreVideo/CVOpenGLBuffer.h | raw::kCVOpenGLBufferHeight |
| kCVOpenGLBufferInternalFormat | constant | CoreVideo/CVOpenGLBuffer.h | raw::kCVOpenGLBufferInternalFormat |
| kCVOpenGLBufferMaximumMipmapLevel | constant | CoreVideo/CVOpenGLBuffer.h | raw::kCVOpenGLBufferMaximumMipmapLevel |
| kCVOpenGLBufferPoolMaximumBufferAgeKey | constant | CoreVideo/CVOpenGLBufferPool.h | raw::kCVOpenGLBufferPoolMaximumBufferAgeKey |
| kCVOpenGLBufferPoolMinimumBufferCountKey | constant | CoreVideo/CVOpenGLBufferPool.h | raw::kCVOpenGLBufferPoolMinimumBufferCountKey |
| kCVOpenGLBufferTarget | constant | CoreVideo/CVOpenGLBuffer.h | raw::kCVOpenGLBufferTarget |
| kCVOpenGLBufferWidth | constant | CoreVideo/CVOpenGLBuffer.h | raw::kCVOpenGLBufferWidth |
| kCVOpenGLTextureCacheChromaSamplingModeAutomatic | constant | CoreVideo/CVOpenGLTextureCache.h | raw::kCVOpenGLTextureCacheChromaSamplingModeAutomatic |
| kCVOpenGLTextureCacheChromaSamplingModeBestPerformance | constant | CoreVideo/CVOpenGLTextureCache.h | raw::kCVOpenGLTextureCacheChromaSamplingModeBestPerformance |
| kCVOpenGLTextureCacheChromaSamplingModeHighestQuality | constant | CoreVideo/CVOpenGLTextureCache.h | raw::kCVOpenGLTextureCacheChromaSamplingModeHighestQuality |
| kCVOpenGLTextureCacheChromaSamplingModeKey | constant | CoreVideo/CVOpenGLTextureCache.h | raw::kCVOpenGLTextureCacheChromaSamplingModeKey |
| kCVPixelBufferBytesPerRowAlignmentKey | constant | CoreVideo/CVPixelBuffer.h | raw::kCVPixelBufferBytesPerRowAlignmentKey |
| kCVPixelBufferCGBitmapContextCompatibilityKey | constant | CoreVideo/CVPixelBuffer.h | raw::kCVPixelBufferCGBitmapContextCompatibilityKey |
| kCVPixelBufferCGImageCompatibilityKey | constant | CoreVideo/CVPixelBuffer.h | raw::kCVPixelBufferCGImageCompatibilityKey |
| kCVPixelBufferExtendedPixelsBottomKey | constant | CoreVideo/CVPixelBuffer.h | raw::kCVPixelBufferExtendedPixelsBottomKey |
| kCVPixelBufferExtendedPixelsLeftKey | constant | CoreVideo/CVPixelBuffer.h | raw::kCVPixelBufferExtendedPixelsLeftKey |
| kCVPixelBufferExtendedPixelsRightKey | constant | CoreVideo/CVPixelBuffer.h | raw::kCVPixelBufferExtendedPixelsRightKey |
| kCVPixelBufferExtendedPixelsTopKey | constant | CoreVideo/CVPixelBuffer.h | raw::kCVPixelBufferExtendedPixelsTopKey |
| kCVPixelBufferHeightKey | constant | CoreVideo/CVPixelBuffer.h | raw::kCVPixelBufferHeightKey |
| kCVPixelBufferIOSurfaceCoreAnimationCompatibilityKey | constant | CoreVideo/CVPixelBufferIOSurface.h | raw::kCVPixelBufferIOSurfaceCoreAnimationCompatibilityKey |
| kCVPixelBufferIOSurfaceOpenGLFBOCompatibilityKey | constant | CoreVideo/CVPixelBufferIOSurface.h | raw::kCVPixelBufferIOSurfaceOpenGLFBOCompatibilityKey |
| kCVPixelBufferIOSurfaceOpenGLTextureCompatibilityKey | constant | CoreVideo/CVPixelBufferIOSurface.h | raw::kCVPixelBufferIOSurfaceOpenGLTextureCompatibilityKey |
| kCVPixelBufferIOSurfacePurgeableKey | constant | CoreVideo/CVPixelBuffer.h | raw::kCVPixelBufferIOSurfacePurgeableKey |
| kCVPixelBufferMemoryAllocatorKey | constant | CoreVideo/CVPixelBuffer.h | raw::kCVPixelBufferMemoryAllocatorKey |
| kCVPixelBufferOpenGLCompatibilityKey | constant | CoreVideo/CVPixelBuffer.h | raw::kCVPixelBufferOpenGLCompatibilityKey |
| kCVPixelBufferPixelFormatTypeKey | constant | CoreVideo/CVPixelBuffer.h | raw::kCVPixelBufferPixelFormatTypeKey |
| kCVPixelBufferPoolAllocationThresholdKey | constant | CoreVideo/CVPixelBufferPool.h | raw::kCVPixelBufferPoolAllocationThresholdKey |
| kCVPixelBufferPoolFreeBufferNotification | constant | CoreVideo/CVPixelBufferPool.h | raw::kCVPixelBufferPoolFreeBufferNotification |
| kCVPixelBufferPoolMaximumBufferAgeKey | constant | CoreVideo/CVPixelBufferPool.h | raw::kCVPixelBufferPoolMaximumBufferAgeKey |
| kCVPixelBufferPoolMinimumBufferCountKey | constant | CoreVideo/CVPixelBufferPool.h | raw::kCVPixelBufferPoolMinimumBufferCountKey |
| kCVPixelBufferProResRAWKey_BlackLevel | constant | CoreVideo/CVPixelBuffer.h | raw::kCVPixelBufferProResRAWKey_BlackLevel |
| kCVPixelBufferProResRAWKey_ColorMatrix | constant | CoreVideo/CVPixelBuffer.h | raw::kCVPixelBufferProResRAWKey_ColorMatrix |
| kCVPixelBufferProResRAWKey_GainFactor | constant | CoreVideo/CVPixelBuffer.h | raw::kCVPixelBufferProResRAWKey_GainFactor |
| kCVPixelBufferProResRAWKey_MetadataExtension | constant | CoreVideo/CVPixelBuffer.h | raw::kCVPixelBufferProResRAWKey_MetadataExtension |
| kCVPixelBufferProResRAWKey_RecommendedCrop | constant | CoreVideo/CVPixelBuffer.h | raw::kCVPixelBufferProResRAWKey_RecommendedCrop |
| kCVPixelBufferProResRAWKey_SenselSitingOffsets | constant | CoreVideo/CVPixelBuffer.h | raw::kCVPixelBufferProResRAWKey_SenselSitingOffsets |
| kCVPixelBufferProResRAWKey_WhiteBalanceBlueFactor | constant | CoreVideo/CVPixelBuffer.h | raw::kCVPixelBufferProResRAWKey_WhiteBalanceBlueFactor |
| kCVPixelBufferProResRAWKey_WhiteBalanceCCT | constant | CoreVideo/CVPixelBuffer.h | raw::kCVPixelBufferProResRAWKey_WhiteBalanceCCT |
| kCVPixelBufferProResRAWKey_WhiteBalanceRedFactor | constant | CoreVideo/CVPixelBuffer.h | raw::kCVPixelBufferProResRAWKey_WhiteBalanceRedFactor |
| kCVPixelBufferProResRAWKey_WhiteLevel | constant | CoreVideo/CVPixelBuffer.h | raw::kCVPixelBufferProResRAWKey_WhiteLevel |
| kCVPixelBufferVersatileBayerKey_BayerPattern | constant | CoreVideo/CVPixelBuffer.h | raw::kCVPixelBufferVersatileBayerKey_BayerPattern |
| kCVPixelBufferWidthKey | constant | CoreVideo/CVPixelBuffer.h | raw::kCVPixelBufferWidthKey |
| kCVPixelFormatBitsPerBlock | constant | CoreVideo/CVPixelFormatDescription.h | raw::kCVPixelFormatBitsPerBlock |
| kCVPixelFormatBitsPerComponent | constant | CoreVideo/CVPixelFormatDescription.h | raw::kCVPixelFormatBitsPerComponent |
| kCVPixelFormatBlackBlock | constant | CoreVideo/CVPixelFormatDescription.h | raw::kCVPixelFormatBlackBlock |
| kCVPixelFormatBlockHeight | constant | CoreVideo/CVPixelFormatDescription.h | raw::kCVPixelFormatBlockHeight |
| kCVPixelFormatBlockHorizontalAlignment | constant | CoreVideo/CVPixelFormatDescription.h | raw::kCVPixelFormatBlockHorizontalAlignment |
| kCVPixelFormatBlockVerticalAlignment | constant | CoreVideo/CVPixelFormatDescription.h | raw::kCVPixelFormatBlockVerticalAlignment |
| kCVPixelFormatBlockWidth | constant | CoreVideo/CVPixelFormatDescription.h | raw::kCVPixelFormatBlockWidth |
| kCVPixelFormatCGBitmapContextCompatibility | constant | CoreVideo/CVPixelFormatDescription.h | raw::kCVPixelFormatCGBitmapContextCompatibility |
| kCVPixelFormatCGBitmapInfo | constant | CoreVideo/CVPixelFormatDescription.h | raw::kCVPixelFormatCGBitmapInfo |
| kCVPixelFormatCodecType | constant | CoreVideo/CVPixelFormatDescription.h | raw::kCVPixelFormatCodecType |
| kCVPixelFormatComponentRange | constant | CoreVideo/CVPixelFormatDescription.h | raw::kCVPixelFormatComponentRange |
| kCVPixelFormatComponentRange_FullRange | constant | CoreVideo/CVPixelFormatDescription.h | raw::kCVPixelFormatComponentRange_FullRange |
| kCVPixelFormatComponentRange_VideoRange | constant | CoreVideo/CVPixelFormatDescription.h | raw::kCVPixelFormatComponentRange_VideoRange |
| kCVPixelFormatComponentRange_WideRange | constant | CoreVideo/CVPixelFormatDescription.h | raw::kCVPixelFormatComponentRange_WideRange |
| kCVPixelFormatConstant | constant | CoreVideo/CVPixelFormatDescription.h | raw::kCVPixelFormatConstant |
| kCVPixelFormatContainsAlpha | constant | CoreVideo/CVPixelFormatDescription.h | raw::kCVPixelFormatContainsAlpha |
| kCVPixelFormatContainsGrayscale | constant | CoreVideo/CVPixelFormatDescription.h | raw::kCVPixelFormatContainsGrayscale |
| kCVPixelFormatContainsRGB | constant | CoreVideo/CVPixelFormatDescription.h | raw::kCVPixelFormatContainsRGB |
| kCVPixelFormatContainsSenselArray | constant | CoreVideo/CVPixelFormatDescription.h | raw::kCVPixelFormatContainsSenselArray |
| kCVPixelFormatContainsYCbCr | constant | CoreVideo/CVPixelFormatDescription.h | raw::kCVPixelFormatContainsYCbCr |
| kCVPixelFormatFillExtendedPixelsCallback | constant | CoreVideo/CVPixelFormatDescription.h | raw::kCVPixelFormatFillExtendedPixelsCallback |
| kCVPixelFormatFourCC | constant | CoreVideo/CVPixelFormatDescription.h | raw::kCVPixelFormatFourCC |
| kCVPixelFormatHorizontalSubsampling | constant | CoreVideo/CVPixelFormatDescription.h | raw::kCVPixelFormatHorizontalSubsampling |
| kCVPixelFormatName | constant | CoreVideo/CVPixelFormatDescription.h | raw::kCVPixelFormatName |
| kCVPixelFormatOpenGLFormat | constant | CoreVideo/CVPixelFormatDescription.h | raw::kCVPixelFormatOpenGLFormat |
| kCVPixelFormatOpenGLInternalFormat | constant | CoreVideo/CVPixelFormatDescription.h | raw::kCVPixelFormatOpenGLInternalFormat |
| kCVPixelFormatOpenGLType | constant | CoreVideo/CVPixelFormatDescription.h | raw::kCVPixelFormatOpenGLType |
| kCVPixelFormatPlanes | constant | CoreVideo/CVPixelFormatDescription.h | raw::kCVPixelFormatPlanes |
| kCVPixelFormatQDCompatibility | constant | CoreVideo/CVPixelFormatDescription.h | raw::kCVPixelFormatQDCompatibility |
| kCVPixelFormatVerticalSubsampling | constant | CoreVideo/CVPixelFormatDescription.h | raw::kCVPixelFormatVerticalSubsampling |
| kCVZeroTime | constant | CoreVideo/CVBase.h | raw::kCVZeroTime |
| kIOSurfaceAllocSize | constant | IOSurface/IOSurfaceRef.h | raw::kIOSurfaceAllocSize |
| kIOSurfaceBytesPerElement | constant | IOSurface/IOSurfaceRef.h | raw::kIOSurfaceBytesPerElement |
| kIOSurfaceBytesPerRow | constant | IOSurface/IOSurfaceRef.h | raw::kIOSurfaceBytesPerRow |
| kIOSurfaceCacheMode | constant | IOSurface/IOSurfaceRef.h | raw::kIOSurfaceCacheMode |
| kIOSurfaceColorSpace | constant | IOSurface/IOSurfaceRef.h | raw::kIOSurfaceColorSpace |
| kIOSurfaceContentHeadroom | constant | IOSurface/IOSurfaceRef.h | raw::kIOSurfaceContentHeadroom |
| kIOSurfaceElementHeight | constant | IOSurface/IOSurfaceRef.h | raw::kIOSurfaceElementHeight |
| kIOSurfaceElementWidth | constant | IOSurface/IOSurfaceRef.h | raw::kIOSurfaceElementWidth |
| kIOSurfaceHeight | constant | IOSurface/IOSurfaceRef.h | raw::kIOSurfaceHeight |
| kIOSurfaceICCProfile | constant | IOSurface/IOSurfaceRef.h | raw::kIOSurfaceICCProfile |
| kIOSurfaceName | constant | IOSurface/IOSurfaceRef.h | raw::kIOSurfaceName |
| kIOSurfaceOffset | constant | IOSurface/IOSurfaceRef.h | raw::kIOSurfaceOffset |
| kIOSurfacePixelFormat | constant | IOSurface/IOSurfaceRef.h | raw::kIOSurfacePixelFormat |
| kIOSurfacePixelSizeCastingAllowed | constant | IOSurface/IOSurfaceRef.h | raw::kIOSurfacePixelSizeCastingAllowed |
| kIOSurfacePlaneBase | constant | IOSurface/IOSurfaceRef.h | raw::kIOSurfacePlaneBase |
| kIOSurfacePlaneBitsPerElement | constant | IOSurface/IOSurfaceRef.h | raw::kIOSurfacePlaneBitsPerElement |
| kIOSurfacePlaneBytesPerElement | constant | IOSurface/IOSurfaceRef.h | raw::kIOSurfacePlaneBytesPerElement |
| kIOSurfacePlaneBytesPerRow | constant | IOSurface/IOSurfaceRef.h | raw::kIOSurfacePlaneBytesPerRow |
| kIOSurfacePlaneComponentBitDepths | constant | IOSurface/IOSurfaceRef.h | raw::kIOSurfacePlaneComponentBitDepths |
| kIOSurfacePlaneComponentBitOffsets | constant | IOSurface/IOSurfaceRef.h | raw::kIOSurfacePlaneComponentBitOffsets |
| kIOSurfacePlaneComponentNames | constant | IOSurface/IOSurfaceRef.h | raw::kIOSurfacePlaneComponentNames |
| kIOSurfacePlaneComponentRanges | constant | IOSurface/IOSurfaceRef.h | raw::kIOSurfacePlaneComponentRanges |
| kIOSurfacePlaneComponentTypes | constant | IOSurface/IOSurfaceRef.h | raw::kIOSurfacePlaneComponentTypes |
| kIOSurfacePlaneElementHeight | constant | IOSurface/IOSurfaceRef.h | raw::kIOSurfacePlaneElementHeight |
| kIOSurfacePlaneElementWidth | constant | IOSurface/IOSurfaceRef.h | raw::kIOSurfacePlaneElementWidth |
| kIOSurfacePlaneHeight | constant | IOSurface/IOSurfaceRef.h | raw::kIOSurfacePlaneHeight |
| kIOSurfacePlaneInfo | constant | IOSurface/IOSurfaceRef.h | raw::kIOSurfacePlaneInfo |
| kIOSurfacePlaneOffset | constant | IOSurface/IOSurfaceRef.h | raw::kIOSurfacePlaneOffset |
| kIOSurfacePlaneSize | constant | IOSurface/IOSurfaceRef.h | raw::kIOSurfacePlaneSize |
| kIOSurfacePlaneWidth | constant | IOSurface/IOSurfaceRef.h | raw::kIOSurfacePlaneWidth |
| kIOSurfaceSubsampling | constant | IOSurface/IOSurfaceRef.h | raw::kIOSurfaceSubsampling |
| kIOSurfaceWidth | constant | IOSurface/IOSurfaceRef.h | raw::kIOSurfaceWidth |

## 🔴 GAPS
None — all non-exempt declarations are covered by the existing safe wrappers or `apple_cf::raw`.

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason |
| --- | --- | --- | --- |
| CFAbsoluteTimeAddGregorianUnits | function | CoreFoundation/CFDate.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFAbsoluteTimeGetDayOfWeek | function | CoreFoundation/CFDate.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFAbsoluteTimeGetDayOfYear | function | CoreFoundation/CFDate.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFAbsoluteTimeGetDifferenceAsGregorianUnits | function | CoreFoundation/CFDate.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFAbsoluteTimeGetGregorianDate | function | CoreFoundation/CFDate.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFAbsoluteTimeGetWeekOfYear | function | CoreFoundation/CFDate.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFBundleCloseBundleResourceMap | function | CoreFoundation/CFBundle.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFBundleOpenBundleResourceFiles | function | CoreFoundation/CFBundle.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFBundleOpenBundleResourceMap | function | CoreFoundation/CFBundle.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFGregorianDate | typedef struct | CoreFoundation/CFDate.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFGregorianDateGetAbsoluteTime | function | CoreFoundation/CFDate.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFGregorianDateIsValid | function | CoreFoundation/CFDate.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFGregorianUnitFlags | typedef enum | CoreFoundation/CFDate.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFGregorianUnits | typedef struct | CoreFoundation/CFDate.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFPreferencesCopyApplicationList | function | CoreFoundation/CFPreferences.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFPropertyListCreateFromStream | function | CoreFoundation/CFPropertyList.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFPropertyListCreateFromXMLData | function | CoreFoundation/CFPropertyList.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFPropertyListCreateXMLData | function | CoreFoundation/CFPropertyList.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFPropertyListWriteToStream | function | CoreFoundation/CFPropertyList.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFStreamCreatePairWithPeerSocketSignature | function | CoreFoundation/CFStream.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFStreamCreatePairWithSocket | function | CoreFoundation/CFStream.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFStreamCreatePairWithSocketToHost | function | CoreFoundation/CFStream.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFSwap | struct | ? | Private helper union inside CFByteOrder inline conversions (`CFByteOrder.h:207-210`, `279-282`); not a standalone public API. |
| CFURLCopyParameterString | function | CoreFoundation/CFURL.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFURLCopyQueryString | function | CoreFoundation/CFURL.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFURLCreateBookmarkDataFromAliasRecord | function | CoreFoundation/CFURL.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFURLCreateDataAndPropertiesFromResource | function | CoreFoundation/CFURLAccess.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFURLCreateFromFSRef | function | CoreFoundation/CFURL.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFURLCreatePropertyFromResource | function | CoreFoundation/CFURLAccess.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFURLCreateStringByAddingPercentEscapes | function | CoreFoundation/CFURL.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFURLCreateStringByReplacingPercentEscapesUsingEncoding | function | CoreFoundation/CFURL.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFURLDestroyResource | function | CoreFoundation/CFURLAccess.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFURLEnumeratorGetSourceDidChange | function | CoreFoundation/CFURLEnumerator.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFURLGetFSRef | function | CoreFoundation/CFURL.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFURLPathStyle | typedef enum | CoreFoundation/CFURL.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFURLWriteDataAndPropertiesToResource | function | CoreFoundation/CFURLAccess.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFXMLNodeCreate | function | CoreFoundation/CFXMLNode.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFXMLNodeCreateCopy | function | CoreFoundation/CFXMLNode.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFXMLNodeGetInfoPtr | function | CoreFoundation/CFXMLNode.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFXMLNodeGetString | function | CoreFoundation/CFXMLNode.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFXMLNodeGetTypeCode | function | CoreFoundation/CFXMLNode.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFXMLNodeGetTypeID | function | CoreFoundation/CFXMLNode.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFXMLNodeGetVersion | function | CoreFoundation/CFXMLNode.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFXMLParserAbort | function | CoreFoundation/CFXMLParser.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFXMLParserCopyErrorDescription | function | CoreFoundation/CFXMLParser.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFXMLParserCreate | function | CoreFoundation/CFXMLParser.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFXMLParserCreateWithDataFromURL | function | CoreFoundation/CFXMLParser.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFXMLParserGetCallBacks | function | CoreFoundation/CFXMLParser.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFXMLParserGetContext | function | CoreFoundation/CFXMLParser.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFXMLParserGetDocument | function | CoreFoundation/CFXMLParser.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFXMLParserGetLineNumber | function | CoreFoundation/CFXMLParser.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFXMLParserGetLocation | function | CoreFoundation/CFXMLParser.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFXMLParserGetSourceURL | function | CoreFoundation/CFXMLParser.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFXMLParserGetStatusCode | function | CoreFoundation/CFXMLParser.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFXMLParserGetTypeID | function | CoreFoundation/CFXMLParser.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFXMLParserParse | function | CoreFoundation/CFXMLParser.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFXMLTreeCreateFromData | function | CoreFoundation/CFXMLParser.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFXMLTreeCreateFromDataWithError | function | CoreFoundation/CFXMLParser.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFXMLTreeCreateWithDataFromURL | function | CoreFoundation/CFXMLParser.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFXMLTreeCreateWithNode | function | CoreFoundation/CFXMLNode.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFXMLTreeCreateXMLData | function | CoreFoundation/CFXMLParser.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CFXMLTreeGetNode | function | CoreFoundation/CFXMLNode.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CMTimebaseCopyMaster | function | CoreMedia/CMSync.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CMTimebaseCopyMasterClock | function | CoreMedia/CMSync.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CMTimebaseCopyMasterTimebase | function | CoreMedia/CMSync.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CMTimebaseCopyUltimateMasterClock | function | CoreMedia/CMSync.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CMTimebaseGetMaster | function | CoreMedia/CMSync.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CMTimebaseGetMasterClock | function | CoreMedia/CMSync.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CMTimebaseGetMasterTimebase | function | CoreMedia/CMSync.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CMTimebaseGetUltimateMasterClock | function | CoreMedia/CMSync.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CVBufferGetAttachment | function | CoreVideo/CVBuffer.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CVBufferGetAttachments | function | CoreVideo/CVBuffer.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CVDisplayLinkCreateWithActiveCGDisplays | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CVDisplayLinkCreateWithCGDisplay | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CVDisplayLinkCreateWithCGDisplays | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CVDisplayLinkCreateWithOpenGLDisplayMask | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CVDisplayLinkGetActualOutputVideoRefreshPeriod | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CVDisplayLinkGetCurrentCGDisplay | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CVDisplayLinkGetCurrentTime | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CVDisplayLinkGetNominalOutputVideoRefreshPeriod | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CVDisplayLinkGetOutputVideoLatency | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CVDisplayLinkGetTypeID | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CVDisplayLinkIsRunning | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CVDisplayLinkRef | typedef struct | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CVDisplayLinkRelease | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CVDisplayLinkRetain | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CVDisplayLinkSetCurrentCGDisplay | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CVDisplayLinkSetCurrentCGDisplayFromOpenGLContext | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CVDisplayLinkSetOutputCallback | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CVDisplayLinkSetOutputHandler | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CVDisplayLinkStart | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CVDisplayLinkStop | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CVDisplayLinkTranslateTime | function | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| CVTimeStamp | typedef struct | CoreVideo/CVDisplayLink.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| OS_dispatch_data | protocol | ? | Objective-C OS_OBJECT protocol marker from dispatch headers (`dispatch/object.h:155-165`); Rust binds the C ABI via `dispatch_data_t` instead. |
| OS_dispatch_group | protocol | ? | Objective-C OS_OBJECT protocol marker from dispatch headers (`dispatch/object.h:155-165`); Rust binds the C ABI via `dispatch_group_t` instead. |
| OS_dispatch_io | protocol | ? | Objective-C OS_OBJECT protocol marker from dispatch headers (`dispatch/object.h:155-165`); Rust binds the C ABI via `dispatch_io_t` instead. |
| OS_dispatch_object | protocol | ? | Objective-C OS_OBJECT protocol marker from dispatch headers (`dispatch/object.h:155-165`); Rust binds the C ABI via `dispatch_object_t` instead. |
| OS_dispatch_queue | protocol | ? | Objective-C OS_OBJECT protocol marker from dispatch headers (`dispatch/object.h:155-165`); Rust binds the C ABI via `dispatch_queue_t` instead. |
| OS_dispatch_queue_attr | protocol | ? | Objective-C OS_OBJECT protocol marker from dispatch headers (`dispatch/object.h:155-165`); Rust binds the C ABI via `dispatch_queue_attr_t` instead. |
| OS_dispatch_queue_concurrent | protocol | ? | Objective-C OS_OBJECT protocol marker from dispatch headers (`dispatch/object.h:155-165`); Rust binds the C ABI via `dispatch_queue_concurrent_t` instead. |
| OS_dispatch_queue_global | protocol | ? | Objective-C OS_OBJECT protocol marker from dispatch headers (`dispatch/object.h:155-165`); Rust binds the C ABI via `dispatch_queue_global_t` instead. |
| OS_dispatch_queue_main | protocol | ? | Objective-C OS_OBJECT protocol marker from dispatch headers (`dispatch/object.h:155-165`); Rust binds the C ABI via `dispatch_queue_main_t` instead. |
| OS_dispatch_queue_serial | protocol | ? | Objective-C OS_OBJECT protocol marker from dispatch headers (`dispatch/object.h:155-165`); Rust binds the C ABI via `dispatch_queue_serial_t` instead. |
| OS_dispatch_queue_serial_executor | protocol | ? | Objective-C OS_OBJECT protocol marker from dispatch headers (`dispatch/object.h:155-165`); Rust binds the C ABI via `dispatch_queue_serial_executor_t` instead. |
| OS_dispatch_semaphore | protocol | ? | Objective-C OS_OBJECT protocol marker from dispatch headers (`dispatch/object.h:155-165`); Rust binds the C ABI via `dispatch_semaphore_t` instead. |
| OS_dispatch_source | protocol | Dispatch/object.h | Objective-C OS_OBJECT protocol marker from dispatch headers (`dispatch/object.h:155-165`); Rust binds the C ABI via `dispatch_source_t` instead. |
| OS_dispatch_workloop | protocol | ? | Objective-C OS_OBJECT protocol marker from dispatch headers (`dispatch/object.h:155-165`); Rust binds the C ABI via `dispatch_workloop_t` instead. |
| kCFStreamSocketSecurityLevelSSLv2 | constant | CoreFoundation/CFStream.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| kCFStreamSocketSecurityLevelSSLv3 | constant | CoreFoundation/CFStream.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| kCFURLCustomIconKey | constant | CoreFoundation/CFURL.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| kCFURLEffectiveIconKey | constant | CoreFoundation/CFURL.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| kCFURLFileDirectoryContents | constant | CoreFoundation/CFURLAccess.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| kCFURLFileExists | constant | CoreFoundation/CFURLAccess.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| kCFURLFileLastModificationTime | constant | CoreFoundation/CFURLAccess.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| kCFURLFileLength | constant | CoreFoundation/CFURLAccess.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| kCFURLFileOwnerID | constant | CoreFoundation/CFURLAccess.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| kCFURLFilePOSIXMode | constant | CoreFoundation/CFURLAccess.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| kCFURLHTTPStatusCode | constant | CoreFoundation/CFURLAccess.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| kCFURLHTTPStatusLine | constant | CoreFoundation/CFURLAccess.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| kCFURLLabelColorKey | constant | CoreFoundation/CFURL.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| kCFURLTypeIdentifierKey | constant | CoreFoundation/CFURL.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| kCFURLUbiquitousItemDownloadingStatusKey | constant | CoreFoundation/CFURL.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| kCFURLUbiquitousItemIsDownloadedKey | constant | CoreFoundation/CFURL.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| kCFURLUbiquitousItemPercentDownloadedKey | constant | CoreFoundation/CFURL.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| kCFURLUbiquitousItemPercentUploadedKey | constant | CoreFoundation/CFURL.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| kCVImageBufferTransferFunction_EBU_3213 | constant | CoreVideo/CVImageBuffer.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| kCVImageBufferTransferFunction_ITU_R_2020 | constant | CoreVideo/CVImageBuffer.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| kCVImageBufferTransferFunction_SMPTE_240M_1995 | constant | CoreVideo/CVImageBuffer.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| kCVImageBufferTransferFunction_SMPTE_C | constant | CoreVideo/CVImageBuffer.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| kCVImageBufferTransferFunction_UseGamma | constant | CoreVideo/CVImageBuffer.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| kCVImageBufferTransferFunction_sRGB | constant | CoreVideo/CVImageBuffer.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| kCVImageBufferYCbCrMatrix_DCI_P3 | constant | CoreVideo/CVImageBuffer.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| kCVImageBufferYCbCrMatrix_ITU_R_2020 | constant | CoreVideo/CVImageBuffer.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| kCVImageBufferYCbCrMatrix_ITU_R_601_4 | constant | CoreVideo/CVImageBuffer.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| kCVImageBufferYCbCrMatrix_P3_D65 | constant | CoreVideo/CVImageBuffer.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| kCVImageBufferYCbCrMatrix_SMPTE_240M_1995 | constant | CoreVideo/CVImageBuffer.h | Deprecated on macOS; intentionally excluded from coverage targets. |
| kIOSurfaceIsGlobal | constant | IOSurface/IOSurfaceRef.h | Deprecated on macOS; intentionally excluded from coverage targets. |
