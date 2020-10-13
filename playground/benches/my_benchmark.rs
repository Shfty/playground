use fnv::FnvHashMap;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn test_hash_map(hash_map: FnvHashMap<u8, char>) -> String {
    let mut string = String::new();

    for i in 0..255u8 {
        string.push(hash_map[&i]);
        string.push(' ');
    }

    string
}

fn test_vec_map(vec_map: Vec<(u8, char)>) -> String {
    let mut string = String::new();

    for i in 0..255u8 {
        let idx = vec_map.binary_search_by_key(&i, |(c, _)| *c).unwrap();
        string.push(vec_map[idx].1);
    }

    string
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("HashMap", |b| {
        b.iter(|| {
            let hm: FnvHashMap<u8, char> = (0..255u8).map(|i| (i, i as char)).collect();

            test_hash_map(black_box(hm))
        })
    });

    c.bench_function("VecMap", |b| {
        b.iter(|| {
            test_vec_map(black_box(
                (0..255u8)
                    .map(|i| (i, i as char))
                    .collect::<Vec<(u8, char)>>(),
            ))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
