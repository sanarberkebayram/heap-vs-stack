
namespace HeapVsStack.Console.Heap;

public interface IProductComponent
{
    double GetTotalPrice();
    void Display(int depth);
    void SetDiscountStrategy(IDiscountStrategy strategy);
}
