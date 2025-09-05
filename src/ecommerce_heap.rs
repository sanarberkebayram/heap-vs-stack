use std::cell::RefCell;

// ---------------- Strategy Pattern ----------------
pub trait DiscountStrategy {
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
        "İndirimsiz"
    }
}

impl DiscountStrategy for PercentageDiscount {
    fn apply_discount(&self, price: f64) -> f64 {
        price * (1.0 - self.0 / 100.0)
    }
    fn strategy_name(&self) -> &'static str {
        "Yüzde İndirim"
    } // statik string ile hızlı
}

impl DiscountStrategy for FixedDiscount {
    fn apply_discount(&self, price: f64) -> f64 {
        (price - self.0).max(0.0)
    }
    fn strategy_name(&self) -> &'static str {
        "Sabit İndirim"
    }
}

// ---------------- Composite Pattern ----------------
pub trait ProductComponent {
    fn get_total_price(&self) -> f64;
    fn display(&self, depth: usize);
    fn set_discount_strategy(&mut self, strategy: &'static dyn DiscountStrategy);
}

// Leaf
pub struct Product<'a> {
    name: &'a str,
    price: f64,
    discount_strategy: &'static dyn DiscountStrategy,
}

impl<'a> Product<'a> {
    pub fn new(name: &'a str, price: f64, strategy: &'static dyn DiscountStrategy) -> Self {
        Self {
            name,
            price,
            discount_strategy: strategy,
        }
    }
}

impl<'a> ProductComponent for Product<'a> {
    fn get_total_price(&self) -> f64 {
        self.discount_strategy.apply_discount(self.price)
    }
    fn display(&self, depth: usize) {
        println!(
            "{}Ürün: {} | Fiyat: {:.2}₺ | Strateji: {}",
            "-".repeat(depth),
            self.name,
            self.get_total_price(),
            self.discount_strategy.strategy_name()
        );
    }
    fn set_discount_strategy(&mut self, strategy: &'static dyn DiscountStrategy) {
        self.discount_strategy = strategy;
    }
}

// Composite
pub struct ProductBundle<'a> {
    name: &'a str,
    components: Vec<Box<dyn ProductComponent + 'a>>,
    discount_strategy: &'static dyn DiscountStrategy,
}

impl<'a> ProductBundle<'a> {
    fn new(name: &'a str, strategy: &'static dyn DiscountStrategy) -> Self {
        Self {
            name,
            components: Vec::new(),
            discount_strategy: strategy,
        }
    }
    fn add(&mut self, component: Box<dyn ProductComponent + 'a>) {
        self.components.push(component);
    }

    pub fn new_test() -> Self {
        let mut a = Self {
            name: "Oyun Paketi",
            components: Vec::new(),
            discount_strategy: &PERCENTAGE_15,
        };
        a.add(Box::new(Product::new(
            "Dizüstü Bilgisayar",
            15000.0,
            &NO_DISCOUNT,
        )));
        a.add(Box::new(Product::new("Oyuncu Mouse", 1200.0, &NO_DISCOUNT)));
        a.add(Box::new(Product::new(
            "Mekanik Klavye",
            800.0,
            &NO_DISCOUNT,
        )));

        a
    }
}

impl<'a> ProductComponent for ProductBundle<'a> {
    fn get_total_price(&self) -> f64 {
        self.discount_strategy
            .apply_discount(self.components.iter().map(|c| c.get_total_price()).sum())
    }
    fn display(&self, depth: usize) {
        println!(
            "{}Paket: {} | Toplam: {:.2}₺ | Strateji: {}",
            "-".repeat(depth),
            self.name,
            self.get_total_price(),
            self.discount_strategy.strategy_name()
        );
        for c in &self.components {
            c.display(depth + 2);
        }
    }
    fn set_discount_strategy(&mut self, strategy: &'static dyn DiscountStrategy) {
        self.discount_strategy = strategy;
    }
}

// ---------------- Observer Pattern ----------------
trait PriceObserver {
    fn update(&self, new_price: f64, product_name: &str);
}

struct PriceDisplayObserver;

impl PriceObserver for PriceDisplayObserver {
    fn update(&self, new_price: f64, product_name: &str) {
        println!(
            "💰 Fiyat Güncellendi: {} -> {:.2}₺",
            product_name, new_price
        );
    }
}

struct ObservableProduct<'a> {
    product: Product<'a>,
    observers: RefCell<Vec<&'a dyn PriceObserver>>,
}

impl<'a> ObservableProduct<'a> {
    fn new(name: &'a str, price: f64, strategy: &'static dyn DiscountStrategy) -> Self {
        Self {
            product: Product::new(name, price, strategy),
            observers: RefCell::new(vec![]),
        }
    }
    fn add_observer(&self, observer: &'a dyn PriceObserver) {
        self.observers.borrow_mut().push(observer);
    }
    fn notify(&self) {
        let price = self.product.get_total_price();
        for obs in self.observers.borrow().iter() {
            obs.update(price, self.product.name);
        }
    }
}

impl<'a> ProductComponent for ObservableProduct<'a> {
    fn get_total_price(&self) -> f64 {
        self.product.get_total_price()
    }
    fn display(&self, depth: usize) {
        self.product.display(depth);
    }
    fn set_discount_strategy(&mut self, strategy: &'static dyn DiscountStrategy) {
        self.product.set_discount_strategy(strategy);
        self.notify();
    }
}

// ---------------- Demo ----------------
pub static NO_DISCOUNT: NoDiscount = NoDiscount;
static PERCENTAGE_15: PercentageDiscount = PercentageDiscount(15.0);
static PERCENTAGE_20: PercentageDiscount = PercentageDiscount(20.0);
static FIXED_2000: FixedDiscount = FixedDiscount(2000.0);
static FIXED_200: FixedDiscount = FixedDiscount(200.0);

pub fn heap_test() {
    let mut gaming_bundle = ProductBundle::new("Oyun Paketi", &PERCENTAGE_15);
    gaming_bundle.add(Box::new(Product::new(
        "Dizüstü Bilgisayar",
        15000.0,
        &NO_DISCOUNT,
    )));
    gaming_bundle.add(Box::new(Product::new("Oyuncu Mouse", 1200.0, &NO_DISCOUNT)));
    gaming_bundle.add(Box::new(Product::new(
        "Mekanik Klavye",
        800.0,
        &NO_DISCOUNT,
    )));

    gaming_bundle.display(0);

    gaming_bundle.set_discount_strategy(&PERCENTAGE_20);
    println!("\nStrateji değiştirildikten sonra:");
    gaming_bundle.display(0);
}
