mod english;
use crate::core::*;
use regex::Regex as R;

pub enum Pattern {
    None,
    Skip,
    Comment,
    Expression,
    Id,
    Number,
    Seal,
    Text,
    Time,
    Version,
}

pub struct Vocabulary {
    ignore: R,
    whitespace: R,
    result: R,
    term: R,
    comment_start: R,
    comment_end: R,

    val_fact_false: R,
    val_fact_true: R,
    val_id: R,
    val_number: R,
    val_seal: R,
    val_text: R,
    val_time: R,
    val_version: R,

    mod_assign: R,
    mod_binding: R,
    mod_exp_end: R,
    mod_exp_start: R,
    mod_list_end: R,
    mod_list_start: R,
    mod_selection: R,
    mod_struct_end: R,
    mod_struct_start: R,
    mod_targeting: R,

    case_and: R,
    case_equal: R,
    case_if: R,
    case_then: R,

    cmd_send: R,
    cmd_set: R,
    cmd_show: R,
    cmd_sum: R,

    op_divide: R,
    op_minus: R,
    op_multiply: R,
    op_plus: R,
}

impl Vocabulary {
    pub fn check_pattern(&self, piece: &str) -> Pattern {
        match piece {
            piece if self.skip(piece) => Pattern::Skip,
            piece if self.comment_start.is_match(piece) => Pattern::Comment,
            piece if self.mod_exp_start.is_match(piece) => Pattern::Expression,
            piece if self.val_id.is_match(piece) => Pattern::Id,
            piece if self.val_number.is_match(piece) => Pattern::Number,
            piece if self.val_seal.is_match(piece) => Pattern::Seal,
            piece if self.val_text.is_match(piece) => Pattern::Text,
            piece if self.val_time.is_match(piece) => Pattern::Time,
            piece if self.val_version.is_match(piece) => Pattern::Version,
            _ => Pattern::None,
        }
    }

    pub fn skip(&self, piece: &str) -> bool {
        match piece {
            piece if self.whitespace.is_match(piece) => true,
            piece if self.ignore.is_match(piece) => true,
            _ => false,
        }
    }

    pub fn whitespace(&self, piece: &str) -> bool {
        if self.whitespace.is_match(piece) {
            return true;
        }
        false
    }

    pub fn literal_end(&self, piece: &str) -> bool {
        match piece {
            piece if self.whitespace.is_match(piece) => true,
            _ => false,
        }
    }

    pub fn number(&self, piece: &str) -> bool {
        if self.val_number.is_match(piece) {
            true
        } else {
            false
        }
    }

    pub fn reserved(&self, piece: &str) -> Option<Token> {
        match piece {
            piece if self.result.is_match(piece) => Some(Token::Result),
            
            piece if self.case_and.is_match(piece) => Some(Token::Case(Case::And)),
            piece if self.case_equal.is_match(piece) => Some(Token::Case(Case::Equal)),
            piece if self.case_if.is_match(piece) => Some(Token::Case(Case::If)),
            piece if self.case_then.is_match(piece) => Some(Token::Case(Case::Then)),

            piece if self.cmd_send.is_match(piece) => Some(Token::Cmd(Command::Send)),
            piece if self.cmd_set.is_match(piece) => Some(Token::Cmd(Command::Set)),
            piece if self.cmd_show.is_match(piece) => Some(Token::Cmd(Command::Show)),
            piece if self.cmd_sum.is_match(piece) => Some(Token::Cmd(Command::Sum)),

            
            piece if self.mod_assign.is_match(piece) => Some(Token::Mod(Modifier::Assign)),
            piece if self.mod_binding.is_match(piece) => Some(Token::Mod(Modifier::Binding)),
            piece if self.mod_list_end.is_match(piece) => Some(Token::Mod(Modifier::ListEnd)),
            piece if self.mod_list_start.is_match(piece) => Some(Token::Mod(Modifier::ListStart)),
            piece if self.mod_selection.is_match(piece) => Some(Token::Mod(Modifier::Selection)),
            piece if self.mod_struct_end.is_match(piece) => Some(Token::Mod(Modifier::StructEnd)),
            piece if self.mod_struct_start.is_match(piece) => Some(Token::Mod(Modifier::StructStart)),
            piece if self.mod_targeting.is_match(piece) => Some(Token::Mod(Modifier::Targeting)),
            
            piece if self.val_fact_true.is_match(piece) => {
                Some(Token::Val(Value::Fact(Fact::truth())))
            }
            piece if self.val_fact_false.is_match(piece) => {
                Some(Token::Val(Value::Fact(Fact::falsehood())))
            }
            _ => None,
        }
    }

    pub fn term(&self, piece: &str) -> Option<Token> {
        if self.term.is_match(piece) {
            Some(Token::Term(Text::lowercase(piece)))
        } else {
            None
        }
    }

    pub fn expression_content(&self, piece: &str) -> Option<Token> {
        match piece {
            piece if self.op_divide.is_match(piece) => Some(Token::Op(Op::Divide)),
            piece if self.op_minus.is_match(piece) => Some(Token::Op(Op::Minus)),
            piece if self.op_multiply.is_match(piece) => Some(Token::Op(Op::Multiply)),
            piece if self.op_plus.is_match(piece) => Some(Token::Op(Op::Plus)),
            piece if self.term.is_match(piece) => Some(Token::Term(Text::lowercase(piece))),
            _ => None,
        }
    }

    pub fn text_end(&self, piece: &str) -> bool {
        if self.val_text.is_match(piece) {
            return true;
        }
        false
    }

    pub fn comment_end(&self, piece: &str) -> bool {
        if self.comment_end.is_match(piece) {
            return true;
        }
        false
    }

    pub fn expression_end(&self, piece: &str) -> bool {
        if self.mod_exp_end.is_match(piece) {
            return true;
        }
        false
    }
}
