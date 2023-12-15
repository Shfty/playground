use std::collections::BTreeSet;

use crate::async_db::Key;
use async_trait::async_trait;

/// A type that can act as a virtual table row, containing references to the underlying cell data.
#[async_trait(?Send)]
pub trait Row<'a, DB> {
    type Insert;

    async fn new(db: &'a DB, key: Key) -> Self;
    async fn insert(db: &'a DB, key: Key, row: Self::Insert);
    async fn remove(db: &'a DB, key: Key);
    async fn keys(db: &'a DB) -> BTreeSet<Key>;
    async fn common_keys(db: &'a DB) -> BTreeSet<Key>;
}
