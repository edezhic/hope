use super::Vocabulary;

pub enum Pattern {
    None,
    Skip,
    Comment,
    Id,
    List,
    Number,
    Seal,
    Struct,
    Text,
    Time,
    Version,
}

impl Pattern {
    pub fn check(vocab: &Vocabulary, piece: &str) -> Pattern {
        match piece {
            piece if vocab.whitespace.is_match(piece) || vocab.ignore.is_match(piece) => Pattern::Skip,
            piece if vocab.comment_start.is_match(piece) => Pattern::Comment,
            piece if vocab.list_start.is_match(piece) => Pattern::List,
            piece if vocab.struct_start.is_match(piece) => Pattern::Struct,
            piece if vocab.val_id.is_match(piece) => Pattern::Id,
            piece if vocab.val_number.is_match(piece) => Pattern::Number,
            piece if vocab.val_seal.is_match(piece) => Pattern::Seal,
            piece if vocab.val_text.is_match(piece) => Pattern::Text,
            piece if vocab.val_time.is_match(piece) => Pattern::Time,
            piece if vocab.val_version.is_match(piece) => Pattern::Version,
            _ => Pattern::None,
        }
    }
}