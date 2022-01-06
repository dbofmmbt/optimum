use std::ops::{Index, IndexMut};

/// A slice of genes which represents solutions to a problem.
#[derive(Debug, Clone)]
pub struct Member<G, V> {
    /// The slice of genes
    pub keys: Box<[G]>,
    /// The value obtained by decoding the `keys`.
    pub value: V,
}

impl<G, V> Member<G, V> {
    /// The number of genes in the member.
    ///
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.keys.len()
    }
}

impl<G, V: Ord> Ord for Member<G, V> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.partial_cmp(&other.value).unwrap()
    }
}

impl<G, V: PartialOrd> PartialOrd for Member<G, V> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<G, V: PartialEq> PartialEq for Member<G, V> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<G, V: Eq> Eq for Member<G, V> {}

impl<G, V> Index<usize> for Member<G, V> {
    type Output = G;

    fn index(&self, index: usize) -> &Self::Output {
        &self.keys[index]
    }
}

impl<G, V> IndexMut<usize> for Member<G, V> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.keys[index]
    }
}
