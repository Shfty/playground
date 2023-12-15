use std::borrow::Borrow;

use crate::async_db::{Column, Row};

use super::IntFloatCharRow;

/// A user-created table struct holding columns
#[derive(Debug)]
pub struct MyTable {
    ints: Column<i32>,
    floats: Column<f32>,
    chars: Column<char>,
    strs: Column<&'static str>,
    strings: Column<String>,
}

impl MyTable {
    pub async fn new() -> Self {
        let table = MyTable {
            ints: Default::default(),
            floats: Default::default(),
            chars: Default::default(),
            strs: Default::default(),
            strings: Default::default(),
        };

        // Insert
        IntFloatCharRow::insert(&table, 0.into(), (1, 4.0, '7')).await;
        IntFloatCharRow::insert(&table, 1.into(), (2, 5.0, '8')).await;
        IntFloatCharRow::insert(&table, 2.into(), (2, 5.0, '8')).await;
        IntFloatCharRow::insert(&table, 3.into(), (3, 6.0, '9')).await;

        // Remove
        IntFloatCharRow::remove(&table, 1.into()).await;

        table
    }
}

// Practically speaking, a table is any struct you can borrow tables from
// So all tables should implement Borrow for their table members
// This should eventually be derived
impl Borrow<Column<i32>> for MyTable {
    fn borrow(&self) -> &Column<i32> {
        &self.ints
    }
}

impl Borrow<Column<f32>> for MyTable {
    fn borrow(&self) -> &Column<f32> {
        &self.floats
    }
}

impl Borrow<Column<char>> for MyTable {
    fn borrow(&self) -> &Column<char> {
        &self.chars
    }
}

impl Borrow<Column<&'static str>> for MyTable {
    fn borrow(&self) -> &Column<&'static str> {
        &self.strs
    }
}

impl Borrow<Column<String>> for MyTable {
    fn borrow(&self) -> &Column<String> {
        &self.strings
    }
}
