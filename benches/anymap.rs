use anymap::AnyMap;
use generimap::GeneriMap;

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
};

fn generimap_insert(c: &mut Criterion) {
    c.bench_function("Generimap Insert", |b| {
        let mut map = GeneriMap::new();
        b.iter(
            || {
                map.insert(black_box(42u8));
                map.insert(black_box("Hello World".to_string()));
                map.insert(black_box(vec![1u8, 2u8, 3u8]));
                map.insert(black_box(42u16));
            }
        )
    });
}

fn anymap_insert(c: &mut Criterion) {
    c.bench_function("Anymap Insert", |b| {
        let mut map = AnyMap::new();
        b.iter(
            || {
                map.insert(black_box(42u8));
                map.insert(black_box("Hello World".to_string()));
                map.insert(black_box(vec![1u8, 2u8, 3u8]));
                map.insert(black_box(42u16));
            }
        )
    });
}

fn generimap_get(c: &mut Criterion) {
    c.bench_function("Generimap Get", |b| {
        let mut map = GeneriMap::new();
        map.insert(black_box(42u8));
        map.insert(black_box("Hello World".to_string()));
        map.insert(black_box(vec![1u8, 2u8, 3u8]));
        map.insert(black_box(42u16));
        b.iter(
            || {
                let _ = black_box(map.get::<u8>());
                let _ = black_box(map.get::<String>());
                let _ = black_box(map.get::<Vec<u8>>());
                let _ = black_box(map.get::<u16>());
            }
        )
    });
}

fn anymap_get(c: &mut Criterion) {
    c.bench_function("Anymap Get", |b| {
        let mut map = AnyMap::new();
        map.insert(black_box(42u8));
        map.insert(black_box("Hello World".to_string()));
        map.insert(black_box(vec![1u8, 2u8, 3u8]));
        map.insert(black_box(42u16));
        b.iter(
            || {
                let _ = black_box(map.get::<u8>());
                let _ = black_box(map.get::<String>());
                let _ = black_box(map.get::<Vec<u8>>());
                let _ = black_box(map.get::<u16>());
            }
        )
    });
}

fn generimap_insert_remove(c: &mut Criterion) {
    c.bench_function("Generimap Insert/Remove", |b| {
        let mut map = GeneriMap::new();
        b.iter(
            || {
                map.insert(black_box(42u8));
                map.insert(black_box("Hello World".to_string()));
                map.insert(black_box(vec![1u8, 2u8, 3u8]));
                map.insert(black_box(42u16));
                map.remove::<u8>();
                map.remove::<String>();
                map.remove::<Vec<u8>>();
                map.remove::<u16>();
            }
        )
    });
}

fn anymap_insert_remove(c: &mut Criterion) {
    c.bench_function("Anymap Insert/Remove", |b| {
        let mut map = AnyMap::new();
        b.iter(
            || {
                map.insert(black_box(42u8));
                map.insert(black_box("Hello World".to_string()));
                map.insert(black_box(vec![1u8, 2u8, 3u8]));
                map.insert(black_box(42u16));
                map.remove::<u8>();
                map.remove::<String>();
                map.remove::<Vec<u8>>();
                map.remove::<u16>();
            }
        )
    });
}

criterion_group!(
    benches,
    generimap_insert,
    anymap_insert,
    generimap_get,
    anymap_get,
    generimap_insert_remove,
    anymap_insert_remove
);

criterion_main!(benches);
