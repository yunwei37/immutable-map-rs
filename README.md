# An Immutable Map implement for rust

This is an immutable (functional) set or map implementation based on an immutable AVL tree, purely safe rust.

The time and space complexity of add or remove operations is `log(N)` in the size of the original set.

This little project was inspired by using the ImmutableMap in LLVM/Clang static analyzer.

## Usage

```rs
let set = ImmutableSet::new();
let new_set = set.insert(1);

let map = ImmutableMap::new();
let new_map = new_map.insert(1, "abc");
let new_map = new_map.delete(1, "abc");
let size = new_map.size();
let data = new_map.get_val_as_ref(1);
```

## Reference

- [https://llvm.org/docs/ProgrammersManual.html#llvm-adt-immutableset-h](https://llvm.org/docs/ProgrammersManual.html#llvm-adt-immutableset-h)