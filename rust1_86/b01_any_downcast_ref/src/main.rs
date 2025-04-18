use std::any::Any;

trait MyAny: Any {}

impl<T: Any> MyAny for T {}

impl dyn MyAny {
    fn downcast_ref<T: Any>(&self) -> Option<&T> {
        (self as &dyn Any).downcast_ref()
    }
}

fn main() {
    let value: i32 = 123;

    // Store the value behind a trait object
    let my_any_ref: &dyn MyAny = &value;

    // Attempt to downcast to the original type
    if let Some(downcasted) = my_any_ref.downcast_ref::<i32>() {
        println!("Successfully downcasted: {}", downcasted);
    } else {
        println!("Downcasting failed.");
    }
}
