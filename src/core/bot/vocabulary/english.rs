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

            flow_break: R::new(r"^(\.|\n|\p{Zl})$")?,
            flow_if: R::new(r"^(?i)if$")?,
            flow_then: R::new(r"^(?i)then$")?,
            flow_else: R::new(r"^(?i)else$")?,
            flow_for: R::new(r"^(?i)for$")?,
            flow_return: R::new(r"^(?i)return$")?,
            
            case_and: R::new(r"^(?i)and$")?,
            case_or: R::new(r"^(?i)or$")?,
            case_any: R::new(r"^(?i)any$")?,
            case_each: R::new(r"^(?i)each$")?,
            case_has: R::new(r"^(?i)(has|have)$")?,

            cmd_add: R::new(r"^(?i)add$")?,
            cmd_substract: R::new(r"^(?i)substract$")?,
            cmd_divide: R::new(r"^(?i)divide$")?,
            cmd_multiply: R::new(r"^(?i)multiply$")?,
            cmd_send: R::new(r"^(?i)send$")?,
            cmd_show: R::new(r"^(?i)show$")?,
            cmd_plot: R::new(r"^(?i)plot$")?,
            cmd_sum: R::new(r"^(?i)sum$")?,
            cmd_filter: R::new(r"^(?i)filter$")?,
            cmd_collect: R::new(r"^(?i)collect$")?,
            cmd_read: R::new(r"^(?i)read$")?,
            cmd_write: R::new(r"^(?i)write$")?,
            cmd_request: R::new(r"^(?i)request$")?,
            cmd_sort: R::new(r"^(?i)sort$")?,
            cmd_mean: R::new(r"^(?i)mean$")?,
            cmd_deviation: R::new(r"^(?i)deviation$")?,
            cmd_sign: R::new(r"^(?i)sign$")?,
            cmd_check: R::new(r"^(?i)check$")?,
            cmd_predict: R::new(r"^(?i)predict$")?,

            op_plus: R::new(r"^\+$")?,
            op_minus: R::new(r"^\-$")?,
            op_multiplication: R::new(r"^\*$")?,
            op_division: R::new(r"^/$")?,
            
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
