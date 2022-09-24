// https://stackoverflow.com/questions/64816028/how-can-i-determine-if-i-have-a-unique-arc-when-its-dropped
// https://doc.rust-lang.org/std/sync/struct.Arc.html

use std::sync::Arc;

#[derive(Debug)]
enum Action {
    One,
    Two,
    Three,
}

// Thing trait which operates on an Action, which should be a enum, allowing for
// different action sets.
trait Thing<T> {
    fn disconnected(&self);
    fn action(&self, action: T);
}

// There are many instances of an ActionController.
// There may be zero or more clones of an instance.
// The final drop of the instances should call thing.disconnected()
// In a multi-core environment, the same instance may be running on multiple cores
// ActionController should not be generic.
#[derive(Clone)]
struct ActionController {
    id: usize,
    thing: Arc<dyn Thing<Action>>,
}
impl ActionController {
    fn new(id: usize, thing: Box<dyn Thing<Action>>) -> Self {
        Self {
            id,
            thing: Arc::from(thing),
        }
    }
    fn invoke(&self, action: Action) {
        self.thing.action(action);
    }
}
// Decrement the reference counter, and drop and deallocate if it reaches zero.
// Note: Even if this doesn’t yet decrement the counter to zero,
// another thread might decrement it to zero and deallocate the data
// before we exit this function, or even before fetch_sub()  returns,
// invalidating our refcount reference. This is okay, though, because 
// it’s a reference to an AtomicUsize.

struct MyArc {}
impl Drop for MyArc<T> {
    fn drop(&mut self) {
        if refcount.fetch_sub(1, AcqRel) == 1 {
            unsafe {drop{Box::from_raw(ptr)}}
        }
    }
}
//
// To work around the drop issue, I've implemented Clone for ActionController which
// performs a fetch_add(1) on clone and a fetch_sub(1) on drop. This provides
// suficient information to call disconnected() -- but it just seems like there's
// got to be a better way.
impl Drop for ActionController {
    fn drop(&mut self) {
        // drop will be called for each clone of an Controller instance. When
        // the unique instance is dropped, disconnected() must be called
        self.thing.disconnected();
    }
}

struct Controlled {}
impl Thing<Action> for Controlled {
    fn disconnected(&self) {
        println!("disconnected")
    }
    fn action(&self, action: Action) {
        println!("action: {:#?}", action)
    }
}

fn bad() {
    let controlled = Controlled {};
    let controlled = Box::new(controlled) as Box<dyn Thing<Action>>;
    let controller = ActionController::new(1, controlled);
    let clone = controller.clone();
    controller.invoke(Action::One);
    clone.invoke(Action::Two);
    drop(controller);
    clone.invoke(Action::Three);
}

fn main() {
    bad();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn incorrect() {
        bad();
    }
}
