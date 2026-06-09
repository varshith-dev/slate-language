#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    StringLiteral(String),
    Number(f64),
    LeftAngle,      // <
    RightAngle,     // >
    LeftParen,      // (
    RightParen,     // )
    LeftBrace,      // {
    RightBrace,     // }
    LeftBracket,    // [
    RightBracket,   // ]
    Arrow,          // ->
    At,             // @
    Hash,           // #
    Exclamation,    // !
    Question,       // ?
    DoubleColon,    // ::
    DoubleDollar,   // $$
    Comma,          // ,
    Colon,          // :
    Eof,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub line: usize,
    pub col: usize,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    pub fn get_input_string(&self) -> String {
        self.input.iter().collect()
    }

    pub fn current_span(&self) -> Span {
        Span {
            line: self.line,
            col: self.col,
        }
    }

    pub fn peek_char(&self) -> Option<char> {
        if self.pos < self.input.len() {
            Some(self.input[self.pos])
        } else {
            None
        }
    }

    pub fn peek_char_n(&self, n: usize) -> Option<char> {
        if self.pos + n < self.input.len() {
            Some(self.input[self.pos + n])
        } else {
            None
        }
    }

    pub fn read_char(&mut self) -> Option<char> {
        if self.pos >= self.input.len() {
            return None;
        }
        let c = self.input[self.pos];
        self.pos += 1;
        if c == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(c)
    }

    pub fn skip_whitespace_and_comments(&mut self) {
        while let Some(c) = self.peek_char() {
            if c == ' ' || c == '\t' || c == '\r' || c == '\n' {
                self.read_char();
            } else if c == '/' && self.peek_char_n(1) == Some('/') {
                // Line comment, skip until newline
                self.read_char(); // '/'
                self.read_char(); // '/'
                while let Some(nc) = self.peek_char() {
                    self.read_char();
                    if nc == '\n' {
                        break;
                    }
                }
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace_and_comments();

        let c = match self.peek_char() {
            Some(ch) => ch,
            None => return Token::Eof,
        };

        // Check for double symbols first
        if c == '-' && self.peek_char_n(1) == Some('>') {
            self.read_char();
            self.read_char();
            return Token::Arrow;
        }
        if c == ':' && self.peek_char_n(1) == Some(':') {
            self.read_char();
            self.read_char();
            return Token::DoubleColon;
        }
        if c == '$' && self.peek_char_n(1) == Some('$') {
            self.read_char();
            self.read_char();
            return Token::DoubleDollar;
        }

        // Single symbols
        match c {
            '<' => { self.read_char(); Token::LeftAngle }
            '>' => { self.read_char(); Token::RightAngle }
            '(' => { self.read_char(); Token::LeftParen }
            ')' => { self.read_char(); Token::RightParen }
            '{' => { self.read_char(); Token::LeftBrace }
            '}' => { self.read_char(); Token::RightBrace }
            '[' => { self.read_char(); Token::LeftBracket }
            ']' => { self.read_char(); Token::RightBracket }
            '@' => { self.read_char(); Token::At }
            '#' => { self.read_char(); Token::Hash }
            '!' => { self.read_char(); Token::Exclamation }
            '?' => { self.read_char(); Token::Question }
            ',' => { self.read_char(); Token::Comma }
            ':' => { self.read_char(); Token::Colon }
            '"' | '\'' => {
                self.read_string_literal(c)
            }
            _ => {
                // Numbers: starts with digit, or +/- followed by a digit
                if self.is_number_start() {
                    self.read_number()
                } else if self.is_ident_start(c) {
                    self.read_identifier()
                } else {
                    // Unknown character, skip it to recover gracefully
                    self.read_char();
                    self.next_token()
                }
            }
        }
    }

    // Read raw text inside braces without tokenizing
    pub fn read_raw_text_until(&mut self, stop_chars: &[char]) -> String {
        let mut text = String::new();
        while let Some(c) = self.peek_char() {
            // Check if we hit any stop character
            if stop_chars.contains(&c) {
                // Special check for comment start in raw text
                if c == '/' && self.peek_char_n(1) == Some('/') {
                    break;
                }
                // Special check for double characters like -> or :: or $$
                if c == '-' && self.peek_char_n(1) == Some('>') && stop_chars.contains(&'-') {
                    break;
                }
                break;
            }
            text.push(c);
            self.read_char();
        }
        text
    }

    fn is_number_start(&self) -> bool {
        if let Some(c) = self.peek_char() {
            if c >= '0' && c <= '9' {
                return true;
            }
            if c == '+' || c == '-' {
                if let Some(nc) = self.peek_char_n(1) {
                    return nc >= '0' && nc <= '9';
                }
            }
        }
        false
    }

    fn read_number(&mut self) -> Token {
        let mut num_str = String::new();
        
        // Consume sign if present
        if let Some(c) = self.peek_char() {
            if c == '+' || c == '-' {
                num_str.push(c);
                self.read_char();
            }
        }

        let mut has_dot = false;
        while let Some(c) = self.peek_char() {
            if c >= '0' && c <= '9' {
                num_str.push(c);
                self.read_char();
            } else if c == '.' && !has_dot {
                // Check if dot is followed by a digit
                if let Some(nc) = self.peek_char_n(1) {
                    if nc >= '0' && nc <= '9' {
                        has_dot = true;
                        num_str.push(c);
                        self.read_char();
                        continue;
                    }
                }
                break;
            } else {
                break;
            }
        }

        // Parse to f64
        let val = num_str.parse::<f64>().unwrap_or(0.0);
        Token::Number(val)
    }

    fn is_ident_start(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || 
        (c >= 'A' && c <= 'Z') || 
        c == '_' || c == '.' || c == '/' || c == '#' || c == '-' || c == '$' || c == '+' || c == '%'
    }

    pub fn is_ident_char(&self, c: char) -> bool {
        self.is_ident_start(c) || (c >= '0' && c <= '9') || c == '@'
    }

    fn read_identifier(&mut self) -> Token {
        let mut ident = String::new();
        while let Some(c) = self.peek_char() {
            if self.is_ident_char(c) {
                ident.push(c);
                self.read_char();
            } else {
                break;
            }
        }
        Token::Identifier(ident)
    }

    fn read_string_literal(&mut self, quote: char) -> Token {
        self.read_char(); // consume opening quote
        let mut string = String::new();
        while let Some(c) = self.peek_char() {
            if c == quote {
                self.read_char(); // consume closing quote
                break;
            }
            if c == '\\' {
                self.read_char(); // consume '\\'
                if let Some(nc) = self.peek_char() {
                    string.push(nc);
                    self.read_char();
                }
            } else {
                string.push(c);
                self.read_char();
            }
        }
        Token::StringLiteral(string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokens() {
        let mut lexer = Lexer::new("page<Home>(align: center) {\n  title(\"Welcome to Slate\")\n}");
        assert_eq!(lexer.next_token(), Token::Identifier("page".to_string()));
        assert_eq!(lexer.next_token(), Token::LeftAngle);
        assert_eq!(lexer.next_token(), Token::Identifier("Home".to_string()));
        assert_eq!(lexer.next_token(), Token::RightAngle);
        assert_eq!(lexer.next_token(), Token::LeftParen);
        assert_eq!(lexer.next_token(), Token::Identifier("align".to_string()));
        assert_eq!(lexer.next_token(), Token::Colon);
        assert_eq!(lexer.next_token(), Token::Identifier("center".to_string()));
        assert_eq!(lexer.next_token(), Token::RightParen);
        assert_eq!(lexer.next_token(), Token::LeftBrace);
        assert_eq!(lexer.next_token(), Token::Identifier("title".to_string()));
        assert_eq!(lexer.next_token(), Token::LeftParen);
        assert_eq!(lexer.next_token(), Token::StringLiteral("Welcome to Slate".to_string()));
        assert_eq!(lexer.next_token(), Token::RightParen);
        assert_eq!(lexer.next_token(), Token::RightBrace);
        assert_eq!(lexer.next_token(), Token::Eof);
    }
}
