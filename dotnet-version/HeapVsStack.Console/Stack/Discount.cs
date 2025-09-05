
namespace HeapVsStack.Console.Stack;

public interface IDiscountStrategy
{
    double ApplyDiscount(double price);
    string StrategyName { get; }
}

public readonly struct NoDiscount : IDiscountStrategy
{
    public double ApplyDiscount(double price) => price;
    public string StrategyName => "No Discount";
}

public readonly struct PercentageDiscount : IDiscountStrategy
{
    private readonly double _percentage;

    public PercentageDiscount(double percentage)
    {
        _percentage = percentage;
    }

    public double ApplyDiscount(double price) => price * (1.0 - _percentage / 100.0);
    public string StrategyName => "Percentage Discount";
}

public readonly struct FixedDiscount : IDiscountStrategy
{
    private readonly double _amount;

    public FixedDiscount(double amount)
    {
        _amount = amount;
    }

    public double ApplyDiscount(double price) => Math.Max(0, price - _amount);
    public string StrategyName => "Fixed Discount";
}
