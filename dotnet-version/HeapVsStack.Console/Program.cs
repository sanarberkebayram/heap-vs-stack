using HeapVsStack.Console.Heap;
using HeapVsStack.Console.Stack;

Console.WriteLine("Heap");
var gamingBundle = ProductBundle.NewTest();
gamingBundle.Display(0);

gamingBundle.SetDiscountStrategy(new HeapVsStack.Console.Heap.PercentageDiscount(20.0));
Console.WriteLine("\nAfter changing the strategy:");
gamingBundle.Display(0);

Console.WriteLine("\nStack");
var gamingBundleStack = HeapVsStack.Console.Stack.ProductBundle<HeapVsStack.Console.Stack.Product<HeapVsStack.Console.Stack.NoDiscount>, HeapVsStack.Console.Stack.PercentageDiscount>.NewTest();
gamingBundleStack.Display(0);

var gamingBundleStackFixed = gamingBundleStack.SetDiscountStrategy(new HeapVsStack.Console.Stack.FixedDiscount(3000.0));
Console.WriteLine("\nAfter changing the strategy:");
gamingBundleStackFixed.Display(0);
