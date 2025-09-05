
namespace HeapVsStack.Console.Stack;

public readonly struct Product<S> : IProductComponent where S : IDiscountStrategy
{
    private readonly string _name;
    private readonly double _price;
    private readonly S _discountStrategy;

    public Product(string name, double price, S discountStrategy)
    {
        _name = name;
        _price = price;
        _discountStrategy = discountStrategy;
    }

    public double GetTotalPrice()
    {
        return _discountStrategy.ApplyDiscount(_price);
    }

    public void Display(int depth)
    {
        System.Console.WriteLine(new string(' ', depth) + $"- {_name} | Price: {GetTotalPrice():F2}₺ | Strategy: {_discountStrategy.StrategyName}");
    }

    public Product<T> SetStrategy<T>(T strategy) where T : IDiscountStrategy
    {
        return new Product<T>(_name, _price, strategy);
    }
}
