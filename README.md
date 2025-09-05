# Heap vs. Stack Performance Comparison in Rust

## Purpose

This project aims to compare the performance of heap and stack memory allocation in a Rust application. It uses a simple e-commerce scenario where a product bundle's total price is calculated. Two implementations are provided:

*   **`ecommerce_heap`**: This implementation uses heap allocation (`Box<dyn ...>`) for product components and discount strategies.
*   **`ecommerce_stack`**: This implementation uses stack allocation with generics for product components and discount strategies.

The goal is to demonstrate the performance difference between these two approaches.

## Benchmarking

The project uses the Criterion library for benchmarking. The benchmarks measure the time it takes to calculate the total price of a product bundle for both the heap and stack implementations.

### Results

The benchmarks were run 3 times on a machine with the following specifications:
* OS: darwin

The average results are as follows:

| Benchmark               | Time (ns) |
| ----------------------- | --------- |
| `heap_get_total_price`  | 5.3373    |
| `stack_get_total_price` | 1.4846    |

As you can see, the stack-based implementation is significantly faster than the heap-based one.

### How to Run

To run the benchmarks yourself, you can use the following command:

```bash
cargo bench
```

## Conclusion

The results clearly show that for this specific use case, stack allocation provides a significant performance advantage over heap allocation. This is expected, as stack allocation is generally faster than heap allocation.

This project serves as a practical example of the performance implications of choosing between heap and stack allocation in Rust.
