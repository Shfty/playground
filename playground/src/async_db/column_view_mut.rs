use std::ops::{Deref, DerefMut};

use super::{BorrowColumn, ColumnCollection, WriteColumn};

/// A view into one a [`Column`]
#[derive(Debug)]
pub struct ColumnViewMut<'a, T> {
    column_guard: WriteColumn<'a, T>,
}

impl<'a, T> ColumnViewMut<'a, T> {
    pub async fn new<DB>(db: &'a DB) -> ColumnViewMut<'a, T>
    where
        T: 'a,
        DB: BorrowColumn<T>,
    {
        let column = db.borrow();
        let column_guard = column.write().await;
        ColumnViewMut { column_guard }
    }

    pub fn column(&self) -> &ColumnCollection<T> {
        self.column_guard.deref()
    }

    pub fn column_mut(&mut self) -> &mut ColumnCollection<T> {
        self.column_guard.deref_mut()
    }
}

impl<'a, T> Deref for ColumnViewMut<'a, T> {
    type Target = ColumnCollection<T>;

    fn deref(&self) -> &Self::Target {
        self.column()
    }
}

impl<'a, T> DerefMut for ColumnViewMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.column_mut()
    }
}