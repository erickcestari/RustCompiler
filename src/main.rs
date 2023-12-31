use std::{collections::HashMap, fs};

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct File {
    name: String,
    expression: Term,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Print {
    value: Box<Term>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Int {
    value: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Str {
    value: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Bool {
    value: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Binary {
    lhs: Box<Term>,
    op: BinaryOp,
    rhs: Box<Term>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct If {
    condition: Box<Term>,
    then: Box<Term>,
    otherwise: Box<Term>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Let {
    name: Parameter,
    value: Box<Term>,
    next: Box<Term>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Function {
    parameters: Vec<Parameter>,
    value: Box<Term>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Call {
    callee: Box<Term>,
    arguments: Vec<Term>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Var {
    text: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Parameter {
    text: String,
}

#[derive(Debug, Deserialize, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Lt,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "kind")]
pub enum Term {
    Int(Int),
    Str(Str),
    Print(Print),
    Binary(Binary),
    Bool(Bool),
    If(If),
    Let(Let),
    Var(Var),
    Function(Function),
    Call(Call),
}
#[derive(Debug, Clone)]
pub enum Val {
    Void,
    Int(i32),
    Bool(bool),
    Str(String),
    Closure {
        body: Term,
        params: Vec<Parameter>,
        env: Scope,
    },
}

pub type Scope = HashMap<String, Val>;

fn eval(term: Term, scope: &mut Scope) -> Val {
    match term {
        Term::Int(number) => Val::Int(number.value),
        Term::Str(string) => Val::Str(string.value),
        Term::Bool(bool) => Val::Bool(bool.value),
        Term::Print(print) => {
            let val = eval(*print.value, scope);

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
                let lhs = eval(*bin.lhs, scope);
                let rhs = eval(*bin.rhs, scope);

                match (lhs, rhs) {
                    (Val::Int(a), Val::Int(b)) => Val::Int(a + b),
                    (Val::Str(s), Val::Str(b)) => Val::Str(format!("{s}{b}")),
                    (Val::Str(s), Val::Int(b)) => Val::Str(format!("{s}{b}")),
                    (Val::Int(a), Val::Str(b)) => Val::Str(format!("{a}{b}")),
                    _ => panic!("Cannot add non-integers"),
                }
            }
            BinaryOp::Sub => {
                let lhs = eval(*bin.lhs, scope);
                let rhs = eval(*bin.rhs, scope);

                match (lhs, rhs) {
                    (Val::Int(a), Val::Int(b)) => Val::Int(a - b),
                    _ => panic!("Cannot subtract non-integers"),
                }
            }

            BinaryOp::Lt => {
                let lhs = eval(*bin.lhs, scope);
                let rhs = eval(*bin.rhs, scope);

                match (lhs, rhs) {
                    (Val::Int(a), Val::Int(b)) => Val::Bool(a < b),
                    _ => panic!("Cannot subtract non-integers"),
                }
            }
        },
        Term::If(i) => match eval(*i.condition, scope) {
            Val::Bool(true) => eval(*i.then, scope),
            Val::Bool(false) => eval(*i.otherwise, scope),
            _ => panic!("Invalid Condition"),
        },
        Term::Let(l) => {
            let name = l.name.text;
            let value = eval(*l.value, scope);
            scope.insert(name, value);
            eval(*l.next, scope)
        }

        Term::Var(v) => match scope.get(&v.text) {
            Some(val) => val.clone(),
            None => panic!("Variable not found"),
        },

        Term::Function(f) => Val::Closure {
            body: *f.value,
            params: f.parameters,
            env: scope.clone(),
        },

        Term::Call(call) => match eval(*call.callee, scope) {
            Val::Closure { body, params, env } => {
                let mut new_scope = scope.clone();
                for (param, arg) in params.into_iter().zip(call.arguments) {
                    new_scope.insert(param.text, eval(arg, scope));
                }
                eval(body, &mut new_scope)
            }
            _ => panic!("Is not a fuction"),
        },
    }
}

fn main() {
    let program = fs::read_to_string("./examples/fib.json").unwrap();
    let program: File = serde_json::from_str::<File>(&program).unwrap();

    let term = program.expression;
    let mut scope = HashMap::new();
    eval(term, &mut scope);
}
