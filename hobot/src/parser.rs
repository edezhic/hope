use crate::{*, Preposition::*, Algebra::*, Function::*, Control::*, Selector::*, Relation::*};
use regex::Regex as R;
use std::iter::Peekable;
use unicode_segmentation::UWordBoundIndices;

pub struct Parser<'a> {
    iter: Peekable<UWordBoundIndices<'a>>,
    pub peek: Option<(usize, &'a str)>,
}
impl<'a> Parser<'a> {
    pub fn convert(s: &'a str) -> Result<Vec<(usize, Token)>> {
        let text = Text::from_str(s);
        let iter = text.split_by_word_bounds().peekable();
        let mut parser = Parser { iter, peek: None };
        parser.update_peek();
        let mut vec = vec![];
        while let Some((index, piece)) = parser.peek {
            if let Some(value) = parser.match_value(piece)? {
                vec.push((index, value));
            } else if let Some(keyword) = parser.match_keyword(piece) {
                vec.push((index, keyword));
            } else if let Some(reference) = parser.match_reference(piece) {
                vec.push((index, reference));
            } else {
                return Err(Parsing(format!(
                    r#"I don't know how to translate '{:?}'"#,
                    piece
                )));
            }
        }
        Ok(vec)
    }
    fn update_peek(&mut self) {
        self.skip();
        if let Some(piece) = self.iter.peek() {
            self.peek = Some(*piece)
        } else {
            self.peek = None
        }
    }
    pub fn next(&mut self) -> Option<(usize, &'a str)> {
        self.iter.next();
        self.update_peek();
        self.peek
    }
    pub fn collect_until(&mut self, pattern: &regex::Regex, skip_after: bool) -> Text {
        self.iter.next();
        let mut text = Text::empty();
        while let Some((_, piece)) = self.iter.peek() {
            if pattern.is_match(piece) || BREAK.is_match(piece) {
                break;
            } else {
                text.add(piece);
                self.iter.next();
            }
        }
        if skip_after {
            self.iter.next();
        }
        self.update_peek();
        text
    }
    pub fn collect_literal(&mut self) -> Text {
        self.collect_until(&SKIP, false)
    }
    pub fn skip(&mut self) {
        while let Some((_, piece)) = self.iter.peek() {
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
                Value::Yes
            }
            piece if FALSE.is_match(piece) => {
                self.next();
                Value::No
            }
            piece if NUMBER.is_match(piece) => {
                let num = piece.replacen(",", ".", 1);
                self.next();
                Value::Num(Number::from_string(num)?)
            }
            piece if TEXT.is_match(piece) => Value::Text(self.collect_until(&TEXT, true)),
            piece if ID.is_match(piece) => Value::I(Id::from_text(self.collect_literal())?),
            //piece if SEAL.is_match(piece) => Value::Seal(Seal::from_text(self.collect_literal())?),
            piece if TIME.is_match(piece) => Value::Dt(Datetime::from_text(self.collect_literal())?),
            piece if VERSION.is_match(piece) => Value::Ver(Version::from_text(self.collect_literal())?),
            _ => return Ok(None),
        };
        Ok(Some(V(value)))
    }
    pub fn match_keyword(&mut self, s: &str) -> Option<Token> {
        let token = match s {
            s if BE.is_match(s) => Be,
            s if THIS.is_match(s) => This,
            s if AND.is_match(s) => And,
            s if OR.is_match(s) => Or,

            s if WHERE.is_match(s) => S(Where),
            s if ANY.is_match(s) => S(Any),
            s if EACH.is_match(s) => S(Each),
            
            s if LESS.is_match(s) => R(Less),
            s if MORE.is_match(s) => R(More),
            s if THAN.is_match(s) => R(Than),
            s if CONTAINS.is_match(s) => R(Contains),
            
            s if STRUCT_START.is_match(s) => V(Value::new_struct()),
            s if STRUCT_END.is_match(s) => C(Closure),
            s if LIST_START.is_match(s) => V(Value::new_list()),
            s if LIST_END.is_match(s) => C(Closure),

            s if WITH.is_match(s) => P(With),
            s if BY.is_match(s) => P(By),
            s if OF.is_match(s) => P(Of),
            s if FROM.is_match(s) => P(From),
            s if TO.is_match(s) => P(To),
            s if IN.is_match(s) => P(In),
            s if AT.is_match(s) => P(At),
            s if AS.is_match(s) => P(As),
            s if FOR.is_match(s) => P(For),
            
            s if IF.is_match(s) => C(If),
            s if THEN.is_match(s) => C(Then),
            s if ELSE.is_match(s) => C(Else),
            s if BREAK.is_match(s) => C(Break),
            s if RETURN.is_match(s) => C(Return),
            s if MATCH.is_match(s) => C(Match),
            s if WHILE.is_match(s) => C(While),
            s if TRY.is_match(s) => C(Try),
            s if PANIC.is_match(s) => C(Panic),

            s if ADD.is_match(s) => F(Add),
            s if SEND.is_match(s) => F(Send),
            s if SHOW.is_match(s) => F(Show),
            s if SUBSTRACT.is_match(s) => F(Substract),
            s if SUM.is_match(s) => F(Sum),
            s if FILTER.is_match(s) => F(Filter),
            s if GET.is_match(s) => F(Get),
            s if SORT.is_match(s) => F(Sort),
            s if SIGN.is_match(s) => F(Sign),
            s if GROUP.is_match(s) => F(Group),
            s if SELECT.is_match(s) => F(Select),
            
            s if FORMULA_START.is_match(s) => A(Start),
            s if FORMULA_END.is_match(s) => A(End),
            s if PLUS.is_match(s) => A(Plus),
            s if MINUS.is_match(s) => A(Minus),
            s if MULTIPLICATION.is_match(s) => A(Multiplication),
            s if DIVISION.is_match(s) => A(Division),
            _ => return None,
        };
        self.next();
        Some(token)
    }

    pub fn match_reference(&mut self, piece: &str) -> Option<Token> {
        if REFERENCE.is_match(piece) {
            self.next();
            Some(V(Value::I(Id::reference(piece))))
        } else {
            None
        }
    }
}

lazy_static! {
    static ref SKIP: R = R::new(r"^(?i)(a|the|let|,|\t| |\?)+$").unwrap();
    static ref BE: R = R::new(r"^(?i)(:|=|is|are|equal)$").unwrap();
    static ref REFERENCE: R = R::new(r"^\p{Letter}+").unwrap();
    static ref THIS: R = R::new(r"^(?i)(result|this|it|that)$").unwrap(); // remove 'result' from it?
    static ref AND: R = R::new(r"^(?i)and$").unwrap();
    static ref OR: R = R::new(r"^(?i)or$").unwrap();

    static ref FORMULA_START: R = R::new(r"^\($").unwrap();
    static ref FORMULA_END: R = R::new(r"^\)$").unwrap();
    static ref LIST_START: R = R::new(r"^\[$").unwrap();
    static ref LIST_END: R = R::new(r"^\]$").unwrap();
    static ref STRUCT_START: R = R::new(r"^\{$").unwrap();
    static ref STRUCT_END: R = R::new(r"^\}$").unwrap();

    static ref WITH: R = R::new(r"^(?i)with$").unwrap();
    static ref BY: R = R::new(r"^(?i)by$").unwrap();
    static ref OF: R = R::new(r"^(?i)of$").unwrap();
    static ref FROM: R = R::new(r"^(?i)from$").unwrap();
    static ref TO: R = R::new(r"^(?i)to$").unwrap();
    static ref IN: R = R::new(r"^(?i)in$").unwrap();
    static ref AT: R = R::new(r"^(?i)at$").unwrap();
    static ref AS: R = R::new(r"^(?i)as$").unwrap();
    static ref FOR: R = R::new(r"^(?i)for$").unwrap();

    static ref BREAK: R = R::new(r"^(\.|\n|\p{Zl})$").unwrap();
    static ref IF: R = R::new(r"^(?i)if$").unwrap();
    static ref THEN: R = R::new(r"^(?i)then$").unwrap();
    static ref ELSE: R = R::new(r"^(?i)else$").unwrap();
    static ref RETURN: R = R::new(r"^(?i)return$").unwrap();
    static ref MATCH: R = R::new(r"^(?i)match$").unwrap();
    static ref WHERE: R = R::new(r"^(?i)where$").unwrap();
    static ref WHILE: R = R::new(r"^(?i)while$").unwrap();
    static ref TRY: R = R::new(r"^(?i)try$").unwrap();
    static ref PANIC: R = R::new(r"^(?i)panic$").unwrap();

    static ref ANY: R = R::new(r"^(?i)any$").unwrap();
    static ref EACH: R = R::new(r"^(?i)each$").unwrap();
    static ref LESS: R = R::new(r"^(?i)less$").unwrap();
    static ref MORE: R = R::new(r"^(?i)more$").unwrap();
    static ref THAN: R = R::new(r"^(?i)than$").unwrap();
    static ref CONTAINS: R = R::new(r"^(?i)contains$").unwrap();

    static ref ADD: R = R::new(r"^(?i)add$").unwrap();
    static ref SUBSTRACT: R = R::new(r"^(?i)substract$").unwrap();
    static ref SEND: R = R::new(r"^(?i)send$").unwrap();
    static ref SHOW: R = R::new(r"^(?i)show$").unwrap();
    static ref SUM: R = R::new(r"^(?i)sum$").unwrap();
    static ref FILTER: R = R::new(r"^(?i)filter$").unwrap();
    static ref GET: R = R::new(r"^(?i)get$").unwrap();
    static ref SORT: R = R::new(r"^(?i)sort$").unwrap();
    static ref SIGN: R = R::new(r"^(?i)sign$").unwrap();
    static ref CHECK: R = R::new(r"^(?i)check$").unwrap();
    static ref GROUP: R = R::new(r"^(?i)group$").unwrap();
    static ref SELECT: R = R::new(r"^(?i)select$").unwrap();

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
    static ref VERSION: R = R::new(r"^#$").unwrap(); // FIX change literal to "v{VERSION}"? keep "#" for smth else
}
