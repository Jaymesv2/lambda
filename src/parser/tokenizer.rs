use super::pos::*;
//use std::str::{CharIndices, Chars};
//use unicode_segmentation::{Graphemes, GraphemeIndices};
//use clap::Command;

/*const RESERVED_OPS: [&str; 8] = [
    ":",
    "::",
    "=",
    "\\",
    "|",
    "->",
    "@",
    "=>"
];*/

//pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;


fn is_varident_start(ch: char) -> bool {
    ch.is_ascii_lowercase() || ch == '_'
}

fn is_varident_cont(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}

fn is_operator_char(ch: char) -> bool {
    //"!#$%&*+-./<=>?@\\^|-~:".chars().any(|x| x == ch)
    "#$%&*+-./<=>?@\\^|-:".chars().any(|x| x == ch)
}

fn is_digit(ch: char) -> bool {
    ch.is_digit(10)
}

fn is_hex_digit(ch: char) -> bool {
    ch.is_digit(16)
}


/*enum Literal {
    Integer(i64),
    Float(f64),
}*/

#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
    VarIdent(&'a str),
    Operator(&'a str),

    Whitespace(&'a str),

    StringLiteral(String),
    CharLiteral(char),
    IntegerLiteral(i64),
    ByteLiteral(u8),
    FloatLiteral(f64),

    Comment,
    Newline,
    //Operator(String),
    Special,
    If,
    Then,
    Else,
    Let,
    In,
    Case,
    Of,

    Pipe, //'|'
    Comma, //','
    Semi, //';'
    Tilde, //'`'
    LBracket, //'[' 
    LBrace, //'{' 
    LParen, //'('
    RBracket, //']'
    RBrace, //'}'
    RParen, //')'
    Backslash, // '\\'

    // these are inserted by the tokenizer while it is parsing input
    VirtualRBrace,
    VirtualLBrace,

    Equals, 
    Arrow,
    BigArrow,


    Eof,
    Error(TokenizerError),
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenizerError {
    Unknown(String)
}

type TokenizerResult<'a> = Result<Spanned<Token<'a>, Location>, Spanned<TokenizerError, Location>>;

pub struct Tokenizer<'a> {
    source: &'a str,
    chars: CharLocations<'a>,
    lookahead: Option<(Location, char)>,
    /// The current location in the character stream
    location: Location,
}


impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut chars = CharLocations::new(source);
        Self {
            source,
            lookahead: chars.next(),
            chars,
            location: Location::new()
        }
    }


    /*
    fn next_char_boundary(&self, ch: Location) -> Option<usize> {
        let mut idx = ch.to_usize() + 1;

        while self.source.is_char_boundary(idx)  {
            idx+=1;
        };

        if idx > self.source.len() {
            None
        } else {
            Some(idx)
        }
    }

    // tries to get the character at start
    fn char_at(&self, start: Location) -> Option<char> {
        self.source.get(start.to_usize()..).and_then(|x| x.chars().next())
    }
    */


    fn slice(&self, start: Location, end: Location) -> &'a str {
        self.slice_checked(start, end).unwrap()
    }

    fn slice_checked(&self, start: Location, end: Location) -> Option<&'a str> {
        return self.source.get(start.to_usize()..end.to_usize())
    }

    fn test_lookahead<F: FnOnce(char) -> bool>(&self, f: F) -> bool {
        self.lookahead.map_or(false, |x| f(x.1))
    }
    
    fn take_while<F: FnMut(char) -> bool>(&mut self, start: Location, mut f: F) -> (Location, &'a str) {
        self.take_until(start, |x| !f(x))
    }

    fn take_until<F: FnMut(char) -> bool>(&mut self, start: Location, mut f: F) -> (Location, &'a str) {
        while let Some((end, ch)) = self.lookahead {
            if f(ch) {
                return (end, self.slice(start, end));
            } else {
                self.next_char();
            }
        }
        (self.location, self.slice(start, self.location))
    }

    /// returns the next character and updates the lookahead
    fn next_char(&mut self) -> Option<(Location, char)> {
        let Some(data@(location, _)) = self.lookahead else {
            return None;
        };
        self.location = location;
        self.lookahead = self.chars.next();
        return Some(data);
    }

    fn operator(&mut self, start: Location) -> TokenizerResult<'a> {
        let (end, st) = self.take_while(start, |ch| is_operator_char(ch));

        //spanned(start, end, )       
        let tok = match st {
            "=" => Token::Equals,
            "\\" => Token::Backslash,
            "->" => Token::Arrow,
            "=>" => Token::BigArrow,
            s => Token::Operator(s),
        };
        Ok(spanned(start, end, tok))
    }

    fn identifier(&mut self, start: Location) -> TokenizerResult<'a> {
        let (end, st) = self.take_while(start, |ch| is_varident_cont(ch));

        //spanned(start, end, )       
        let tok = match st {
            /*"type",
            "class",
            "data",
            "do",
            "import",
            "module",
            */

            // let expressions
            "let" => Token::Let,
            "in" => Token::In,
            // if expr
            "if" => Token::If,
            "else" => Token::Else,
            "then" => Token::Then,
            // case expr
            "case" => Token::Case,
            "of" => Token::Of,
            //"where",
            s => Token::VarIdent(s),
        };
        Ok(spanned(start, end, tok))
    }

    fn string_literal(&mut self, start: Location) -> TokenizerResult<'a> {
        let mut buf = String::new();
        let mut end = start.step('"');

        // figure out how to handle unicode
        // \u{D801}\u{DC00}
        while let Some((loc, ch)) = self.next_char() {
            end = loc;
            match ch {
                //'"' => return Ok(spanned(start, loc.step('"'), Token::StringLiteral(buf))),
                '"' => break,
                '\\' => {
                    println!("esc seq");
                    // test escape chars
                    let (loc, ch) = self.escape_character(start)?;
                    end = loc;
                    buf.push(ch);
                },
                ch => {
                    buf.push(ch);
                }
            }
        } 

        return Ok(spanned(start, end, Token::StringLiteral(buf.to_string())));
    }

    // assumes the previous character is '\'
    fn escape_character(&mut self, _start: Location) -> Result<(Location, char), Spanned<TokenizerError, Location>> {
        match self.next_char() {
            Some((loc, '\\')) => Ok((loc, '\\')),
            Some((loc, 'n')) => Ok((loc, '\n')),
            Some((_loc, 'u')) => {
                let Some((seq_start, '{')) = self.next_char() else {
                    return Err(spanned(_start, _loc, TokenizerError::Unknown("invalid unicode escape sequence".to_string())))
                };
                let seq_start = seq_start.step('{');
                // take characters until this
                let (_seq_end, seq) = self.take_while(seq_start, is_hex_digit);

                let Some((end, '}')) = self.next_char() else {
                    return Err(spanned(_start, _seq_end, TokenizerError::Unknown("invalid unicode escape sequence".to_string())))
                };
                if seq.len() == 4 {
                    let code = u32::from_str_radix(seq, 16).expect("unexpected non hex string");
                    let Some(s) = char::from_u32(code) else {
                        todo!()
                    };
                    Ok((end, s))
                } else {
                    todo!("handle non bmp unicode escape sequences")
                }
            },
            //Some(_) => Err(TokenizerError::UnexpectedEOF),
            Some((end, _)) => Err(spanned( _start, end, TokenizerError::Unknown("unknown escape sequence".to_string()))),
            _ => Err(spanned( _start, _start, TokenizerError::Unknown("unknown escape sequence".to_string()))),
        }
    }

    /// numeric literals can take lots of forms:
    /// integers: 123, -132
    /// floatings: .231, -32.0, 
    /// bin: 0b100110
    /// hex: 0x3132
    /// 
    /// for now it only parses integer literals
    fn numeric_literal(&mut self, start: Location) -> TokenizerResult<'a> { 
        let (end, st) = self.take_while(start, is_digit);

        let tok = match st.parse::<i64>() {
            Ok(i) => Token::IntegerLiteral(i),
            Err(e) => Token::Error(TokenizerError::Unknown(e.to_string())),
        };

        Ok(spanned(start, end, tok))
    }

    fn line_comment(&mut self, start: Location) -> Result<(), TokenizerError> { 
        let _ = self.next_char();
        let (_end, _st) = self.take_until(start, |ch| ch == '\n');
        Ok(())
    }

    



}

//pub type SpannedToken<'input> = Spanned<Token<'input>, Location>;


//pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

impl<'a> Iterator for Tokenizer<'a> {
    //type Item = Result<Spanned<Token<'a>, Location>, TokenizerError>;
    //type Item = Result<(Location, Token<'a>, Location), Spanned<TokenizerError, Location>>;
    type Item = Result<(BytePos, Token<'a>, BytePos), Spanned<TokenizerError, BytePos>>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((loc, ch)) = self.next_char() {
            let a = Some(match ch {
                '|' => Ok(spanned(loc, loc.step(ch), Token::Pipe)),
                ',' => Ok(spanned(loc, loc.step(ch), Token::Comma)),
                ';' => Ok(spanned(loc, loc.step(ch), Token::Semi)),
                '`' => Ok(spanned(loc, loc.step(ch), Token::Tilde)),

                ']' => Ok(spanned(loc, loc.step(ch), Token::RBracket)),
                '}' => Ok(spanned(loc, loc.step(ch), Token::RBrace)),
                ')' => Ok(spanned(loc, loc.step(ch), Token::RParen)),
                '[' => Ok(spanned(loc, loc.step(ch), Token::LBracket)),
                '{' => Ok(spanned(loc, loc.step(ch), Token::LBrace)),
                '(' => Ok(spanned(loc, loc.step(ch), Token::LParen)),
                '"' => self.string_literal(loc),

                //'\\' => Ok(spanned(loc, loc.step(ch), Token::Backslash)),
                '/' if self.test_lookahead(|ch| ch=='/') => {
                    let _ = self.line_comment(loc);
                    continue
                },
                ch if is_varident_start(ch) => self.identifier(loc),
                ch if is_digit(ch)  => {
                    println!("first is digit {loc}");
                    self.numeric_literal(loc)
                },
                ch if (ch == '-' && self.test_lookahead(is_digit)) => {
                    println!("minus digit");
                    self.numeric_literal(loc)
                },
                ch if is_operator_char(ch) => self.operator(loc),
                ch if ch.is_whitespace() => continue,
                //_ => Err(),//unimplemented!(),
                _ => Ok(spanned(loc, loc.step(ch), Token::Error(TokenizerError::Unknown(self.slice(loc, loc.step(ch)).to_string()))))
            });
            return a.map(|a| a.map(|x| {
                (BytePos::from(x.span.start), x.value, BytePos::from(x.span.end))
            }).map_err(|e| {
                spanned(BytePos::from(e.span.start), BytePos::from(e.span.end), e.value)
            }));
        }
        None
    }
}

// DOES THIS NOT VIOLATE ORPHAN RULES????
impl<T, Loc> From<Spanned<T,Loc>> for (Loc, T, Loc) {
    fn from(value: Spanned<T,Loc>) -> Self {
        unimplemented!()
    }
}
/*
impl<T,P> super::grammar::__TO_TRIPLE for Spanned<T,P> {

}
*/