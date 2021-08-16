use crate::*;
use crate::{Command::*, Flow::*, Op::*, Specifier::*, Token::*};
use regex::Regex as R;
use std::iter::Peekable;
use unicode_segmentation::UWordBounds;

pub struct Pieces<'a> {
    iter: Peekable<UWordBounds<'a>>,
    pub peek: Option<&'a str>,
}
impl<'a> Pieces<'a> {
    pub fn translate(s: &'a str) -> Result<Vec<Token>> {
        let text = Text::from_str(s);
        let mut iter = text.split_by_word_bounds().peekable();
        let mut pieces = Pieces { iter, peek: None };
        pieces.update_peek();
        let mut vec = vec![];
        while let Some(piece) = pieces.peek {
            if let Some(value) = pieces.match_value(piece)? {
                vec.push(value);
            } else if let Some(keyword) = pieces.match_keyword(piece) {
                vec.push(keyword);
            } else if let Some(name) = pieces.match_name(piece) {
                vec.push(name);
            } else {
                return Err(Error::ParsingError(format!(
                    r#"I don't know how to translate '{}'"#,
                    piece
                )));
            }
        }
        Ok(vec)
    }
    fn update_peek(&mut self) {
        self.skim();
        if let Some(piece) = self.iter.peek() {
            self.peek = Some(*piece)
        } else {
            self.peek = None
        }
    }
    pub fn next(&mut self) -> Option<&'a str> {
        self.iter.next();
        self.update_peek();
        self.peek
    }
    pub fn collect_until(&mut self, pattern: &regex::Regex, skip: bool) -> Text {
        self.iter.next();
        let mut text = Text::empty();
        while let Some(piece) = self.iter.next() {
            if pattern.is_match(piece) {
                break;
            } else {
                text.add(piece);
            }
        }
        if skip {
            self.iter.next();
        }
        self.update_peek();
        text
    }
    pub fn collect_literal(&mut self) -> Text {
        self.collect_until(&SKIP, false)
    }
    pub fn skim(&mut self) {
        while let Some(piece) = self.iter.peek() {
            if SKIP.is_match(piece) {
                self.iter.next();
            } else {
                break;
            }
        }
    }

    pub fn match_value(&mut self, piece: &str) -> Result<Option<Token>> {
        let value = match piece {
            piece if TRUE.is_match(piece) => {
                self.next();
                Value::truth()
            }
            piece if FALSE.is_match(piece) => {
                self.next();
                Value::falsehood()
            }
            piece if NUMBER.is_match(piece) => {
                let num = piece.replacen(",", ".", 1);
                self.next();
                Value::Number(Number::from_string(num)?)
            }
            piece if TEXT.is_match(piece) => Value::Text(self.collect_until(&TEXT, true)),
            piece if ID.is_match(piece) => Value::Id(Id::from_text(self.collect_literal())?),
            piece if SEAL.is_match(piece) => {
                Value::Seal(Seal::from_text(self.collect_literal())?)
            }
            piece if TIME.is_match(piece) => {
                Value::Time(Time::from_text(self.collect_literal())?)
            }
            piece if VERSION.is_match(piece) => {
                Value::Version(Version::from_text(self.collect_literal())?)
            }
            _ => return Ok(None),
        };
        Ok(Some(V(value)))
    }
    pub fn match_keyword(&mut self, piece: &str) -> Option<Token> {
        let token = match piece {
            piece if BE.is_match(piece) => Being,
            piece if RESULT.is_match(piece) => This,
            piece if AND.is_match(piece) => And,
            piece if OR.is_match(piece) => Or,

            piece if FORMULA_START.is_match(piece) => FormulaStart,
            piece if FORMULA_END.is_match(piece) => FormulaEnd,
            piece if STRUCT_START.is_match(piece) => StructStart,
            piece if STRUCT_END.is_match(piece) => StructEnd,
            piece if LIST_START.is_match(piece) => ListStart,
            piece if LIST_END.is_match(piece) => ListEnd,

            piece if BINDING.is_match(piece) => S(Binding),
            piece if SELECTION.is_match(piece) => S(Selection),
            piece if TARGETING.is_match(piece) => S(Targeting),

            piece if ANY.is_match(piece) => S(Any),
            piece if EACH.is_match(piece) => S(Each),

            piece if IF.is_match(piece) => F(If),
            piece if FOR.is_match(piece) => F(For),
            piece if THEN.is_match(piece) => F(Then),
            piece if ELSE.is_match(piece) => F(Else),
            piece if BREAK.is_match(piece) => F(Break),
            piece if RETURN.is_match(piece) => F(Return),

            piece if ADD.is_match(piece) => C(Add),
            piece if SEND.is_match(piece) => C(Send),
            piece if SHOW.is_match(piece) => C(Show),
            piece if SUBSTRACT.is_match(piece) => C(Substract),
            piece if SUM.is_match(piece) => C(Sum),
            piece if FILTER.is_match(piece) => C(Filter),
            piece if REQUEST.is_match(piece) => C(Request),
            piece if SORT.is_match(piece) => C(Sort),
            piece if SIGN.is_match(piece) => C(Sign),
            piece if PLUS.is_match(piece) => O(Plus),
            piece if MINUS.is_match(piece) => O(Minus),
            piece if MULTIPLICATION.is_match(piece) => O(Multiplication),
            piece if DIVISION.is_match(piece) => O(Division),
            _ => return None,
        };
        self.next();
        Some(token)
    }

    pub fn match_name(&mut self, piece: &str) -> Option<Token> {
        if TERM.is_match(piece) {
            let name = N(Text::lowercase(piece));
            self.next();
            return Some(name);
        }
        None
    }
}

lazy_static! {
    static ref SKIP: R = R::new(r"^(a|(?i)(the|let|,|\p{Zs}|\t|\?|!))+$").unwrap();
    static ref BE: R = R::new(r"^(?i)(:|=|is|are|equal)$").unwrap();
    static ref TERM: R = R::new(r"^\p{Letter}+").unwrap(); // + {Number}?
    static ref RESULT: R = R::new(r"^(?i)(result|this|it|that)$").unwrap();
    static ref AND: R = R::new(r"^(?i)and$").unwrap();
    static ref OR: R = R::new(r"^(?i)or$").unwrap();

    static ref FORMULA_START: R = R::new(r"^\($").unwrap();
    static ref FORMULA_END: R = R::new(r"^\)$").unwrap();
    static ref LIST_START: R = R::new(r"^\[$").unwrap();
    static ref LIST_END: R = R::new(r"^\]$").unwrap();
    static ref STRUCT_START: R = R::new(r"^\{$").unwrap();
    static ref STRUCT_END: R = R::new(r"^\}$").unwrap();

    static ref BINDING: R = R::new(r"^(?i)(with|by)$").unwrap();
    static ref SELECTION: R = R::new(r"^(?i)(of|from)$").unwrap();
    static ref TARGETING: R = R::new(r"^(?i)(to|in|at|into)$").unwrap();

    static ref BREAK: R = R::new(r"^(\.|\n|\p{Zl})$").unwrap();
    static ref IF: R = R::new(r"^(?i)if$").unwrap();
    static ref THEN: R = R::new(r"^(?i)then$").unwrap();
    static ref ELSE: R = R::new(r"^(?i)else$").unwrap();
    static ref FOR: R = R::new(r"^(?i)for$").unwrap();
    static ref RETURN: R = R::new(r"^(?i)return$").unwrap();

    static ref ANY: R = R::new(r"^(?i)any$").unwrap();
    static ref EACH: R = R::new(r"^(?i)each$").unwrap();

    static ref ADD: R = R::new(r"^(?i)add$").unwrap();
    static ref SUBSTRACT: R = R::new(r"^(?i)substract$").unwrap();
    static ref SEND: R = R::new(r"^(?i)send$").unwrap();
    static ref SHOW: R = R::new(r"^(?i)show$").unwrap();
    static ref SUM: R = R::new(r"^(?i)sum$").unwrap();
    static ref FILTER: R = R::new(r"^(?i)filter$").unwrap();
    static ref REQUEST: R = R::new(r"^(?i)request$").unwrap();
    static ref SORT: R = R::new(r"^(?i)sort$").unwrap();
    static ref SIGN: R = R::new(r"^(?i)sign$").unwrap();
    static ref CHECK: R = R::new(r"^(?i)check$").unwrap();

    static ref PLUS: R = R::new(r"^\+$").unwrap();
    static ref MINUS: R = R::new(r"^\-$").unwrap();
    static ref MULTIPLICATION: R = R::new(r"^\*$").unwrap();
    static ref DIVISION: R = R::new(r"^/$").unwrap();

    static ref FALSE: R = R::new(r"^(?i)(false|no)$").unwrap();
    static ref TRUE: R = R::new(r"^(?i)(true|yes|ok)$").unwrap();
    static ref ID: R = R::new(r"^@$").unwrap();
    static ref NUMBER: R = R::new(r"^(\d+([\.,]\d+)?)$").unwrap();
    static ref SEAL: R = R::new(r"^\&$").unwrap();
    static ref TEXT: R = R::new(r#"^("|')$"#).unwrap();
    static ref TIME: R = R::new(r"^~$").unwrap();
    static ref VERSION: R = R::new(r"^#$").unwrap();
}
