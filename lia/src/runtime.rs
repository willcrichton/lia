use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

pub type LiaAny = Rc<RefCell<Rc<RefCell<Box<Any>>>>>;
pub type LiaObject = HashMap<String, LiaAny>;
pub type LiaClosure = Box<Fn(Vec<LiaAny>) -> LiaAny>;
pub struct LiaNull;

pub fn alloc<T: Any>(t: T) -> LiaAny {
    Rc::new(RefCell::new(Rc::new(RefCell::new(Box::new(t)))))
}

pub fn new_obj() -> LiaObject {
    let obj: LiaObject = HashMap::new();
    obj
}

#[macro_export]
macro_rules! cast {
    (let mut $id:ident : $t:ty = $e:expr) => {
        let _tmp = $e;
        let mut _tmp = _tmp.borrow_mut();
        let mut _tmp = _tmp.borrow_mut();
        let _tmp = _borrow_type!(_tmp $t);
        let mut $id = _tmp;
    };
    (let $id:ident : $t:ty = $e:expr) => {
        let _tmp = $e;
        let mut _tmp = _tmp.borrow_mut();
        let mut _tmp = _tmp.borrow_mut();
        let _tmp = _borrow_type!(_tmp $t);
        let $id = _tmp;
    };
}

#[macro_export]
macro_rules! call {
    ($id:ident ( $( $x:expr ),* )) => {{
        concat_idents!(_lia_, $id)(vec![alloc(new_obj()), $( alloc($x) ),*])
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
        $( printed = printed || lia_print!($arg, $ty);)*;
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

pub fn val_to_key(val: LiaAny) -> String {
    let _tmp = val.borrow();
    let val = _tmp.borrow();
    if val.is::<i32>() {
        format!("{}", val.downcast_ref::<i32>().unwrap())
    } else {
        val.downcast_ref::<String>().expect("Object key must be string or int").clone()
    }
}

pub fn _lia_call(args: Vec<LiaAny>) -> LiaAny {
    let fun = args.get(1).expect("Arg 1").clone();
    let ctx = args.get(2).expect("Arg 2").clone();

    let _tmp = fun.borrow();
    let fun = _tmp.borrow();
    (fun.downcast_ref::<LiaClosure>().expect("Closure"))(vec![ctx])
}
