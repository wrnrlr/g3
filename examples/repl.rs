#[allow(unused_imports)]
use g3::{point,line,plane,Point,Line,Plane};
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
        = s:expression() ** "," { Sequence(s) }

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
        = l:number() r:atom() { Expression::Binary(Operator::Multiply, Box::new(l), Box::new(r)) }
        / l:atom() _ "*"? _ r:atom() { Expression::Binary(Operator::Multiply, Box::new(l), Box::new(r)) }
        / division()

    rule division() -> Expression
        = l:atom() _ "/" _ r:atom() { Expression::Binary(Operator::Divide, Box::new(l), Box::new(r)) }
        / call()

    rule call() -> Expression
        = h:symbol() "[" s:sequence() "]" { Expression::Call(Box::new(h), Box::new(s)) }
        / atom()

    rule assignment() -> Expression
        = l:symbol() "=" r:expression() { Expression::Binary(Operator::Assign, Box::new(l), Box::new(r)) }

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
  println!("g3 repl");
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
}
