use crate::core::*;
use regex::Regex as R;
use super::Vocabulary;

impl Vocabulary {
    pub fn english() -> Result<Vocabulary> {
        Ok(Vocabulary {
            term: R::new(r"^\p{Letter}+")?,
            result: R::new(r"^(?i)(result|this|it)$")?,
            skip: R::new(r"^(?i)(the|let|,|\p{Zs}|\t)+$")?,
            comment_end: R::new(r"^`$")?,
            comment_start: R::new(r"^`$")?,
            list_end: R::new(r"^\]$")?,
            list_start: R::new(r"^\[$")?,
            struct_end: R::new(r"^\}$")?,
            struct_start: R::new(r"^\{$")?,

            mod_binding: R::new(r"^(?i)(with)$")?,
            mod_break: R::new(r"^[;\.]$")?,
            mod_c_and: R::new(r"^(?i)and$")?,
            mod_c_identical: R::new(r"^(?i)(is|are)$")?,
            mod_c_if: R::new(r"^(?i)if$")?,
            mod_c_then: R::new(r"^(?i)(then)$")?,
            mod_gap: R::new(r"^[\n\p{Zl}]$")?,
            mod_s_any: R::new(r"^(?i)any$")?,
            mod_s_each: R::new(r"^(?i)each$")?,
            mod_s_of: R::new(r"^(?i)of$")?,
            mod_targeting: R::new(r"^(?i)(to|in)$")?,
            op_add: R::new(r"^(?i)(add|\+)$")?,
            op_assign: R::new(r"^(?i)(define|set|assign|as|:|=)$")?,
            op_divide: R::new(r"^(?i)(divide|/)$")?,
            op_multiply: R::new(r"^(?i)(multiply|\*)$")?,
            op_send: R::new(r"^(?i)send$")?,
            op_show: R::new(r"^(?i)show$")?,
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
