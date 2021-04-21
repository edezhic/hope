use crate::core::*;
use regex::Regex as R;
use super::Vocabulary;

impl Vocabulary {
    pub fn english() -> Result<Vocabulary> {
        Ok(Vocabulary {
            be: R::new(r"^(?i)(define|set|assign|as|:|=|is|are)$")?, // Shall/Will?
            term: R::new(r"^\p{Letter}+")?,
            result: R::new(r"^(?i)(result|this|it)$")?,
            skip: R::new(r"^(?i)(a|the|let|,|\p{Zs}|\t|\?|!)+$")?,
            expression_start: R::new(r"^\($")?,
            expression_end: R::new(r"^\)$")?,
            binding: R::new(r"^(?i)(with|by)$")?,
            selection: R::new(r"^(?i)(of|from)$")?,
            targeting: R::new(r"^(?i)(to|in|at)$")?,
            
            list_end: R::new(r"^\]$")?,
            list_start: R::new(r"^\[$")?,
            struct_end: R::new(r"^\}$")?,
            struct_start: R::new(r"^\{$")?,

            flow_break: R::new(r"^(;|\.|\n|\p{Zl})$")?,
            flow_if: R::new(r"^(?i)if$")?,
            flow_then: R::new(r"^(?i)(then)$")?,
            flow_expect: R::new(r"^(?i)(expect|when)$")?,
            
            case_and: R::new(r"^(?i)and$")?,
            case_any: R::new(r"^(?i)any$")?,
            case_each: R::new(r"^(?i)each$")?,

            op_add: R::new(r"^(?i)(add|\+)$")?,
            op_divide: R::new(r"^(?i)(divide|/)$")?,
            op_multiply: R::new(r"^(?i)(multiply|\*)$")?,
            op_send: R::new(r"^(?i)send$")?,
            op_show: R::new(r"^(?i)show$")?,
            op_substract: R::new(r"^(?i)(substract|\-)$")?,
            op_sum: R::new(r"^(?i)sum$")?,
            
            v_fact_false: R::new(r"^(?i)(false|no)$")?,
            v_fact_true: R::new(r"^(?i)(true|yes|ok)$")?,
            v_id: R::new(r"^@$")?,
            v_number: R::new(r"^(\d+([\.,]\d+)?)$")?,
            v_seal: R::new(r"^\&$")?,
            v_text: R::new(r#"^"$"#)?,
            v_time: R::new(r"^~$")?,
            v_version: R::new(r"^#$")?,
        }) 
    }
}
