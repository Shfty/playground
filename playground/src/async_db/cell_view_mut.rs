use std::ops::{Deref, DerefMut};
use std::{marker::PhantomPinned, pin::Pin, ptr::NonNull};

use super::{BorrowColumn, ColumnCollection, ColumnView, Key};

/// A view into one of the [`Cell`]s of a [`Column`]
#[derive(Debug)]
pub struct CellViewMut<'a, T>(Pin<Box<CellViewMutInner<'a, T>>>);

impl<'a, T> CellViewMut<'a, T> {
    pub async fn new<DB>(db: &'a DB, index: Key) -> CellViewMut<'a, T>
    where
        T: 'a,
        DB: BorrowColumn<T>,
    {
        CellViewMut(CellViewMutInner::new(db, index).await)
    }

    pub fn cell(&self) -> &T {
        self.0.deref().deref()
    }

    pub fn cell_mut(&mut self) -> &mut T {
        unsafe {
            let mut_ref = Pin::as_mut(&mut self.0);
            Pin::get_unchecked_mut(mut_ref).deref_mut()
        }
    }

    #[allow(dead_code)]
    pub fn column(&'a self) -> &'a ColumnCollection<T> {
        self.0.deref().column_guard.deref()
    }
}

impl<'a, T> Deref for CellViewMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.cell()
    }
}

impl<'a, T> DerefMut for CellViewMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.cell_mut()
    }
}

/// Inner workings of [`CellView`].
/// Self-referential struct that holds both the column and cell read guards
#[derive(Debug)]
struct CellViewMutInner<'a, T> {
    column_guard: ColumnView<'a, T>,
    item_guard: NonNull<T>,
    _pin: PhantomPinned,
}

impl<'a, T> CellViewMutInner<'a, T> {
    pub async fn new<DB>(db: &'a DB, index: Key) -> Pin<Box<CellViewMutInner<'a, T>>>
    where
        T: 'a,
        DB: BorrowColumn<T>,
    {
        let column_guard = ColumnView::new(db).await;

        let guard = CellViewMutInner {
            column_guard,
            item_guard: NonNull::dangling(),
            _pin: PhantomPinned,
        };
        let mut boxed = Box::pin(guard);

        let item_guard = NonNull::from(
            boxed
                .column_guard
                .get(&index)
                .unwrap()
                .write()
                .await
                .deref(),
        );

        unsafe {
            let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).item_guard = item_guard;
        }

        boxed
    }
}

impl<'a, T> Deref for CellViewMutInner<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.item_guard.as_ref() }
    }
}

impl<'a, T> DerefMut for CellViewMutInner<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.item_guard.as_mut() }
    }
}
