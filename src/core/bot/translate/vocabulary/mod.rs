mod english;
use regex::Regex as R;

pub struct Vocabulary {
    pub result: R,
    pub term: R,
    pub skip: R,
    pub comment_start: R,
    pub comment_end: R,
    pub list_end: R,
    pub list_start: R,
    pub struct_end: R,
    pub struct_start: R,

    pub val_fact_false: R,
    pub val_fact_true: R,
    pub val_id: R,
    pub val_number: R,
    pub val_seal: R,
    pub val_text: R,
    pub val_time: R,
    pub val_version: R,

    pub mod_binding: R,
    pub mod_break: R,
    pub mod_c_and: R,
    pub mod_c_identical: R,
    pub mod_c_if: R,
    pub mod_c_then: R,
    pub mod_gap: R,
    pub mod_s_any:R,
    pub mod_s_each: R,
    pub mod_s_of: R,
    pub mod_targeting: R,

    pub op_add: R,
    pub op_divide: R,
    pub op_multiply: R,
    pub op_send: R,
    pub op_assign: R,
    pub op_substract: R,
    pub op_show: R,
    pub op_sum: R,
}
