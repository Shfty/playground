use std::{borrow::Borrow, ops::Deref, ops::DerefMut};

use async_std::sync::{RwLock, RwLockReadGuard};

use futures::{Future, Stream, StreamExt};

type TableInner<T> = Vec<RwLock<T>>;

/// A collection of row structs
#[derive(Debug, Default)]
struct Table<T>(TableInner<T>);

impl<T> Deref for Table<T> {
    type Target = TableInner<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Table<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A type that can borrow a table containing some type `T`
trait BorrowTable<T>: Borrow<Table<T>> {}
impl<T, U> BorrowTable<T> for U where U: Borrow<Table<T>> {}

/// A user-created database struct holding tables
#[derive(Debug, Default)]
struct Database {
    ints: Table<i32>,
    floats: Table<f32>,
    chars: Table<char>,
    strs: Table<&'static str>,
    strings: Table<String>,
}

// Practically speaking, a database is any struct you can borrow tables from
// So all databases should implement Borrow for their table members
// This should eventually be derived
impl Borrow<Table<i32>> for Database {
    fn borrow(&self) -> &Table<i32> {
        &self.ints
    }
}

impl Borrow<Table<f32>> for Database {
    fn borrow(&self) -> &Table<f32> {
        &self.floats
    }
}

impl Borrow<Table<char>> for Database {
    fn borrow(&self) -> &Table<char> {
        &self.chars
    }
}

impl Borrow<Table<&'static str>> for Database {
    fn borrow(&self) -> &Table<&'static str> {
        &self.strs
    }
}

impl Borrow<Table<String>> for Database {
    fn borrow(&self) -> &Table<String> {
        &self.strings
    }
}

// Since this database model is column-first, rows are 'virtual' - types containing references to the underlying data

/// A user-created row query result holding references to database cells
#[derive(Debug)]
struct IntCharRow<'a> {
    key: usize,
    int: RwLockReadGuard<'a, i32>,
    char: RwLockReadGuard<'a, char>,
}

impl<'a> IntCharRow<'a> {
    pub fn get(
        ints: &'a Table<i32>,
        chars: &'a Table<char>,
        key: usize,
    ) -> impl Future<Output = IntCharRow<'a>> {
        let (int, char) = (&ints[key], &chars[key]);
        async move {
            let (int, char) = futures::join!(int.read(), char.read());
            IntCharRow { key, int, char }
        }
    }
}

#[derive(Debug)]
struct IntCharRowQuery<'a> {
    ints: &'a Table<i32>,
    chars: &'a Table<char>,
}

impl<'a> IntCharRowQuery<'a> {
    pub fn new<DB>(db: &'a DB) -> Self
    where
        DB: BorrowTable<i32> + BorrowTable<char>,
    {
        let (ints, chars): (&Table<i32>, &Table<char>) = (db.borrow(), db.borrow());
        IntCharRowQuery { ints, chars }
    }

    pub fn keys(
        &self,
        keys: impl IntoIterator<Item = usize>,
    ) -> impl Stream<Item = IntCharRow<'a>> {
        let ints = self.ints;
        let chars = self.chars;
        async_std::stream::from_iter(keys.into_iter())
            .then(move |i| async move { IntCharRow::get(ints, chars, i).await })
    }
}

#[async_std::main]
pub async fn main() {
    let mut db = Database::default();
    db.ints.extend(vec![1, 2, 3].into_iter().map(Into::into));
    db.floats
        .extend(vec![4.0, 5.0, 6.0].into_iter().map(Into::into));
    db.chars
        .extend(vec!['7', '8', '9'].into_iter().map(Into::into));

    let query = IntCharRowQuery::new(&db);
    let stream = query.keys(vec![2, 1, 0]);
    futures::pin_mut!(stream);
    while let Some(IntCharRow { key, int, char }) = stream.next().await {
        println!("Key: {:?}, Int: {:?}, Char: {:?}", key, int, char);
    }
}
