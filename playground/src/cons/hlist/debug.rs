#[macro_export]
macro_rules! impl_hlist_debug {
    ($a:ident, $($bs:ident),*) => {
        #[allow(dead_code, unused_variables)]
        impl<$a, $($bs),*> Debug for HList![$a, $($bs),*]
        where
            $a: Debug,
            $(
                $bs: Debug,
            )*
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut debug_list = f.debug_list();
                let (entry, list) = (&self.0, &self.1);
                debug_list.entry(&entry);
                $(
                let _: $bs;
                let (entry, list) = (&list.0, &list.1);
                debug_list.entry(&entry);
                )*
                debug_list.finish()
            }
        }
    };
    ($a:ident) => {
        #[allow(dead_code, unused_variables)]
        impl<$a> Debug for HList![$a]
        where
            $a: Debug,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut debug_list = f.debug_list();
                debug_list.entry(&self.0);
                debug_list.finish()
            }
        }
    };
}
