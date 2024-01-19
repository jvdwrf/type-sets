[![Crates.io](https://img.shields.io/crates/v/type-sets)](https://crates.io/crates/type-sets)
[![Documentation](https://docs.rs/type-sets/badge.svg)](https://docs.rs/type-sets)

Sets implemented in the rust type-system.

This crate allows the creation of type-sets within the rust type-system. These sets can be compared using `SubsetOf` and `SupersetOf`. All traits are marked unsafe, except for `AsSet`, giving guarantees about conflicting implementations for use in `unsafe` code.

Sets are implemented up to 12 items.

This library was created for use in [`meslin`](https://github.com/jvdwrf/Meslin), but is general enough that there could be other purposes as well.

## Example
```rust
use type_sets::*;
use std::any::TypeId;

// We can define functions, that may only be called if the parameter `T` is
// a subset or superset of another set.
fn is_subset<T: SubsetOf<Set![u32, u64]>>() {}
fn is_superset<T: SupersetOf<Set![u32, u64]>>() {}
fn contains_u64<T: Contains<u64>>() {}

// We can also use custom structs as sets
struct MySet;
impl AsSet for MySet {
    type Set = Set![u32];
}

fn main() {
    is_subset::<Set![u32, u64]>(); // compiles
    is_subset::<Set![u64, u32]>(); // compiles
    is_subset::<Set![u32]>();
    // is_subset::<Set![u32, u64, u32]>(); // does not compile

    is_superset::<Set![u32, u64]>(); // compiles
    is_superset::<Set![u32, u64, u128]>(); // compiles
    is_superset::<Set![u128, u32, u64, u32, u32, u32]>(); // compiles
    // is_superset::<Set![u32]>(); // does not compile

    is_subset::<MySet>(); // compiles
    // is_superset::<MySet>(); // does not compile

    contains_u64::<Set![u64]>();
    // contains_u64::<Set![u32]>(); // does not compile

    assert_eq!(
        <Set![u64, u32]>::members(), 
        [TypeId::of::<u64>(), TypeId::of::<u32>()]
    );
    assert_eq!(
        MySet::members(),
        [TypeId::of::<u32>()]
    );
}
```