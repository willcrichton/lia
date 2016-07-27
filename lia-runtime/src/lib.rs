#![feature(plugin)]
#![plugin(lia_plugin)]

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use std::fmt;

pub type LiaNumber = i32;
pub type LiaBool = bool;
pub type LiaString = String;
pub type LiaClosure = Box<Fn(Vec<LiaPtr>) -> LiaPtr>;
pub type LiaObject = HashMap<String, LiaPtr>;

pub enum LiaValue {
    Number(LiaNumber),
    String(LiaString),
    Closure(LiaClosure),
    Bool(LiaBool),
    Object(LiaObject),
    Null,
    Unknown(Box<Any>)
}

pub type LiaPtr = Rc<RefCell<Rc<RefCell<LiaValue>>>>;

fn wrap(t: LiaValue) -> LiaPtr {
    Rc::new(RefCell::new(Rc::new(RefCell::new(t))))
}

macro_rules! make_allocator {
    ($fun:ident, $ty:ty, $path:path) => {
        pub fn $fun (t: $ty) -> LiaPtr {
            wrap($path(t))
        }
    }
}

make_allocator!(alloc_number, LiaNumber, LiaValue::Number);
make_allocator!(alloc_string, LiaString, LiaValue::String);
make_allocator!(alloc_closure, LiaClosure, LiaValue::Closure);
make_allocator!(alloc_bool, LiaBool, LiaValue::Bool);
make_allocator!(alloc_object, LiaObject, LiaValue::Object);

pub fn alloc_null(_: ()) -> LiaPtr {
    wrap(LiaValue::Null)
}

pub fn alloc_other<T: Any>(t: T) -> LiaPtr {
    wrap(LiaValue::Unknown(Box::new(t)))
}

pub fn new_obj() -> LiaObject {
    let obj: LiaObject = HashMap::new();
    obj
}

#[macro_export]
macro_rules! alloc {
    ($t:ty, $e:expr) => {
        _alloc!($t {$e})
    }
}

#[macro_export]
macro_rules! cast {
    (let mut $id:ident : $t:ty = $e:expr) => {
        let _tmp = $e;
        let mut _tmp = _tmp.borrow_mut();
        let mut _tmp = _tmp.borrow_mut();
        let mut _tmp = _borrow_type!(_tmp $t true);
        let mut $id = _tmp;
    };
    (let $id:ident : $t:ty = $e:expr) => {
        let _tmp = $e;
        let _tmp = _tmp.borrow();
        let _tmp = _tmp.borrow();
        let _tmp = _borrow_type!(_tmp $t false);
        let $id = _tmp;
    };
}

#[macro_export]
macro_rules! call {
    ($id:ident ( $( $x:expr ),* )) => {{
        concat_idents!(_lia_, $id)(vec![alloc_object(new_obj()), $( $x ),*])
    }}
}

impl fmt::Debug for LiaValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &LiaValue::Number(ref n) => n.fmt(f),
            &LiaValue::String(ref s) => s.fmt(f),
            &LiaValue::Closure(_) => f.pad("<LiaClosure>"),
            &LiaValue::Bool(b) => b.fmt(f),
            &LiaValue::Object(ref o) => o.fmt(f),
            &LiaValue::Null => f.pad("null"),
            &LiaValue::Unknown(_) => f.pad("<Unknown>")
        }
    }
}

pub fn _lia_print(args: Vec<LiaPtr>) -> LiaPtr {
    for arg in args[1..].into_iter() {
        let _tmp = arg.borrow();
        let arg = _tmp.borrow();
        println!("{:?}", arg);
    }
    alloc_null(())
}

pub fn val_to_key(val: LiaPtr) -> String {
    let _tmp = val.borrow();
    let val = _tmp.borrow();
    match *val {
        LiaValue::Number(ref n) => format!("{}", n),
        LiaValue::String(ref s) => s.clone(),
        _ => panic!("Object key must be string or number"),
    }
}

pub fn _lia_call(args: Vec<LiaPtr>) -> LiaPtr {
    let fun = args.get(1).expect("Arg 1").clone();
    let ctx = args.get(2).expect("Arg 2").clone();

    cast!(let fun: &LiaClosure = fun);
    fun(vec![ctx])
}
