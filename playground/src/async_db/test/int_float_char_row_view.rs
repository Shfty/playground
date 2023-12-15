use std::collections::BTreeSet;

use crate::async_db::{
    BorrowColumn, CellView, CellViewMut, ColumnView, ColumnViewMut, Key, Row,
};

use async_trait::async_trait;

/// A user-created row query result holding references to table cells.
/// Used as the output type for table queries.
#[derive(Debug)]
pub struct IntFloatCharRow<'a> {
    pub int: CellView<'a, i32>,
    pub float: CellView<'a, f32>,
    pub char: CellViewMut<'a, char>,
}

#[async_trait(?Send)]
impl<'a, DB> Row<'a, DB> for IntFloatCharRow<'a>
where
    DB: BorrowColumn<i32> + BorrowColumn<f32> + BorrowColumn<char> + Send + Sync,
{
    type Insert = (i32, f32, char);

    async fn new(db: &'a DB, key: Key) -> Self {
        let int = CellView::<i32>::new(db, key).await;
        let float = CellView::<f32>::new(db, key).await;
        let char = CellViewMut::<char>::new(db, key).await;

        IntFloatCharRow { int, float, char }
    }

    async fn insert(db: &'a DB, key: Key, (int, float, char): (i32, f32, char)) {
        let mut ints = ColumnViewMut::<i32>::new(db).await;
        let mut floats = ColumnViewMut::<f32>::new(db).await;
        let mut chars = ColumnViewMut::<char>::new(db).await;

        ints.insert(key, int.into());
        floats.insert(key, float.into());
        chars.insert(key, char.into());
    }

    async fn remove(db: &'a DB, key: Key) {
        let mut ints = ColumnViewMut::<i32>::new(db).await;
        let mut floats = ColumnViewMut::<f32>::new(db).await;
        let mut chars = ColumnViewMut::<char>::new(db).await;

        ints.remove(&key);
        floats.remove(&key);
        chars.remove(&key);
    }

    async fn keys(db: &'a DB) -> BTreeSet<Key> {
        let ints = ColumnView::<i32>::new(db).await;
        let floats = ColumnView::<f32>::new(db).await;
        let chars = ColumnView::<char>::new(db).await;

        std::iter::empty()
            .chain(ints.keys())
            .chain(floats.keys())
            .chain(chars.keys())
            .copied()
            .collect::<BTreeSet<_>>()
    }

    async fn common_keys(db: &'a DB) -> BTreeSet<Key> {
        let ints = ColumnView::<i32>::new(db).await;
        let floats = ColumnView::<f32>::new(db).await;
        let chars = ColumnView::<char>::new(db).await;

        Self::keys(db)
            .await
            .into_iter()
            .filter(move |key| {
                ints.contains_key(key) && floats.contains_key(key) && chars.contains_key(key)
            })
            .collect::<BTreeSet<_>>()
    }
}
