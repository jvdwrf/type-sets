Sets implemented in the rust type-system.

This crate allows you to create sets of types in the rust type-system. These sets can be compared using [`SubsetOf`] and [`SupersetOf`].

This library was developed for use in [`meslin`](https://github.com/jvdwrf/Meslin), but is general enough that there might be other purposes as well.

## Example
```rust
use type_sets::*;

// We can define functions, that may only be called if the parameter `T` is
// a subset or superset of another set.
fn is_subset<T: SubsetOf<Set![u32, u64]>>() {}
fn is_superset<T: SupersetOf<Set![u32, u64]>>() {}

// We can also use custom structs as sets
struct MySet;
impl AsSet for MySet {
    type Set = Set![u32];
}

fn main() {
    is_subset::<Set![u32, u64]>(); // compiles
    is_subset::<Set![u32]>(); // compiles
    // is_subset::<Set![u32, u64, u32]>(); // does not compile

    is_superset::<Set![u32, u64]>(); // compiles
    is_superset::<Set![u32, u64, u128]>(); // compiles
    // is_superset::<Set![u32]>(); // does not compile

    is_subset::<MySet>(); // compiles
    // is_superset::<MySet>(); // does not compile
}