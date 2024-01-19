use crate::*;
use std::{any::TypeId, fmt::Debug, sync::OnceLock};

macro_rules! create_sets {
    ($(
        $n:literal $set:ident<$($gen:ident),*> $(:)? $($sub_sets:path),*;
    )*) => {
        $(
            // Create the sets as auto-traits
            #[doc = concat!("A set containing ", $n, " types.")]
            ///
            /// This should not be implemented manually.
            ///
            /// For easy usage, see the macro [`macro@Set`].
            pub trait $set<$($gen),*>: $($sub_sets +)* {}
            impl<$($gen,)* S: ?Sized> $set<$($gen),*> for S where S: $($sub_sets +)* {}

            // Implement the correct IsSubsetOf implementations
            unsafe impl<$($gen,)* S> SubsetOf<S> for Set<dyn $set<$($gen),*>>
            where
                S: $set<$($gen),*>
            {}

            unsafe impl<$($gen: 'static,)*> Members for Set<dyn $set<$($gen),*>>
            {
                fn members() -> &'static [TypeId] {
                    static LOCK: OnceLock<[TypeId; $n]> = OnceLock::new();
                    LOCK.get_or_init(|| [ $(TypeId::of::<$gen>()),* ])
                }
            }


            impl<$($gen: 'static,)*> Debug for Set<dyn $set<$($gen),*>> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_struct(stringify!($set))
                        $(.field(stringify!($gen), &std::any::type_name::<$gen>()))*
                        .finish()
                }
            }
        )*
    };
}

create_sets!(
    0 Zero<>;
    1 One<T1>: Zero, Contains<T1>;
    2 Two<T1, T2>: One<T1>, Contains<T2>;
    3 Three<T1, T2, T3>: Two<T1, T2>, Contains<T3>;
    4 Four<T1, T2, T3, T4>: Three<T1, T2, T3>, Contains<T4>;
    5 Five<T1, T2, T3, T4, T5>: Four<T1, T2, T3, T4>, Contains<T5>;
    6 Six<T1, T2, T3, T4, T5, T6>: Five<T1, T2, T3, T4, T5>, Contains<T6>;
    7 Seven<T1, T2, T3, T4, T5, T6, T7>: Six<T1, T2, T3, T4, T5, T6>, Contains<T7>;
    8 Eight<T1, T2, T3, T4, T5, T6, T7, T8>: Seven<T1, T2, T3, T4, T5, T6, T7>, Contains<T8>;
    9 Nine<T1, T2, T3, T4, T5, T6, T7, T8, T9>: Eight<T1, T2, T3, T4, T5, T6, T7, T8>, Contains<T9>;
    10 Ten<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>: Nine<T1, T2, T3, T4, T5, T6, T7, T8, T9>, Contains<T10>;
    11 Eleven<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>: Ten<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>, Contains<T11>;
    12 Twelve<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>: Eleven<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>, Contains<T12>;
);
