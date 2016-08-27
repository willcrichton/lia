#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Var {
    name: String
}

impl Var {
    pub fn new(name: String) -> Var {
        Var {
            name: name
        }
    }
}
