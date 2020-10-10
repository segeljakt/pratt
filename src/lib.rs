use std::iter::Peekable;

#[derive(Copy, Clone)]
pub enum Associativity {
    Left,
    Right,
    Neither,
}

#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub struct Precedence(pub u32);

impl Precedence {
    const fn raise(mut self) -> Precedence {
        self.0 += 1;
        self
    }
    const fn lower(mut self) -> Precedence {
        self.0 -= 1;
        self
    }
    const fn normalize(mut self) -> Precedence {
        self.0 *= 10;
        self
    }
    const fn min() -> Precedence {
        Precedence(std::u32::MIN)
    }
    const fn max() -> Precedence {
        Precedence(std::u32::MAX)
    }
}

#[derive(Copy, Clone)]
pub enum Affix {
    Infix(Precedence, Associativity),
    Prefix(Precedence),
    Postfix(Precedence),
}

pub trait PrattParser<Inputs>
where
    Inputs: Iterator<Item = Self::Input>,
{
    type Error;
    type Input: std::fmt::Debug;
    type Output: Sized;

    fn query(&mut self, input: &Self::Input) -> Option<Affix>;

    fn primary(&mut self, input: Self::Input) -> Result<Self::Output, Self::Error>;

    fn infix(
        &mut self,
        lhs: Self::Output,
        op: Self::Input,
        rhs: Self::Output,
    ) -> Result<Self::Output, Self::Error>;

    fn prefix(&mut self, op: Self::Input, rhs: Self::Output) -> Result<Self::Output, Self::Error>;

    fn postfix(&mut self, lhs: Self::Output, op: Self::Input) -> Result<Self::Output, Self::Error>;

    fn parse(&mut self, inputs: &mut Inputs) -> Result<Self::Output, Self::Error> {
        self.parse_input(&mut inputs.peekable(), Precedence(0))
    }

    fn parse_input(
        &mut self,
        tail: &mut Peekable<&mut Inputs>,
        rbp: Precedence,
    ) -> Result<Self::Output, Self::Error> {
        if let Some(head) = tail.next() {
            let info = self.query(&head);
            let mut nbp = self.nbp(info)?;
            let mut node = self.nud(head, tail, info);
            loop {
                if let Some(head) = tail.peek() {
                    let info = self.query(head);
                    let lbp = self.lbp(info)?;
                    if rbp < lbp && lbp < nbp {
                        let head = tail.next().unwrap();
                        nbp = self.nbp(info)?;
                        node = self.led(head, tail, info, node?);
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
        head: Self::Input,
        tail: &mut Peekable<&mut Inputs>,
        info: Option<Affix>,
    ) -> Result<Self::Output, Self::Error> {
        match info {
            Some(Affix::Prefix(precedence)) => {
                let rhs = self.parse_input(tail, precedence.normalize().lower());
                self.prefix(head, rhs?)
            }
            None => self.primary(head),
            _ => panic!(
                "Expected unary-prefix or primary expression, found {:?}",
                head
            ),
        }
    }

    /// Left-Denotation
    fn led(
        &mut self,
        head: Self::Input,
        tail: &mut Peekable<&mut Inputs>,
        info: Option<Affix>,
        lhs: Self::Output,
    ) -> Result<Self::Output, Self::Error> {
        match info {
            Some(Affix::Infix(precedence, associativity)) => {
                let precedence = precedence.normalize();
                let rhs = match associativity {
                    Associativity::Left => self.parse_input(tail, precedence),
                    Associativity::Right => self.parse_input(tail, precedence.lower()),
                    Associativity::Neither => self.parse_input(tail, precedence.raise()),
                };
                self.infix(lhs, head, rhs?)
            }
            Some(Affix::Postfix(_)) => self.postfix(lhs, head),
            _ => panic!(
                "Expected unary-postfix or binary-infix expression, found {:?}",
                head
            ),
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
    fn lbp(&mut self, info: Option<Affix>) -> Result<Precedence, Self::Error> {
        let lbp = match info {
            None => panic!("Expected operator"),
            Some(Affix::Prefix(_)) => Precedence::min(),
            Some(Affix::Postfix(precedence)) => precedence.normalize(),
            Some(Affix::Infix(precedence, _)) => precedence.normalize(),
        };
        Ok(lbp)
    }

    /// Next-Binding-Power
    fn nbp(&mut self, info: Option<Affix>) -> Result<Precedence, Self::Error> {
        let nbp = match info {
            None => Precedence::max(),
            Some(Affix::Prefix(_)) => Precedence::max(),
            Some(Affix::Postfix(_)) => Precedence::max(),
            Some(Affix::Infix(precedence, Associativity::Left)) => precedence.normalize().raise(),
            Some(Affix::Infix(precedence, Associativity::Right)) => precedence.normalize().raise(),
            Some(Affix::Infix(precedence, Associativity::Neither)) => precedence.normalize(),
        };
        Ok(nbp)
    }
}
