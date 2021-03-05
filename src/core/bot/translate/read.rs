use crate::core::*;
use std::{iter::Peekable, vec::IntoIter};
use unicode_segmentation::UWordBounds;

impl Bot {
    pub fn read(&self, pieces: &mut Peekable<UWordBounds<'_>>) -> Result<Token> {
        while let Some(piece) = pieces.peek() {
            if self.vocab.skip.is_match(piece) {
                pieces.next();
            } else {
                break;
            }
        }
        let token = match *pieces.peek().unwrap() {
            piece if self.vocab.comment_start.is_match(piece) => {
                self.collect_comment(pieces)?;
                self.read(pieces)?
            }
            piece if self.vocab.list_start.is_match(piece) => {
                Token::Ref(Value::List(self.collect_list(pieces)?))
            }
            piece if self.vocab.struct_start.is_match(piece) => {
                Token::Ref(Value::Structure(self.collect_structure(pieces)?))
            }
            piece if self.vocab.val_id.is_match(piece) => {
                Token::Ref(Value::Id(self.collect_id(pieces)?))
            }
            piece if self.vocab.val_number.is_match(piece) => {
                Token::Ref(Value::Number(self.collect_number(pieces)?))
            }
            piece if self.vocab.val_seal.is_match(piece) => {
                Token::Ref(Value::Seal(self.collect_seal(pieces)?))
            }
            piece if self.vocab.val_text.is_match(piece) => {
                Token::Ref(Value::Text(self.collect_text(pieces)?))
            }
            piece if self.vocab.val_time.is_match(piece) => {
                Token::Ref(Value::Time(self.collect_time(pieces)?))
            }
            piece if self.vocab.val_version.is_match(piece) => {
                Token::Ref(Value::Version(self.collect_version(pieces)?))
            }
            piece => {
                if let Some(fact) = self.lookup_fact(pieces) {
                    Token::Ref(Value::Fact(fact))
                } else if let Some(token) = self.lookup_keyword(pieces) {
                    token
                } else if self.vocab.result.is_match(piece) {
                    Token::Ref(Value::Id(Id::ref_result()))
                } else if self.vocab.term.is_match(piece) {
                    Token::Ref(Value::Id(self.collect_reference(pieces)?))
                } else {
                    return Err(Error::ParsingError(format!(
                        r#"I don't know how to translate '{}'"#,
                        piece
                    )));
                }
            }
        };
        Ok(token)
    }

    fn lookup_keyword(&self, pieces: &mut Peekable<UWordBounds<'_>>) -> Option<Token> {
        let keyword = match *pieces.peek().unwrap() {
            piece if self.vocab.mod_c_and.is_match(piece) => Some(Token::Mod(Modifier::Case(Case::And))),
            piece if self.vocab.mod_c_identical.is_match(piece) =>
                Some(Token::Mod(Modifier::Case(Case::Identical))),
            piece if self.vocab.mod_c_if.is_match(piece) => Some(Token::Mod(Modifier::Flow(Flow::If))),
            piece if self.vocab.mod_c_then.is_match(piece) => Some(Token::Mod(Modifier::Flow(Flow::Then))),

            piece if self.vocab.op_add.is_match(piece) => Some(Token::Op(Op::Add)),
            piece if self.vocab.op_divide.is_match(piece) => Some(Token::Op(Op::Divide)),
            piece if self.vocab.op_multiply.is_match(piece) => Some(Token::Op(Op::Multiply)),
            piece if self.vocab.op_send.is_match(piece) => Some(Token::Op(Op::Send)),
            piece if self.vocab.op_assign.is_match(piece) => Some(Token::Op(Op::Assign)),
            piece if self.vocab.op_substract.is_match(piece) => Some(Token::Op(Op::Substract)),
            piece if self.vocab.op_sum.is_match(piece) => Some(Token::Op(Op::Sum)),

            piece if self.vocab.mod_binding.is_match(piece) => Some(Token::Mod(Modifier::Binding)),
            piece if self.vocab.mod_break.is_match(piece) => Some(Token::Mod(Modifier::Break)),
            piece if self.vocab.mod_gap.is_match(piece) => Some(Token::Mod(Modifier::Gap)),
            piece if self.vocab.mod_s_of.is_match(piece) => {
                Some(Token::Mod(Modifier::Selector(Selector::Of)))
            }
            piece if self.vocab.mod_targeting.is_match(piece) => {
                Some(Token::Mod(Modifier::Targeting))
            }
            _ => None,
        };
        if let Some(_) = keyword {
            pieces.next();
        }
        keyword
    }

    fn lookup_fact(&self, pieces: &mut Peekable<UWordBounds<'_>>) -> Option<Fact> {
        let fact = match *pieces.peek().unwrap() {
            piece if self.vocab.val_fact_true.is_match(piece) => Some(Fact::truth()),
            piece if self.vocab.val_fact_false.is_match(piece) => Some(Fact::falsehood()),
            _ => None,
        };
        if let Some(_) = fact {
            pieces.next();
        }
        fact
    }
}
