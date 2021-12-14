use std::ops::{Index, IndexMut};

pub type Key = f64;

#[derive(Debug, Clone)]
pub struct Member<V> {
    pub keys: Box<[Key]>,
    pub value: V,
}

impl<V: Ord> Ord for Member<V> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.partial_cmp(&other.value).unwrap()
    }
}

impl<V: PartialOrd> PartialOrd for Member<V> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<V: PartialEq> PartialEq for Member<V> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<V: Eq> Eq for Member<V> {}

impl<V> Index<usize> for Member<V> {
    type Output = Key;

    fn index(&self, index: usize) -> &Self::Output {
        &self.keys[index]
    }
}

impl<V> IndexMut<usize> for Member<V> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.keys[index]
    }
}
