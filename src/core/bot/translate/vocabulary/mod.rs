mod english;
use regex::Regex as R;

pub struct Vocabulary {
    pub ignore: R,
    pub result: R,
    pub term: R,
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
    pub mod_selection: R,
    pub mod_targeting: R,

    pub case_and: R,
    pub case_identical: R,
    pub case_if: R,
    pub case_then: R,

    pub op_add: R,
    pub op_divide: R,
    pub op_multiply: R,
    pub op_send: R,
    pub op_define: R,
    pub op_substract: R,
    pub op_sum: R,

    pub cmd_show: R,
}
