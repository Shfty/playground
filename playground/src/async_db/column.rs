use super::ColumnLock;
use std::{borrow::{Borrow, BorrowMut}, ops::{Deref, DerefMut}};

/// A collection of row structs
#[derive(Debug, Default)]
pub struct Column<T>(ColumnLock<T>);

impl<T> Deref for Column<T> {
    type Target = ColumnLock<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Column<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A type that can borrow a table containing some type `T`
pub trait BorrowColumn<T>: Borrow<Column<T>> {}
impl<T, U> BorrowColumn<T> for U where U: Borrow<Column<T>> {}

/// A type that can mutably borrow a table containing some type `T`
pub trait BorrowColumnMut<T>: BorrowMut<Column<T>> {}
impl<T, U> BorrowColumnMut<T> for U where U: BorrowMut<Column<T>> {}
