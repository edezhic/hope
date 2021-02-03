use crate::core::*;
use regex::Regex as R;

impl Vocabulary {
    pub fn english() -> Result<Vocabulary> {
        Ok(Vocabulary {
            assign: R::new(r"^(?i)(as|:)$")?,
            whitespace: R::new(r"^(\p{Zs}|\t)+$")?,
            new: R::new(r"^[;\.\n\p{Zl}]$")?,
            next: R::new(r"^,$")?,
            ignore: R::new(r"^(?i)(the|a|let)$")?,
            term: R::new(r"^\p{Letter}+")?,
            this: R::new(r"^(?i)(this|result|it)$")?,
            list_end: R::new(r"^\]$")?,
            list_start: R::new(r"^\[$")?,
            struct_end: R::new(r"^\}$")?,
            struct_start: R::new(r"^\{$")?,
            comment_end: R::new(r"^`$")?,
            comment_start: R::new(r"^`$")?,
            exp_start: R::new(r"^\($")?,
            exp_end: R::new(r"^\)$")?,
            
            case_and: R::new(r"^(?i)and$")?,
            case_equal: R::new(r"^(?i)(=|is)$")?,
            case_if: R::new(r"^(?i)if$")?,
            case_then: R::new(r"^(?i)(then|do)$")?,
            cmd_set: R::new(r"^(?i)set$")?,
            cmd_send: R::new(r"^(?i)send$")?,
            cmd_show: R::new(r"^(?i)show$")?,
            cmd_sum: R::new(r"^(?i)sum$")?,
            exp_divide: R::new(r"^/$")?,
            exp_minus: R::new(r"^\-$")?,
            exp_multiply: R::new(r"^\*$")?,
            exp_plus: R::new(r"^\+$")?,
            mod_binding: R::new(r"^(?i)(with)$")?,
            mod_selection: R::new(r"^(?i)(of|from)$")?,
            mod_targeting: R::new(r"^(?i)(to|for|in)$")?,
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
