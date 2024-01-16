#[doc = include_str!("../README.md")]

use std::{any::TypeId, marker::PhantomData};

/// Sets of types from 0 to 10 elements.
pub mod sets;

/// Indicates that a set contains element `E`.
pub trait Contains<E> {}
impl<T: ?Sized, E> Contains<E> for Set<T> where T: Contains<E> {}

/// Trait to get the members (type-ids) of a set.
pub trait Members {
    /// Get the members (type-ids) of this set.
    fn members() -> &'static [TypeId];
}

/// Implemented if set is a subset of `S`.
pub trait SubsetOf<S> {}

/// Implemented if set is a superset of `S`.
pub trait SupersetOf<S> {}
impl<T, S> SupersetOf<S> for T where S: SubsetOf<T> {}

/// Trait that allows usage of custom types as a set.
pub trait AsSet {
    /// The [`struct@Set`] type.
    type Set: Members;
}

impl<T, S> SubsetOf<S> for T
where
    T: AsSet,
    T::Set: SubsetOf<S>,
{
}

impl<T, E> Contains<E> for T
where
    T: AsSet,
    T::Set: Contains<E>,
{
}

impl<T> Members for T
where
    T: AsSet,
{
    fn members() -> &'static [TypeId] {
        <T::Set as Members>::members()
    }
}

/// Marker struct to act as a set of elements.
///
/// Usually created using the [`macro@Set`] macro.
///
/// The inner type `T` is usually one of the [`sets`]:
/// - `Set![]` == `Set<dyn Zero>`
/// - `Set![u32]` == `Set<dyn One<u32>>`
/// - `Set![u32, u64]` == `Set<dyn Two<u32, u64>>`
pub struct Set<T: ?Sized>(PhantomData<fn() -> T>);

/// Create a [`struct@Set`] from a list of types.
///
/// Example:
/// - `Set![]` == `Set<dyn Zero>`
/// - `Set![u32]` == `Set<dyn One<u32>>`
/// - `Set![u32, u64]` == `Set<dyn Two<u32, u64>>`
#[macro_export]
macro_rules! Set {
    ($(,)?) => {
        $crate::Set<dyn $crate::sets::Zero>
    };
    ($t1:ty $(,)?) => {
        $crate::Set<dyn $crate::sets::One<$t1>>
    };
    ($t1:ty, $t2:ty $(,)?) => {
        $crate::Set<dyn $crate::sets::Two<$t1, $t2>>
    };
    ($t1:ty, $t2:ty, $t3:ty $(,)?) => {
        $crate::Set<dyn $crate::sets::Three<$t1, $t2, $t3>>
    };
    ($t1:ty, $t2:ty, $t3:ty, $t4:ty $(,)?) => {
        $crate::Set<dyn $crate::sets::Four<$t1, $t2, $t3, $t4>>
    };
    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty $(,)?) => {
        $crate::Set<dyn $crate::sets::Five<$t1, $t2, $t3, $t4, $t5>>
    };
    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty $(,)?) => {
        $crate::Set<dyn $crate::sets::Six<$t1, $t2, $t3, $t4, $t5, $t6>>
    };
    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty $(,)?) => {
        $crate::Set<dyn $crate::sets::Seven<$t1, $t2, $t3, $t4, $t5, $t6, $t7>>
    };
    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty $(,)?) => {
        $crate::Set<dyn $crate::sets::Eight<$t1, $t2, $t3, $t4, $t5, $t6, $t7, $t8>>
    };
    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty $(,)?) => {
        $crate::Set<dyn $crate::sets::Nine<$t1, $t2, $t3, $t4, $t5, $t6, $t7, $t8, $t9>>
    };
    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty $(,)?) => {
        $crate::Set<dyn $crate::sets::Ten<$t1, $t2, $t3, $t4, $t5, $t6, $t7, $t8, $t9, $t10>>
    };
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
        is_subset::<Set![u32, u64, u32, u32]>();
        is_subset::<Set![u32]>();
        is_subset::<Set![u64]>();
        is_subset2::<Set![]>();
        is_subset::<MySet>();
        // is_subset2::<MySet>(); // does not compile
        // is_subset::<Set![u32, u64, u128]>(); // does not compile

        is_superset1::<Set![u32, u64]>();
        is_superset2::<Set![u32, u64, u128]>();
        is_superset1::<MySet>();
        // is_superset1::<Set![u32]>(); // does not compile
        // is_superset2::<MySet>(); // does not compile
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
