use crate::*;

impl Token {
    pub fn is_valid_ref_start(&self) -> bool {
        match self {
            Each | Term(_) => true,
            _ => false,
        }
    }
}

impl PartialEq for IndexedToken {
    fn eq(&self, other: &Self) -> bool {
        if self.token == other.token {
            return true;
        }
        false
    }
    fn ne(&self, other: &Self) -> bool {
        if self.token != other.token {
            return true;
        }
        false
    }
}
impl PartialEq<Token> for IndexedToken {
    fn eq(&self, other: &Token) -> bool {
        self.token.eq(other)
    }
    fn ne(&self, other: &Token) -> bool {
        self.token.ne(other)
    }
}
impl PartialEq<&Token> for IndexedToken {
    fn eq(&self, other: &&Token) -> bool {
        self.token.eq(other)
    }
    fn ne(&self, other: &&Token) -> bool {
        self.token.ne(other)
    }
}
impl PartialEq<Token> for &IndexedToken {
    fn eq(&self, other: &Token) -> bool {
        self.token.eq(other)
    }
    fn ne(&self, other: &Token) -> bool {
        self.token.ne(other)
    }
}

impl<'a> PartialEq<&'a Token> for Token {
    fn eq(&self, other: &&'a Token) -> bool {
        self == *other
    }
}

impl<'a> PartialEq<Token> for &'a Token {
    fn eq(&self, other: &Token) -> bool {
        *self == other
    }
}
