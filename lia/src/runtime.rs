use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

pub type LiaAny = Rc<RefCell<Rc<RefCell<Box<Any>>>>>;
pub type LiaObject = HashMap<String, LiaAny>;
pub type LiaClosure = Box<Fn(Vec<LiaAny>) -> LiaAny>;


pub fn alloc<T: Any>(t: T) -> LiaAny {
    Rc::new(RefCell::new(Rc::new(RefCell::new(Box::new(t)))))
}

#[macro_export]
macro_rules! cast {
    (let $id:ident : $t:ty = $e:expr) => {
        let _tmp = $e;
        let mut _tmp = _tmp.borrow_mut();
        let mut _tmp = _tmp.borrow_mut();
        let $id = _tmp.downcast_mut::<$t>().unwrap();
    }
}

#[macro_export]
macro_rules! call {
    ($id:ident ( $( $x:expr ),* )) => {{
        $id(vec![$( alloc($x) ),*])
    }}
}


macro_rules! lia_print {
    ($arg:ident, $ty:ty) => (
        if $arg.is::<$ty>() {
            print!("{:?}", $arg.downcast_ref::<$ty>().unwrap());
            true
        } else { false }
    )
}

macro_rules! lia_print_iter {
    ($arg:ident, $( $ty:ty ),*) => {{
        let mut printed = false;
        $( if lia_print!($arg, $ty) { printed = true; } )*;
        printed
    }}
}

pub fn print(args: Vec<LiaAny>) -> LiaAny {
    for arg in args.into_iter() {
        let _tmp = arg.borrow();
        let arg = _tmp.borrow();
        let printed = lia_print_iter!(arg, i32, String, LiaObject);
        if !printed {
            print!("Not a known type, cannot print")
        }
    }
    print!("\n");
    alloc(())
}
