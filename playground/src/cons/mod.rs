mod hlist;
pub use hlist::*;

pub struct Cons<A, B>(A, B);

pub fn cons<A, B>(a: A, b: B) -> Cons<A, B> {
    Cons(a, b)
}

impl<A, B> Cons<A, B> {
    pub fn car(&self) -> &A {
        &self.0
    }

    pub fn cdr(&self) -> &B {
        &self.1
    }
    pub fn into_car(self) -> A {
        self.0
    }

    pub fn into_cdr(self) -> B {
        self.1
    }
}

impl<A, B> From<(A, B)> for Cons<A, B> {
    fn from((a, b): (A, B)) -> Self {
        Cons(a, b)
    }
}

impl<A, B> Into<(A, B)> for Cons<A, B> {
    fn into(self) -> (A, B) {
        (self.0, self.1)
    }
}

impl<'a, A, B> Into<(&'a A, &'a B)> for &'a Cons<A, B> {
    fn into(self) -> (&'a A, &'a B) {
        (&self.0, &self.1)
    }
}
