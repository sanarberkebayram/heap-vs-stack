use criterion::{Criterion, criterion_group, criterion_main};
use heap_vs_stack::ecommerce_heap;
use heap_vs_stack::ecommerce_heap::ProductComponent;
use heap_vs_stack::ecommerce_stack;

fn benchmark_heap(c: &mut Criterion) {
    let heap_bundle = ecommerce_heap::ProductBundle::new_test();
    c.bench_function("heap_get_total_price", |b| {
        b.iter(|| heap_bundle.get_total_price())
    });
}

fn benchmark_stack(c: &mut Criterion) {
    let stack_bundle = ecommerce_stack::new_test();

    c.bench_function("heap_get_total_price", |b| {
        b.iter(|| stack_bundle.get_total_price())
    });
}

criterion_group!(benches, benchmark_heap, benchmark_stack);
criterion_main!(benches);
