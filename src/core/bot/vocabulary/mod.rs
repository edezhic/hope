mod english;
mod pieces;
use crate::core::Token::*;
use crate::core::*;
pub use pieces::Pieces;
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
    pub flow_else: R,
    pub flow_for: R,
    pub flow_return: R,

    pub case_and: R,
    pub case_or: R,
    pub case_any: R,
    pub case_each: R,
    pub case_has: R,

    pub op_add: R,
    pub op_divide: R,
    pub op_multiply: R,
    pub op_send: R,
    pub op_substract: R,
    pub op_show: R,
    pub op_plot: R,
    pub op_sum: R,
    pub op_expect: R,
    pub op_filter: R,
    pub op_collect: R,
    pub op_read: R,
    pub op_write: R,
    pub op_request: R,
    pub op_sort: R,
    pub op_mean: R,
    pub op_deviation: R,
    pub op_sync: R,
    pub op_sign: R,
    pub op_check: R,
    pub op_predict: R,
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
            piece if self.v_version.is_match(piece) => Some(Value::Version(Version::from_text(
                pieces.collect_literal(),
            )?)),
            _ => None,
        };
        Ok(value)
    }
    pub fn match_token(&self, piece: &str) -> Option<Token> {
        match piece {
            piece if self.v_fact_true.is_match(piece) => Some(V(Value::truth())),
            piece if self.v_fact_false.is_match(piece) => Some(V(Value::falsehood())),
            piece if self.be.is_match(piece) => Some(Being),
            piece if self.result.is_match(piece) => Some(This),

            piece if self.expression_start.is_match(piece) => Some(F(Flow::ExpressionStart)),
            piece if self.expression_end.is_match(piece) => Some(F(Flow::ExpressionEnd)),
            piece if self.struct_start.is_match(piece) => Some(S(Set::StructStart)),
            piece if self.struct_end.is_match(piece) => Some(S(Set::StructEnd)),
            piece if self.list_start.is_match(piece) => Some(S(Set::ListStart)),
            piece if self.list_end.is_match(piece) => Some(S(Set::ListEnd)),

            piece if self.binding.is_match(piece) => Some(M(Modifier::Binding)),
            piece if self.selection.is_match(piece) => Some(M(Modifier::Selection)),
            piece if self.targeting.is_match(piece) => Some(M(Modifier::Targeting)),

            piece if self.case_and.is_match(piece) => Some(C(Case::And)),
            piece if self.case_or.is_match(piece) => Some(C(Case::Or)),
            piece if self.case_any.is_match(piece) => Some(C(Case::Any)),
            piece if self.case_each.is_match(piece) => Some(C(Case::Each)),
            piece if self.case_has.is_match(piece) => Some(C(Case::Has)),

            piece if self.flow_if.is_match(piece) => Some(F(Flow::If)),
            piece if self.flow_then.is_match(piece) => Some(F(Flow::Then)),
            piece if self.flow_else.is_match(piece) => Some(F(Flow::Else)),
            piece if self.flow_break.is_match(piece) => Some(F(Flow::Break)),
            piece if self.flow_return.is_match(piece) => Some(F(Flow::Return)),
            piece if self.op_add.is_match(piece) => Some(O(Op::Add)),
            piece if self.op_divide.is_match(piece) => Some(O(Op::Divide)),
            piece if self.op_multiply.is_match(piece) => Some(O(Op::Multiply)),
            piece if self.op_send.is_match(piece) => Some(O(Op::Send)),
            piece if self.op_show.is_match(piece) => Some(O(Op::Show)),
            piece if self.op_substract.is_match(piece) => Some(O(Op::Substract)),
            piece if self.op_sum.is_match(piece) => Some(O(Op::Sum)),
            piece if self.op_expect.is_match(piece) => Some(O(Op::Expect)),
            piece if self.op_plot.is_match(piece) => Some(O(Op::Plot)),
            piece if self.op_filter.is_match(piece) => Some(O(Op::Filter)),
            piece if self.op_request.is_match(piece) => Some(O(Op::Request)),
            piece if self.op_collect.is_match(piece) => Some(O(Op::Collect)),
            piece if self.op_read.is_match(piece) => Some(O(Op::Read)),
            piece if self.op_write.is_match(piece) => Some(O(Op::Write)),
            piece if self.op_sort.is_match(piece) => Some(O(Op::Sort)),
            piece if self.op_mean.is_match(piece) => Some(O(Op::Mean)),
            piece if self.op_deviation.is_match(piece) => Some(O(Op::Deviation)),
            piece if self.op_sync.is_match(piece) => Some(O(Op::Sync)),
            piece if self.op_sign.is_match(piece) => Some(O(Op::Sign)),
            piece if self.op_check.is_match(piece) => Some(O(Op::Check)),
            piece if self.op_predict.is_match(piece) => Some(O(Op::Predict)),
            _ => None,
        }
    }

    pub fn valid_term(&self, piece: &str) -> bool {
        self.term.is_match(piece)
    }
}
