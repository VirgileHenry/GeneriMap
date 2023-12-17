use std::{
    any::TypeId,
    collections::HashMap,
    hash::{
        Hasher,
        BuildHasherDefault
    }
};

#[cfg(test)]
pub mod test;

#[derive(Debug, Clone)]
pub struct GeneriMap {
    map: HashMap<TypeId, Vec<u8>, BuildHasherDefault<GeneriMapHasher>>,
}

impl GeneriMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::with_hasher(Default::default()),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            map: HashMap::with_capacity_and_hasher(capacity, Default::default()),
        }
    }

    pub fn capacity(&self) -> usize {
        self.map.capacity()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn insert<T: 'static>(&mut self, elem: T) {
        self.map.insert(TypeId::of::<T>(), unsafe {
            let mut vec = Vec::<u8>::with_capacity(std::mem::size_of::<T>());
            vec.set_len(std::mem::size_of::<T>());
            (vec.as_ptr() as *mut T).write(elem);
            vec
        });
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        unsafe { Some(&*(self.map.get(&TypeId::of::<T>())?.as_ptr() as *const T)) }
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        unsafe { Some(&mut *(self.map.get_mut(&TypeId::of::<T>())?.as_mut_ptr() as *mut T)) }
    }

    pub fn remove<T: 'static>(&mut self) -> Option<T> {
        unsafe { Some((self.map.remove(&TypeId::of::<T>())?.as_ptr() as *mut T).read()) }
    }

    pub unsafe fn iter_blobs(&self) -> impl Iterator<Item = &Vec<u8>> {
        self.map.values()
    }

    pub unsafe fn iter_blobs_mut(&mut self) -> impl Iterator<Item = &mut Vec<u8>> {
        self.map.values_mut()
    }
}

#[derive(Default)]
struct GeneriMapHasher {
    state: u64,
}

/// got from the anymap original crate : https://github.com/chris-morgan/anymap
impl Hasher for GeneriMapHasher {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        // This expects to receive one and exactly one 64-bit value
        debug_assert!(bytes.len() == 8);
        unsafe {
            std::ptr::copy_nonoverlapping(&bytes[0] as *const u8 as *const u64, &mut self.state, 1)
        }
    }

    #[inline]
    fn finish(&self) -> u64 { self.state }
}

