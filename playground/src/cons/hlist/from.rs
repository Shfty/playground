#[macro_export]
macro_rules! impl_hlist_from {
    ($a:ident, $($bs:ident),*) => {
        #[allow(dead_code, unused_variables, non_snake_case)]
        impl<$a, $($bs),*> From<($a, $($bs),*)> for HList![$a, $($bs),*] {
            fn from(($a, $($bs),*): ($a, $($bs),*)) -> HList![$a, $($bs),*] {
                hlist![$a, $($bs),*]
            }
        }
    };
    ($a:ident) => {
        #[allow(dead_code, unused_variables, non_snake_case)]
        impl<$a> From<($a,)> for HList![$a] {
            fn from(($a,): ($a,)) -> HList![$a] {
                hlist![$a]
            }
        }
    };
}
