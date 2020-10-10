use lalrpop_util::lalrpop_mod;
use pratt::{Affix, Associativity, PrattParser, Precedence};

lalrpop_mod!(pub grammar);

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

#[derive(Debug, Eq, PartialEq)]
pub enum TokenTree {
    Prefix(char),
    Postfix(char),
    Infix(char),
    Primary(i32),
    Group(Vec<TokenTree>),
}

struct ExprParser;

impl<I> PrattParser<I> for ExprParser
where
    I: Iterator<Item = TokenTree>,
{
    type Error = ();
    type Input = TokenTree;
    type Output = Expr;

    // Query information about an operator (Affix, Precedence, Associativity)
    fn query(&mut self, tree: &TokenTree) -> Option<Affix> {
        let affix = match tree {
            TokenTree::Infix('=') => Affix::Infix(Precedence(2), Associativity::Neither),
            TokenTree::Infix('+') => Affix::Infix(Precedence(3), Associativity::Left),
            TokenTree::Infix('-') => Affix::Infix(Precedence(3), Associativity::Left),
            TokenTree::Infix('*') => Affix::Infix(Precedence(4), Associativity::Left),
            TokenTree::Infix('/') => Affix::Infix(Precedence(4), Associativity::Left),
            TokenTree::Postfix('?') => Affix::Postfix(Precedence(5)),
            TokenTree::Prefix('-') => Affix::Prefix(Precedence(6)),
            TokenTree::Prefix('!') => Affix::Prefix(Precedence(6)),
            TokenTree::Infix('^') => Affix::Infix(Precedence(7), Associativity::Right),
            _ => None?,
        };
        Some(affix)
    }

    // Construct a primary expression, e.g. a number
    fn primary(&mut self, tree: TokenTree) -> Result<Expr, ()> {
        match tree {
            TokenTree::Primary(num) => Ok(Expr::Int(num)),
            TokenTree::Group(group) => self.parse(&mut group.into_iter()),
            _ => Err(()),
        }
    }

    // Construct an binary infix expression, e.g. 1+1
    fn infix(&mut self, lhs: Expr, tree: TokenTree, rhs: Expr) -> Result<Expr, ()> {
        let op = match tree {
            TokenTree::Infix('+') => BinOpKind::Add,
            TokenTree::Infix('-') => BinOpKind::Sub,
            TokenTree::Infix('*') => BinOpKind::Mul,
            TokenTree::Infix('/') => BinOpKind::Div,
            TokenTree::Infix('^') => BinOpKind::Pow,
            TokenTree::Infix('=') => BinOpKind::Eq,
            _ => Err(())?,
        };
        Ok(Expr::BinOp(Box::new(lhs), op, Box::new(rhs)))
    }

    // Construct an unary prefix expression, e.g. !1
    fn prefix(&mut self, tree: TokenTree, rhs: Expr) -> Result<Expr, ()> {
        let op = match tree {
            TokenTree::Prefix('!') => UnOpKind::Not,
            TokenTree::Prefix('-') => UnOpKind::Neg,
            _ => Err(())?,
        };
        Ok(Expr::UnOp(op, Box::new(rhs)))
    }

    // Construct an unary postfix expression, e.g. 1?
    fn postfix(&mut self, lhs: Expr, tree: TokenTree) -> Result<Expr, ()> {
        let op = match tree {
            TokenTree::Postfix('?') => UnOpKind::Try,
            _ => Err(())?,
        };
        Ok(Expr::UnOp(op, Box::new(lhs)))
    }
}

fn main() {
    let mut args = std::env::args();
    let _ = args.next();
    let input = args.next().expect("Expected input string");
    println!("Code: {}", input);
    let tt = grammar::TokenTreeParser::new().parse(&input).unwrap();
    println!("TokenTree: {:?}", tt);
    let expr = ExprParser.parse(&mut tt.into_iter()).unwrap();
    println!("Expression: {:?}", expr);
}

#[cfg(test)]
mod test {
    fn parse(input: &str) -> Expr {
        let tt = grammar::TokenTreeParser::new()
            .parse(input)
            .unwrap()
            .into_iter();
        ExprParser.parse(&mut tt.into_iter()).unwrap()
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
