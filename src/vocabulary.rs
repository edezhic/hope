use crate::*;
use crate::{Token::*, Command::*, Op::*, Flow::*, Case::*, Modifier::*};
use regex::Regex as R;

pub fn match_value(piece: &str, pieces: &mut Pieces) -> Result<Option<Value>> {
    let value = match piece {
        piece if V_NUMBER.is_match(piece) => {
            let num = piece.replacen(",", ".", 1);
            pieces.next();
            Some(Value::Number(Number::from_string(num)?))
        }
        piece if V_TEXT.is_match(piece) => {
            Some(Value::Text(pieces.collect_until(&V_TEXT, true)))
        }
        piece if V_ID.is_match(piece) => {
            Some(Value::Id(Id::from_text(pieces.collect_literal())?))
        }
        piece if V_SEAL.is_match(piece) => {
            Some(Value::Seal(Seal::from_text(pieces.collect_literal())?))
        }
        piece if V_TIME.is_match(piece) => {
            Some(Value::Time(Time::from_text(pieces.collect_literal())?))
        }
        piece if V_VERSION.is_match(piece) => Some(Value::Version(Version::from_text(
            pieces.collect_literal(),
        )?)),
        _ => None,
    };
    Ok(value)
}
pub fn match_token(piece: &str) -> Option<Token> {
    match piece {
        piece if V_TRUE.is_match(piece) => Some(Val(Value::truth())),
        piece if V_FALSE.is_match(piece) => Some(Val(Value::falsehood())),
        piece if BE.is_match(piece) => Some(Being),
        piece if RESULT.is_match(piece) => Some(This),
        piece if AND.is_match(piece) => Some(And),
        piece if OR.is_match(piece) => Some(Or),
        

        piece if FORMULA_START.is_match(piece) => Some(FormulaStart),
        piece if FORMULA_END.is_match(piece) => Some(FormulaEnd),
        piece if STRUCT_START.is_match(piece) => Some(StructStart),
        piece if STRUCT_END.is_match(piece) => Some(StructEnd),
        piece if LIST_START.is_match(piece) => Some(ListStart),
        piece if LIST_END.is_match(piece) => Some(ListEnd),

        piece if MOD_BINDING.is_match(piece) => Some(Mod(Binding)),
        piece if MOD_SELECTION.is_match(piece) => Some(Mod(Selection)),
        piece if MOD_TARGETING.is_match(piece) => Some(Mod(Targeting)),

        piece if CASE_ANY.is_match(piece) => Some(C(Any)),
        piece if CASE_EACH.is_match(piece) => Some(C(Each)),
        piece if CASE_HAS.is_match(piece) => Some(C(Has)),

        piece if FLOW_IF.is_match(piece) => Some(F(If)),
        piece if FLOW_FOR.is_match(piece) => Some(F(For)),
        piece if FLOW_THEN.is_match(piece) => Some(F(Then)),
        piece if FLOW_ELSE.is_match(piece) => Some(F(Else)),
        piece if FLOW_BREAK.is_match(piece) => Some(F(Break)),
        piece if FLOW_RETURN.is_match(piece) => Some(F(Return)),
        piece if CMD_ADD.is_match(piece) => Some(Cmd(Add)),
        piece if CMD_DIVIDE.is_match(piece) => Some(Cmd(Divide)),
        piece if CMD_MULTIPLY.is_match(piece) => Some(Cmd(Multiply)),
        piece if CMD_SEND.is_match(piece) => Some(Cmd(Send)),
        piece if CMD_SHOW.is_match(piece) => Some(Cmd(Show)),
        piece if CMD_SUBSTRACT.is_match(piece) => Some(Cmd(Substract)),
        piece if CMD_SUM.is_match(piece) => Some(Cmd(Sum)),
        piece if CMD_PLOT.is_match(piece) => Some(Cmd(Plot)),
        piece if CMD_FILTER.is_match(piece) => Some(Cmd(Filter)),
        piece if CMD_REQUEST.is_match(piece) => Some(Cmd(Request)),
        piece if CMD_COLLECT.is_match(piece) => Some(Cmd(Collect)),
        piece if CMD_READ.is_match(piece) => Some(Cmd(Read)),
        piece if CMD_WRITE.is_match(piece) => Some(Cmd(Write)),
        piece if CMD_SORT.is_match(piece) => Some(Cmd(Sort)),
        piece if CMD_SIGN.is_match(piece) => Some(Cmd(Sign)),
        piece if CMD_CHECK.is_match(piece) => Some(Cmd(Check)),
        piece if CMD_PREDICT.is_match(piece) => Some(Cmd(Predict)),
        piece if OP_PLUS.is_match(piece) => Some(O(Plus)),
        piece if OP_MINUS.is_match(piece) => Some(O(Minus)),
        piece if OP_MULTIPLICATION.is_match(piece) => Some(O(Multiplication)),
        piece if OP_DIVISION.is_match(piece) => Some(O(Division)),
        _ => None,
    }
}

pub fn valid_term(piece: &str) -> bool {
    TERM.is_match(piece)
}

lazy_static! {
    pub static ref SKIP: R = R::new(r"^(a|(?i)(the|let|,|\p{Zs}|\t|\?|!))+$").unwrap();
    pub static ref BE: R = R::new(r"^(?i)(:|=|is|are)$").unwrap();
    pub static ref TERM: R = R::new(r"^\p{Letter}+").unwrap(); // + {Number}?
    pub static ref RESULT: R = R::new(r"^(?i)(result|this|it|that)$").unwrap();
    pub static ref AND: R = R::new(r"^(?i)and$").unwrap();
    pub static ref OR: R = R::new(r"^(?i)or$").unwrap();
    
    pub static ref FORMULA_START: R = R::new(r"^\($").unwrap();
    pub static ref FORMULA_END: R = R::new(r"^\)$").unwrap();
    pub static ref LIST_START: R = R::new(r"^\[$").unwrap();
    pub static ref LIST_END: R = R::new(r"^\]$").unwrap();
    pub static ref STRUCT_START: R = R::new(r"^\{$").unwrap();
    pub static ref STRUCT_END: R = R::new(r"^\}$").unwrap();

    pub static ref MOD_BINDING: R = R::new(r"^(?i)(with|by)$").unwrap();
    pub static ref MOD_SELECTION: R = R::new(r"^(?i)(of|from)$").unwrap();
    pub static ref MOD_TARGETING: R = R::new(r"^(?i)(to|in|at|into)$").unwrap();

    pub static ref FLOW_BREAK: R = R::new(r"^(\.|\n|\p{Zl})$").unwrap();
    pub static ref FLOW_IF: R = R::new(r"^(?i)if$").unwrap();
    pub static ref FLOW_THEN: R = R::new(r"^(?i)then$").unwrap();
    pub static ref FLOW_ELSE: R = R::new(r"^(?i)else$").unwrap();
    pub static ref FLOW_FOR: R = R::new(r"^(?i)for$").unwrap();
    pub static ref FLOW_RETURN: R = R::new(r"^(?i)return$").unwrap();

    pub static ref CASE_ANY: R = R::new(r"^(?i)any$").unwrap();
    pub static ref CASE_EACH: R = R::new(r"^(?i)each$").unwrap();
    pub static ref CASE_HAS: R = R::new(r"^(?i)(has|have)$").unwrap();

    pub static ref CMD_ADD: R = R::new(r"^(?i)add$").unwrap();
    pub static ref CMD_SUBSTRACT: R = R::new(r"^(?i)substract$").unwrap();
    pub static ref CMD_DIVIDE: R = R::new(r"^(?i)divide$").unwrap();
    pub static ref CMD_MULTIPLY: R = R::new(r"^(?i)multiply$").unwrap();
    pub static ref CMD_SEND: R = R::new(r"^(?i)send$").unwrap();
    pub static ref CMD_SHOW: R = R::new(r"^(?i)show$").unwrap();
    pub static ref CMD_PLOT: R = R::new(r"^(?i)plot$").unwrap();
    pub static ref CMD_SUM: R = R::new(r"^(?i)sum$").unwrap();
    pub static ref CMD_FILTER: R = R::new(r"^(?i)filter$").unwrap();
    pub static ref CMD_COLLECT: R = R::new(r"^(?i)collect$").unwrap();
    pub static ref CMD_READ: R = R::new(r"^(?i)read$").unwrap();
    pub static ref CMD_WRITE: R = R::new(r"^(?i)write$").unwrap();
    pub static ref CMD_REQUEST: R = R::new(r"^(?i)request$").unwrap();
    pub static ref CMD_SORT: R = R::new(r"^(?i)sort$").unwrap();
    pub static ref CMD_SIGN: R = R::new(r"^(?i)sign$").unwrap();
    pub static ref CMD_CHECK: R = R::new(r"^(?i)check$").unwrap();
    pub static ref CMD_PREDICT: R = R::new(r"^(?i)predict$").unwrap();

    pub static ref OP_PLUS: R = R::new(r"^\+$").unwrap();
    pub static ref OP_MINUS: R = R::new(r"^\-$").unwrap();
    pub static ref OP_MULTIPLICATION: R = R::new(r"^\*$").unwrap();
    pub static ref OP_DIVISION: R = R::new(r"^/$").unwrap();

    pub static ref V_FALSE: R = R::new(r"^(?i)(false|no)$").unwrap();
    pub static ref V_TRUE: R = R::new(r"^(?i)(true|yes|ok)$").unwrap();
    pub static ref V_ID: R = R::new(r"^@$").unwrap();
    pub static ref V_NUMBER: R = R::new(r"^(\d+([\.,]\d+)?)$").unwrap();
    pub static ref V_SEAL: R = R::new(r"^\&$").unwrap();
    pub static ref V_TEXT: R = R::new(r#"^("|')$"#).unwrap();
    pub static ref V_TIME: R = R::new(r"^~$").unwrap();
    pub static ref V_VERSION: R = R::new(r"^#$").unwrap();
}

