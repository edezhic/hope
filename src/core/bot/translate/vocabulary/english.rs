use crate::core::*;
use regex::Regex as R;
use super::Vocabulary;

impl Vocabulary {
    pub fn english() -> Result<Vocabulary> {
        Ok(Vocabulary {
            ignore: R::new(r"^(?i)(the|let|,|\p{Zs}|\t)+$")?,
            term: R::new(r"^\p{Letter}+")?,
            result: R::new(r"^(?i)(result|this|it)$")?,
            comment_end: R::new(r"^`$")?,
            comment_start: R::new(r"^`$")?,
            list_end: R::new(r"^\]$")?,
            list_start: R::new(r"^\[$")?,
            struct_end: R::new(r"^\}$")?,
            struct_start: R::new(r"^\{$")?,
            
            case_and: R::new(r"^(?i)and$")?,
            case_identical: R::new(r"^(?i)(is|are)$")?,
            case_if: R::new(r"^(?i)if$")?,
            case_then: R::new(r"^(?i)(then)$")?,
            mod_binding: R::new(r"^(?i)(with)$")?,
            mod_break: R::new(r"^[;\.\n\p{Zl}]$")?,
            mod_selection: R::new(r"^(?i)(of|from)$")?,
            mod_targeting: R::new(r"^(?i)(to|for|in)$")?,
            cmd_show: R::new(r"^(?i)show$")?,
            op_add: R::new(r"^(?i)(add|\+)$")?,
            op_assign: R::new(r"^(?i)(define|set|assign|as|:|=)$")?,
            op_divide: R::new(r"^(?i)(divide|/)$")?,
            op_multiply: R::new(r"^(?i)(multiply|\*)$")?,
            op_send: R::new(r"^(?i)send$")?,
            op_substract: R::new(r"^(?i)(substract|\-)$")?,
            op_sum: R::new(r"^(?i)sum$")?,
            val_fact_false: R::new(r"^(?i)(false|no)$")?,
            val_fact_true: R::new(r"^(?i)(true|yes|ok)$")?,
            val_id: R::new(r"^@$")?,
            val_number: R::new(r"^(\d+([\.,]\d+)?)$")?,
            val_seal: R::new(r"^\&$")?,
            val_text: R::new(r#"^"$"#)?,
            val_time: R::new(r"^fixmepls$")?, // FIXME need time pattern
            val_version: R::new(r"^#$")?,
        }) 
    }
}
