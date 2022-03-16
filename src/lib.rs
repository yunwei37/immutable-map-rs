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

type ImmutableMap<T, V> = ImmutableSet<ImmutableMapContent<T, V>>;

impl<T: PartialOrd + Clone, V: Clone + Default> ImmutableMap<T, V> {
    pub fn insert_map(&self, key: T, val: V) -> Self {
        self.insert(ImmutableMapContent { key, val })
    }
    pub fn delete_map(&self, key: T, val: V) -> Self {
        self.insert(ImmutableMapContent { key, val })
    }
    pub fn contains_map(&self, key: T, val: V) -> bool {
        self.contains(ImmutableMapContent { key, val })
    }
    pub fn get_val_as_ref(&self, key: T) -> Option<&V> {
        let content = self.get_as_ref(ImmutableMapContent { key, val:V::default() });
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

        let set = ImmutableMap::new();
        let new_set = set.insert_map(1, "abc");
        assert_eq!(new_set.size(), 1);

    }
}
