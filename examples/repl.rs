#[allow(unused_imports)]
use g3::{point,line,plane,Point,Line,Plane};
use peg::parser;

parser! {
  grammar algebra() for str {
    pub rule expression() -> Expression
        = sum()

    rule _ = [' ' | '\n']*

    rule sum() -> Expression
        = l:product() _ "+" _ r:product() { Expression::Sum(Box::new(l), Box::new(r)) }
        / product()

    rule product() -> Expression
        = l:atom() _ "*" _ r:atom() { Expression::Product(Box::new(l), Box::new(r)) }
        / atom()

    rule atom() -> Expression
        = coefficiant()
        / symbol()
        / number()
        / "(" _ v:sum() _ ")" { v }
    
    rule coefficiant() ->Expression
        = n:number() s:symbol() { Expression::Coefficiant(Box::new(n), Box::new(s)) }

    rule number() -> Expression
        = n:$((['0'..='9']+".")?['0'..='9']+) { Expression::Number(n.parse().unwrap()) }
    
    rule symbol() -> Expression
        = s:$(['a'..='z' | 'A'..='Z']['a'..='z' | 'A'..='Z' | '0'..='9']*) { Expression::Symbol(s.parse().unwrap()) }
  }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Expression {
  Number(f32),
  Symbol(String),
  Coefficiant(Box<Expression>,Box<Expression>),
  Sum(Box<Expression>,Box<Expression>),
  Product(Box<Expression>,Box<Expression>),
}

fn main() {
  println!("g3 repl");
  assert_eq!(algebra::expression("4"), Ok(Expression::Number(4.0)));
  assert_eq!(algebra::expression("1.2"), Ok(Expression::Number(1.2)));
  assert_eq!(algebra::expression("a"), Ok(Expression::Symbol("a".to_string())));
  assert_eq!(algebra::expression("Abc"), Ok(Expression::Symbol("Abc".to_string())));
  assert_eq!(algebra::expression("e01"), Ok(Expression::Symbol("e01".to_string())));
  assert_eq!(algebra::expression("3e01"), Ok(Expression::Coefficiant(
    Box::new(Expression::Number(3.0)),
    Box::new(Expression::Symbol("e01".to_string())))));
  assert_eq!(algebra::expression("1+1"), Ok(Expression::Sum(
      Box::new(Expression::Number(1.0)),
      Box::new(Expression::Number(1.0)))));
  // assert_eq!(algebra::expression("5*5"), Ok(Expression::Product(
  //     Box::new(Expression::Number(5.0)),
  //     Box::new(Expression::Number(5.0)))));
  // assert_eq!(algebra::expression("2+3*4"), Ok(Expression::Sum(
  //     Box::new(Expression::Number(2.0)),
  //     Box::new(Expression::Product(
  //         Box::new(Expression::Number(3.0)),
  //         Box::new(Expression::Number(4.0)))))));
  // assert_eq!(algebra::expression("(2+3) * 4"), Ok(Expression::Product(
  //     Box::new(Expression::Sum(
  //         Box::new(Expression::Number(2.0)),
  //         Box::new(Expression::Number(3.0)))),  
  //     Box::new(Expression::Number(4.0)))));
  // assert_eq!(algebra::expression("a"), Ok(Expression::Symbol("a".to_string())));
  // assert_eq!(algebra::expression("4.1"), Ok(Expression::Number(4.1)));
  // assert_eq!(algebra::expression("4e0"), Ok(Expression::Number(4.1)));
  // assert_eq!(algebra::expression("4.2e123"), Ok(Expression::Number(4.1)));

}
