use std::any::Any;
use std::rc::Rc;
use std::cell::RefCell;

pub type LiaAny = Rc<RefCell<Any>>;

pub fn alloc<T: Any>(t: T) -> LiaAny {
    Rc::new(RefCell::new(t))
}

#[macro_export]
macro_rules! cast {
    ($e:expr, $t:ty) => ($e.borrow_mut().downcast_mut::<$t>().unwrap())
}
