use std::fs;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct File {
    name: String,
    expression: Term,
}

#[derive(Debug, Deserialize)]
pub struct Print {
    value: Box<Term>,
}

#[derive(Debug, Deserialize)]
pub struct Int {
    value: i32,
}

#[derive(Debug, Deserialize)]
pub struct Str {
    value: String,
}

#[derive(Debug, Deserialize)]
pub struct Bool {
    value: bool,
}

#[derive(Debug, Deserialize)]
pub struct Binary {
    lhs: Box<Term>,
    op: BinaryOp,
    rhs: Box<Term>,
}

#[derive(Debug, Deserialize)]
pub struct If {
    condition: Box<Term>,
    then: Box<Term>,
    otherwise: Box<Term>,
}

#[derive(Debug, Deserialize)]
pub enum BinaryOp {
    Add,
    Sub,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind")]
pub enum Term {
    Int(Int),
    Str(Str),
    Print(Print),
    Binary(Binary),
    Bool(Bool),
    If(If)
}
#[derive(Debug)]
pub enum Val {
    Void,
    Int(i32),
    Bool(bool),
    Str(String),
}

fn eval(term: Term) -> Val {
    match term {
        Term::Int(number) => Val::Int(number.value),
        Term::Str(string) => Val::Str(string.value),
        Term::Bool(bool) => Val::Bool(bool.value),
        Term::Print(print) => {
            let val = eval(*print.value);

            match val {
                Val::Int(n) => print!("{n}"),
                Val::Bool(b) => print!("{b}"),
                Val::Str(s) => print!("{s}"),
                _ => panic!("Cannot print void"),
            };
            Val::Void
        }
        Term::Binary(bin) => match bin.op {
            BinaryOp::Add => {
                let lhs = eval(*bin.lhs);
                let rhs = eval(*bin.rhs);

                match (lhs, rhs) {
                    (Val::Int(a), Val::Int(b)) => Val::Int(a + b),
                    (Val::Str(s), Val::Str(b)) => Val::Str(format!("{s}{b}")),
                    (Val::Str(s), Val::Int(b)) => Val::Str(format!("{s}{b}")),
                    (Val::Int(a), Val::Str(b)) => Val::Str(format!("{a}{b}")),
                    _ => panic!("Cannot add non-integers"),
                }
            },
            BinaryOp::Sub => {
                let lhs = eval(*bin.lhs);
                let rhs = eval(*bin.rhs);

                match (lhs, rhs) {
                    (Val::Int(a), Val::Int(b)) => Val::Int(a - b),
                    _ => panic!("Cannot subtract non-integers"),
                }
            },
        },
        Term::If(i) => {
            match eval(*i.condition) {
                Val::Bool(true) => eval(*i.then),
                Val::Bool(false) => eval(*i.then),
                _ => panic!("Invalid Condition")
            }
        }
    }
}

fn main() {
    let program = fs::read_to_string("./examples/if.json").unwrap();
    let program: File = serde_json::from_str::<File>(&program).unwrap();

    let term = program.expression;
    eval(term);
}
