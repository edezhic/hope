use std::iter::Peekable;
use unicode_segmentation::UWordBounds;

use crate::core::*;

impl Bot {
    pub fn collect_keyword(
        &self,
        pieces: &mut Peekable<UWordBounds<'_>>,
        tokens: &mut Vec<Token>,
    ) -> Result<bool> {
        let piece = *pieces.peek().unwrap();
        match piece {
            piece if self.vocab.case_and.is_match(piece) => tokens.push(Token::Case(Case::And)),
            piece if self.vocab.case_identical.is_match(piece) => tokens.push(Token::Case(Case::Identical)),
            piece if self.vocab.case_if.is_match(piece) => tokens.push(Token::Case(Case::If)),
            piece if self.vocab.case_then.is_match(piece) => tokens.push(Token::Case(Case::Then)),

            piece if self.vocab.op_add.is_match(piece) => tokens.push(Token::Op(Op::Add)),
            piece if self.vocab.op_divide.is_match(piece) => tokens.push(Token::Op(Op::Divide)),
            piece if self.vocab.op_multiply.is_match(piece) => tokens.push(Token::Op(Op::Multiply)),
            piece if self.vocab.op_send.is_match(piece) => tokens.push(Token::Op(Op::Send)),
            piece if self.vocab.op_define.is_match(piece) => tokens.push(Token::Op(Op::Define)),
            piece if self.vocab.op_substract.is_match(piece) => tokens.push(Token::Op(Op::Substract)),
            piece if self.vocab.op_sum.is_match(piece) => tokens.push(Token::Op(Op::Sum)),
            piece if self.vocab.cmd_show.is_match(piece) => {
                tokens.push(Token::Op(Op::Send));
                tokens.push(Token::Mod(Modifier::Targeting));
                tokens.push(Token::Ref(Value::Id(Id::from_str("display")?)));
            }
            
            piece if self.vocab.mod_binding.is_match(piece) => tokens.push(Token::Mod(Modifier::Binding)),
            piece if self.vocab.mod_selection.is_match(piece) => tokens.push(Token::Mod(Modifier::Selection)),
            piece if self.vocab.mod_targeting.is_match(piece) => tokens.push(Token::Mod(Modifier::Targeting)),

            piece if self.vocab.val_fact_true.is_match(piece) => {
                tokens.push(Token::Ref(Value::Fact(Fact::truth())))
            }
            piece if self.vocab.val_fact_false.is_match(piece) => {
                tokens.push(Token::Ref(Value::Fact(Fact::falsehood())))
            }
            _ => return Ok(false),
        }
        Ok(true)
    }
    
}
