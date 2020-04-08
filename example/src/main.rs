use lalrpop_util::lalrpop_mod;
use pratt::{Affix, Arity, Associativity, Op, PrattParser, Precedence};

lalrpop_mod!(pub grammar);

#[derive(Debug)]
pub enum Expr {
    UnaryOp(UnaryOp, Box<Expr>),
    BinaryOp(BinaryOp, Box<Expr>, Box<Expr>),
    TernaryOp(TernaryOp, Box<Expr>, Box<Expr>, Box<Expr>),
    Int(i32),
    Ident(String),
}

#[derive(Debug)]
pub enum TernaryOp {
    IfThenElse,
    Assign,
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
}

#[derive(Debug)]
pub enum UnaryOp {
    Fac,
    Ref,
    Abs,
    Norm,
}

#[derive(Debug, PartialEq)]
pub enum TokenTree<'i> {
    Group(Vec<TokenTree<'i>>),
    Literal(&'i str),
    Ident(&'i str),
    Keyword(&'i str),
    Punct(&'i str),
}

struct ExprParser;

impl<'i, I> PrattParser<I> for ExprParser
where
    I: Iterator<Item = TokenTree<'i>>,
{
    type Error = ();
    type Input = TokenTree<'i>;
    type Output = Expr;

    // Query information about an operator
    fn query(&mut self, t: &TokenTree) -> Result<Op, ()> {
        use {Affix::*, Arity::*, Associativity::*, TokenTree::*};
        let affix = match t {
            Punct("||") => Op(Circumfix, Unary, Precedence(5)),
            Punct("|") => Op(Circumfix, Unary, Precedence(5)),
            Punct("==") => Op(Infix(Null), Binary, Precedence(1)),
            Punct("+") => Op(Infix(Left), Binary, Precedence(3)),
            Punct("-") => Op(Infix(Left), Binary, Precedence(3)),
            Punct("*") => Op(Infix(Right), Binary, Precedence(3)),
            Punct("/") => Op(Infix(Right), Binary, Precedence(3)),
            Punct("&") => Op(Prefix, Unary, Precedence(4)),
            Punct("!") => Op(Postfix, Unary, Precedence(4)),
            Punct("?") => Op(Infix(Right), Ternary, Precedence(4)),
            Punct(":") => Op(Interfix, Nullary, Precedence(0)),
            Keyword("if") => Op(Prefix, Ternary, Precedence(4)),
            Keyword("then") => Op(Interfix, Nullary, Precedence(0)),
            Keyword("else") => Op(Interfix, Nullary, Precedence(0)),
            Punct("=") => Op(Postfix, Ternary, Precedence(9)),
            Ident(_) => Op(Nilfix, Nullary, Precedence(0)),
            Literal(_) => Op(Nilfix, Nullary, Precedence(0)),
            Group(_) => Op(Nilfix, Nullary, Precedence(0)),
            _ => Err(())?,
        };
        Ok(affix)
    }

    // Construct a nullary expression, e.g. a number
    fn nullary(&mut self, t: TokenTree<'i>) -> Result<Expr, ()> {
        use TokenTree::*;
        let expr = match t {
            Literal(s) => Expr::Int(s.parse::<i32>().unwrap()),
            Ident(s) => Expr::Ident(s.to_owned()),
            Group(group) => self.parse(&mut group.into_iter())?,
            _ => Err(())?,
        };
        Ok(expr)
    }

    // Construct a unary expression, e.g. 1! or &1
    fn unary(&mut self, t: TokenTree<'i>, r: Expr) -> Result<Expr, ()> {
        use TokenTree::*;
        let op = match t {
            Punct("!") => UnaryOp::Fac,
            Punct("&") => UnaryOp::Ref,
            Punct("|") => UnaryOp::Abs,
            Punct("||") => UnaryOp::Norm,
            _ => Err(())?,
        };
        Ok(Expr::UnaryOp(op, Box::new(r)))
    }

    // Construct an binary expression, e.g. 1+1
    fn binary(&mut self, t: TokenTree<'i>, l: Expr, r: Expr) -> Result<Expr, ()> {
        use TokenTree::*;
        println!("!");
        let op = match t {
            Punct("+") => BinaryOp::Add,
            Punct("-") => BinaryOp::Sub,
            Punct("*") => BinaryOp::Mul,
            Punct("/") => BinaryOp::Div,
            Punct("==") => BinaryOp::Eq,
            _ => Err(())?,
        };
        Ok(Expr::BinaryOp(op, Box::new(l), Box::new(r)))
    }

    // Construct an ternary expression, e.g. 1 ? 2 : 3 or if a then b else c
    fn ternary(&mut self, t: TokenTree<'i>, l: Expr, m: Expr, r: Expr) -> Result<Expr, ()> {
        use TokenTree::*;
        let op = match t {
            Keyword("if") => TernaryOp::IfThenElse,
            Punct("?") => TernaryOp::IfThenElse,
            Punct("=") => TernaryOp::Assign,
            _ => Err(())?,
        };
        Ok(Expr::TernaryOp(op, Box::new(l), Box::new(m), Box::new(r)))
    }
}

fn main() {
//     let ts = grammar::TokenStreamParser::new()
//         .parse("1 == 1 == 1")
//         .unwrap();
//     ExprParser.parse(&mut ts.into_iter()).unwrap();
//
//     let ts = grammar::TokenStreamParser::new()
//         .parse("1?1:1?1:1")
//         .unwrap();
//     ExprParser.parse(&mut ts.into_iter()).unwrap();
//
//     let ts = grammar::TokenStreamParser::new()
//         .parse("if 1 then if 2 then 3 else 4 else 5")
//         .unwrap();
//     ExprParser.parse(&mut ts.into_iter()).unwrap();
//
//     let ts = grammar::TokenStreamParser::new()
//         .parse("x = 1 (y = x z = y z)")
//         .unwrap();
//     ExprParser.parse(&mut ts.into_iter()).unwrap();
//
    let ts = grammar::TokenStreamParser::new()
        .parse("|x| + |y|")
        .unwrap();
    dbg!(ExprParser.parse(&mut ts.into_iter()).unwrap());
}
