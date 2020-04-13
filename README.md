<h1 align="center">pratt - A General Purpose Pratt Parser for Rust</h1>
<h2 align="center">(EXPERIMENTAL WORK IN PROGRESS)</h2>

![crates.io (latest)](https://img.shields.io/crates/dv/pratt)

<p align="center">
  <img src="https://github.com/segeljakt/assets/blob/master/Trees.jpg?raw=true">
</p>

This crate leverages a high-level interface for implementing Pratt parsers in Rust.

> In computer science, a Pratt parser is an improved recursive descent parser that associates semantics with tokens instead of grammar rules.
- https://en.wikipedia.org/wiki/Pratt_parser

In other words, you can use a Pratt parser to parse trees of expressions that might contain different kinds of operators.

# Operator Parsing

**Operators** in programming languages have properties which impact how they are parsed. There are four notable properties: **arity**, **affix**, **precedence**, and **associativity**. These are explained in the following sections.

## Arity

[**Arity**](https://en.wikipedia.org/wiki/Arity) determines the number of operands an operator takes.
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

## Affix

[**Affix**](https://en.wikipedia.org/wiki/Affix) determines in which position the operator occurs:

```
a    # Nilfix    (Nowhere)
!a   # Prefix    (Before)
b?   # Postfix   (After)
a+b  # Infix     (Inbetween, connecting)
a,b  # Interfix  (Inbetween, separating)
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

## Precedence

[**Precedence**](https://en.wikipedia.org/wiki/Order_of_operations) (or binding power) determines the order of operations.

```
a*b+c == (a*b)+c  # * > +
a*b?  == a*(b?)   # ? > *
!b?   == (!b)?    # ! > ?
```

## Associativity

[**Associativity**](https://en.wikipedia.org/wiki/Associative_property) determines how operators nest.

```
a-b-c     == (a-b)-c         # Left-associative
a^b^c     == a^(b^c)         # Right-associative
a==b      == (a==b)          # Non-associative (Must be parenthesized)
a?b:c?d:e == a?(b:(c?(d:e))) # Right-associative
```

# Reflections

From one point of view, a Pratt parser can be viewed as a virtual machine which reads tokens (operators) and executes operations.

If we look at tokens from common language constructs, I believe some have almost direct translations into the operator terminology:

```
if a then b else c
  * if        # prefix-ternary (w.r.t a)
  * a,b,c     # nilfix-nullary
  * then      # interfix-nullary
  * else      # interfix-nullary

* Nullary operators have minimum precedence:
  if a then if b then c else d else e == if a then (if b then c else d) else e

x = 1
x
  * x,1       # nilfix-nullary
  * =         # postfix-ternary

[a;b;c]
  * [,]       # circumfix-N-ary
  * ;         # interfix-nullary
  * a,b,c     # nilfix-nullary
```

## Example

The goal of this crate is to offer a declarative interface for parsing generic operators. An generic operator is defined by its name, arity, affix, and precedence. Infix operators also have associativity.

```rust
// Op::new(<Name>, <Arity>, <Affix>, <Associativity>);

pub enum Arity {
    Nullary,
    Unary,
    Binary,
    Ternary,
}

pub enum Affix {
    Circumfix,
    Interfix,
    Nilfix,
    Prefix,
    Postfix,
    Infix(Associativity),
}

pub enum Associativity {
    Null,
    Left,
    Right,
}
```

A work in progress example of using the parser can be viewed here [here](https://github.com/segeljakt/pratt/tree/master/example/src/main.rs).
