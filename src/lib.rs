use std::cmp::Ordering;

mod avl;

pub type ImmutableSet<T> = avl::ImmutAvlTree<T>;

#[derive(Clone, Debug)]
struct ImmutableMapContent<T: PartialOrd + Clone, V: Clone + Default> {
    key: T,
    val: V,
}

impl<T: PartialOrd + Clone, V: Clone + Default> PartialOrd for ImmutableMapContent<T, V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.key.partial_cmp(&other.key)
    }
}

impl<T: PartialOrd + Clone, V: Clone + Default> PartialEq for ImmutableMapContent<T, V> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

pub struct ImmutableMap<T: PartialOrd + Clone, V: Clone + Default> {
    set: ImmutableSet<ImmutableMapContent<T, V>>,
}

impl<T: PartialOrd + Clone, V: Clone + Default> ImmutableMap<T, V> {
    pub fn new() -> Self {
        ImmutableMap {
            set: ImmutableSet::new(),
        }
    }
    pub fn insert(&self, key: T, val: V) -> Self {
        ImmutableMap {
            set: self.set.insert(ImmutableMapContent { key, val }),
        }
    }
    pub fn delete(&self, key: T) -> Self {
        ImmutableMap {
            set: self.set.delete(ImmutableMapContent {
                key,
                val: V::default(),
            }),
        }
    }
    pub fn contains(&self, key: T, val: V) -> bool {
        self.set.contains(ImmutableMapContent { key, val })
    }
    pub fn size(&self) -> usize {
        self.set.size()
    }
    pub fn get_val_as_ref(&self, key: T) -> Option<&V> {
        let content = self.set.get_as_ref(ImmutableMapContent {
            key,
            val: V::default(),
        });
        if let Some(content) = content {
            Some(&content.val)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let set = ImmutableSet::new();
        let new_set = set.insert(1);
        assert_eq!(new_set.size(), 1);

        let map = ImmutableMap::new();
        let new_map = map.insert(1, "abc");
        assert_eq!(new_map.size(), 1);
        let new_map = new_map.insert(2, "def");
        assert_eq!(new_map.get_val_as_ref(2), Some(&"def"));
        let new_map = new_map.delete(1);
        assert_eq!(new_map.size(), 1);
    }
}
