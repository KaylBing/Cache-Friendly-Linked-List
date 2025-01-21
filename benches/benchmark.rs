// benches/benchmark.rs
use Cache_Friendly_Linked_List::LinkedList;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;

fn benchmark_vec_linked_list(c: &mut Criterion) {
    let mut list = LinkedList::new();
    let mut rng = rand::thread_rng();

    c.bench_function("Vec LinkedList Push Front", |b| {
        b.iter(|| {
            list.push_front(black_box(rng.gen::<u8>()));
        });
    });

    c.bench_function("Vec LinkedList Traverse", |b| {
        b.iter(|| {
            list.print_list();
        });
    });
}

criterion_group!(benches, benchmark_vec_linked_list);
criterion_main!(benches);
