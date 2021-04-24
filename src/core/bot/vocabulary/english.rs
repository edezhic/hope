use crate::core::*;
use regex::Regex as R;
use super::Vocabulary;

impl Vocabulary {
    pub fn english() -> Result<Vocabulary> {
        Ok(Vocabulary {
            skip: R::new(r"^(a|(?i)(the|let|,|\p{Zs}|\t|\?|!))+$")?,
            be: R::new(r"^(?i)(:|=|is|are)$")?,
            term: R::new(r"^\p{Letter}+")?, // + {Number}?
            result: R::new(r"^(?i)(result|this|it|that)$")?,
            expression_start: R::new(r"^\($")?,
            expression_end: R::new(r"^\)$")?,
            binding: R::new(r"^(?i)(with|by)$")?,
            selection: R::new(r"^(?i)(of|from)$")?,
            targeting: R::new(r"^(?i)(to|in|at|into)$")?,
            
            list_end: R::new(r"^\]$")?,
            list_start: R::new(r"^\[$")?,
            struct_end: R::new(r"^\}$")?,
            struct_start: R::new(r"^\{$")?,

            flow_break: R::new(r"^(;|\.|\n|\p{Zl})$")?,
            flow_if: R::new(r"^(?i)if$")?,
            flow_then: R::new(r"^(?i)then$")?,
            flow_else: R::new(r"^(?i)else$")?,
            flow_for: R::new(r"^(?i)for$")?,
            flow_return: R::new(r"^(?i)return$")?,
            
            case_and: R::new(r"^(?i)and$")?,
            case_or: R::new(r"^(?i)or$")?,
            case_any: R::new(r"^(?i)any$")?,
            case_each: R::new(r"^(?i)each$")?,

            op_add: R::new(r"^(?i)(add|\+)$")?,
            op_divide: R::new(r"^(?i)(divide|/)$")?,
            op_multiply: R::new(r"^(?i)(multiply|\*)$")?,
            op_send: R::new(r"^(?i)send$")?,
            op_show: R::new(r"^(?i)show$")?,
            op_plot: R::new(r"^(?i)plot$")?,
            op_substract: R::new(r"^(?i)(substract|\-)$")?,
            op_sum: R::new(r"^(?i)sum$")?,
            op_expect: R::new(r"^(?i)(expect|when)$")?,
            op_filter: R::new(r"^(?i)filter$")?,
            op_collect: R::new(r"^(?i)collect$")?,
            op_read: R::new(r"^(?i)read$")?,
            op_write: R::new(r"^(?i)write$")?,
            op_request: R::new(r"^(?i)request$")?,
            op_sort: R::new(r"^(?i)sort$")?,
            op_mean: R::new(r"^(?i)mean$")?,
            op_deviation: R::new(r"^(?i)deviation$")?,
            op_sync: R::new(r"^(?i)sync$")?,
            op_sign: R::new(r"^(?i)sign$")?,
            op_verify: R::new(r"^(?i)verify$")?,
            op_predict: R::new(r"^(?i)predict$")?,
            
            v_fact_false: R::new(r"^(?i)(false|no)$")?,
            v_fact_true: R::new(r"^(?i)(true|yes|ok)$")?,
            v_id: R::new(r"^@$")?,
            v_number: R::new(r"^(\d+([\.,]\d+)?)$")?,
            v_seal: R::new(r"^\&$")?,
            v_text: R::new(r#"^("|')$"#)?,
            v_time: R::new(r"^~$")?,
            v_version: R::new(r"^#$")?,
        }) 
    }
}
