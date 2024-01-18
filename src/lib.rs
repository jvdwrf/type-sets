#![doc = include_str!("../README.md")]

use private::*;
use std::any::TypeId;

mod set;
pub use set::*;

/// Sets of types from 0 to 10 elements.
pub mod sets;

mod private {
    pub trait SealedSupersetOf<T> {}
    pub trait SealedSubsetOf<T> {}
    pub trait SealedContains<T> {}
    pub trait SealedMembers {}
}

//-------------------------------------
// Contains
//-------------------------------------

/// Indicates that a set contains element `E`.
///
/// This trait is sealed, implemente [`AsSet`] instead.
pub trait Contains<E>: SealedContains<E> {}

// Implement Contains for `Set<dyn ..>`
impl<T: ?Sized, E> Contains<E> for Set<T> where T: Contains<E> {}
impl<T: ?Sized, E> SealedContains<E> for Set<T> where T: Contains<E> {}

// Implement Contains for `impl AsSet`
impl<T, E> Contains<E> for T
where
    T: AsSet,
    T::Set: Contains<E>,
{
}
impl<T, E> SealedContains<E> for T
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
/// This trait is sealed, implement [`AsSet`] instead.
pub trait Members: SealedMembers {
    /// Get the members (type-ids) of this set.
    fn members() -> &'static [TypeId];
}

// Implement Members for `impl AsSet`
impl<T> Members for T
where
    T: AsSet,
{
    fn members() -> &'static [TypeId] {
        <T::Set as Members>::members()
    }
}
impl<T> SealedMembers for T where T: AsSet {}

//-------------------------------------
// SubsetOf
//-------------------------------------

/// Implemented if set is a subset of `S`.
///
/// This trait is sealed, implement [`AsSet`] instead.
pub trait SubsetOf<S>: SealedSubsetOf<S> {}

// Implement SubsetOf for `impl AsSet`
impl<T, S> SubsetOf<S> for T
where
    T: AsSet,
    T::Set: SubsetOf<S>,
{
}
impl<T, S> SealedSubsetOf<S> for T
where
    T: AsSet,
    T::Set: SubsetOf<S>,
{
}

//-------------------------------------
// SupersetOf
//-------------------------------------

/// Implemented if set is a superset of `S`.
///
/// This trait is sealed, implement [`AsSet`] instead.
pub trait SupersetOf<S>: SealedSupersetOf<S> {}

// Implement SupersetOf for `impl AsSet`
impl<T, S> SupersetOf<S> for T where S: SubsetOf<T> {}
impl<T, S> SealedSupersetOf<S> for T where S: SubsetOf<T> {}

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
