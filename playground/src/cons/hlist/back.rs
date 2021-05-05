#[macro_export]
macro_rules! impl_hlist_back {
    ($a:ident, $($bs:ident),*) => {
        #[allow(dead_code, unused_variables, non_snake_case)]
        impl<$a, $($bs),*> HList![$a, $($bs),*] {
            pub fn push_back<New>(self, d: New) -> HList![$a, $($bs),*, New] {
                let ($a, $($bs),*) = self.into();
                hlist![$a, $($bs),*, d]
            }
        }
    };
    ($a:ident) => {
        #[allow(dead_code)]
        impl<$a> HList![$a] {
            pub fn push_back<New>(self, d: New) -> HList![$a, New] {
                hlist![self.0, d]
            }

            pub fn pop_back(self) -> (HList![HNil], $a) {
                (hlist![HNil], self.0)
            }
        }
    };
}
