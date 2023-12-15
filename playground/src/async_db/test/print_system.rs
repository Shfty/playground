use futures::StreamExt;

use crate::async_db::{BorrowColumn, IntFloatCharRow, Row};

pub async fn print_system<T>(table: &T)
where
    T: BorrowColumn<i32> + BorrowColumn<f32> + BorrowColumn<char> + Send + Sync,
{
    let keys = IntFloatCharRow::common_keys(table).await;
    
    let stream = async_std::stream::from_iter(keys)
        .then(move |i| async move { (i, IntFloatCharRow::new(table, i).await) });

    futures::pin_mut!(stream);

    while let Some((
        key,
        IntFloatCharRow {
            int,
            float,
            mut char,
        },
    )) = stream.next().await
    {
        *char = (*int as u8).into();
        println!(
            "{:?}, Int: {}, Float: {}, Char: {}",
            key, *int, *float, *char
        );
    }
}
