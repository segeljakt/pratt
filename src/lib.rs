pub enum Associativity {
    Left,
    Right,
}

#[derive(PartialEq, PartialOrd)]
pub struct Precedence(pub u32);

impl Precedence {
    fn lower(mut self) -> Precedence {
        self.0 -= 1;
        self
    }
}

pub enum Affix {
    Infix(Precedence, Associativity),
    Prefix(Precedence),
    Postfix(Precedence),
}

use std::iter::Peekable;

pub trait PrattParser<Inputs> where Inputs: Iterator<Item = Self::Input>{
    type Error;
    type Input: std::fmt::Debug;
    type Output: Sized;

    fn query(&mut self, input: &Self::Input) -> Option<Affix>;

    fn primary(&mut self, input: Self::Input) -> Result<Self::Output, Self::Error>;

    fn infix(&mut self, lhs: Self::Output, op: Self::Input, rhs: Self::Output) -> Result<Self::Output, Self::Error>;

    fn prefix(&mut self, op: Self::Input, rhs: Self::Output) -> Result<Self::Output, Self::Error>;

    fn postfix(&mut self, lhs: Self::Output, op: Self::Input) -> Result<Self::Output, Self::Error>;

    fn parse(&mut self, ref mut inputs: Inputs) -> Result<Self::Output, Self::Error> {
        self.parse_input(&mut inputs.peekable(), Precedence(0))
    }

    fn parse_input(&mut self, inputs: &mut Peekable<&mut Inputs>, rbp: Precedence) -> Result<Self::Output, Self::Error> {
        let mut lhs = self.nud(inputs); // Parse the prefix
        while rbp < self.lbp(inputs) {
            lhs = self.led(inputs, lhs?);
        }
        lhs
    }

    /// Null-Denotation
    fn nud(&mut self, inputs: &mut Peekable<&mut Inputs>) -> Result<Self::Output, Self::Error> {
        let input = inputs.next().expect("Pratt parsing expects non-empty inputs");
        match self.query(&input) {
            Some(Affix::Prefix(precedence)) => {
                let rhs = self.parse_input(inputs, precedence.lower());
                self.prefix(input, rhs?)
            }
            None => self.primary(input),
            _ => panic!(
                "Expected unary-prefix or primary expression, found {:?}",
                input
            ),
        }
    }

    /// Left-Denotation
    fn led(&mut self, inputs: &mut Peekable<&mut Inputs>, lhs: Self::Output) -> Result<Self::Output, Self::Error> {
        let input = inputs.next().expect("Pratt parsing expects non-empty inputs");
        match self.query(&input) {
            Some(Affix::Infix(precedence, associativity)) => {
                let rhs = match associativity {
                    Associativity::Left => self.parse_input(inputs, precedence),
                    Associativity::Right => self.parse_input(inputs, precedence.lower()),
                };
                self.infix(lhs, input, rhs?)
            }
            Some(Affix::Postfix(_)) => self.postfix(lhs, input),
            _ => panic!(
                "Expected unary-postfix or binary-infix expression, found {:?}",
                input
            ),
        }
    }

    /// Left-Binding-Power
    fn lbp(&mut self, inputs: &mut Peekable<&mut Inputs>) -> Precedence {
        match inputs.peek() {
            Some(input) => match self.query(input) {
                Some(Affix::Infix(precedence, _))
                | Some(Affix::Prefix(precedence))
                | Some(Affix::Postfix(precedence)) => precedence,
                None => panic!("Expected operator, found {:?}", input),
            },
            None => Precedence(0),
        }
    }
}
