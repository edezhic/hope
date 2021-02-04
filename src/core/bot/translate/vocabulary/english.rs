use crate::core::*;
use regex::Regex as R;
use super::Vocabulary;

impl Vocabulary {
    pub fn english() -> Result<Vocabulary> {
        Ok(Vocabulary {
            whitespace: R::new(r"^(\p{Zs}|\t)+$")?,
            ignore: R::new(r"^(?i)(the|a|let|,)$")?,
            term: R::new(r"^\p{Letter}+")?,
            result: R::new(r"^(?i)(result|this|it)$")?,
            comment_end: R::new(r"^`$")?,
            comment_start: R::new(r"^`$")?,
            
            case_and: R::new(r"^(?i)and$")?,
            case_equal: R::new(r"^(?i)(=|is)$")?,
            case_if: R::new(r"^(?i)if$")?,
            case_then: R::new(r"^(?i)(then|[;\.\n\p{Zl}])$")?,
            cmd_send: R::new(r"^(?i)send$")?,
            cmd_set: R::new(r"^(?i)set$")?,
            cmd_show: R::new(r"^(?i)show$")?,
            cmd_sum: R::new(r"^(?i)sum$")?,
            mod_assign: R::new(r"^(?i)(as|:)$")?,
            mod_binding: R::new(r"^(?i)(with)$")?,
            mod_exp_end: R::new(r"^\)$")?,
            mod_exp_start: R::new(r"^\($")?,
            mod_list_end: R::new(r"^\]$")?,
            mod_list_start: R::new(r"^\[$")?,
            mod_selection: R::new(r"^(?i)(of|from)$")?,
            mod_struct_end: R::new(r"^\}$")?,
            mod_struct_start: R::new(r"^\{$")?,
            mod_targeting: R::new(r"^(?i)(to|for|in)$")?,
            op_divide: R::new(r"^/$")?,
            op_minus: R::new(r"^\-$")?,
            op_multiply: R::new(r"^\*$")?,
            op_plus: R::new(r"^\+$")?,
            val_fact_false: R::new(r"^(?i)(false|no)$")?,
            val_fact_true: R::new(r"^(?i)(true|yes|ok)$")?,
            val_id: R::new(r"^@$")?,
            val_number: R::new(r"^(\d+([\.,]\d+)?)$")?,
            val_seal: R::new(r"^\&$")?,
            val_text: R::new(r#"^"$"#)?,
            val_time: R::new(r"^\*$")?,
            val_version: R::new(r"^#$")?,
        }) 
    }
}
