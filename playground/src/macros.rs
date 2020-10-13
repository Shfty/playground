// AST Coercion
#[macro_export]
macro_rules! as_expr {
    ($e:expr) => {
        $e
    };
}
#[macro_export]
macro_rules! as_item {
    ($i:item) => {
        $i
    };
}

#[macro_export]
macro_rules! as_pat {
    ($p:pat) => {
        $p
    };
}

#[macro_export]
macro_rules! as_stmt {
    ($s:stmt) => {
        $s
    };
}

#[macro_export]
macro_rules! as_array {
    (
        $($tts:tt),*
    ) => {
        [$($tts),*];
    }
}

// Primitives
#[macro_export]
macro_rules ! skip {
    (
        [$head:tt $(, $tts:tt)*];
        [$n:tt $(, $ns:tt)*];
    ) => {
        skip! {
            [$($tts),*];
            [$($ns),*];
        }
    };

    (
        [$($tts:tt),*];
        [];
    ) => {
        [$($tts),*];
    };
}

#[test]
fn test_skip() {
    stringify!(1, 2, 3, 4, 5);
    let result = skip! {
        [1, 2, 3, 4, 5];
        [0, 0, 0];
    };
    assert_eq!(result, [4, 5]);
}

#[macro_export]
macro_rules ! take {
    // Calling Case
    (
        [$($tts:tt),*];
        [$($ns:tt),*];
    ) => {
        take! {
            ($($tts),*) -> ();
            [$($ns),*];
        }
    };

    (
        ($head:tt $(,$ltts:tt)*) -> ($($rtts:tt),*);
        [$n:tt $(,$ns:tt)*];
    ) => {
        take! {
            ($($ltts),*) -> ($($rtts,)* $head);
            [$($ns),*];
        }
    };

    (
        ($($ltts:tt),*) -> ($($rtts:tt),*);
        [];
    ) => {
        [$($rtts),*]
    };
}

#[test]
fn test_take() {
    stringify!(1, 2, 3, 4, 5);
    let result = take! {
        [1, 2, 3, 4, 5];
        [0, 0, 0];
    };
    assert_eq!(result, [1, 2, 3]);
}

#[macro_export]
macro_rules ! join {
    (
        [$($ltts:tt),*];
        [$($rtts:tt),*];
    ) => {
        [$($ltts),* $(, $rtts)*]
    };
}

#[test]
fn test_join() {
    stringify!(1, 2, 3, 4, 5);
    stringify!(6, 7, 8, 9, 10);
    let result = join! {
        [1, 2, 3, 4, 5];
        [6, 7, 8, 9, 10];
    };
    assert_eq!(result, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
}

#[macro_export]
macro_rules ! iter {
    (
        [$head:tt];
        $f:ident;
    ) => {
        $f!($head);
    };

    (
        [$head:tt $(, $tts:tt)*];
        $f:ident;
    ) => {
        $f!($head);
        iter!{
            [$($tts),*];
            $f;
        }
    };
}

#[test]
fn test_iter() {
    stringify!(1, 2, 3, 4, 5);

    let mut string = String::new();

    macro_rules! push_string {
        ($tt:tt) => {
            string.push_str(&$tt.to_string());
        };
    }

    iter! {
        [1, 2, 3, 4, 5];
        push_string;
    }

    assert_eq!(string, "12345");
}

#[macro_export]
macro_rules! iter_rev {
    // Calling Case
    (
        [$($tts:tt),*];
        $f:ident;
    ) => {
        iter_rev ! {
            ($($tts),*) -> ();
            $f;
        }
    };

    // Iteration Case
    (
        ($head:tt, $($ltts:tt),*) -> ($($rtts:tt),*);
        $f:ident;
    ) => {
        stringify!(
            ($head:tt, $($ltts:tt),+) -> ($($rtts:tt),*);
            $f:ident;
        );
        stringify!(
            ($($ltts),*) -> ($($rtts,)* $head);
            $f;
        );
        iter_rev ! {
            ($($ltts),*) -> ($($rtts,)* $head);
            $f;
        }
    };

    // Reset Case
    (
        ($head:tt) -> ($($rtts:tt),*);
        $f:ident;
    ) => {
        $f!($head);
        iter_rev ! {
            ($($rtts),*) -> ();
            $f;
        }
    };

    // Final Case
    (
        () -> ();
        $f:ident;
    ) => {}
}

#[test]
fn test_iter_rev() {
    stringify!(1, 2, 3, 4, 5);

    let mut string = String::new();

    macro_rules! push_string {
        ($tt:tt) => {
            string.push_str(&$tt.to_string());
        };
    }

    iter_rev! {
        [1, 2, 3, 4, 5];
        push_string;
    }

    assert_eq!(string, "54321");
}

#[macro_export]
macro_rules ! fold {
    (
        [$head:tt, $($tts:tt),*];
        $acc:expr;
        $f:ident;
    ) => {
        fold! {
            [$($tts),*];
            $f!($acc, $head);
            $f;
        };
    };

    (
        [$head:tt];
        $acc:expr;
        $f:ident;
    ) => {
        $f!($acc, $head);
    };
}

#[allow(dead_code)]
fn test_fold() {
    stringify!(1, 2, 3, 4, 5);

    macro_rules ! add {
        ($a:tt, $b: tt) => { $a + $b }
    }

    let result = fold! {
        [1, 2, 3, 4, 5];
        0;
        add;
    };

    assert_eq!(result, 1 + 2 + 3 + 4 + 5);
}

#[macro_export]
macro_rules ! fold_first {
    (
        [$head:tt $(,$tts:tt)*];
        $f:ident;
    ) => {
        fold! {
            [$($tts),*];
            $head;
            $f;
        }
    };
}

#[test]
fn test_fold_first() {
    stringify!(1, 2, 3, 4, 5);

    macro_rules ! add {
        ($a:tt, $b: tt) => { $a + $b }
    }

    let result = fold_first! {
        [1, 2, 3, 4, 5];
        add;
    };

    assert_eq!(result, 1 + 2 + 3 + 4 + 5);
}

#[macro_export]
macro_rules ! fold_set {
    // Iteration Case
    (
        [$head:tt, $($tts:tt),+];
        $f:ident
    ) => {
        $f!($head $(, $tts)*);
        fold_set! {
            [$($tts),*];
            $f
        }
    };

    // Final Case
    ([$head:tt]; $f:ident) => {
        $f!($head);
    };
}

#[test]
fn test_fold_set() {
    stringify!(1, 2, 3, 4, 5);
    fold_set! {
        [1, 2, 3, 4, 5];
        stringify
    }
}

#[macro_export]
macro_rules ! rfold_set {
    // Calling Case
    (
        [$($tts:tt),*];
        $f:ident
    ) => {
        rfold_set ! {
            ($($tts),*) -> ();
            $f
        }
    };

    // Iteration Case
    (
        ($head:tt, $($ltts:tt),*) -> ($($rtts:tt),*);
         $f:ident
    ) => {
        rfold_set ! {
            ($($ltts),*) -> ($($rtts,)* $head);
            $f
        }
    };

    // Reset Case
    (
        ($head:tt) -> ($($rtts:tt),*);
        $f:ident
    ) => {
        $f!($($rtts,)* $head);
        rfold_set ! {
            ($($rtts),*) -> ();
            $f
        }
    };

    // Final Case
    (
        () -> ();
        $f:ident
    ) => {}
}

#[test]
fn test_rfold_set() {
    stringify!(1, 2, 3, 4, 5);
    rfold_set! {
        [1, 2, 3, 4, 5];
        stringify
    }
}

#[macro_export]
macro_rules ! fold_set_permutations_inner {
    (
        [$($src:tt),+];
        [$($dst:tt),+];
        $f:ident;
    ) => {
        fold_set_permutations_inner! {
            ($($src),+) -> ();
            ($($dst),+) -> ();
            $f;
        }
    };

    (
        ($hsrc:tt $(, $lsrc:tt)*) -> ($($rsrc:tt),*);
        ($hdst:tt $(, $ldst:tt)*) -> ($($rdst:tt),*);
        $f:ident;
    ) => {
        $f!($hsrc $(,$lsrc)* $(,$rdst)*);

        fold_set_permutations_inner! {
            ($($lsrc),*) -> ($hsrc $(,$rsrc)*);
            ($($ldst),*) -> ($($rdst,)* $hdst);
            $f;
        }
    };

    (
        () -> ($($rsrc:tt),*);
        () -> ($($rdst:tt),*);
        $f:ident;
    ) => {
        $f!(
            $($rdst),*
        );
    };
}

#[macro_export]
macro_rules ! fold_set_permutations {
    // Top-level iteration case - pop elements from both arrays and recurse
    (
        [$lhead:tt, $($ltts:tt),+];
        [$rhead:tt, $($rtts:tt),+];
        $f:ident;
    ) => {
        fold_set_permutations_inner! {
            [$lhead, $($ltts),+];
            [$rhead, $($rtts),+];
            $f;
        }
        fold_set_permutations! {
            [$($ltts),+];
            [$($rtts),+];
            $f;
        }
    };
    // Top-level finish case
    (
        [$lhead:tt];
        [$rhead:tt];
        $f:ident;
    ) => {
        fold_set_permutations_inner! {
            [$lhead];
            [$rhead];
            $f;
        }
    };
}

#[allow(dead_code)]
fn test_fold_set_permutations() {
    fold_set_permutations! {
        [1, 2, 3];
        [A, B, C];
        stringify;
    }
}

// test_impl_trait
trait Constraint {}

trait Trait {}
trait TypedTrait<T> {}
