/// Struct for treating a tuple as a collection
#[derive(Debug)]
pub struct Tup<D>(pub D);

impl Tup<()> {
    pub fn push<T0>(self, data: T0) -> Tup<(T0,)> {
        Tup((data,))
    }
}

impl<T0> Tup<(T0,)> {
    pub fn push<T1>(self, data: T1) -> Tup<(T0, T1)> {
        let Tup(self_data) = self;
        Tup((self_data.0, data))
    }

    pub fn pop(self) -> (Tup<()>, T0) {
        let Tup(self_data) = self;
        (Tup(()), self_data.0)
    }
}

macro_rules! impl_tup {
    ($(($x:ident, $nx:tt)),*, Pop($y:ident, $ny:tt), Push($z:ident)) => {
        impl<$($x,)* $y> Tup<($($x,)* $y)> {
            pub fn push<$z>(self, data: $z) -> Tup<($($x,)* $y, $z)> {
                let Tup(self_data) = self;
                Tup(($(self_data.$nx,)* self_data.$ny, data))
            }

            pub fn pop(self) -> (Tup<($($x,)*)>, $y) {
                let Tup(self_data) = self;
                (Tup(($(self_data.$nx,)*)), self_data.$ny)
            }
        }
    };
}

impl_tup!((T0, 0), Pop(T1, 1), Push(T2));
impl_tup!((T0, 0), (T1, 1), Pop(T2, 2), Push(T3));
impl_tup!((T0, 0), (T1, 1), (T2, 2), Pop(T3, 3), Push(T4));
impl_tup!((T0, 0), (T1, 1), (T2, 2), (T3, 3), Pop(T4, 4), Push(T5));
impl_tup!(
    (T0, 0),
    (T1, 1),
    (T2, 2),
    (T3, 3),
    (T4, 4),
    Pop(T5, 5),
    Push(T6)
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_pop() {
        let comp = Tup(());
        println!("Comp: {:?}", comp);
        let comp = comp.push(1);
        println!("Comp: {:?}", comp);
        let comp = comp.push("Two");
        println!("Comp: {:?}", comp);
        let comp = comp.push(3.0);
        println!("Comp: {:?}", comp);
        let comp = comp.push('4');
        println!("Comp: {:?}", comp);
        let (comp, four) = comp.pop();
        println!("Comp: {:?}", comp);
        let (comp, three) = comp.pop();
        println!("Comp: {:?}", comp);
        let (comp, two) = comp.pop();
        println!("Comp: {:?}", comp);
        let (comp, one) = comp.pop();
        println!("Comp: {:?}", comp);
        println!(
            "One: {:?}, Two: {:?}, Three: {:?}, Four: {:?}",
            one, two, three, four
        );
    }
}
