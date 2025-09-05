using System;
using BenchmarkDotNet;
using BenchmarkDotNet.Attributes;
using HeapVsStack.Console.Heap;
using HeapVsStack.Console.Stack;

namespace HeapVsStack.Benchmark
{
    public class Benchmarks
    {
        private readonly ProductBundle _heapBundle = ProductBundle.NewTest();
        private readonly ProductBundle<HeapVsStack.Console.Stack.Product<HeapVsStack.Console.Stack.NoDiscount>, HeapVsStack.Console.Stack.PercentageDiscount> _stackBundle = HeapVsStack.Console.Stack.ProductBundle<HeapVsStack.Console.Stack.Product<HeapVsStack.Console.Stack.NoDiscount>, HeapVsStack.Console.Stack.PercentageDiscount>.NewTest();
        [Benchmark]
        public void Scenario1()
        {
            _heapBundle.GetTotalPrice();
            return;
        }

        [Benchmark]
        public void Scenario2()
        {
            _stackBundle.GetTotalPrice();
            return;
        }
    }
}
