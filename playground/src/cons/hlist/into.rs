#[macro_export]
macro_rules! impl_hlist_into {
    ($a:ident, $($bs:ident),*) => {
        #[allow(dead_code, unused_variables, non_snake_case)]
        impl<$a, $($bs),*> Into<($a, $($bs),*)> for HList![$a, $($bs),*] {
            fn into(self) -> ($a, $($bs),*) {
                let list = self;
                let ($a, list) = (list.0, list.1);
                $(
                    let ($bs, list) = (list.0, list.1);
                )*

                ($a, $($bs),*)
            }
        }

        #[allow(dead_code, unused_variables, non_snake_case)]
        impl<'a, $a, $($bs),*> Into<(&'a $a, $(&'a $bs),*)> for &'a HList![$a, $($bs),*] {
            fn into(self) -> (&'a $a, $(&'a $bs),*) {
                let list = self;
                let ($a, list) = (&list.0, &list.1);
                $(
                    let ($bs, list) = (&list.0, &list.1);
                )*

                ($a, $($bs),*)
            }
        }
    };
    ($a:ident) => {
        #[allow(dead_code, unused_variables, non_snake_case)]
        impl<$a> Into<($a,)> for HList![$a] {
            fn into(self) -> ($a,) {
                (self.0,)
            }
        }

        #[allow(dead_code, unused_variables, non_snake_case)]
        impl<'a, $a> Into<(&'a $a,)> for &'a HList![$a] {
            fn into(self) -> (&'a $a,) {
                (&self.0,)
            }
        }
    };
}
