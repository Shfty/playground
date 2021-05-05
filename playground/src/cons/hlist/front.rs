#[macro_export]
macro_rules! impl_hlist_front {
    ($a:ident, $($bs:ident),*) => {
        #[allow(dead_code, unused_variables)]
        impl<$a, $($bs),*> HList![$a, $($bs),*] {
            pub fn push_front<New>(self, d: New) -> HList![New, $a, $($bs),*] {
                Cons(d, self)
            }

            pub fn pop_front(self) -> ($a, HList![$($bs),*]) {
                (self.0, self.1)
            }
        }
    };
    ($a:ident) => {
        #[allow(dead_code)]
        impl<$a> HList![$a] {
            pub fn push_front<New>(self, d: New) -> HList![New, $a] {
                Cons(d, self)
            }

            pub fn pop_front(self) -> ($a, HList![HNil]) {
                (self.0, hlist![HNil])
            }
        }
    };
}
