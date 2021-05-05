use std::fmt::Debug;

trait RecursableKey: Copy {}

impl<T> RecursableKey for T where T: Copy {}

trait RecursiveTuple<K>
where
    K: RecursableKey,
{
    fn recurse(self, key: K);
}

impl<K, T0> RecursiveTuple<K> for (T0,)
where
    K: RecursableKey,
    T0: RecursiveTuple<K>,
{
    fn recurse(self, key: K) {
        self.0.recurse(key);
    }
}

impl<K, T0, T1> RecursiveTuple<K> for (T0, T1)
where
    K: RecursableKey,
    T0: RecursiveTuple<K>,
    T1: RecursiveTuple<K>,
{
    fn recurse(self, key: K) {
        self.0.recurse(key);
        self.1.recurse(key);
    }
}

impl<K, T0, T1, T2> RecursiveTuple<K> for (T0, T1, T2)
where
    K: RecursableKey,
    T0: RecursiveTuple<K>,
    T1: RecursiveTuple<K>,
    T2: RecursiveTuple<K>,
{
    fn recurse(self, key: K) {
        self.0.recurse(key);
        self.1.recurse(key);
        self.2.recurse(key);
    }
}

impl<K, T0, T1, T2, T3> RecursiveTuple<K> for (T0, T1, T2, T3)
where
    K: RecursableKey,
    T0: RecursiveTuple<K>,
    T1: RecursiveTuple<K>,
    T2: RecursiveTuple<K>,
    T3: RecursiveTuple<K>,
{
    fn recurse(self, key: K) {
        self.0.recurse(key);
        self.1.recurse(key);
        self.2.recurse(key);
        self.3.recurse(key);
    }
}

trait Recursable: Debug {
    fn finish_recursion(&self, key: &str) {
        println!("Finish recursion: {:?}, key: {:?}", self, key);
    }
}

impl Recursable for i32 {}

impl<T> RecursiveTuple<&str> for T
where
    T: Recursable + Debug,
{
    fn recurse(self, key: &str) {
        self.finish_recursion(key);
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recurse_tuple() {
        (
            (((0,),),),
            (((0,),), ((1,), (2, 3))),
            (((0,),), ((1,), (2, 3)), ((4,), (5, 6), (7, 8, 9))),
            (
                ((0,),),
                ((1,), (2, 3)),
                ((4,), (5, 6), (7, 8, 9)),
                ((10,), (11, 12), (13, 14, 15), (16, 17, 18, 19)),
            ),
        )
            .recurse("Hello");
    }
}
