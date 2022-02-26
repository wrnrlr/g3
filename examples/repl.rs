use std::hash::Hasher;
use g3::{point, line, plane, Point, Line, Plane};
use peg::{parser};

parser! {
  grammar algebra() for str {
    pub rule expression() -> Expression
        = sum()

    rule _ = [' ' | '\n']*

    rule number() -> Expression
        = n:$("-"?['0'..='9']+("."['0'..='9']+)?) { Expression::Number(n.parse().unwrap()) }

    rule symbol() -> Expression
        = s:$(['a'..='z' | 'A'..='Z']['a'..='z' | 'A'..='Z' | '0'..='9']*) { Expression::Symbol(s.parse().unwrap()) }

    rule list() -> Expression
        = "{" s:sequence() "}" { Expression::List(Box::new(s)) }

    rule sequence() -> Sequence
        = _ s:(expression() ** ",") _ { Sequence(s) }

    rule sum() -> Expression
        = l:minus() _ "+" _ r:minus() { Expression::Binary(Operator::Plus, Box::new(l), Box::new(r)) }
        / minus()

    rule minus() -> Expression
        = l:negative() _ "-" _ r:negative() { Expression::Binary(Operator::Minus, Box::new(l), Box::new(r)) }
        / negative()

    rule negative() -> Expression
        = "-" _ r:product() { Expression::Unary(Operator::Minus, Box::new(r)) }
        / product()

    rule product() -> Expression
        = l:division() _ "*" _ r:division() { Expression::Binary(Operator::Multiply, Box::new(l), Box::new(r)) }
        / l:division() [' ' | '\n']* r:division() { Expression::Binary(Operator::Multiply, Box::new(l), Box::new(r)) }
        / l:division() [' ' | '\n']+ r:division() { Expression::Binary(Operator::Multiply, Box::new(l), Box::new(r)) }
        / division()

    rule division() -> Expression
        = l:atom() _ "/" _ r:atom() { Expression::Binary(Operator::Divide, Box::new(l), Box::new(r)) }
        / call()

    rule call() -> Expression
        = h:symbol() _ "[" s:sequence() "]" { Expression::Call(Box::new(h), Box::new(s)) }
        / assignment()

    rule assignment() -> Expression
        = l:symbol() _ "=" _ r:atom() { Expression::Binary(Operator::Assign, Box::new(l), Box::new(r)) }
        / atom()

    rule atom() -> Expression
        = symbol()
        / number()
        / list()
        / "(" _ v:expression() _ ")" { v }
  }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Operator {
  Plus, Minus, Negative, Multiply, Divide, Power, Assign
}

#[derive(Clone, PartialEq, Debug)]
pub struct Sequence(Vec<Expression>);

#[derive(Clone, PartialEq, Debug)]
pub enum Expression {
  Number(f32),
  Symbol(String),
  List(Box<Sequence>),
  Binary(Operator, Box<Expression>,Box<Expression>),
  Unary(Operator, Box<Expression>),
  Call(Box<Expression>, Box<Sequence>)
}

// https://corywalker.me/2018/06/03/introduction-to-computer-algebra.html
fn main() {
  assert!(algebra::expression("f[]*1").is_ok());
  assert_eq!(algebra::expression("4"), Ok(Expression::Number(4.0)));
  assert!(algebra::expression("-1").is_ok());
  assert!(algebra::expression("1.2").is_ok());
  assert!(algebra::expression("a").is_ok());
  assert!(algebra::expression("Abc").is_ok());
  assert!(algebra::expression("e01").is_ok());
  assert!(algebra::expression("3e01").is_ok());
  assert!(algebra::expression("4.0 e01").is_ok());
  assert!(algebra::expression("a2 b3").is_ok());
  assert!(algebra::expression("1+1").is_ok());
  assert!(algebra::expression("5*5").is_ok());
  assert!(algebra::expression("2+3*4").is_ok());
  assert!(algebra::expression("(2+3)*4").is_ok());
  assert!(algebra::expression("1*-2").is_ok());
  assert!(algebra::expression("-2*a").is_ok());
  assert!(algebra::expression("{}").is_ok());
  assert!(algebra::expression("{1,a,b+2.0}").is_ok());
  assert!(algebra::expression("f[]").is_ok());
  assert!(algebra::expression("f[1,a]").is_ok());
  assert!(algebra::expression("f[]+g[]").is_ok());
  assert!(algebra::expression("a=1+a").is_ok());
  assert!(algebra::expression("a = 1 + a").is_ok());
  // assert!(algebra::expression("{ 1 , a }").is_ok());
}

pub struct Ex {
  parts:Vec<Atom>
}

pub struct Symbol {
  name:String
}

pub struct Number {
  value:f32
}

pub enum Atom {
  Ex(Ex),
  Symbol(Symbol),
  Number(Number)
}

pub trait Expr {}

pub struct State {

}

impl State {
  fn get_def()->Option<Ex> {
    todo!()
  }

  fn eval(&mut self, atom:Atom) {
    match atom {
      Atom::Ex(e) => e.eval(),
      Atom::Symbol(s) => s.eval(),
      Atom::Number(n) => n.eval()
    }
  }

  fn eval_symbol(self, s:Symbol) {}

}

impl Ex {
  fn eval(self) {}
}

impl Symbol {
  fn eval(self) {}

  fn hash(&self)->u64 {
    let mut h = fxhash::FxHasher64::default();
    h.write_u64(1);
    h.write(self.name.as_bytes());
    h.finish()
  }
}

impl Number {
  fn eval(self) {}
}


/*
=  Let[a,4]
:= Yet[]

Blockdoc[
  Heading[Level->1,"Hello"],
  Paragraph[Level->1,«Hello»]
  Input[«a=1»]
  Output[«a=1»]
]

 */