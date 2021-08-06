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

    pub cmd_add: R,
    pub cmd_divide: R,
    pub cmd_multiply: R,
    pub cmd_send: R,
    pub cmd_substract: R,
    pub cmd_show: R,
    pub cmd_plot: R,
    pub cmd_sum: R,
    pub cmd_filter: R,
    pub cmd_collect: R,
    pub cmd_read: R,
    pub cmd_write: R,
    pub cmd_request: R,
    pub cmd_sort: R,
    pub cmd_mean: R,
    pub cmd_deviation: R,
    pub cmd_sign: R,
    pub cmd_check: R,
    pub cmd_predict: R,

    pub op_plus: R,
    pub op_minus: R,
    pub op_multiplication: R,
    pub op_division: R,
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
            piece if self.v_fact_true.is_match(piece) => Some(Val(Value::truth())),
            piece if self.v_fact_false.is_match(piece) => Some(Val(Value::falsehood())),
            piece if self.be.is_match(piece) => Some(Being),
            piece if self.result.is_match(piece) => Some(This),

            piece if self.expression_start.is_match(piece) => Some(F(Flow::ExpressionStart)),
            piece if self.expression_end.is_match(piece) => Some(F(Flow::ExpressionEnd)),
            piece if self.struct_start.is_match(piece) => Some(S(Set::StructStart)),
            piece if self.struct_end.is_match(piece) => Some(S(Set::StructEnd)),
            piece if self.list_start.is_match(piece) => Some(S(Set::ListStart)),
            piece if self.list_end.is_match(piece) => Some(S(Set::ListEnd)),

            piece if self.binding.is_match(piece) => Some(Mod(Modifier::Binding)),
            piece if self.selection.is_match(piece) => Some(Mod(Modifier::Selection)),
            piece if self.targeting.is_match(piece) => Some(Mod(Modifier::Targeting)),

            piece if self.case_and.is_match(piece) => Some(C(Case::And)),
            piece if self.case_or.is_match(piece) => Some(C(Case::Or)),
            piece if self.case_any.is_match(piece) => Some(C(Case::Any)),
            piece if self.case_each.is_match(piece) => Some(C(Case::Each)),
            piece if self.case_has.is_match(piece) => Some(C(Case::Has)),

            piece if self.flow_if.is_match(piece) => Some(F(Flow::If)),
            piece if self.flow_for.is_match(piece) => Some(F(Flow::For)),
            piece if self.flow_then.is_match(piece) => Some(F(Flow::Then)),
            piece if self.flow_else.is_match(piece) => Some(F(Flow::Else)),
            piece if self.flow_break.is_match(piece) => Some(F(Flow::Break)),
            piece if self.flow_return.is_match(piece) => Some(F(Flow::Return)),
            piece if self.cmd_add.is_match(piece) => Some(Cmd(Command::Add)),
            piece if self.cmd_divide.is_match(piece) => Some(Cmd(Command::Divide)),
            piece if self.cmd_multiply.is_match(piece) => Some(Cmd(Command::Multiply)),
            piece if self.cmd_send.is_match(piece) => Some(Cmd(Command::Send)),
            piece if self.cmd_show.is_match(piece) => Some(Cmd(Command::Show)),
            piece if self.cmd_substract.is_match(piece) => Some(Cmd(Command::Substract)),
            piece if self.cmd_sum.is_match(piece) => Some(Cmd(Command::Sum)),
            piece if self.cmd_plot.is_match(piece) => Some(Cmd(Command::Plot)),
            piece if self.cmd_filter.is_match(piece) => Some(Cmd(Command::Filter)),
            piece if self.cmd_request.is_match(piece) => Some(Cmd(Command::Request)),
            piece if self.cmd_collect.is_match(piece) => Some(Cmd(Command::Collect)),
            piece if self.cmd_read.is_match(piece) => Some(Cmd(Command::Read)),
            piece if self.cmd_write.is_match(piece) => Some(Cmd(Command::Write)),
            piece if self.cmd_sort.is_match(piece) => Some(Cmd(Command::Sort)),
            piece if self.cmd_mean.is_match(piece) => Some(Cmd(Command::Mean)),
            piece if self.cmd_deviation.is_match(piece) => Some(Cmd(Command::Deviation)),
            piece if self.cmd_sign.is_match(piece) => Some(Cmd(Command::Sign)),
            piece if self.cmd_check.is_match(piece) => Some(Cmd(Command::Check)),
            piece if self.cmd_predict.is_match(piece) => Some(Cmd(Command::Predict)),
            piece if self.op_plus.is_match(piece) => Some(O(Op::Plus)),
            piece if self.op_minus.is_match(piece) => Some(O(Op::Minus)),
            piece if self.op_multiplication.is_match(piece) => Some(O(Op::Multiplication)),
            piece if self.op_division.is_match(piece) => Some(O(Op::Division)),
            _ => None,
        }
    }

    pub fn valid_term(&self, piece: &str) -> bool {
        self.term.is_match(piece)
    }
}
