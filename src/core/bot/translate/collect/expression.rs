use std::iter::Peekable;
use unicode_segmentation::UWordBounds;

use crate::core::*;

impl Bot {
    pub fn collect_expression(
        &self,
        pieces: &mut Peekable<UWordBounds<'_>>,
        tokens: &mut Vec<Token>,
    ) -> Result<()> {
        tokens.push(Token::Exp(Expression::Start));
        while let Some(piece) = pieces.next() {
            if self.vocab.expression_end(piece) {
                tokens.push(Token::Exp(Expression::End));
                break;
            } else {
                if self.vocab.skip(piece) {
                    continue;
                } else if self.vocab.number(piece) {
                    self.collect_number(piece, tokens)?
                } else if let Some(t) = self.vocab.expression_content(piece) {
                    tokens.push(t);
                } else {
                    return Err(Error::ParsingError(format!(
                        r#"Unrecognized expression piece: '{}'"#,
                        piece
                    )));
                }
            }
        }
        Ok(())
    }
}