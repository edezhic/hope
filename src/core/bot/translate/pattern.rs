use super::Vocabulary;
use crate::core::*;

pub enum Pattern {
    Command(Vec<Token>),
    Comment,
    Fact(Token),
    Id,
    Ignore,
    Keyword(Token),
    List,
    None,
    Number,
    Reference,
    Result(Token),
    Seal,
    Struct,
    Text,
    Time,
    Version,
}

impl Pattern {
    pub fn check(vocab: &Vocabulary, piece: &str) -> Pattern {
        match piece {
            piece if vocab.ignore.is_match(piece) => Pattern::Ignore,
            piece if vocab.comment_start.is_match(piece) => Pattern::Comment,
            piece if vocab.list_start.is_match(piece) => Pattern::List,
            piece if vocab.struct_start.is_match(piece) => Pattern::Struct,
            piece if vocab.val_id.is_match(piece) => Pattern::Id,
            piece if vocab.val_number.is_match(piece) => Pattern::Number,
            piece if vocab.val_seal.is_match(piece) => Pattern::Seal,
            piece if vocab.val_text.is_match(piece) => Pattern::Text,
            piece if vocab.val_time.is_match(piece) => Pattern::Time,
            piece if vocab.val_version.is_match(piece) => Pattern::Version,
            _ => {
                if let Some(fact) = Pattern::fact(vocab, piece) { 
                    Pattern::Fact(fact) 
                } else if let Some(case) = Pattern::case(vocab, piece) { 
                    Pattern::Keyword(case) 
                } else if let Some(op) = Pattern::op(vocab, piece) {
                    Pattern::Keyword(op)
                } else if let Some(modifier) = Pattern::modifier(vocab, piece) {
                    Pattern::Keyword(modifier)
                } else if let Some(command) = Pattern::command(vocab, piece) {
                    Pattern::Command(command)
                } else if vocab.result.is_match(piece) {
                    Pattern::Result(Token::Ref(Value::Id(Id::ref_result())))
                } else if vocab.term.is_match(piece) {
                    Pattern::Reference
                } else {
                    Pattern::None
                }
            }
        }
    }

    fn case(vocab: &Vocabulary, piece: &str) -> Option<Token> {
        match piece {
            piece if vocab.case_and.is_match(piece) => Some(Token::Case(Case::And)),
            piece if vocab.case_identical.is_match(piece) => Some(Token::Case(Case::Identical)),
            piece if vocab.case_if.is_match(piece) => Some(Token::Case(Case::If)),
            piece if vocab.case_then.is_match(piece) => Some(Token::Case(Case::Then)),
            _ => None,
        }
    }

    fn op(vocab: &Vocabulary, piece: &str) -> Option<Token> { 
        match piece {
            piece if vocab.op_add.is_match(piece) => Some(Token::Op(Op::Add)),
            piece if vocab.op_divide.is_match(piece) => Some(Token::Op(Op::Divide)),
            piece if vocab.op_multiply.is_match(piece) => Some(Token::Op(Op::Multiply)),
            piece if vocab.op_send.is_match(piece) => Some(Token::Op(Op::Send)),
            piece if vocab.op_define.is_match(piece) => Some(Token::Op(Op::Define)),
            piece if vocab.op_substract.is_match(piece) => Some(Token::Op(Op::Substract)),
            piece if vocab.op_sum.is_match(piece) => Some(Token::Op(Op::Sum)),
            _ => None,
        }
    }

    fn modifier(vocab: &Vocabulary, piece: &str) -> Option<Token> { 
        match piece {
            piece if vocab.mod_binding.is_match(piece) => Some(Token::Mod(Modifier::Binding)),
            piece if vocab.mod_selection.is_match(piece) => Some(Token::Mod(Modifier::Selection)),
            piece if vocab.mod_targeting.is_match(piece) => Some(Token::Mod(Modifier::Targeting)),
            _ => None,
        }
    }

    fn fact(vocab: &Vocabulary, piece: &str) -> Option<Token> {
        match piece {
            piece if vocab.val_fact_true.is_match(piece) => {
                Some(Token::Ref(Value::Fact(Fact::truth())))
            }
            piece if vocab.val_fact_false.is_match(piece) => {
                Some(Token::Ref(Value::Fact(Fact::falsehood())))
            }
            _ => None,
        }
    }

    fn command(vocab: &Vocabulary, piece: &str) -> Option<Vec<Token>> {
        match piece {
            piece if vocab.cmd_show.is_match(piece) => {
                Some(vec![
                    Token::Op(Op::Send),
                    Token::Mod(Modifier::Targeting),
                    Token::Ref(Value::Id(Id::from_str("display").unwrap()))
                ])
            }
            _ => None
        }
    }
}