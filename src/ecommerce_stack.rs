trait DiscountStrategy {
    fn apply_discount(&self, price: f64) -> f64;
    fn strategy_name(&self) -> &'static str;
}

pub struct NoDiscount;
pub struct PercentageDiscount(f64);
pub struct FixedDiscount(f64);

impl DiscountStrategy for NoDiscount {
    fn apply_discount(&self, price: f64) -> f64 {
        price
    }
    fn strategy_name(&self) -> &'static str {
        "No Discount"
    }
}

impl DiscountStrategy for PercentageDiscount {
    fn apply_discount(&self, price: f64) -> f64 {
        price * (1.0 - self.0 / 100.0)
    }
    fn strategy_name(&self) -> &'static str {
        "Percentage Discount"
    }
}

impl DiscountStrategy for FixedDiscount {
    fn apply_discount(&self, price: f64) -> f64 {
        (price - self.0).max(0.0)
    }
    fn strategy_name(&self) -> &'static str {
        "Fixed Discount"
    }
}

trait ProductComponent {
    fn get_total_price(&self) -> f64;
    fn display(&self, depth: usize);
    fn set_discount_strategy(&mut self);
}

pub struct Product<S: DiscountStrategy> {
    name: &'static str,
    price: f64,
    discount_strategy: S,
}

impl<S: DiscountStrategy> Product<S> {
    fn new(name: &'static str, price: f64, strategy: S) -> Self {
        Self {
            name,
            price,
            discount_strategy: strategy,
        }
    }

    fn set_strategy<T: DiscountStrategy>(self, strategy: T) -> Product<T> {
        Product {
            name: self.name,
            price: self.price,
            discount_strategy: strategy,
        }
    }
}

impl<S: DiscountStrategy> ProductComponent for Product<S> {
    fn get_total_price(&self) -> f64 {
        self.discount_strategy.apply_discount(self.price)
    }
    fn display(&self, depth: usize) {
        println!(
            "{}Product: {} | Price: {:.2}₺ | Strategy: {}",
            "-".repeat(depth),
            self.name,
            self.get_total_price(),
            self.discount_strategy.strategy_name()
        );
    }
    fn set_discount_strategy(&mut self) {}
}

pub struct ProductBundle<C: ProductComponent, S: DiscountStrategy> {
    name: &'static str,
    components: Vec<C>,
    discount_strategy: S,
}

impl<C: ProductComponent, S: DiscountStrategy> ProductBundle<C, S> {
    pub fn new(name: &'static str, components: Vec<C>, strategy: S) -> Self {
        Self {
            name,
            components,
            discount_strategy: strategy,
        }
    }

    pub fn get_total_price(&self) -> f64 {
        let sum: f64 = self.components.iter().map(|c| c.get_total_price()).sum();
        self.discount_strategy.apply_discount(sum)
    }

    pub fn display(&self, depth: usize) {
        println!(
            "{}Bundle: {} | Total: {:.2}₺ | Strategy: {}",
            "-".repeat(depth),
            self.name,
            self.get_total_price(),
            self.discount_strategy.strategy_name()
        );
        for c in &self.components {
            c.display(depth + 2);
        }
    }

    pub fn set_discount_strategy<T: DiscountStrategy>(self, strategy: T) -> ProductBundle<C, T> {
        ProductBundle {
            name: self.name,
            components: self.components,
            discount_strategy: strategy,
        }
    }
}

pub fn stack_test() {
    let mut laptop = Product::new("Laptop", 15000.0, NoDiscount);
    let mut mouse = Product::new("Gaming Mouse", 1200.0, NoDiscount);
    let mut keyboard = Product::new("Mechanical Keyboard", 800.0, NoDiscount);

    let gaming_bundle = ProductBundle::new(
        "Gaming Bundle",
        vec![laptop, mouse, keyboard],
        PercentageDiscount(15.0),
    );

    gaming_bundle.display(0);

    let gaming_bundle_fixed = gaming_bundle.set_discount_strategy(FixedDiscount(3000.0));
    println!("
After changing the strategy:");
    gaming_bundle_fixed.display(0);
}

pub fn new_test() -> ProductBundle<Product<NoDiscount>, PercentageDiscount> {
    let laptop = Product::new("Laptop", 15000.0, NoDiscount);
    let mouse = Product::new("Gaming Mouse", 1200.0, NoDiscount);
    let keyboard = Product::new("Mechanical Keyboard", 800.0, NoDiscount);

    let gaming_bundle = ProductBundle::new(
        "Gaming Bundle",
        vec![laptop, mouse, keyboard],
        PercentageDiscount(15.0),
    );
    return gaming_bundle;
}
