
namespace HeapVsStack.Console.Heap;

public class Product : IProductComponent
{
    private readonly string _name;
    private readonly double _price;
    private IDiscountStrategy _discountStrategy;

    public Product(string name, double price, IDiscountStrategy discountStrategy)
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

    public void SetDiscountStrategy(IDiscountStrategy strategy)
    {
        _discountStrategy = strategy;
    }
}
