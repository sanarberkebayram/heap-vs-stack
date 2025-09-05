
using BenchmarkDotNet.Attributes;
using HeapVsStack.Console.Heap;
using HeapVsStack.Console.Stack;

[MemoryDiagnoser]
public class Benchmark
{
    private readonly ProductBundle _heapBundle = ProductBundle.NewTest();
    private readonly ProductBundle<HeapVsStack.Console.Stack.Product<HeapVsStack.Console.Stack.NoDiscount>, HeapVsStack.Console.Stack.PercentageDiscount> _stackBundle = HeapVsStack.Console.Stack.ProductBundle<HeapVsStack.Console.Stack.Product<HeapVsStack.Console.Stack.NoDiscount>, HeapVsStack.Console.Stack.PercentageDiscount>.NewTest();

    [Benchmark]
    public double HeapGetTotalPrice()
    {
        return _heapBundle.GetTotalPrice();
    }

    [Benchmark]
    public double StackGetTotalPrice()
    {
        return _stackBundle.GetTotalPrice();
    }
}
