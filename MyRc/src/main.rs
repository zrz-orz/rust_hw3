use std::cell::RefCell;
use std::ops::Deref;

struct Rc<T> {
    count: RefCell<usize>,
    value: T,
}

impl<T: Clone> Rc<T> {
    fn new(value: T) -> Rc<T> {
        Rc {count: RefCell::new(1), value : value}
    }

    fn clone(&self) -> Rc<T> {
        *self.count.borrow_mut() += 1;
        Rc {
            count: self.count.clone(),
            value: self.value.clone(),
        }
    }

    fn drop(&mut self) {
        let mut count = self.count.borrow_mut();
        *count -= 1;
        if *count == 0 {
            drop(&self.value);
        }
    }

    fn strong_count(&self) -> usize {
        return *self.count.borrow();
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}

fn main() {
    let five = Rc::new(5);
    let five1 = Rc::clone(&five);
    println!("{}", Rc::strong_count(&five1));
    println!("{}", *five1);
}
