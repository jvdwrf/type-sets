use std::marker::PhantomData;

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
    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty $(,)?) => {
        $crate::Set<dyn $crate::sets::Eleven<$t1, $t2, $t3, $t4, $t5, $t6, $t7, $t8, $t9, $t10, $t11>>
    };
    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty $(,)?) => {
        $crate::Set<dyn $crate::sets::Twelve<$t1, $t2, $t3, $t4, $t5, $t6, $t7, $t8, $t9, $t10, $t11, $t12>>
    };
}
