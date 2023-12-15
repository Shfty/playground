/// Allows using .then(f) to apply function f to self and return the result
/// Useful for running a series of functions on a value without using intermediate variable bindings
pub trait Map<R>: Sized {
    fn map(self, f: impl FnOnce(Self) -> R) -> R {
        f(self)
    }
}

/// Implement Then for any Sized type
impl<T: Sized, R> Map<R> for T {}

fn main() {
    let foo = Some(1234);

    Option::map(foo, |foo| foo + 1); // Explicit Option call
    Map::map(foo, |_| Some(1235)); // Explicit Map call

    foo.map(|foo| foo + 1); // Implicitly uses Option implementation
    foo.map(|_| Some(1235)); // Implicitly uses Option implementation
}
