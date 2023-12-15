mod cell_view;
mod cell_view_mut;
mod column;
mod column_view;
mod column_view_mut;
mod key;
mod row;
mod test;

pub use cell_view::*;
pub use cell_view_mut::*;
pub use column::*;
pub use column_view::*;
pub use column_view_mut::*;
pub use key::*;
pub use row::*;
pub use test::*;

use std::collections::BTreeMap;

use async_std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub type ColumnLock<T> = RwLock<ColumnCollection<T>>;
pub type ReadColumn<'a, T> = RwLockReadGuard<'a, ColumnCollection<T>>;
pub type WriteColumn<'a, T> = RwLockWriteGuard<'a, ColumnCollection<T>>;

pub type ColumnCollection<T> = BTreeMap<Key, CellLock<T>>;

pub type CellLock<T> = RwLock<T>;

pub async fn main() {
    let table = MyTable::new().await;
    print_system(&table).await;
}
