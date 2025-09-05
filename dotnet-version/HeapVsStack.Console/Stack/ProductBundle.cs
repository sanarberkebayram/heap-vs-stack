
namespace HeapVsStack.Console.Stack;

public readonly struct ProductBundle<C, S> where C : IProductComponent where S : IDiscountStrategy
{
    private readonly string _name;
    private readonly C[] _components;
    private readonly S _discountStrategy;

    public ProductBundle(string name, C[] components, S discountStrategy)
    {
        _name = name;
        _components = components;
        _discountStrategy = discountStrategy;
    }

    public double GetTotalPrice()
    {
        double sum = 0;
        foreach (var component in _components)
        {
            sum += component.GetTotalPrice();
        }
        return _discountStrategy.ApplyDiscount(sum);
    }

    public void Display(int depth)
    {
        System.Console.WriteLine(new string(' ', depth) + $"- {_name} | Total: {GetTotalPrice():F2}₺ | Strategy: {_discountStrategy.StrategyName}");
        foreach (var component in _components)
        {
            component.Display(depth + 2);
        }
    }

    public ProductBundle<C, T> SetDiscountStrategy<T>(T strategy) where T : IDiscountStrategy
    {
        return new ProductBundle<C, T>(_name, _components, strategy);
    }

    public static ProductBundle<Product<NoDiscount>, PercentageDiscount> NewTest()
    {
        var components = new Product<NoDiscount>[100000];
        for (int i = 0; i < 100000; i++)
        {
            components[i] = new Product<NoDiscount>($"Product {i}", (i + 1) * 100.0, new NoDiscount());
        }

        var gamingBundle = new ProductBundle<Product<NoDiscount>, PercentageDiscount>(
            "Gaming Bundle",
            components,
            new PercentageDiscount(15.0)
        );
        return gamingBundle;
    }
}
