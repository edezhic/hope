use crate::core::*;
use std::{iter::Peekable, vec::IntoIter};
use unicode_segmentation::UWordBounds;

impl Bot {
    pub fn read(&self, pieces: &mut Peekable<UWordBounds<'_>>) -> Result<Lexeme> {
        while let Some(piece) = pieces.peek() {
            if self.vocab.ignore.is_match(piece) {
                pieces.next();
            } else {
                break;
            }
        }
        let lexeme = match *pieces.peek().unwrap() {
            piece if self.vocab.comment_start.is_match(piece) => {
                Lexeme::Comment(self.collect_comment(pieces)?)
            }
            piece if self.vocab.list_start.is_match(piece) => {
                Lexeme::Value(Value::List(self.collect_list(pieces)?))
            }
            piece if self.vocab.struct_start.is_match(piece) => {
                Lexeme::Value(Value::Structure(self.collect_structure(pieces)?))
            }
            piece if self.vocab.val_id.is_match(piece) => {
                Lexeme::Value(Value::Id(self.collect_id(pieces)?))
            }
            piece if self.vocab.val_number.is_match(piece) => {
                Lexeme::Value(Value::Number(self.collect_number(pieces)?))
            }
            piece if self.vocab.val_seal.is_match(piece) => {
                Lexeme::Value(Value::Seal(self.collect_seal(pieces)?))
            }
            piece if self.vocab.val_text.is_match(piece) => {
                Lexeme::Value(Value::Text(self.collect_text(pieces)?))
            }
            piece if self.vocab.val_time.is_match(piece) => {
                Lexeme::Value(Value::Time(self.collect_time(pieces)?))
            }
            piece if self.vocab.val_version.is_match(piece) => {
                Lexeme::Value(Value::Version(self.collect_version(pieces)?))
            }
            piece => {
                if let Some(fact) = self.lookup_fact(pieces) {
                    Lexeme::Value(Value::Fact(fact))
                } else if let Some(keyword) = self.lookup_keyword(pieces) {
                    Lexeme::Keyword(keyword)
                } else if let Some(command) = self.lookup_command(pieces) {
                    Lexeme::Command(command)
                } else if self.vocab.result.is_match(piece) {
                    Lexeme::Reference(Value::Id(Id::ref_result()))
                } else if self.vocab.term.is_match(piece) {
                    Lexeme::Reference(Value::Id(self.collect_reference(pieces)?))
                } else {
                    return Err(Error::ParsingError(format!(
                        r#"I don't know how to translate '{}'"#,
                        piece
                    )));
                }
            }
        };
        Ok(lexeme)
    }

    fn lookup_keyword(&self, pieces: &mut Peekable<UWordBounds<'_>>) -> Option<Token> {
        let keyword = match *pieces.peek().unwrap() {
            piece if self.vocab.case_and.is_match(piece) => Some(Token::Case(Case::And)),
            piece if self.vocab.case_identical.is_match(piece) => {
                Some(Token::Case(Case::Identical))
            }
            piece if self.vocab.case_if.is_match(piece) => Some(Token::Case(Case::If)),
            piece if self.vocab.case_then.is_match(piece) => Some(Token::Case(Case::Then)),

            piece if self.vocab.op_add.is_match(piece) => Some(Token::Op(Op::Add)),
            piece if self.vocab.op_divide.is_match(piece) => Some(Token::Op(Op::Divide)),
            piece if self.vocab.op_multiply.is_match(piece) => Some(Token::Op(Op::Multiply)),
            piece if self.vocab.op_send.is_match(piece) => Some(Token::Op(Op::Send)),
            piece if self.vocab.op_assign.is_match(piece) => Some(Token::Op(Op::Assign)),
            piece if self.vocab.op_substract.is_match(piece) => Some(Token::Op(Op::Substract)),
            piece if self.vocab.op_sum.is_match(piece) => Some(Token::Op(Op::Sum)),

            piece if self.vocab.mod_binding.is_match(piece) => Some(Token::Mod(Modifier::Binding)),
            piece if self.vocab.mod_break.is_match(piece) => Some(Token::Mod(Modifier::Break)),
            piece if self.vocab.mod_selection.is_match(piece) => {
                Some(Token::Mod(Modifier::Selection))
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

    fn lookup_command(&self, pieces: &mut Peekable<UWordBounds<'_>>) -> Option<Vec<Token>> {
        let command = match *pieces.peek().unwrap() {
            piece if self.vocab.cmd_show.is_match(piece) => Some(vec![
                Token::Op(Op::Send),
                Token::Mod(Modifier::Targeting),
                Token::Ref(Value::Id(Id::from_str("display").unwrap())),
            ]),
            _ => None,
        };
        if let Some(_) = command {
            pieces.next();
        }
        command
    }
}
