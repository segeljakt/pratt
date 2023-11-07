#![no_std]

#[derive(Copy, Clone)]
pub enum Associativity {
    Left,
    Right,
    Neither,
}

#[derive(PartialEq, Eq, PartialOrd, Copy, Clone)]
pub struct Precedence(pub u32);

impl Precedence {
    const fn raise(mut self) -> Precedence {
        self.0 = self.0.saturating_add(1);
        self
    }
    const fn lower(mut self) -> Precedence {
        self.0 = self.0.saturating_sub(1);
        self
    }
    const fn normalize(mut self) -> Precedence {
        self.0 = self.0.saturating_mul(10);
        self
    }
    const fn min() -> Precedence {
        Precedence(u32::MIN)
    }
    const fn max() -> Precedence {
        Precedence(u32::MAX)
    }
}

#[derive(Copy, Clone)]
pub enum Affix {
    Nilfix,
    Infix(Precedence, Associativity),
    Prefix(Precedence),
    Postfix(Precedence),
}

#[derive(Debug)]
pub enum PrattError<I: core::fmt::Debug, E: core::fmt::Display> {
    UserError(E),
    EmptyInput,
    UnexpectedNilfix(I),
    UnexpectedPrefix(I),
    UnexpectedInfix(I),
    UnexpectedPostfix(I),
}

impl<I: core::fmt::Debug, E: core::fmt::Display> core::fmt::Display for PrattError<I, E> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            PrattError::UserError(e) => write!(f, "{}", e),
            PrattError::EmptyInput => write!(f, "Pratt parser was called with empty input."),
            PrattError::UnexpectedNilfix(t) => {
                write!(f, "Expected Infix or Postfix, found Nilfix {:?}", t)
            }
            PrattError::UnexpectedPrefix(t) => {
                write!(f, "Expected Infix or Postfix, found Prefix {:?}", t)
            }
            PrattError::UnexpectedInfix(t) => {
                write!(f, "Expected Nilfix or Prefix, found Infix {:?}", t)
            }
            PrattError::UnexpectedPostfix(t) => {
                write!(f, "Expected Nilfix or Prefix, found Postfix {:?}", t)
            }
        }
    }
}

#[derive(Debug)]
pub struct NoError;

impl core::fmt::Display for NoError {
    fn fmt(&self, _: &mut core::fmt::Formatter) -> core::fmt::Result {
        Ok(())
    }
}

pub type Result<T> = core::result::Result<T, NoError>;

pub trait PrattParser<Inputs>
where
    Inputs: Iterator<Item = Self::Input>,
{
    type Error: core::fmt::Display;
    type Input: core::fmt::Debug;
    type Output: Sized;

    fn query(&mut self, input: &Self::Input) -> core::result::Result<Affix, Self::Error>;

    fn primary(&mut self, input: Self::Input) -> core::result::Result<Self::Output, Self::Error>;

    fn infix(
        &mut self,
        lhs: Self::Output,
        op: Self::Input,
        rhs: Self::Output,
    ) -> core::result::Result<Self::Output, Self::Error>;

    fn prefix(
        &mut self,
        op: Self::Input,
        rhs: Self::Output,
    ) -> core::result::Result<Self::Output, Self::Error>;

    fn postfix(
        &mut self,
        lhs: Self::Output,
        op: Self::Input,
    ) -> core::result::Result<Self::Output, Self::Error>;

    fn parse(
        &mut self,
        inputs: Inputs,
    ) -> core::result::Result<Self::Output, PrattError<Self::Input, Self::Error>> {
        self.parse_input(&mut inputs.peekable(), Precedence::min())
    }

    fn parse_peekable(
        &mut self,
        inputs: &mut core::iter::Peekable<Inputs>,
    ) -> core::result::Result<Self::Output, PrattError<Self::Input, Self::Error>> {
        self.parse_input(inputs, Precedence::min())
    }

    fn parse_input(
        &mut self,
        tail: &mut core::iter::Peekable<Inputs>,
        rbp: Precedence,
    ) -> core::result::Result<Self::Output, PrattError<Self::Input, Self::Error>> {
        if let Some(head) = tail.next() {
            let info = self.query(&head).map_err(PrattError::UserError)?;
            let mut nbp = self.nbp(info);
            let mut node = self.nud(head, tail, info);
            while let Some(head) = tail.peek() {
                let info = self.query(head).map_err(PrattError::UserError)?;
                let lbp = self.lbp(info);
                if rbp < lbp && lbp < nbp {
                    let head = tail.next().unwrap();
                    nbp = self.nbp(info);
                    node = self.led(head, tail, info, node?);
                } else {
                    break;
                }
            }
            node
        } else {
            Err(PrattError::EmptyInput)
        }
    }

    /// Null-Denotation
    fn nud(
        &mut self,
        head: Self::Input,
        tail: &mut core::iter::Peekable<Inputs>,
        info: Affix,
    ) -> core::result::Result<Self::Output, PrattError<Self::Input, Self::Error>> {
        match info {
            Affix::Prefix(precedence) => {
                let rhs = self.parse_input(tail, precedence.normalize().lower());
                self.prefix(head, rhs?).map_err(PrattError::UserError)
            }
            Affix::Nilfix => self.primary(head).map_err(PrattError::UserError),
            Affix::Postfix(_) => Err(PrattError::UnexpectedPostfix(head)),
            Affix::Infix(_, _) => Err(PrattError::UnexpectedInfix(head)),
        }
    }

    /// Left-Denotation
    fn led(
        &mut self,
        head: Self::Input,
        tail: &mut core::iter::Peekable<Inputs>,
        info: Affix,
        lhs: Self::Output,
    ) -> core::result::Result<Self::Output, PrattError<Self::Input, Self::Error>> {
        match info {
            Affix::Infix(precedence, associativity) => {
                let precedence = precedence.normalize();
                let rhs = match associativity {
                    Associativity::Left => self.parse_input(tail, precedence),
                    Associativity::Right => self.parse_input(tail, precedence.lower()),
                    Associativity::Neither => self.parse_input(tail, precedence.raise()),
                };
                self.infix(lhs, head, rhs?).map_err(PrattError::UserError)
            }
            Affix::Postfix(_) => self.postfix(lhs, head).map_err(PrattError::UserError),
            Affix::Nilfix => Err(PrattError::UnexpectedNilfix(head)),
            Affix::Prefix(_) => Err(PrattError::UnexpectedPrefix(head)),
        }
    }

    //         <lbp>  <rbp>  <nbp> <kind>
    // Nilfix:  MIN |  MIN |  MAX | nud
    // Prefix:  MIN |   bp |  MAX | nud
    // Postfix:  bp |  MIN |  MAX | led
    // InfixL:   bp |   bp | bp+1 | led
    // InfixR:   bp | bp-1 | bp+1 | led
    // InfixN:   bp |   bp |   bp | led

    /// Left-Binding-Power
    fn lbp(&mut self, info: Affix) -> Precedence {
        match info {
            Affix::Nilfix => Precedence::min(),
            Affix::Prefix(_) => Precedence::min(),
            Affix::Postfix(precedence) => precedence.normalize(),
            Affix::Infix(precedence, _) => precedence.normalize(),
        }
    }

    /// Next-Binding-Power
    fn nbp(&mut self, info: Affix) -> Precedence {
        match info {
            Affix::Nilfix => Precedence::max(),
            Affix::Prefix(_) => Precedence::max(),
            Affix::Postfix(_) => Precedence::max(),
            Affix::Infix(precedence, Associativity::Left) => precedence.normalize().raise(),
            Affix::Infix(precedence, Associativity::Right) => precedence.normalize().raise(),
            Affix::Infix(precedence, Associativity::Neither) => precedence.normalize(),
        }
    }
}
