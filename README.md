<h1 align="center">pratt - A General Purpose Pratt Parser for Rust</h1>

[<img alt="github" src="https://img.shields.io/badge/github-segeljakt/pratt-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/segeljakt/pratt)
[<img alt="crates.io" src="https://img.shields.io/crates/v/pratt.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/pratt)
[<img alt="crates.io" src="https://img.shields.io/crates/dv/pratt?style=for-the-badge&labelColor=555555&logoColor=white&logo=rust" height="20">](https://crates.io/crates/pratt)

<p align="center">
  <img src="https://github.com/segeljakt/assets/blob/master/Trees.jpg?raw=true">
</p>

This crate leverages a high-level interface for implementing Pratt parsers in Rust.

> In computer science, a Pratt parser is an improved recursive descent parser that associates semantics with tokens instead of grammar rules.
- https://en.wikipedia.org/wiki/Pratt_parser

In other words, you can use a Pratt parser to parse trees of expressions that might contain *unary* and *binary* operators of varying *precedence* and *associativity*.

## Example

Assume we want to parse an expression `!1?*-3+3/!2^4?-1` into `(((((!(1))?)*(-(3)))+((3)/((!((2)^(4)))?)))-(1))`.

Our strategy is to implement a parser which parses source code into token trees, and then token-trees into an expression tree. The full implementation can be viewed [here](https://github.com/segeljakt/pratt/tree/master/example).

```rust
// From this
#[derive(Debug)]
pub enum TokenTree {
    Prefix(char),
    Postfix(char),
    Infix(char),
    Primary(i32),
    Group(Vec<TokenTree>),
}

// To this
#[derive(Debug)]
pub enum Expr {
    BinOp(Box<Expr>, BinOpKind, Box<Expr>),
    UnOp(UnOpKind, Box<Expr>),
    Int(i32),
}

#[derive(Debug)]
pub enum BinOpKind {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Pow, // ^
    Eq,  // =
}

#[derive(Debug)]
pub enum UnOp {
    Not, // !
    Neg, // -
    Try, // ?
}
```

We implement the parser from source code into token-trees with [LALRPOP](https://github.com/lalrpop/lalrpop).

<details><summary>LALRPOP Grammar</summary>
<p>

```rust
use crate::TokenTree;

grammar;

pub TokenTree = Group;

Group: Vec<TokenTree> = <prefix:Prefix*> <primary:Primary> <mut postfix:Postfix*>
                   <rest:(Infix Prefix* Primary Postfix*)*> => {
    let mut group = prefix;
    group.push(primary);
    group.append(&mut postfix);
    for (infix, mut prefix, primary, mut postfix) in rest {
        group.push(infix);
        group.append(&mut prefix);
        group.push(primary);
        group.append(&mut postfix);
    }
    group
};

Primary: TokenTree = {
    "(" <Group> ")" => TokenTree::Group(<>),
    r"[0-9]+"       => TokenTree::Primary(<>.parse::<i32>().unwrap()),
}

Infix: TokenTree = {
    "+" => TokenTree::Infix('+'),
    "-" => TokenTree::Infix('-'),
    "*" => TokenTree::Infix('*'),
    "/" => TokenTree::Infix('/'),
    "=" => TokenTree::Infix('='),
    "^" => TokenTree::Infix('^'),
}

Prefix: TokenTree = {
    "-" => TokenTree::Prefix('-'),
    "!" => TokenTree::Prefix('!'),
}

Postfix: TokenTree = {
    "?" => TokenTree::Postfix('?'),
}
```

</p>
</details>

Then, for the Pratt parser, we define a `struct ExprParser` and implement `pratt::ExprParser` for it.

```rust
use pratt::{Affix, Associativity, PrattParser, Precedence, Result};

struct ExprParser;

impl<I> PrattParser<I> for ExprParser
where
    I: Iterator<Item = TokenTree>,
{
    type Error = pratt::NoError;
    type Input = TokenTree;
    type Output = Expr;

    // Query information about an operator (Affix, Precedence, Associativity)
    fn query(&mut self, tree: &TokenTree) -> Result<Affix> {
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
            TokenTree::Group(_) => Affix::Nilfix,
            TokenTree::Primary(_) => Affix::Nilfix,
            _ => unreachable!(),
        };
        Ok(affix)
    }

    // Construct a primary expression, e.g. a number
    fn primary(&mut self, tree: TokenTree) -> Result<Expr> {
        let expr = match tree {
            TokenTree::Primary(num) => Expr::Int(num),
            TokenTree::Group(group) => self.parse(&mut group.into_iter()).unwrap(),
            _ => unreachable!(),
        };
        Ok(expr)
    }

    // Construct a binary infix expression, e.g. 1+1
    fn infix(&mut self, lhs: Expr, tree: TokenTree, rhs: Expr) -> Result<Expr> {
        let op = match tree {
            TokenTree::Infix('+') => BinOpKind::Add,
            TokenTree::Infix('-') => BinOpKind::Sub,
            TokenTree::Infix('*') => BinOpKind::Mul,
            TokenTree::Infix('/') => BinOpKind::Div,
            TokenTree::Infix('^') => BinOpKind::Pow,
            TokenTree::Infix('=') => BinOpKind::Eq,
            _ => unreachable!(),
        };
        Ok(Expr::BinOp(Box::new(lhs), op, Box::new(rhs)))
    }

    // Construct a unary prefix expression, e.g. !1
    fn prefix(&mut self, tree: TokenTree, rhs: Expr) -> Result<Expr> {
        let op = match tree {
            TokenTree::Prefix('!') => UnOpKind::Not,
            TokenTree::Prefix('-') => UnOpKind::Neg,
            _ => unreachable!(),
        };
        Ok(Expr::UnOp(op, Box::new(rhs)))
    }

    // Construct a unary postfix expression, e.g. 1?
    fn postfix(&mut self, lhs: Expr, tree: TokenTree) -> Result<Expr> {
        let op = match tree {
            TokenTree::Postfix('?') => UnOpKind::Try,
            _ => unreachable!(),
        };
        Ok(Expr::UnOp(op, Box::new(lhs)))
    }
}
```

Note that methods take `&mut self`, which allows the parser to store state while parsing, e.g. to accumulate errors and keep precedence/associativity information.

To run the parser:

```rust
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
```

Plus some tests:

```rust
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
```
