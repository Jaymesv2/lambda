use super::tokenizer::{
    Tokenizer,
    TokenizerError,
    Token
};
use super::pos::*;

pub struct Layout<'a> {
    source: &'a str,
    ctx_stack: Vec<Context>,
    token_queue: Vec<Token<'a>>,
    tokenizer: Tokenizer<'a>,
}

impl<'a> Layout<'a> {
    pub fn new(source: &'a str, tokenizer: Tokenizer<'a>) -> Self {
        Self {
            source,
            ctx_stack: Vec::new(),
            token_queue: Vec::new(),
            tokenizer,
        }
    }
}



impl<'a> Iterator for Layout<'a> {
    type Item = Result<Spanned<Token<'a>, Location>, Spanned<TokenizerError, Location>>;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(token) = self.tokenizer.next() {
            match &token {
                Ok(v) => {
                    match v.value {
                        Token::Comment(_) => continue,
                        Token::Whitespace(_) => continue,
                        _ => {}
                    }
                },
                _ => {}
            }
            return Some(token);
        }
        return None;
    }
}

enum ContextType {
    Let(usize)
}

struct Context {

}


