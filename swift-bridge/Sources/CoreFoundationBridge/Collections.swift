import CoreFoundation
import Foundation

private func acfArrayFromRawPointers(_ pointers: [UnsafeRawPointer?]) -> CFArray? {
    var callbacks = kCFTypeArrayCallBacks
    var mutablePointers = pointers
    return mutablePointers.withUnsafeMutableBufferPointer { buffer in
        CFArrayCreate(nil, buffer.baseAddress, buffer.count, &callbacks)
    }
}

private func acfDictionaryFromRawPointers(
    keys: [UnsafeRawPointer?],
    values: [UnsafeRawPointer?]
) -> CFDictionary? {
    precondition(keys.count == values.count)
    let count = keys.count
    var keys = keys
    var values = values
    var keyCallbacks = kCFTypeDictionaryKeyCallBacks
    var valueCallbacks = kCFTypeDictionaryValueCallBacks
    return keys.withUnsafeMutableBufferPointer { keyBuffer in
        values.withUnsafeMutableBufferPointer { valueBuffer in
            CFDictionaryCreate(
                nil,
                keyBuffer.baseAddress,
                valueBuffer.baseAddress,
                count,
                &keyCallbacks,
                &valueCallbacks
            )
        }
    }
}

private func acfBagFromRawPointers(_ pointers: [UnsafeRawPointer?]) -> CFBag? {
    var callbacks = kCFTypeBagCallBacks
    var pointers = pointers
    return pointers.withUnsafeMutableBufferPointer { buffer in
        CFBagCreate(nil, buffer.baseAddress, buffer.count, &callbacks)
    }
}

@_cdecl("cf_array_get_type_id")
public func cf_array_get_type_id() -> Int {
    Int(CFArrayGetTypeID())
}

@_cdecl("cf_array_create")
public func cf_array_create(_ values: UnsafePointer<UnsafeMutableRawPointer?>?, _ count: Int) -> UnsafeMutableRawPointer? {
    let pointers = UnsafeBufferPointer(start: values, count: count).map { raw -> UnsafeRawPointer? in
        raw.map(UnsafeRawPointer.init)
    }
    guard let array = acfArrayFromRawPointers(pointers) else { return nil }
    return Unmanaged.passRetained(array).toOpaque()
}

@_cdecl("cf_array_get_count")
public func cf_array_get_count(_ value: UnsafeMutableRawPointer) -> Int {
    let array = Unmanaged<CFArray>.fromOpaque(value).takeUnretainedValue()
    return CFArrayGetCount(array)
}

@_cdecl("cf_array_get_value_at_index")
public func cf_array_get_value_at_index(_ value: UnsafeMutableRawPointer, _ index: Int) -> UnsafeMutableRawPointer? {
    let array = Unmanaged<CFArray>.fromOpaque(value).takeUnretainedValue()
    guard index >= 0, index < CFArrayGetCount(array) else { return nil }
    let raw = CFArrayGetValueAtIndex(array, index)
    return acfRetainedCFType(UnsafeMutableRawPointer(mutating: raw))
}

@_cdecl("cf_dictionary_get_type_id")
public func cf_dictionary_get_type_id() -> Int {
    Int(CFDictionaryGetTypeID())
}

@_cdecl("cf_dictionary_create")
public func cf_dictionary_create(
    _ keys: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ values: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int
) -> UnsafeMutableRawPointer? {
    let keyPointers = UnsafeBufferPointer(start: keys, count: count).map { $0.map(UnsafeRawPointer.init) }
    let valuePointers = UnsafeBufferPointer(start: values, count: count).map { $0.map(UnsafeRawPointer.init) }
    guard let dictionary = acfDictionaryFromRawPointers(keys: keyPointers, values: valuePointers) else {
        return nil
    }
    return Unmanaged.passRetained(dictionary).toOpaque()
}

@_cdecl("cf_dictionary_get_count")
public func cf_dictionary_get_count(_ value: UnsafeMutableRawPointer) -> Int {
    let dictionary = Unmanaged<CFDictionary>.fromOpaque(value).takeUnretainedValue()
    return CFDictionaryGetCount(dictionary)
}

@_cdecl("cf_dictionary_contains_key")
public func cf_dictionary_contains_key(_ value: UnsafeMutableRawPointer, _ key: UnsafeMutableRawPointer) -> Bool {
    let dictionary = Unmanaged<CFDictionary>.fromOpaque(value).takeUnretainedValue()
    return CFDictionaryContainsKey(dictionary, UnsafeRawPointer(key))
}

@_cdecl("cf_dictionary_get_value")
public func cf_dictionary_get_value(_ value: UnsafeMutableRawPointer, _ key: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer? {
    let dictionary = Unmanaged<CFDictionary>.fromOpaque(value).takeUnretainedValue()
    let raw = CFDictionaryGetValue(dictionary, UnsafeRawPointer(key))
    return acfRetainedCFType(UnsafeMutableRawPointer(mutating: raw))
}

@_cdecl("cf_dictionary_copy_keys")
public func cf_dictionary_copy_keys(_ value: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer? {
    let dictionary = Unmanaged<CFDictionary>.fromOpaque(value).takeUnretainedValue()
    let count = CFDictionaryGetCount(dictionary)
    var keys = Array<UnsafeRawPointer?>(repeating: nil, count: count)
    CFDictionaryGetKeysAndValues(dictionary, &keys, nil)
    guard let array = acfArrayFromRawPointers(keys) else { return nil }
    return Unmanaged.passRetained(array).toOpaque()
}

@_cdecl("cf_dictionary_copy_values")
public func cf_dictionary_copy_values(_ value: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer? {
    let dictionary = Unmanaged<CFDictionary>.fromOpaque(value).takeUnretainedValue()
    let count = CFDictionaryGetCount(dictionary)
    var values = Array<UnsafeRawPointer?>(repeating: nil, count: count)
    CFDictionaryGetKeysAndValues(dictionary, nil, &values)
    guard let array = acfArrayFromRawPointers(values) else { return nil }
    return Unmanaged.passRetained(array).toOpaque()
}

@_cdecl("cf_bag_get_type_id")
public func cf_bag_get_type_id() -> Int {
    Int(CFBagGetTypeID())
}

@_cdecl("cf_bag_create")
public func cf_bag_create(_ values: UnsafePointer<UnsafeMutableRawPointer?>?, _ count: Int) -> UnsafeMutableRawPointer? {
    let pointers = UnsafeBufferPointer(start: values, count: count).map { $0.map(UnsafeRawPointer.init) }
    guard let bag = acfBagFromRawPointers(pointers) else { return nil }
    return Unmanaged.passRetained(bag).toOpaque()
}

@_cdecl("cf_bag_get_count")
public func cf_bag_get_count(_ value: UnsafeMutableRawPointer) -> Int {
    let bag = Unmanaged<CFBag>.fromOpaque(value).takeUnretainedValue()
    return CFBagGetCount(bag)
}

@_cdecl("cf_bag_contains_value")
public func cf_bag_contains_value(_ value: UnsafeMutableRawPointer, _ candidate: UnsafeMutableRawPointer) -> Bool {
    let bag = Unmanaged<CFBag>.fromOpaque(value).takeUnretainedValue()
    return CFBagContainsValue(bag, UnsafeRawPointer(candidate))
}

@_cdecl("cf_bag_get_count_of_value")
public func cf_bag_get_count_of_value(_ value: UnsafeMutableRawPointer, _ candidate: UnsafeMutableRawPointer) -> Int {
    let bag = Unmanaged<CFBag>.fromOpaque(value).takeUnretainedValue()
    return CFBagGetCountOfValue(bag, UnsafeRawPointer(candidate))
}

@_cdecl("cf_attributed_string_get_type_id")
public func cf_attributed_string_get_type_id() -> Int {
    Int(CFAttributedStringGetTypeID())
}

@_cdecl("cf_attributed_string_create")
public func cf_attributed_string_create(_ string: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer? {
    let string = Unmanaged<CFString>.fromOpaque(string).takeUnretainedValue()
    guard let attributed = CFAttributedStringCreate(nil, string, nil) else { return nil }
    return Unmanaged.passRetained(attributed).toOpaque()
}

@_cdecl("cf_attributed_string_get_string")
public func cf_attributed_string_get_string(_ value: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer? {
    let attributed = Unmanaged<CFAttributedString>.fromOpaque(value).takeUnretainedValue()
    return Unmanaged.passRetained(CFAttributedStringGetString(attributed)).toOpaque()
}

@_cdecl("cf_attributed_string_get_length")
public func cf_attributed_string_get_length(_ value: UnsafeMutableRawPointer) -> Int {
    let attributed = Unmanaged<CFAttributedString>.fromOpaque(value).takeUnretainedValue()
    return CFAttributedStringGetLength(attributed)
}

final class ACFTreeNode {
    let value: AnyObject?
    var children: [ACFTreeNode] = []

    init(value: AnyObject?) {
        self.value = value
    }
}

@_cdecl("cf_tree_create")
public func cf_tree_create(_ value: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    let object = value.map(acfBorrowedAnyObject)
    return Unmanaged.passRetained(ACFTreeNode(value: object)).toOpaque()
}

@_cdecl("cf_tree_append_child")
public func cf_tree_append_child(_ parent: UnsafeMutableRawPointer, _ child: UnsafeMutableRawPointer) {
    let parentNode = Unmanaged<ACFTreeNode>.fromOpaque(parent).takeUnretainedValue()
    let childNode = Unmanaged<ACFTreeNode>.fromOpaque(child).takeUnretainedValue()
    parentNode.children.append(childNode)
}

@_cdecl("cf_tree_get_child_count")
public func cf_tree_get_child_count(_ value: UnsafeMutableRawPointer) -> Int {
    let node = Unmanaged<ACFTreeNode>.fromOpaque(value).takeUnretainedValue()
    return node.children.count
}

@_cdecl("cf_tree_get_child_at_index")
public func cf_tree_get_child_at_index(_ value: UnsafeMutableRawPointer, _ index: Int) -> UnsafeMutableRawPointer? {
    let node = Unmanaged<ACFTreeNode>.fromOpaque(value).takeUnretainedValue()
    guard index >= 0, index < node.children.count else { return nil }
    return Unmanaged.passRetained(node.children[index]).toOpaque()
}

@_cdecl("cf_tree_copy_value")
public func cf_tree_copy_value(_ value: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer? {
    let node = Unmanaged<ACFTreeNode>.fromOpaque(value).takeUnretainedValue()
    guard let object = node.value else { return nil }
    return Unmanaged.passRetained(object).toOpaque()
}
