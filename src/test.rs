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