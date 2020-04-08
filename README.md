<h1 align="center">pratt - A General Purpose Pratt Parser for Rust</h1>

![Crates.io (latest)](https://img.shields.io/crates/dv/pratt)

<p align="center">
  <img src="https://github.com/segeljakt/assets/blob/master/Trees.jpg?raw=true">
</p>

This crate leverages a high-level interface for implementing Pratt parsers in Rust.

> In computer science, a Pratt parser is an improved recursive descent parser that associates semantics with tokens instead of grammar rules.
- https://en.wikipedia.org/wiki/Pratt_parser

In other words, you can use a Pratt parser to parse trees of expressions that might contain different kinds of operators.

# Operator Theory

Operators in programming, and mathematics for that matter, have properties which impact how they are parsed. There are four notable properties:

* [Arity](https://en.wikipedia.org/wiki/Arity): Determines the number of operands an operator takes.

```
a     # Nullary (0 operands)
!a    # Unary   (1 operand)
a+b   # Binary  (2 operands)
a?b:c # Ternary (3 operands)
```

The full list:

```
 #  Arity (Latin) : Adicity (Greek)
  -----------------------------------
 0: Nullary       : Niladic
 1: Unary         : Monadic
 2: Binary        : Dyadic
 3: Ternary       : Triadic
 4: Quaternary    : Tetradic
 5: Quinary       : Pentadic
 6: Senary        : Hexadic
 7: Septenary     : Hebdomadic
 8: Octonary      : Ogdoadic
 9: Novenary      : Enneadic
10: Denary        : Decadic

>2: Multary       : Polyadic
  : N-Ary         : Variadic
```

* [Affix](https://en.wikipedia.org/wiki/Affix): Determines in which position the operator occurs:

```
a    # Nilfix    (Nowhere)
!a   # Prefix    (Before)
b?   # Postfix   (After)
a+b  # Infix     (Inbetween connector)
a,b  # Interfix  (Inbetween separator)
[a]  # Circumfix (Around)
```

In linguistics, we have:

```
Affix        Schema                       Description
-----------------------------------------------------
Nilfix     : expr                       : Appears nowhere
Prefix     : <prefix-expr               : Appears before the expr
Prefixoid  : <prefixoid>-expr           : Appears before the expr, but is only partially bound to it
Postfix    : expr-<postfix>             : Appears after the expr
Postfixoid : expr-<postfixoid>          : Appears after the expr, but is only partially bound to it
Infix      : ex<infix>pr                : Appears within a expr
Circumfix  : <circumfix>expr<circumfix> : One portion appears before the expr, the other after
Interfix   : expr<interfix>expr         : Links two expr together in a compound
Duplifix   : expr<duplifix>             : Incorporates a reduplicated portion of an expr
Transfix   : e<transfix>xp<transfix>r   : A discontinuous affix that interleaves within a discontinuous expr
Simulfix   : expr\simulfix              : Changes a segment of a expr
Suprafix   : expr\suprafix              : Changes a suprasegmental feature of a expr
Disfix     : ex⟩disfix⟨pr               : The elision of a portion of a expr
```

* [Precedence](https://en.wikipedia.org/wiki/Order_of_operations) (or binding power): Determines the order of operations.

```
a*b+c == (a*b)+c  # * > +
a*b?  == a*(b?)   # ? > *
!b?   == (!b)?    # ! > ?
```

* [Associativity](https://en.wikipedia.org/wiki/Associative_property): Determines how operators nest.

```
a-b-c     == (a-b)-c         # Left-associative
a^b^c     == a^(b^c)         # Right-associative
a==b      == (a==b)          # Non-associative (Must be parenthesized)
a?b:c?d:e == a?(b:(c?(d:e))) # Right-associative
```

# Reflections

From standard terminology, a parser can be viewed as a virtual machine which reads tokens (operators) and executes operations.

We should be able to translate any kind of token into an operation.
```
if a then b else c
  * if        = prefix-ternary (w.r.t a)
  * a,b,c     = nilfix-nullary
  * then      = interfix-nullary
  * else      = interfix-nullary

* Nullary operators always have minimum precedence:
  if a then if b then c else d else e == if a then (if b then c else d) else e

x = 1
x
  * x,1       = nilfix-nullary
  * =         = postfix-ternary

[a;b;c]
  * [,]       = circumfix-N-ary
  * ;         = interfix-nullary
  * a,b,c     = nilfix-nullary
```

## Example

Theory aside, what is this crate good for? Assume we have a strange language which should parse strings such as `-1?+1*!-1?` into `(((((-(1))?)+(1))*(!(-(1))))?)`.

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
