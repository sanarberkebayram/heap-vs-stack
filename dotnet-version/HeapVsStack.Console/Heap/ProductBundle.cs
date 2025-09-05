
namespace HeapVsStack.Console.Heap;

public class ProductBundle : IProductComponent
{
    private readonly string _name;
    private readonly List<IProductComponent> _components = new();
    private IDiscountStrategy _discountStrategy;

    public ProductBundle(string name, IDiscountStrategy discountStrategy)
    {
        _name = name;
        _discountStrategy = discountStrategy;
    }

    public void Add(IProductComponent component)
    {
        _components.Add(component);
    }

    public double GetTotalPrice()
    {
        return _discountStrategy.ApplyDiscount(_components.Sum(c => c.GetTotalPrice()));
    }

    public void Display(int depth)
    {
        System.Console.WriteLine(new string(' ', depth) + $"- {_name} | Total: {GetTotalPrice():F2}₺ | Strategy: {_discountStrategy.StrategyName}");
        foreach (var component in _components)
        {
            component.Display(depth + 2);
        }
    }

    public void SetDiscountStrategy(IDiscountStrategy strategy)
    {
        _discountStrategy = strategy;
    }

    public static ProductBundle NewTest()
    {
        var bundle = new ProductBundle("Gaming Bundle", new PercentageDiscount(15.0));
        for (int i = 0; i < 100000; i++)
        {
            bundle.Add(new Product($"Product {i}", (i + 1) * 100.0, new NoDiscount()));
        }
        return bundle;
    }
}
