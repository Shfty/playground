use std::ops::Deref;
use std::{marker::PhantomPinned, pin::Pin, ptr::NonNull};

use super::{BorrowColumn, ColumnCollection, ColumnView, Key};

/// A view into one of the [`Cell`]s of a [`Column`]
#[derive(Debug)]
pub struct CellView<'a, T>(Pin<Box<CellViewInner<'a, T>>>);

impl<'a, T> CellView<'a, T> {
    pub async fn new<DB>(db: &'a DB, index: Key) -> CellView<'a, T>
    where
        T: 'a,
        DB: BorrowColumn<T>,
    {
        CellView(CellViewInner::new(db, index).await)
    }

    pub fn cell(&'a self) -> &'a T {
        self.0.deref().deref()
    }

    #[allow(dead_code)]
    pub fn column(&'a self) -> &'a ColumnCollection<T> {
        self.0.deref().column_guard.deref()
    }
}

impl<'a, T> Deref for CellView<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.cell()
    }
}

/// Inner workings of [`CellView`].
/// Self-referential struct that holds both the column and cell read guards
#[derive(Debug)]
struct CellViewInner<'a, T> {
    column_guard: ColumnView<'a, T>,
    item_guard: NonNull<T>,
    _pin: PhantomPinned,
}

impl<'a, T> CellViewInner<'a, T> {
    pub async fn new<DB>(db: &'a DB, index: Key) -> Pin<Box<CellViewInner<'a, T>>>
    where
        T: 'a,
        DB: BorrowColumn<T>,
    {
        let column_guard = ColumnView::new(db).await;

        let guard = CellViewInner {
            column_guard,
            item_guard: NonNull::dangling(),
            _pin: PhantomPinned,
        };
        let mut boxed = Box::pin(guard);

        let item_guard =
            NonNull::from(boxed.column_guard.get(&index).unwrap().read().await.deref());

        unsafe {
            let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).item_guard = item_guard;
        }

        boxed
    }
}

impl<'a, T> Deref for CellViewInner<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.item_guard.as_ref() }
    }
}
