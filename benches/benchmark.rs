// benches/benchmark.rs
use Cache_Friendly_Linked_List::LinkedList;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
use std::collections::{VecDeque, LinkedList as StdLinkedList};

fn benchmark_vec_linked_list(c: &mut Criterion) {
    let mut list = LinkedList::new();
    let mut rng = rand::thread_rng();

    c.bench_function("Custom LinkedList Push Front", |b| {
        b.iter(|| {
            list.push_front(black_box(rng.gen::<u8>()));
        });
    });

    c.bench_function("Custom LinkedList Traverse", |b| {
        b.iter(|| {
            list.print_list();
        });
    });
}

fn benchmark_vec_deque(c: &mut Criterion) {
    let mut list = VecDeque::new();
    let mut rng = rand::thread_rng();

    c.bench_function("VecDeque Push Front", |b| {
        b.iter(|| {
            list.push_front(black_box(rng.gen::<u8>()));
        });
    });

    c.bench_function("VecDeque Traverse", |b| {
        b.iter(|| {
            for &value in list.iter() {
                black_box(value);
            }
        });
    });
}

fn benchmark_std_linked_list(c: &mut Criterion) {
    let mut list = StdLinkedList::new();
    let mut rng = rand::thread_rng();

    c.bench_function("Std LinkedList Push Front", |b| {
        b.iter(|| {
            list.push_front(black_box(rng.gen::<u8>()));
        });
    });

    c.bench_function("Std LinkedList Traverse", |b| {
        b.iter(|| {
            for value in list.iter() {
                black_box(value);
            }
        });
    });
}

criterion_group!(
    benches,
    benchmark_vec_linked_list,
    benchmark_vec_deque,
    benchmark_std_linked_list
);
criterion_main!(benches);