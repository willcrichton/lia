use std::any::Any;
use std::sync::{Arc, Mutex};

pub type LiaAny = Arc<Mutex<Box<Any>>>;
pub type LiaObject = ();
pub type LiaClosure = Box<Fn(Vec<LiaAny>) -> LiaAny>;

pub fn alloc<T: Any>(t: T) -> LiaAny {
    Arc::new(Mutex::new(Box::new(t)))
}

#[macro_export]
macro_rules! cast {
    ($id:ident, $e:expr, $t:ty) => {
        let mut _tmp = $e.lock().unwrap();
        let $id = _tmp.downcast_mut::<$t>().unwrap();
    }
}

#[macro_export]
macro_rules! call {
    ($id:ident ( $( $x:expr ),* )) => {{
        $id(vec![$( alloc($x) ),*])
    }}
}
