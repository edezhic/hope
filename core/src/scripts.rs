use crate::{Text, Modifier, Token};

pub enum Script {
    Raw {
        head: Text,
        body: Text,
    },
    Tokenized {
        name: Text,
        args: Vec<(Option<Modifier>, Text)>,
        body: Vec<Token>,
    },
    // Compiled?
}

pub const TESTSCRIPT: &'static str = 
r#"If any X, then show "running with an argument"
List is [1.333, 2, 3,5], structure is {list, flag: yes}
Sort list of structure, sum it, show and send to @scheme://domain/path/
If any from the list is less than 1 or flag of the structure is yes, then show "Ok"
(2 + 2 * (2 + 2))
Script1 X1 of command1 of X2 of X3 with Script2 of X4 
User, account, key, auth, login, storage, etc"#;

pub const LISTSCRIPT: &'static str = 
r#""#;