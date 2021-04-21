mod english;
use crate::core::Token::*;
use crate::core::*;
use regex::Regex as R;

pub struct Vocabulary {
    pub be: R,
    pub result: R,
    pub term: R,
    pub skip: R,
    pub selection: R,
    pub targeting: R,
    pub binding: R,
    pub expression_start: R,
    pub expression_end: R,
    pub list_end: R,
    pub list_start: R,
    pub struct_end: R,
    pub struct_start: R,

    pub v_fact_false: R,
    pub v_fact_true: R,
    pub v_id: R,
    pub v_number: R,
    pub v_seal: R,
    pub v_text: R,
    pub v_time: R,
    pub v_version: R,

    pub flow_break: R,
    pub flow_if: R,
    pub flow_then: R,
    pub flow_expect: R,

    pub case_and: R,
    pub case_any: R,
    pub case_each: R,

    pub op_add: R,
    pub op_divide: R,
    pub op_multiply: R,
    pub op_send: R,
    pub op_substract: R,
    pub op_show: R,
    pub op_sum: R,
}

impl Vocabulary {
    pub fn match_value(&self, piece: &str, pieces: &mut Pieces) -> Result<Option<Value>> {
        let value = match piece {
            piece if self.v_number.is_match(piece) => {
                let num = piece.replacen(",", ".", 1);
                pieces.next();
                Some(Value::Number(Number::from_string(num)?))
            }
            piece if self.v_text.is_match(piece) => { 
                Some(Value::Text(pieces.collect_until(&self.v_text, true))) 
            }
            piece if self.v_id.is_match(piece) => {
                Some(Value::Id(Id::from_text(pieces.collect_literal())?))
            }
            piece if self.v_seal.is_match(piece) => {
                Some(Value::Seal(Seal::from_text(pieces.collect_literal())?))
            }
            piece if self.v_time.is_match(piece) => {
                Some(Value::Time(Time::from_text(pieces.collect_literal())?))
            }
            piece if self.v_version.is_match(piece) => {
                Some(Value::Version(Version::from_text(pieces.collect_literal())?))
            }
            _ => None
        };
        Ok(value)
    }
    pub fn match_token(&self, piece: &str) -> Option<Token> {
        match piece {
            piece if self.v_fact_true.is_match(piece) => Some(V(Value::truth())),
            piece if self.v_fact_false.is_match(piece) => Some(V(Value::falsehood())),
            piece if self.be.is_match(piece) => Some(Being),
            piece if self.result.is_match(piece) => Some(V(Value::Id(Id::ref_result()))),
            piece if self.case_and.is_match(piece) => Some(C(Case::And)),
            piece if self.flow_if.is_match(piece) => Some(F(Flow::If)),
            piece if self.flow_expect.is_match(piece) => Some(F(Flow::Expect)),
            piece if self.flow_then.is_match(piece) => Some(F(Flow::Then)),
            piece if self.flow_break.is_match(piece) => Some(F(Flow::Break)),
            piece if self.expression_start.is_match(piece) => Some(F(Flow::ExpressionStart)),
            piece if self.expression_end.is_match(piece) => Some(F(Flow::ExpressionEnd)),
            piece if self.op_add.is_match(piece) => Some(O(Op::Add)),
            piece if self.op_divide.is_match(piece) => Some(O(Op::Divide)),
            piece if self.op_multiply.is_match(piece) => Some(O(Op::Multiply)),
            piece if self.op_send.is_match(piece) => Some(O(Op::Send)),
            piece if self.op_show.is_match(piece) => Some(O(Op::Show)),
            piece if self.op_substract.is_match(piece) => Some(O(Op::Substract)),
            piece if self.op_sum.is_match(piece) => Some(O(Op::Sum)),
            piece if self.struct_start.is_match(piece) => Some(S(Set::StructStart)),
            piece if self.struct_end.is_match(piece) => Some(S(Set::StructEnd)),
            piece if self.list_start.is_match(piece) => Some(S(Set::ListStart)),
            piece if self.list_end.is_match(piece) => Some(S(Set::ListEnd)),
            piece if self.binding.is_match(piece) => Some(M(Modifier::Binding)),
            piece if self.selection.is_match(piece) => Some(M(Modifier::Selection)),
            piece if self.targeting.is_match(piece) => Some(M(Modifier::Targeting)),
            _ => None,
        }
    }

    pub fn valid_term(&self, piece: &str) -> bool {
        self.term.is_match(piece)
    }
}
