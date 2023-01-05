use crate::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Syntax {
    pub expects_input: bool,
    pub expected_args: Vec<Preposition>,
    pub returns: bool,
}

pub struct IndexedTokensIter(MultiPeek<IntoIter<IndexedToken>>);

impl IndexedTokensIter {
    pub fn init(indexed_tokens: Vec<IndexedToken>) -> Self {
        IndexedTokensIter(indexed_tokens.into_iter().multipeek())
    }
    pub fn take_token(&mut self) -> Result<Token> {
        if let Some(IndexedToken { token, .. }) = self.next() {
            Ok(token)
        } else {
            Err(Message("Expected token at the end of script"))
        }
    }
    pub fn take_relationship(&mut self) -> Result<Relational> {
        if let R(comparison) = self.take_token()? {
            Ok(comparison)
        } else {
            Err(Message("Expected comparison"))
        }
    }
    pub fn take_command(&mut self) -> Result<Command> {
        if let C(command) = self.take_token()? {
            Ok(command)
        } else {
            Err(Message("Expected command"))
        }
    }
    pub fn take_preposition(&mut self) -> Result<Preposition> {
        if let P(preposition) = self.take_token()? {
            Ok(preposition)
        } else {
            Err(Message("Expected preposition"))
        }
    }
    pub fn take_term(&mut self) -> Result<Text> {
        if let D(Term(term)) = self.take_token()? {
            Ok(term)
        } else {
            Err(Message("Expected term"))
        }
    }
    pub fn next_sentence_starts_with(&mut self, expected_token: Token) -> Result<bool> {
        self.0.reset_peek();
        while let Some(itoken) = self.0.peek() {
            if itoken == expected_token {
                return Ok(true);
            } else if itoken == Dot || itoken == Linebreak {
                continue;
            } else {
                break;
            }
        }
        Ok(false)
    }
    pub fn peek_token(&mut self) -> Result<&Token> {
        Ok(&self.peek_Itoken()?.token)
    }
    pub fn peek_Itoken(&mut self) -> Result<&IndexedToken> {
        if let Some(Itoken) = self.peek() {
            Ok(Itoken)
        } else {
            Err(Message("Expected some token but found nothing"))
        }
    }
    pub fn skip_optional(&mut self, token: Token) -> Result<()> {
        if self.peek_token()? == token {
            self.next();
        }
        Ok(())
    }
    pub fn skip_until(&mut self, token: Token) -> Result<()> {
        while self.peek_token()? != token {
            self.skip_any();
        }
        self.skip(token);
        Ok(())
    }
    pub fn skip(&mut self, token: Token) -> Result<()> {
        if self.peek_token()? == token {
            self.next();
            return Ok(());
        } else {
            return Err(ExpectedToken(token, 999));
        }
    }
    pub fn skip_any(&mut self) {
        self.next();
    }
    pub fn expect(&mut self, expected_token: Token) -> Result<Token> { // -> assert?
        if let Some(itoken) = self.next() {
            if itoken == expected_token {
                return Ok(expected_token);
            } else {
                return Err(ExpectedToken(expected_token, itoken.index));
            }
        }
        return Err(UnexpectedEnd);
    }
    pub fn peek_until(&mut self, end_token: Token) -> Result<Option<&IndexedToken>> {
        if let Some(itoken) = self.peek() {
            if itoken != end_token {
                Ok(Some(itoken))
            } else {
                Ok(None)
            }
        } else {
            Err(FormattedMesssage(format!(
                "Unexpected end, was looking for {:?}",
                end_token
            )))
        }
    }
    pub fn next_isnt(&mut self, token: Token) -> Result<bool> {
        if self.peek_token()? != token {
            Ok(true)
        } else {
            self.next();
            Ok(false)
        }
    }
    pub fn next(&mut self) -> Option<IndexedToken> {
        self.0.next()
    }
    pub fn peek(&mut self) -> Option<&IndexedToken> {
        self.0.reset_peek();
        self.0.peek()
    }
    pub fn more(&mut self) -> bool {
        if let Some(_) = self.peek() {
            true
        } else {
            false
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
