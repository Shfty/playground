/*
Example of static conversion between two known trait types
    Use case: Public APIs where 'dyn Any' should be avoided for the sake of clarity
    Pattern:
        Define A-to-B conversion as a separate trait with 'as_*' methods
        Use blanket implementation to apply conversion trait to all types that share to/from traits
        Blanket impl approach avoids coupling base traits to conversions
    FooBarTrait demonstrates packing both conversions into one trait for types that are mutually inclusive
*/

use std::fmt::Debug;

#[derive(Debug)]
struct FooStruct {}

impl FooTrait for FooStruct {}
impl BarTrait for FooStruct {}

trait FooTrait: Debug {
    fn do_foo(&self) {
        println!("Foo");
    }
}

trait FooAsBarTrait: FooTrait {
    fn as_bar(&self) -> &dyn BarTrait;
}

impl<T> FooAsBarTrait for T
where
    T: FooTrait + BarTrait + 'static,
{
    fn as_bar(&self) -> &dyn BarTrait {
        self
    }
}

trait BarTrait: Debug {
    fn do_bar(&self) {
        println!("Bar");
    }
}

trait BarAsFooTrait: BarTrait {
    fn as_foo(&self) -> &dyn FooTrait;
}

impl<T> BarAsFooTrait for T
where
    T: FooTrait + BarTrait + 'static,
{
    fn as_foo(&self) -> &dyn FooTrait {
        self
    }
}

trait FooBarTrait: FooTrait + BarTrait {
    fn as_bar(&self) -> &dyn BarTrait;
    fn as_foo(&self) -> &dyn FooTrait;
}

impl<T> FooBarTrait for T
where
    T: FooTrait + BarTrait + 'static,
{
    fn as_bar(&self) -> &dyn BarTrait {
        self
    }

    fn as_foo(&self) -> &dyn FooTrait {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_to_trait() {
        let foo_struct: FooStruct = FooStruct {};

        let foo_trait = BarAsFooTrait::as_foo(&foo_struct);
        let bar_trait = FooAsBarTrait::as_bar(&foo_struct);

        println!(
            "Foo: {:?}, as FooTrait: {:?}, as BarTrait: {:?}",
            foo_struct, foo_trait, bar_trait
        );

        let foo_trait = FooBarTrait::as_foo(&foo_struct);
        let bar_trait = FooBarTrait::as_bar(&foo_struct);

        println!(
            "Foo: {:?}, as FooTrait: {:?}, as BarTrait: {:?}",
            foo_struct, foo_trait, bar_trait
        );
    }
}
