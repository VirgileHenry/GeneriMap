// some test for our anymap.

use crate::GeneriMap;

#[test]
pub fn simple_insert() {
    let mut map = GeneriMap::new();
    map.insert(42u8);
    assert_eq!(map.len(), 1);
    map.insert(42u16);
    assert_eq!(map.len(), 2);
    map.insert(42u32);
    assert_eq!(map.len(), 3);
}

#[test]
pub fn simple_add_get() {
    let mut map = GeneriMap::new();
    map.insert(42u8);
    map.insert("Hello World".to_string());
    map.insert(vec![1u8, 2u8, 3u8]);
    assert_eq!(map.get::<u8>(), Some(&42u8));
    assert_eq!(map.get::<String>(), Some(&"Hello World".to_string()));
    assert_eq!(map.get::<Vec<u8>>(), Some(&vec![1u8, 2u8, 3u8]));
}

#[test]
pub fn iter_blobs() {
    let mut map = GeneriMap::new();
    // we can't test more than one value as we can't retain the hashmap iter order
    map.insert(42u64);
    unsafe {
        for bytes in map.iter_blobs() {
            let actual_bytes = 42u64.to_ne_bytes();
            for (b1, b2) in bytes.iter().zip(actual_bytes.iter()) {
                assert_eq!(b1, b2);
            }
        }
    }
}