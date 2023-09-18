pub enum Val {
    Void,
    Int(i32),
    Bool(bool),
    Str(String),
}

fn eval(program: Program) -> Val {
    todo!("eval")
}

fn main() {
    println!("Hello, world!");
}
