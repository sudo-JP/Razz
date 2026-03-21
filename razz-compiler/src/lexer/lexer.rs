use crate::lexer::tokens::{Token, TokenKind};
use std::fmt;

pub struct LexError {
    pub kind: LexErrorKind,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug)]
pub enum LexErrorKind {
    InvalidChar(char),
    InvalidNumber,
    InvalidEndpoint(String),
    UnterminatedComment,
    UnterminatedString,
}

// For debugging and test
impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ERROR: {:?} Line: {} Col: {}", self.kind, self.line, self.col)
    }
}

pub struct Lexer {
    chars: Vec<u8>, 
    tokens: Vec<Token>, 
    lex_errors: Vec<LexError>,

    start: usize, 
    current: usize, 

    line: usize, 
    col: usize,
    curr_col: usize,
}

impl Lexer {
    pub fn new(contents: &str) -> Self {
        let chars = contents.as_bytes().to_vec();
        Self { 
            chars, 
            tokens: Vec::new(), 
            lex_errors: Vec::new(),

            start: 0, 
            current: 0, 
            line: 1, 
            col: 1,
            curr_col: 1
        }
    }

    pub fn lex(mut self) -> Result<Vec<Token>, Vec<LexError>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.curr_col = self.col;
            self.scan_token();
        }

        self.add_token(TokenKind::Eof);

        if self.lex_errors.len() > 0 { Err(self.lex_errors) }
        else { Ok(self.tokens) }
    }

    // Utilities functions 
    fn is_at_end(&self) -> bool { self.current >= self.chars.len() }

    // Append token to the token array with line and col  
    // Also ignore tokens when there's error, saving space
    fn add_token(&mut self, kind: TokenKind) {
        if self.lex_errors.len() == 0 {
            self.tokens.push(Token{kind, line: self.line, col: self.curr_col});
        }
    }

    fn add_err(&mut self, kind: LexErrorKind) {
        self.lex_errors.push(LexError{kind, line: self.line, col: self.curr_col});
    }

    // Advancing to the next char 
    fn advance(&mut self) -> u8 {
        let c = self.chars[self.current];
        self.current += 1; 
        self.col += 1;
        c
    }

    // Conditional advance
    fn expect(&mut self, expected: u8) -> bool {
        // Out of bound 
        if self.is_at_end() { return false; }

        // Compare
        let c = self.chars[self.current];
        if c != expected { return false; }

        self.current += 1; 
        self.col += 1; 
        return true; 
    }

    // Peek ahead without consuming 
    fn peek(&self) -> u8 {
        if self.is_at_end() { b'\0' }
        else { self.chars[self.current] }
    }

    fn peak_next(&self) -> u8 {
        if self.current + 1 >= self.chars.len() {
            b'\0'
        } else { self.chars[self.current + 1] }
    }

    // Handle string types
    fn string(&mut self) {
        // Keep going until " or end of file
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1; 
                self.col = 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.add_err(LexErrorKind::UnterminatedString);
            return; 
        }

        // close "
        self.advance();

        let str_slice = &self.chars[self.start + 1..self.current - 1];
        
        let value = String::from_utf8(str_slice.to_vec()).unwrap();
        self.add_token(TokenKind::StringLit(value));
    }

    // Handle number
    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // For fraction 
        if self.peek() == b'.' && self.peak_next().is_ascii_digit() {
            self.advance();
        } else if self.peek() == b'.' && !self.peak_next().is_ascii_digit() {
            self.advance();
            self.add_err(LexErrorKind::InvalidNumber);
            return;
        }
        else {
            // For Int 
            let str_slice = &self.chars[self.start..self.current];
            let value: i32 = String::from_utf8(str_slice.to_vec())
                .unwrap()
                .parse()
                .unwrap();
            self.add_token(TokenKind::IntLit(value));
            return;  
        }

        // Only fractions get here
        // Same loop to get all the number
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        let str_slice = &self.chars[self.start..self.current];
        let value: f64 = String::from_utf8(str_slice.to_vec())
            .unwrap()
            .parse()
            .unwrap(); 
        self.add_token(TokenKind::FloatLit(value));
    }

    fn match_endpoint(&self, s: &str) -> Option<TokenKind> {
        match s {
            "camera" => Some(TokenKind::EPCamera),
            "sphere" => Some(TokenKind::EPSphere),
            "background" => Some(TokenKind::EPBackground),
            "image" => Some(TokenKind::EPImage),
            "output" => Some(TokenKind::EPOutput),
            _ => None,
        }
    }

    fn handle_slash(&mut self) {
        // Single line comment 
        if self.expect(b'/') {
            while self.peek() != b'\n' && !self.is_at_end() {
                self.advance();
            }
        } 
        // Multiple lines comment 
        else if self.expect(b'*') {
            let start_line = self.line;
            let start_col = self.col;
            while (self.peek() != b'*' || self.peak_next() != b'/')
                && !self.is_at_end() {
                if self.peek() == b'\n' {
                    self.line += 1; 
                    self.col = 1;
                }
                self.advance();
            }
            if self.is_at_end() {
                self.lex_errors.push(LexError { 
                    kind: LexErrorKind::UnterminatedComment, 
                    line: start_line, col: start_col 
                });
                return;
            }

            // Consume */
            self.advance();
            self.advance();
        } 
        // Division equal
        else if self.expect(b'=') {
            self.add_token(TokenKind::DivE);
        }
        // Endpoints
        else if self.peek().is_ascii_alphabetic() {
            while self.peek().is_ascii_alphabetic() 
            && !self.is_at_end() {
                self.advance();
            }

            let str_slice = &self.chars[self.start + 1..self.current];

            let endpoint = String::from_utf8(str_slice.to_vec())
                .unwrap();

            match self.match_endpoint(&endpoint) {
                Some(t) => self.add_token(t),
                None => self.add_err(LexErrorKind::InvalidEndpoint(endpoint)),
            }
        } 
        // Div 
        else {
            self.add_token(TokenKind::Div);
        }
    }

    #[inline]
    fn is_valid_ident_char(&self, c: u8) -> bool {
        c.is_ascii_alphanumeric() || c == b'_'
    }

    fn identifier(&mut self) {
        while self.is_valid_ident_char(self.peek()) { 
            self.advance();
        }
        let str_slice = &self.chars[self.start..self.current];
        let ident = String::from_utf8(str_slice.to_vec())
            .unwrap();

        match ident.as_str() {
            // HTTP 
            "GET" => self.add_token(TokenKind::Get),
            "PUT" => self.add_token(TokenKind::Put),
            "POST" => self.add_token(TokenKind::Post),
            "PATCH" => self.add_token(TokenKind::Patch),
            
            // Function
            "fn" => self.add_token(TokenKind::Fn),
            "return" => self.add_token(TokenKind::Return),

            // Cond
            "if" => self.add_token(TokenKind::If),
            "else" => self.add_token(TokenKind::Else),

            // Loop
            "for" => self.add_token(TokenKind::For),
            "while" => self.add_token(TokenKind::While),

            // Types
            "int" => self.add_token(TokenKind::Int),
            "float" => self.add_token(TokenKind::Float),
            "true" => self.add_token(TokenKind::BoolLit(true)),
            "false" => self.add_token(TokenKind::BoolLit(false)),
            "string" => self.add_token(TokenKind::String),
            "null" => self.add_token(TokenKind::NullLit),
            "bool" => self.add_token(TokenKind::Bool),

            // Very specific types
            "Vec3" => self.add_token(TokenKind::Vec3),
            "Point3" => self.add_token(TokenKind::Point3),
            "Color" => self.add_token(TokenKind::Color),
            "Output" => self.add_token(TokenKind::Output),
            "Background" => self.add_token(TokenKind::Background),
            "Camera" => self.add_token(TokenKind::Camera),
            "Sphere" => self.add_token(TokenKind::Sphere),

            "PPM" => self.add_token(TokenKind::PPM),
            "Arduino" => self.add_token(TokenKind::Arduino),

            _ => self.add_token(TokenKind::Ident(ident)),
        }
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            // DELIMITERS
            b'(' => self.add_token(TokenKind::LParen),
            b')' => self.add_token(TokenKind::RParen),
            b'{' => self.add_token(TokenKind::LBrace),
            b'}' => self.add_token(TokenKind::RBrace),
            b';' => self.add_token(TokenKind::SemiCol),
            b':' => self.add_token(TokenKind::Colon),
            b',' => self.add_token(TokenKind::Comma),

            // ARITHMETIC, HANDLES + and +=, also other things
            b'+' => if self.expect(b'=') {
                self.add_token(TokenKind::AddE);
            } else { self.add_token(TokenKind::Add); }

            b'-' => if self.expect(b'=') {
                self.add_token(TokenKind::SubE);
            } else if self.expect(b'>') {
                self.add_token(TokenKind::Arrow);
            } else { self.add_token(TokenKind::Sub); }

            b'*' => if self.expect(b'=') {
                self.add_token(TokenKind::MultE);
            } else { self.add_token(TokenKind::Mult); }

            // Boolean stuff, except for assign 
            b'=' => if self.expect(b'=') {
                self.add_token(TokenKind::Eq);
            } else { self.add_token(TokenKind::Assign); }

            b'<' => if self.expect(b'=') {
                self.add_token(TokenKind::Le);
            } else { self.add_token(TokenKind::Lt); }

            b'>' => if self.expect(b'=') {
                self.add_token(TokenKind::Ge);
            } else { self.add_token(TokenKind::Gt); }

            b'!' => if self.expect(b'=') {
                self.add_token(TokenKind::Neq);
            } else { self.add_token(TokenKind::Not); }

            b'&' => if self.expect(b'&') {
                self.add_token(TokenKind::And);
            } else { self.add_err(LexErrorKind::InvalidChar(c as char)); }

            b'|' => if self.expect(b'|') {
                self.add_token(TokenKind::Or);
            } else { self.add_err(LexErrorKind::InvalidChar(c as char)); }

            b'"' => self.string(),

            b'/' => self.handle_slash(),

            // WHITESPACES 
            b'\n' => { self.col = 1; self.line += 1; }
            b' ' | b'\r' | b'\t' => {}
            _ => { 
                if c.is_ascii_digit() {
                    self.number();
                } else if self.is_valid_ident_char(c) {
                    self.identifier();
                } else { self.add_err(LexErrorKind::InvalidChar(c as char)); }
            }
        }
    }
}
