#[macro_use]
extern crate pest_derive;

use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct TokenTreeParser;

use pratt::{Affix, Associativity, PrattParser, Precedence, Result};

#[derive(Debug, Eq, PartialEq)]
pub enum Expr {
    BinOp(Box<Expr>, BinOpKind, Box<Expr>),
    UnOp(UnOpKind, Box<Expr>),
    Int(i32),
}

#[derive(Debug, Eq, PartialEq)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Eq,
}

#[derive(Debug, Eq, PartialEq)]
pub enum UnOpKind {
    Not,
    Neg,
    Try,
}

struct ExprParser;

impl<'i, I> PrattParser<I> for ExprParser
where
    I: Iterator<Item = Pair<'i, Rule>>,
{
    type Error = pratt::NoError;
    type Input = Pair<'i, Rule>;
    type Output = Expr;

    // Query information about an operator (Affix, Precedence, Associativity)
    fn query(&mut self, tree: &Self::Input) -> Result<Affix> {
        let affix = match (tree.as_rule(), tree.as_str()) {
            (Rule::infix, "=") => Affix::Infix(Precedence(2), Associativity::Neither),
            (Rule::infix, "+") => Affix::Infix(Precedence(3), Associativity::Left),
            (Rule::infix, "-") => Affix::Infix(Precedence(3), Associativity::Left),
            (Rule::infix, "*") => Affix::Infix(Precedence(4), Associativity::Left),
            (Rule::infix, "/") => Affix::Infix(Precedence(4), Associativity::Left),
            (Rule::postfix, "?") => Affix::Postfix(Precedence(5)),
            (Rule::prefix, "-") => Affix::Prefix(Precedence(6)),
            (Rule::prefix, "!") => Affix::Prefix(Precedence(6)),
            (Rule::infix, "^") => Affix::Infix(Precedence(7), Associativity::Right),
            (Rule::group, _) => Affix::Nilfix,
            (Rule::primary, _) => Affix::Nilfix,
            (Rule::num, _) => Affix::Nilfix,
            _ => unreachable!(),
        };
        Ok(affix)
    }

    // Construct a primary expression, e.g. a number
    fn primary(&mut self, tree: Self::Input) -> Result<Expr> {
        let expr = match tree.as_rule() {
            Rule::num => Expr::Int(tree.as_str().parse().unwrap()),
            Rule::group => self.parse(&mut tree.into_inner()).unwrap(),
            _ => unreachable!(),
        };
        Ok(expr)
    }

    // Construct a binary infix expression, e.g. 1+1
    fn infix(&mut self, lhs: Expr, tree: Self::Input, rhs: Expr) -> Result<Expr> {
        let op = match tree.as_str() {
            "+" => BinOpKind::Add,
            "-" => BinOpKind::Sub,
            "*" => BinOpKind::Mul,
            "/" => BinOpKind::Div,
            "^" => BinOpKind::Pow,
            "=" => BinOpKind::Eq,
            _ => unreachable!(),
        };
        Ok(Expr::BinOp(Box::new(lhs), op, Box::new(rhs)))
    }

    // Construct a unary prefix expression, e.g. !1
    fn prefix(&mut self, tree: Self::Input, rhs: Expr) -> Result<Expr> {
        let op = match tree.as_str() {
            "!" => UnOpKind::Not,
            "-" => UnOpKind::Neg,
            _ => unreachable!(),
        };
        Ok(Expr::UnOp(op, Box::new(rhs)))
    }

    // Construct a unary postfix expression, e.g. 1?
    fn postfix(&mut self, lhs: Expr, tree: Self::Input) -> Result<Expr> {
        let op = match tree.as_str() {
            "?" => UnOpKind::Try,
            _ => unreachable!(),
        };
        Ok(Expr::UnOp(op, Box::new(lhs)))
    }
}

fn main() {
    let mut args = std::env::args();
    let _ = args.next();

    let input = args.next().expect("Expected input string");
    println!("Code: {}", input);

    let tt = TokenTreeParser::parse(Rule::group, &input).unwrap_or_else(|e| panic!("{}", e));
    println!("TokenTree: {:?}", tt);

    let expr = ExprParser.parse(tt.into_iter()).unwrap();
    println!("Expression: {:?}", expr);
}

#[cfg(test)]
mod test {
    fn parse(input: &str) -> Expr {
        let tt = TokenTreeParser::parse(Rule::group, &input)
            .unwrap()
            .into_iter();
        ExprParser.parse(tt.into_iter()).unwrap()
    }
    use super::BinOpKind::*;
    use super::Expr::*;
    use super::UnOpKind::*;
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            parse("1=2=3"),
            BinOp(Box::new(Int(1)), Eq, Box::new(Int(2)))
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            parse("1*2+3"),
            BinOp(
                Box::new(BinOp(Box::new(Int(1)), Mul, Box::new(Int(2)))),
                Add,
                Box::new(Int(3))
            )
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            parse("1*!2^3"),
            BinOp(
                Box::new(Int(1)),
                Mul,
                Box::new(UnOp(
                    Not,
                    Box::new(BinOp(Box::new(Int(2)), Pow, Box::new(Int(3))))
                ))
            )
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            parse("-1?*!2^3+3/2?-1"),
            BinOp(
                Box::new(BinOp(
                    Box::new(BinOp(
                        Box::new(UnOp(Try, Box::new(UnOp(Neg, Box::new(Int(1)))))),
                        Mul,
                        Box::new(UnOp(
                            Not,
                            Box::new(BinOp(Box::new(Int(2)), Pow, Box::new(Int(3))))
                        ))
                    )),
                    Add,
                    Box::new(BinOp(
                        Box::new(Int(3)),
                        Div,
                        Box::new(UnOp(Try, Box::new(Int(2))))
                    ))
                )),
                Sub,
                Box::new(Int(1))
            )
        );
    }
}
