use apple_cf::cf::{CFArray, CFAttributedString, CFBag, CFDictionary, CFString, CFTree};

fn main() {
    let first = CFString::new("first");
    let second = CFString::new("second");

    let array = CFArray::from_values(&[&first, &second]);
    assert_eq!(array.len(), 2);

    let dict = CFDictionary::from_pairs(&[(&first, &second)]);
    assert!(dict.contains_key(&first));

    let bag = CFBag::from_values(&[&first, &first, &second]);
    assert_eq!(bag.count_of_value(&first), 2);

    let attributed = CFAttributedString::new(&first);
    assert_eq!(attributed.string().to_string(), "first");

    let root = CFTree::new(Some(&first));
    let child = CFTree::new(Some(&second));
    root.append_child(&child);
    assert_eq!(root.child_count(), 1);
}
