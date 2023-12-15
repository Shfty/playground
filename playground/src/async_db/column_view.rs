use std::ops::Deref;

use super::{BorrowColumn, ColumnCollection, ReadColumn};

/// A view into one a [`Column`]
#[derive(Debug)]
pub struct ColumnView<'a, T> {
    column_guard: ReadColumn<'a, T>,
}

impl<'a, T> ColumnView<'a, T> {
    pub async fn new<DB>(db: &'a DB) -> ColumnView<'a, T>
    where
        T: 'a,
        DB: BorrowColumn<T>,
    {
        let column = db.borrow();
        let column_guard = column.read().await;
        ColumnView { column_guard }
    }

    pub fn column(&'a self) -> &'a ColumnCollection<T> {
        self.column_guard.deref()
    }
}

impl<'a, T> Deref for ColumnView<'a, T> {
    type Target = ColumnCollection<T>;

    fn deref(&self) -> &Self::Target {
        self.column()
    }
}