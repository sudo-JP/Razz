use crate::lexer::tokens::{Token, TokenKind};

pub struct Lexer {
    chars: Vec<char>, 
    tokens: Vec<Token>, 

    start: usize, 
    current: usize, 
    line: usize, 
    col: usize,
}

impl Lexer {
    pub fn new(contents: &str) -> Self {
        let chars = contents.chars().collect();
        Self { 
            chars, 
            tokens: Vec::new(), 
            start: 0, 
            current: 0, 
            line: 1, 
            col: 1,
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.add_token(TokenKind::Eof);
        self.tokens 
    }

    // Utilities functions 
    fn is_at_end(&self) -> bool { self.current >= self.chars.len() }

    fn add_token(&mut self, kind: TokenKind) {
        self.tokens.push(Token::new(kind, self.line, self.col));
    }

    fn advance(&mut self) -> &char {
        let c = &self.chars[self.current];
        self.current += 1; 
        self.col += 1;
        c
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

            // WHITESPACES 
            ' ' | '\r' | '\t' => {}
            '\n' => { self.col = 1; self.line += 1; }
            _ => {}
        }
    }
}
