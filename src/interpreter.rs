use std::collections::HashMap;

use crate::parse::{Atom, Op, TokenTree};

#[derive(Clone, Debug)]
pub enum Value {
    Nil,
    Number(f64),
    Bool(bool),
    String(String),
}

impl Value {
    fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Bool(b) => *b,
            _ => true,
        }
    }

    fn to_display(&self) -> String {
        match self {
            Value::Nil => "nil".to_string(),
            Value::Number(n) => {
                if *n == n.trunc() {
                    format!("{n}.0")
                } else {
                    n.to_string()
                }
            }
            Value::Bool(b) => b.to_string(),
            Value::String(s) => s.clone(),
        }
    }
}

#[derive(Default)]
pub struct Env {
    vars: HashMap<String, Value>,
}

impl Env {
    fn define(&mut self, name: &str, value: Value) {
        self.vars.insert(name.to_string(), value);
    }

    fn assign(&mut self, name: &str, value: Value) {
        if let Some(slot) = self.vars.get_mut(name) {
            *slot = value;
        } else {
            self.define(name, value);
        }
    }

    fn get(&self, name: &str) -> Value {
        self.vars
            .get(name)
            .cloned()
            .unwrap_or(Value::Nil)
    }
}

pub struct Interpreter {
    env: Env,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { env: Env::default() }
    }

    pub fn eval_program<'de>(&mut self, stmts: &[TokenTree<'de>]) {
        for stmt in stmts {
            self.exec(stmt);
        }
    }

    fn exec<'de>(&mut self, node: &TokenTree<'de>) {
        match node {
            TokenTree::Cons(Op::Group, children) => {
                for stmt in children {
                    self.exec(stmt);
                }
            }
            TokenTree::If { condition, yes, no } => {
                if self.eval_expr(condition).is_truthy() {
                    self.exec(yes);
                } else if let Some(no_branch) = no {
                    self.exec(no_branch);
                }
            }
            TokenTree::Cons(Op::Var, children) => {
                if let [TokenTree::Atom(Atom::Ident(name)), expr] = &children[..] {
                    let value = self.eval_expr(expr);
                    self.env.define(name, value);
                }
            }
            TokenTree::Cons(Op::Print, children) => {
                if let [expr] = &children[..] {
                    let value = self.eval_expr(expr);
                    println!("{}", value.to_display());
                }
            }
            TokenTree::Cons(Op::While, children) => {
                if let [cond, body] = &children[..] {
                    while self.eval_expr(cond).is_truthy() {
                        self.exec(body);
                    }
                }
            }
            other => {
                let _ = self.eval_expr(other);
            }
        }
    }

    fn eval_expr<'de>(&mut self, node: &TokenTree<'de>) -> Value {
        match node {
            TokenTree::Atom(atom) => match atom {
                Atom::Number(n) => Value::Number(*n),
                Atom::Bool(b) => Value::Bool(*b),
                Atom::Nil => Value::Nil,
                Atom::String(s) => Value::String(s.to_string()),
                Atom::Ident(name) => self.env.get(name),
                Atom::Super | Atom::This => Value::Nil,
            },
            TokenTree::Cons(op, children) => match (op, &children[..]) {
                (Op::Group, children) => {
                    if let Some(first) = children.first() {
                        self.eval_expr(first)
                    } else {
                        Value::Nil
                    }
                }
                (Op::Minus, [expr]) => {
                    if let Value::Number(n) = self.eval_expr(expr) {
                        Value::Number(-n)
                    } else {
                        Value::Nil
                    }
                }
                (Op::Bang, [expr]) => {
                    let v = self.eval_expr(expr);
                    Value::Bool(!v.is_truthy())
                }
                (Op::Assign, [TokenTree::Atom(Atom::Ident(name)), expr]) => {
                    let value = self.eval_expr(expr);
                    self.env.assign(name, value.clone());
                    value
                }
                (Op::Plus, [lhs, rhs]) => match (self.eval_expr(lhs), self.eval_expr(rhs)) {
                    (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
                    (Value::String(a), Value::String(b)) => Value::String(format!("{a}{b}")),
                    (Value::String(a), b) => Value::String(format!("{a}{}", b.to_display())),
                    (a, Value::String(b)) => Value::String(format!("{}{}", a.to_display(), b)),
                    _ => Value::Nil,
                },
                (Op::Minus, [lhs, rhs]) => match (self.eval_expr(lhs), self.eval_expr(rhs)) {
                    (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
                    _ => Value::Nil,
                },
                (Op::Star, [lhs, rhs]) => match (self.eval_expr(lhs), self.eval_expr(rhs)) {
                    (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
                    _ => Value::Nil,
                },
                (Op::Slash, [lhs, rhs]) => match (self.eval_expr(lhs), self.eval_expr(rhs)) {
                    (Value::Number(_), Value::Number(0.0)) => Value::Nil,
                    (Value::Number(a), Value::Number(b)) => Value::Number(a / b),
                    _ => Value::Nil,
                },
                (Op::Less, [lhs, rhs]) => Value::Bool(
                    matches!(
                        (self.eval_expr(lhs), self.eval_expr(rhs)),
                        (Value::Number(a), Value::Number(b)) if a < b
                    ),
                ),
                (Op::LessEqual, [lhs, rhs]) => Value::Bool(
                    matches!(
                        (self.eval_expr(lhs), self.eval_expr(rhs)),
                        (Value::Number(a), Value::Number(b)) if a <= b
                    ),
                ),
                (Op::Greater, [lhs, rhs]) => Value::Bool(
                    matches!(
                        (self.eval_expr(lhs), self.eval_expr(rhs)),
                        (Value::Number(a), Value::Number(b)) if a > b
                    ),
                ),
                (Op::GreaterEqual, [lhs, rhs]) => Value::Bool(
                    matches!(
                        (self.eval_expr(lhs), self.eval_expr(rhs)),
                        (Value::Number(a), Value::Number(b)) if a >= b
                    ),
                ),
                (Op::EqualEqual, [lhs, rhs]) => {
                    let a = self.eval_expr(lhs);
                    let b = self.eval_expr(rhs);
                    let equal = match (&a, &b) {
                        (Value::Nil, Value::Nil) => true,
                        (Value::Bool(x), Value::Bool(y)) => x == y,
                        (Value::Number(x), Value::Number(y)) => x == y,
                        (Value::String(x), Value::String(y)) => x == y,
                        _ => false,
                    };
                    Value::Bool(equal)
                }
                (Op::BangEqual, [lhs, rhs]) => {
                    if let Value::Bool(eq) =
                        self.eval_expr(&TokenTree::Cons(Op::EqualEqual, vec![lhs.clone(), rhs.clone()]))
                    {
                        Value::Bool(!eq)
                    } else {
                        Value::Bool(false)
                    }
                }
                (Op::And, [lhs, rhs]) => {
                    let left = self.eval_expr(lhs);
                    if !left.is_truthy() {
                        left
                    } else {
                        self.eval_expr(rhs)
                    }
                }
                (Op::Or, [lhs, rhs]) => {
                    let left = self.eval_expr(lhs);
                    if left.is_truthy() {
                        left
                    } else {
                        self.eval_expr(rhs)
                    }
                }
                _ => Value::Nil,
            },
            TokenTree::Fun { .. } | TokenTree::Call { .. } | TokenTree::If { .. } => Value::Nil,
        }
    }
}

