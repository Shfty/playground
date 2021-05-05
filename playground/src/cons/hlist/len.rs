#[macro_export]
macro_rules! impl_hlist_len {
    ($a:ident, $($bs:ident),*) => {
        #[allow(dead_code, unused_variables)]
        impl<$a, $($bs),*> HList![$a, $($bs),*] {
            pub const LENGTH: usize = {
                let len = 1;
                $(
                    let _: $bs;
                    let len = len + 1;
                )*
                len
            };

            pub fn len(&self) -> usize {
                Self::LENGTH
            }
        }
    };
    ($a:ident) => {
        #[allow(dead_code)]
        impl<$a> HList![$a] {
            pub const LENGTH: usize = 1;

            pub fn len(&self) -> usize {
                Self::LENGTH
            }
        }
    };
}
