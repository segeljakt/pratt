use std::fmt;
use std::ops::BitOr;

#[derive(Copy, Clone)]
pub enum Associativity {
    Left,
    Right,
    Neither,
}

#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub struct Precedence(pub u32);

impl Precedence {
    pub const MIN: Precedence = Precedence(u32::MIN);
    pub const MAX: Precedence = Precedence(u32::MAX);

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

    #[allow(dead_code)]
    #[deprecated = "replaced by the `MIN` associated constant on this type"]
    const fn min() -> Precedence {
        Precedence(std::u32::MIN)
    }

    #[allow(dead_code)]
    #[deprecated = "replaced by the `MAX` associated constant on this type"]
    const fn max() -> Precedence {
        Precedence(std::u32::MAX)
    }
}

#[derive(Copy, Clone)]
pub enum Affix {
    Nilfix,
    Infix(Precedence, Associativity),
    Prefix(Precedence),
    Postfix(Precedence),
    NilfixInfix(Precedence, Associativity),
    NilfixPostfix(Precedence),
    PrefixPostfix(Precedence, Precedence),
    PrefixInfix(Precedence, (Precedence, Associativity)),
}

impl Affix {
    fn as_nud(&self) -> Self {
        match *self {
            Self::NilfixInfix(..) | Self::NilfixPostfix(_) => Self::Nilfix,
            Self::PrefixInfix(p, _) | Self::PrefixPostfix(p, _) => Self::Prefix(p),
            other => other,
        }
    }

    fn as_led(&self) -> Self {
        match *self {
            Self::NilfixInfix(p, a) | Self::PrefixInfix(_, (p, a)) => Self::Infix(p, a),
            Self::NilfixPostfix(p) | Self::PrefixPostfix(_, p) => Self::Postfix(p),
            other => other,
        }
    }
}

impl BitOr for Affix {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Nilfix, Self::Infix(p, a)) | (Self::Infix(p, a), Self::Nilfix)
                => Self::NilfixInfix(p, a),
            (Self::Nilfix, Self::Postfix(p)) | (Self::Postfix(p), Self::Nilfix) => Self::NilfixPostfix(p),
            (Self::Prefix(p1), Self::Infix(p2, a)) | (Self::Infix(p2, a), Self::Prefix(p1))
                => Self::PrefixInfix(p1, (p1, a)),
            (Self::Prefix(p1), Self::Postfix(p2)) | (Self::Postfix(p2), Self::Prefix(p1))
                => Self::PrefixPostfix(p1, p2),
            _ => panic!("Invalid affix combination"),
        }
    }
}

#[derive(Debug)]
pub enum PrattError<I: fmt::Debug> {
    EmptyInput,
    UnexpectedNilfix(I),
    UnexpectedPrefix(I),
    UnexpectedInfix(I),
    UnexpectedPostfix(I),
}

impl<I: fmt::Debug> fmt::Display for PrattError<I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
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

pub trait PrattParser {
    type Error: From<PrattError<Self::Input>>;
    type Input: fmt::Debug;
    type Output: Sized;

    fn next(&mut self) -> Option<Self::Input>;

    fn peek(&mut self) -> Option<&Self::Input>;

    fn query(&mut self) -> Result<Affix, Self::Error>;

    fn primary(&mut self, input: Self::Input) -> Result<Self::Output, Self::Error>;

    fn infix(
        &mut self,
        lhs: Self::Output,
        op: Self::Input,
        rhs: Self::Output,
    ) -> Result<Self::Output, Self::Error>;

    fn prefix(&mut self, op: Self::Input, rhs: Self::Output) -> Result<Self::Output, Self::Error>;

    fn postfix(&mut self, lhs: Self::Output, op: Self::Input) -> Result<Self::Output, Self::Error>;

    fn parse(&mut self) -> Result<Self::Output, Self::Error> {
        self.parse_input(Precedence::MIN)
    }

    fn parse_input(&mut self, rbp: Precedence) -> Result<Self::Output, Self::Error> {
        self.parse_until(rbp, |_| false)
            .transpose()
            .unwrap_or(Err(PrattError::EmptyInput.into()))
    }

    fn parse_until<F>(&mut self, rbp: Precedence, mut pred: F) -> Result<Option<Self::Output>, Self::Error>
        where
            F: FnMut(&Self::Input) -> bool,
    {
        let info = self.query()?;
        if self.peek().is_some() && pred(self.peek().unwrap()) {
            return Ok(None);
        }
        let head = self.next().ok_or(PrattError::EmptyInput)?;
        let mut nbp = self.nbp(info);
        let mut node = self.nud(head, info);
        while self.peek().is_some() {
            if pred(self.peek().unwrap()) {
                break;
            }
            let info = self.query()?;
            let lbp = self.lbp(info);
            if rbp < lbp && lbp < nbp {
                let head = self.next().unwrap();
                nbp = self.nbp(info);
                node = self.led(head, info, node?);
            } else {
                break;
            }
        }
        node.map(Some)
    }

    /// Null-Denotation
    fn nud(&mut self, head: Self::Input, info: Affix) -> Result<Self::Output, Self::Error> {
        match info.as_nud() {
            Affix::Prefix(precedence) => {
                let rhs = self.parse_input(precedence.normalize().lower())?;
                self.prefix(head, rhs)
            }
            Affix::Nilfix => self.primary(head),
            Affix::Postfix(_) => Err(PrattError::UnexpectedPostfix(head).into()),
            Affix::Infix(_, _) => Err(PrattError::UnexpectedInfix(head).into()),
            _ => unreachable!()
        }
    }

    /// Left-Denotation
    fn led(
        &mut self,
        head: Self::Input,
        info: Affix,
        lhs: Self::Output,
    ) -> Result<Self::Output, Self::Error> {
        use Associativity::*;
        match info.as_led() {
            Affix::Infix(precedence, associativity) => {
                let precedence = match associativity {
                    Left => precedence.normalize(),
                    Right => precedence.normalize().lower(),
                    Neither => precedence.normalize().raise(),
                };
                let rhs = self.parse_input(precedence)?;
                self.infix(lhs, head, rhs)
            }
            Affix::Postfix(_) => self.postfix(lhs, head),
            Affix::Nilfix => Err(PrattError::UnexpectedNilfix(head).into()),
            Affix::Prefix(_) => Err(PrattError::UnexpectedPrefix(head).into()),
            _ => unreachable!(),
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
        match info.as_led() {
            Affix::Nilfix => Precedence::MIN,
            Affix::Prefix(_) => Precedence::MIN,
            Affix::Postfix(precedence) => precedence.normalize(),
            Affix::Infix(precedence, _) => precedence.normalize(),
            _ => unreachable!(),
        }
    }

    /// Next-Binding-Power
    fn nbp(&mut self, info: Affix) -> Precedence {
        use Associativity::*;
        match info.as_nud() {
            Affix::Nilfix | Affix::Prefix(_) | Affix::Postfix(_) => Precedence::MAX,
            Affix::Infix(precedence, Left | Right) => precedence.normalize().raise(),
            Affix::Infix(precedence, Neither) => precedence.normalize(),
            _ => unreachable!(),
        }
    }
}
