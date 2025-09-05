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

---

# Heap vs. Stack Performance Comparison in .NET

## Purpose

This project extends the heap vs. stack comparison to the .NET ecosystem. It replicates the e-commerce scenario using C# to compare the performance of heap-allocated objects (classes) and stack-allocated objects (structs) for product components and discount strategies.

## Benchmarking

The project uses the BenchmarkDotNet library for benchmarking. The benchmarks measure the time it takes to calculate the total price of a product bundle for both the heap (class-based) and stack (struct-based) implementations.

### How to Run

To run the benchmarks yourself, navigate to the `dotnet-version/HeapVsStack.Benchmark` directory and use the following command:

```bash
dotnet run -c Release
```

## Rust vs. .NET Comparison

After running benchmarks with 100,000 products in both Rust and .NET, here's a summary of the performance:

| Language | Implementation | Mean Time (µs) |
|----------|----------------|----------------|
| Rust     | Heap           | 120.81         |
| Rust     | Stack          | 88.01          |
| C#       | Heap           | 466.50         |
| C#       | Stack          | 91.58          |

**Key Observations:**

*   **Rust generally outperforms C# in this CPU-bound micro-benchmark.** The advantage is more pronounced for the heap-based implementations, where Rust's lack of a garbage collector and direct memory management likely play a larger role.
    *   Rust Heap vs. C# Heap: Rust is approximately **3.86 times faster**.
*   **For stack-based implementations, Rust and C# are very close in performance.** This suggests that when C# can avoid heap allocations and dynamic dispatch (by using structs and generics, which enable monomorphization similar to Rust's), its JIT compiler can generate highly optimized code that rivals Rust's AOT compiled native code.
    *   Rust Stack vs. C# Stack: Rust is approximately **1.04 times faster**.
*   **The "heap vs. stack" performance difference is significant in both languages.** This reinforces the importance of choosing appropriate data structures and allocation strategies for performance-critical code.

**Important Considerations:**

*   **Micro-benchmark Limitations:** This is a micro-benchmark focused on a very specific CPU-bound task. Real-world applications involve many other factors (I/O, network, complex algorithms, UI rendering) where performance differences might be less pronounced or even reversed.
*   **Compilation Models:** Rust's Ahead-of-Time (AOT) compilation to native code provides predictable, low-level performance. C#'s Just-In-Time (JIT) compilation allows for aggressive runtime optimizations, which can be incredibly effective for hot code paths, as demonstrated by the initial "ZeroMeasurement" warnings.
*   **Developer Productivity:** C# and the .NET ecosystem often offer higher developer productivity due to features like garbage collection, a rich standard library, and powerful IDEs.

This comparison highlights that for raw CPU performance in low-level, tight-loop scenarios, Rust can offer an edge. However, C# with its optimized runtime can achieve very competitive performance, especially when leveraging value types and aggressive JITting. The choice between languages often depends on the specific project requirements, performance criticality, and developer ecosystem preferences.