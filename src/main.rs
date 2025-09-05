use crate::{ecommerce_heap::heap_test, ecommerce_stack::stack_test};

pub mod ecommerce_heap;
pub mod ecommerce_stack;

fn main() {
    println!("Heap");
    heap_test();

    println!("Stack");
    stack_test();
}
