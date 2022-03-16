pub mod avl;

pub trait ImmutableOrderSet<T: PartialOrd> {
    fn new(val: T) -> Self;
    fn insert(&self, val: T) -> Self;
    fn delete(&self, val: T) -> Self;
    fn contains(&self, val: T) -> bool;
}

pub trait ImmutableSet<T: Eq> {
    fn new(val: T) -> Self;
    fn insert(&self, val: T) -> Self;
    fn delete(&self, val: T) -> Self;
    fn contains(&self, val: T) -> bool;
}

pub trait ImmutableMap<T: PartialOrd> {
    fn new() -> Self;
    fn insert(&self, val: T) -> Self;
    fn delete(&self, val: T) -> Self;
    fn contains(&self, val: T) -> bool;
}

