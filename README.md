<h1 align="center">pratt - A General Purpose Pratt Parser for Rust</h1>

<p align="center">
  <img src="https://github.com/segeljakt/assets/blob/master/Trees.jpg?raw=true">
</p>

This crate provides offers a high-level interface for implementing Pratt parsers in Rust.

> In computer science, a Pratt parser is an improved recursive descent parser that associates semantics with tokens instead of grammar rules.
- https://en.wikipedia.org/wiki/Pratt_parser

In other words, you can use a Pratt parser to parse trees of expressions that might contain *unary*, *binary*, and *n-ary* operators of varying *precedence* and *associativity*.

## Example

Assume we have a strange language which should parse strings such as `-1?+1*!-1?` into `(((((-(1))?)+(1))*(!(-(1))))?)`.

Our strategy is to implement a parser which parses source code into token trees, and then token-trees into an expression tree. The full implementation can be viewed [here](https://www.github.com/segeljakt/).

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
    BinOp(Box<Expr>, BinOp, Box<Expr>),
    UnOp(UnOp, Box<Expr>),
    Int(i32),
    Unknown(String),
}

#[derive(Debug)]
pub enum BinOp {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
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
use pratt::{Associativity, Affix, ExprParser, Precedence};

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
            TokenTree::Group(group) => self.parse(&mut group.into_iter()),
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
```

Note that methods take `&mut self`, which allows the parser to store state while parsing, e.g. to accumulate errors and keep precedence/associativity information.

To run the parser:

```rust
fn main() {
    let tt = grammar::TokenTreeParser::new()
        .parse("-1?+1*!-1?")
        .unwrap();
    let expr = ExprParser
        .parse(&mut tt.into_iter())
        .unwrap();
    println!("{:#?}", expr);
}
```

Output:

```rust
UnOp(
    Try,
    BinOp(
        BinOp(
            UnOp(
                Try,
                UnOp(
                    Neg,
                    Int(
                        1,
                    ),
                ),
            ),
            Add,
            Int(
                1,
            ),
        ),
        Mul,
        UnOp(
            Not,
            UnOp(
                Neg,
                Int(
                    1,
                ),
            ),
        ),
    ),
)
```
