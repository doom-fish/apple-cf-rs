use apple_cf::cf::{CFArray, CFAttributedString, CFBag, CFDictionary, CFString, CFTree};

#[test]
fn cf_collection_wrappers_work() {
    let first = CFString::new("first");
    let second = CFString::new("second");

    let array = CFArray::from_values(&[&first, &second]);
    assert_eq!(array.len(), 2);
    assert_eq!(array.values().len(), 2);

    let dictionary = CFDictionary::from_pairs(&[(&first, &second)]);
    assert!(dictionary.contains_key(&first));
    assert_eq!(dictionary.keys().len(), 1);
    assert_eq!(dictionary.values().len(), 1);

    let bag = CFBag::from_values(&[&first, &first, &second]);
    assert_eq!(bag.count_of_value(&first), 2);
    assert!(bag.contains(&second));

    let attributed = CFAttributedString::new(&first);
    assert_eq!(attributed.string().to_string(), "first");

    let root = CFTree::new(Some(&first));
    let child = CFTree::new(Some(&second));
    root.append_child(&child);
    assert_eq!(root.child_count(), 1);
    assert!(root.child_at(0).is_some());
    assert!(root.value().is_some());
}
