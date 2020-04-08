use {std::iter::Peekable, Affix::*, Associativity::*};

pub enum Associativity {
    Null,
    Left,
    Right,
}

#[derive(PartialEq, PartialOrd)]
pub struct Precedence(pub u32);

impl Precedence {
    const fn lower(mut self) -> Precedence {
        self.0 -= 1;
        self
    }
    const fn raise(mut self) -> Precedence {
        self.0 += 1;
        self
    }
    const fn min() -> Precedence {
        Precedence(std::u32::MIN)
    }
    const fn max() -> Precedence {
        Precedence(std::u32::MAX)
    }
}

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

pub struct Op(pub Affix, pub Arity, pub Precedence);

pub trait PrattParser<Inputs>
where
    Inputs: Iterator<Item = Self::Input>,
{
    type Error: std::fmt::Debug;
    type Input: std::fmt::Debug + std::cmp::PartialEq;
    type Output: Sized + std::fmt::Debug;

    fn query(&mut self, input: &Self::Input) -> Result<Op, Self::Error>;

    fn nullary(&mut self, input: Self::Input) -> Result<Self::Output, Self::Error>;

    fn unary(&mut self, _: Self::Input, _: Self::Output) -> Result<Self::Output, Self::Error> {
        panic!("Encountered unary ouput while it was not implemented");
    }

    fn binary(
        &mut self,
        _: Self::Input,
        _: Self::Output,
        _: Self::Output,
    ) -> Result<Self::Output, Self::Error> {
        panic!("Encountered binary ouput while it was not implemented");
    }

    fn ternary(
        &mut self,
        _: Self::Input,
        _: Self::Output,
        _: Self::Output,
        _: Self::Output,
    ) -> Result<Self::Output, Self::Error> {
        panic!("Encountered ternary ouput while it was not implemented");
    }

    fn parse(&mut self, inputs: &mut Inputs) -> Result<Self::Output, Self::Error> {
        self.parse_until(&mut inputs.peekable(), Precedence::min())
    }

    fn parse_until(
        &mut self,
        inputs: &mut Peekable<&mut Inputs>,
        rbp: Precedence,
    ) -> Result<Self::Output, Self::Error> {
        if let Some(input) = inputs.next() {
            let mut nbp = self.nbp(&input)?;
            let mut node = self.nud(input, inputs);
            loop {
                if let Some(input) = inputs.peek() {
                    let lbp = self.lbp(input)?;
                    if rbp < lbp && lbp < nbp {
                        let input = inputs.next().unwrap();
                        nbp = self.nbp(&input)?;
                        node = self.led(input, inputs, node?);
                    } else {
                        break node;
                    }
                } else {
                    break node;
                }
            }
        } else {
            panic!()
        }
    }

    /// Null-Denotation
    fn nud(
        &mut self,
        input: Self::Input,
        inputs: &mut Peekable<&mut Inputs>,
    ) -> Result<Self::Output, Self::Error> {
        match self.query(&input)? {
            Op(Affix::Nilfix, Arity::Nullary, _) => self.nullary(input),
            // &a
            Op(Affix::Prefix, Arity::Unary, bp) => {
                let rbp = bp.lower();
                let rhs = self.parse_until(inputs, rbp)?;
                self.unary(input, rhs)
            }
            // if a then b else c
            Op(Affix::Prefix, Arity::Ternary, bp) => {
                let rbp = bp.lower();
                let lhs = self.parse_until(inputs, Precedence::min())?;
                self.eat_interfix(inputs)?;
                let mid = self.parse_until(inputs, Precedence::min())?;
                self.eat_interfix(inputs)?;
                let rhs = self.parse_until(inputs, rbp)?;
                self.ternary(input, lhs, mid, rhs)
            }
            // ||a||
            Op(Affix::Circumfix, Arity::Unary, bp) => {
                let rbp = bp.lower();
                let rhs = self.parse_until(inputs, rbp)?;
                self.eat_interfix(inputs)?;
                self.unary(input, rhs)
            }
            _ => panic!(
                "Expected unary-prefix or nullary-nilfix operator, found {:?}",
                input
            ),
        }
    }

    fn eat_interfix(&mut self, inputs: &mut Peekable<&mut Inputs>) -> Result<(), Self::Error> {
        if let Some(input) = inputs.peek() {
            match self.query(input)? {
                Op(Affix::Interfix, ..) | Op(Affix::Circumfix, ..) => {
                    inputs.next();
                }
                _ => {}
            };
        }
        Ok(())
    }

    /// Left-Denotation
    fn led(
        &mut self,
        input: Self::Input,
        inputs: &mut Peekable<&mut Inputs>,
        lhs: Self::Output,
    ) -> Result<Self::Output, Self::Error> {
        match self.query(&input)? {
            // a!
            Op(Affix::Postfix, Arity::Unary, _) => self.unary(input, lhs),
            // x = 1 x
            Op(Affix::Postfix, Arity::Ternary, bp) => {
                let rbp = bp.lower();
                let mid = self.parse_until(inputs, Precedence::min())?;
                self.eat_interfix(inputs)?;
                let rhs = self.parse_until(inputs, rbp)?;
                self.ternary(input, lhs, mid, rhs)
            }
            // a + b
            Op(Affix::Infix(associativity), Arity::Binary, bp) => {
                let rbp = match associativity {
                    Left => bp,
                    Right => bp.lower(),
                    Null => bp,
                };
                let rhs = self.parse_until(inputs, rbp)?;
                self.binary(input, lhs, rhs)
            }
            // a ? b : c
            Op(Affix::Infix(associativity), Arity::Ternary, bp) => {
                let rbp = match associativity {
                    Left => bp,
                    Right => bp.lower(),
                    Null => bp,
                };
                let mid = self.parse_until(inputs, Precedence::min())?;
                self.eat_interfix(inputs)?;
                let rhs = self.parse_until(inputs, rbp)?;
                self.ternary(input, lhs, mid, rhs)
            }
            _ => panic!(
                "Expected unary-postfix or binary-infix expression, found {:?}",
                input
            ),
        }
    }

    /// Left-Binding-Power
    fn lbp(&mut self, input: &Self::Input) -> Result<Precedence, Self::Error> {
        let lbp = match self.query(input)? {
            Op(Interfix, ..) => Precedence::min(),
            Op(Circumfix, ..) => Precedence::min(),
            Op(Nilfix, ..) => Precedence::min(),
            Op(Prefix, ..) => Precedence::min(),
            Op(Postfix, _, bp) => bp,
            Op(Infix(_), _, bp) => bp,
        };
        Ok(lbp)
    }

    //         <lbp>  <rbp>  <nbp> <kind>
    // Nilfix:  MIN |  MIN |  MAX | nud
    // Prefix:  MIN |   bp |  MAX | nud
    // Postfix:  bp |  MIN |  MAX | led
    // InfixL:   bp |   bp | bp+1 | led
    // InfixR:   bp | bp-1 | bp+1 | led
    // InfixN:   bp |   bp |   bp | led
    // Mixfix:

    /// Next-Binding-Power
    fn nbp(&mut self, input: &Self::Input) -> Result<Precedence, Self::Error> {
        let nbp = match self.query(input)? {
            Op(Interfix, ..) => Precedence::max(),
            Op(Circumfix, ..) => Precedence::max(),
            Op(Nilfix, ..) => Precedence::max(),
            Op(Prefix, ..) => Precedence::max(),
            Op(Postfix, ..) => Precedence::max(),
            Op(Infix(Left), _, bp) => bp.raise(),
            Op(Infix(Right), _, bp) => bp.raise(),
            Op(Infix(Null), _, bp) => bp,
        };
        Ok(nbp)
    }
}
