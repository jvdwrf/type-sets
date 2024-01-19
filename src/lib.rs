#![doc = include_str!("../README.md")]

use std::any::TypeId;

mod set;
pub use set::*;

/// Sets from 0 to 10 elements.
pub mod sets;

//-------------------------------------
// Contains
//-------------------------------------

/// Implemented if a set contains the element `E`.
///
/// # Safety
/// Implementing this is unsafe, for custom set-types, implement [`AsSet`] instead.
pub unsafe trait Contains<E> {}

// Implement Contains for `Set<dyn ..>`
unsafe impl<T: ?Sized, E> Contains<E> for Set<T> where T: Contains<E> {}

// Implement Contains for `impl AsSet`
unsafe impl<T, E> Contains<E> for T
where
    T: AsSet,
    T::Set: Contains<E>,
{
}

//-------------------------------------
// Members
//-------------------------------------

/// Trait to get the members (type-ids) of a set.
///
/// # Safety
/// Implementing this is unsafe, for custom set-types, implement [`AsSet`] instead.
pub unsafe trait Members {
    /// Get the members (type-ids) of this set.
    fn members() -> &'static [TypeId];
}

// Implement Members for `impl AsSet`
unsafe impl<T> Members for T
where
    T: AsSet,
{
    fn members() -> &'static [TypeId] {
        <T::Set as Members>::members()
    }
}

//-------------------------------------
// SubsetOf
//-------------------------------------

/// Implemented a set is a subset of `S`.
///
/// # Safety
/// Implementing this is unsafe, for custom set-types, implement [`AsSet`] instead.
pub unsafe trait SubsetOf<S> {}

// Implement SubsetOf for `impl AsSet`
unsafe impl<T, S> SubsetOf<S> for T
where
    T: AsSet,
    T::Set: SubsetOf<S>,
{
}

//-------------------------------------
// SupersetOf
//-------------------------------------

/// Implemented if a set is a superset of `S`.
///
/// # Safety
/// Implementing this is unsafe, for custom set-types, implement [`AsSet`] instead.
pub unsafe trait SupersetOf<S> {}

// Implement SupersetOf for `impl AsSet`
unsafe impl<T, S> SupersetOf<S> for T where S: SubsetOf<T> {}

//-------------------------------------
// AsSet
//-------------------------------------

/// Trait that allows usage of custom types instead of [`Set`].
pub trait AsSet {
    /// The [`struct@Set`] type.
    type Set: Members;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MySet;

    impl AsSet for MySet {
        type Set = Set![u32, u64];
    }

    #[test]
    fn test() {
        contains_string::<Set![String]>();
        contains_string::<Set![String, u32]>();
        contains_string::<Set![String, u32, u64]>();
        contains_string::<Set![u32, u64, u128, String]>();
        // contains_string::<MySet>(); // does not compile
        // contains_string::<Set![u32]>(); // does not compile

        is_subset::<Set![u32, u64, u32, u32]>();
        is_subset::<Set![u32, u64]>();
        is_subset2::<Set![u32]>();
        is_subset::<Set![]>();
        is_subset::<MySet>();
        // is_subset::<Set![u32, u64, u128]>(); // does not compile
        // is_subset2::<MySet>(); // does not compile
        // is_subset::<Set![u32, u64, u128]>(); // does not compile

        is_superset1::<Set![u32, u64]>();
        is_superset2::<Set![u32, u64, u128]>();
        is_superset1::<MySet>();
        // is_superset1::<Set![u32]>(); // does not compile
        // is_superset2::<MySet>(); // does not compile

        let _: Set![];
        let _: Set![u32];
        let _: Set![u32, u64];
        let _: Set![u32, u64, u128];
        let _: Set![u32, u64, u128, u32];
        let _: Set![u32, u64, u128, u32, u64];
        let _: Set![u32, u64, u128, u32, u64, u128];
        let _: Set![u32, u64, u128, u32, u64, u128, u32];
        let _: Set![u32, u64, u128, u32, u64, u128, u32, u64];
        let _: Set![u32, u64, u128, u32, u64, u128, u32, u64, u128];
        let _: Set![u32, u64, u128, u32, u64, u128, u32, u64, u128, u32];
        let _: Set![u32, u64, u128, u32, u64, u128, u32, u64, u128, u32, u64];
        let _: Set![u32, u64, u128, u32, u64, u128, u32, u64, u128, u32, u64, u128];
    }

    fn contains_string<T>()
    where
        T: Contains<String>,
    {
    }

    fn is_subset<T>()
    where
        T: SubsetOf<Set![u32, u64]>,
    {
    }

    fn is_subset2<T>()
    where
        T: SubsetOf<Set![u32]>,
    {
    }

    fn is_superset1<T>()
    where
        T: SupersetOf<Set![u32, u64]>,
    {
    }

    fn is_superset2<T>()
    where
        T: SupersetOf<Set![u32, u64, u128]>,
    {
    }
}
