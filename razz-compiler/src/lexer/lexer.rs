use crate::lexer::tokens::{Token, TokenKind};

pub struct LexError {
    pub kind: LexErrorKind,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug)]
pub enum LexErrorKind {
    InvalidChar(char),
    UnterminatedString,
}

pub struct Lexer {
    chars: Vec<char>, 
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
        let chars = contents.chars().collect();
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

    pub fn scan_tokens(mut self) -> Result<Vec<Token>, Vec<LexError>> {
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
    fn add_token(&mut self, kind: TokenKind) {
        self.tokens.push(Token{kind, line: self.line, col: self.curr_col});
    }

    fn add_err(&mut self, kind: LexErrorKind) {
        self.lex_errors.push(LexError{kind, line: self.line, col: self.curr_col});
    }

    // Advancing to the next char 
    fn advance(&mut self) -> char {
        let c = self.chars[self.current];
        self.current += 1; 
        self.col += 1;
        c
    }

    // Conditional advance
    fn expect(&mut self, expected: char) -> bool {
        // Out of bound 
        if self.is_at_end() { return false; }

        // Compare
        let c = self.chars[self.current];
        if c != expected { return false; }

        self.current += 1; 
        self.col += 1; 
        return true; 
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenKind::LParen),
            ')' => self.add_token(TokenKind::RParen),
            '{' => self.add_token(TokenKind::LBrace),
            '}' => self.add_token(TokenKind::RBrace),
            ';' => self.add_token(TokenKind::SemiCol),
            ',' => self.add_token(TokenKind::Comma),

            // ARITHMETIC, HANDLES + and +=, also other things
            '+' => if self.expect('=') {
                self.add_token(TokenKind::AddE);
            } else { self.add_token(TokenKind::Add); }

            '-' => if self.expect('=') {
                self.add_token(TokenKind::SubE);
            } else if self.expect('>') {
                self.add_token(TokenKind::Arrow);
            } else { self.add_token(TokenKind::Sub); }

            '*' => if self.expect('=') {
                self.add_token(TokenKind::MultE);
            } else { self.add_token(TokenKind::Mult); }

            // Boolean stuff, except for assign 
            '=' => if self.expect('=') {
                self.add_token(TokenKind::Eq);
            } else { self.add_token(TokenKind::Assign); }

            '<' => if self.expect('=') {
                self.add_token(TokenKind::Le);
            } else { self.add_token(TokenKind::Lt); }

            '>' => if self.expect('=') {
                self.add_token(TokenKind::Ge);
            } else { self.add_token(TokenKind::Gt); }

            '!' => if self.expect('=') {
                self.add_token(TokenKind::Neq);
            } else { self.add_token(TokenKind::Not); }

            '&' => if self.expect('&') {
                self.add_token(TokenKind::And);
            } else { self.add_err(LexErrorKind::InvalidChar(c)); }

            '|' => if self.expect('|') {
                self.add_token(TokenKind::Or);
            } else { self.add_err(LexErrorKind::InvalidChar(c)); }

            // NOTE: Still missing / because its a bit complex with //, /**/, /endpoint and div 
            // TODO: Add Endpoints, string, numbers and Types 

            // WHITESPACES 
            ' ' | '\r' | '\t' => {}
            '\n' => { self.col = 1; self.line += 1; }
            _ => { 
                self.add_err(LexErrorKind::InvalidChar(c));
            }
        }
    }
}
