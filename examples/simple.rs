use pratt::{Affix, Associativity, PrattParser, Precedence};

mod grammar;

#[derive(Debug)]
pub enum Expr {
    BinOp(Box<Expr>, BinOp, Box<Expr>),
    UnOp(UnOp, Box<Expr>),
    Int(i32),
    Unknown(String),
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum UnOp {
    Not,
    Neg,
    Try,
}

#[derive(Debug)]
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
            TokenTree::Postfix('?') => Affix::Postfix(Precedence(1)),
            TokenTree::Infix('+') => Affix::Infix(Precedence(2), Associativity::Left),
            TokenTree::Infix('-') => Affix::Infix(Precedence(2), Associativity::Left),
            TokenTree::Infix('*') => Affix::Infix(Precedence(2), Associativity::Right),
            TokenTree::Infix('/') => Affix::Infix(Precedence(2), Associativity::Right),
            TokenTree::Prefix('-') => Affix::Prefix(Precedence(3)),
            TokenTree::Prefix('!') => Affix::Prefix(Precedence(3)),
            _ => None?,
        };
        Some(affix)
    }

    // Construct a primary expression, e.g. a number
    fn primary(&mut self, tree: TokenTree) -> Result<Expr, ()> {
        match tree {
            TokenTree::Primary(num) => Ok(Expr::Int(num)),
            TokenTree::Group(group) => self.parse(group.into_iter()),
            _ => Err(()),
        }
    }

    // Construct an binary infix expression, e.g. 1+1
    fn infix(&mut self, lhs: Expr, tree: TokenTree, rhs: Expr) -> Result<Expr, ()> {
        let op = match tree {
            TokenTree::Infix('+') => BinOp::Add,
            TokenTree::Infix('-') => BinOp::Sub,
            TokenTree::Infix('*') => BinOp::Mul,
            TokenTree::Infix('/') => BinOp::Div,
            _ => Err(())?,
        };
        Ok(Expr::BinOp(Box::new(lhs), op, Box::new(rhs)))
    }

    // Construct an unary prefix expression, e.g. !1
    fn prefix(&mut self, tree: TokenTree, rhs: Expr) -> Result<Expr, ()> {
        let op = match tree {
            TokenTree::Prefix('!') => UnOp::Not,
            TokenTree::Prefix('-') => UnOp::Neg,
            _ => Err(())?,
        };
        Ok(Expr::UnOp(op, Box::new(rhs)))
    }

    // Construct an unary postfix expression, e.g. 1?
    fn postfix(&mut self, lhs: Expr, tree: TokenTree) -> Result<Expr, ()> {
        let op = match tree {
            TokenTree::Postfix('?') => UnOp::Try,
            _ => Err(())?,
        };
        Ok(Expr::UnOp(op, Box::new(lhs)))
    }
}

fn main() {
    let tt = grammar::TokenTreeParser::new()
        .parse("-1?+1*!-1?")
        .unwrap();
    let expr = ExprParser
        .parse(tt.into_iter())
        .unwrap();
    println!("{:#?}", expr);
}

