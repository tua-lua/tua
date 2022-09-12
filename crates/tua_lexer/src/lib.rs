//! Low-level Tua lexer.
//!
//! The idea with `tua_lexer` is to make a reusable library,
//! by separating out pure lexing and tua-specific concerns, like spans,
//! error reporting, and interning.  So, tua_lexer operates directly on `&str`,
//! produces simple tokens which are a pair of type-tag and a bit of original text,
//! and does not report errors, instead storing them as flags on the token.
//!
//! Tokens produced by this lexer are not yet ready for parsing the Tua syntax.
//! For that see [`tua_parser::lexer`], which converts this basic token stream
//! into wide tokens used by actual parser.
//!
//! The purpose of this crate is to convert raw sources into a labeled sequence
//! of well-known token types, so building an actual Tua token stream will
//! be easier.
//!
//! The main entity of this crate is the [`TokenKind`] enum which represents common
//! lexeme types.
//!
// We want to be able to build this crate with a stable compiler, so no
// `#![feature]` attributes should be added.

mod cursor;

#[cfg(test)]
mod tests;

use self::LiteralKind::*;
use self::TokenKind::*;
use crate::cursor::Cursor;
use crate::cursor::EOF_CHAR;

/// Parsed token.
/// It doesn't contain information about data that has been parsed,
/// only the type of the token and its size.
#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub len: u32,
}

impl Token {
    fn new(kind: TokenKind, len: u32) -> Token {
        Token { kind, len }
    }
}

/// Enum representing common lexeme types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    // Multi-char tokens:
    /// `-- short comment`
    ShortComment,
    /// `--[[ long comment ]]`
    /// `--[=[ long comment ]=]`
    LongComment { terminated: bool },
    /// Any whitespace characters sequence.
    Whitespace,
    /// Identifiers. At this step keywords are also considered identifiers.
    Ident,
    /// `"string"`, `3`, `314.16e-2`
    /// See `LiteralKind` for more details.
    Literal { kind: LiteralKind },

    // One-char tokens:
    /// ";"
    Semi,
    /// ","
    Comma,
    /// "."
    Dot,
    /// "("
    OpenParen,
    /// ")"
    CloseParen,
    /// "{"
    OpenBrace,
    /// "}"
    CloseBrace,
    /// "["
    OpenBracket,
    /// "]"
    CloseBracket,
    /// "#"
    Hash,
    /// "~"
    Tilde,
    /// ":"
    Colon,
    /// "="
    Eq,
    /// "<"
    Lt,
    /// ">"
    Gt,
    /// "-"
    Minus,
    /// "+"
    Plus,
    /// "*"
    Star,
    /// "/"
    Slash,
    /// "^"
    Caret,
    /// "%"
    Percent,

    /// Unknown token, not expected by the lexer.
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LiteralKind {
    /// `3`, `3.0`, `3.1416`, `314.16e-2`, `0.31416E1`, `0xff`, `0x56`
    Number {
        base: NumberBase,
        empty_number: bool,
        empty_exponent: bool,
    },
    /// `'abc'`, `"abc"`
    ShortString { quote: char, terminated: bool },
    /// `[[abc]]`, `[=[abc]=]`
    LongString { level: usize, terminated: bool },
}

/// Base of `Number` literal encoding according to its prefix.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NumberBase {
    /// Literal doesn't contain a prefix.
    Decimal,
    /// Literal starts with `0x` or `0X`.
    Hexadecimal,
}

/// Tua allows files to have a hashbang, e.g. "#!/usr/bin/env tua",
/// but hashbang isn't a part of Tua syntax.
pub fn strip_hashbang(input: &str) -> Option<usize> {
    // Hashbang must start with `#!` literally, without any preceding whitespace.
    // For simplicity we consider any line starting with `#!` a hashbang,
    // regardless of restrictions put on hashbangs by specific platforms.
    if let Some(input_tail) = input.strip_prefix("#!") {
        return Some(2 + input_tail.lines().next().unwrap_or_default().len());
    }
    None
}

/// Creates an iterator that produces tokens from the input string.
pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut cursor = Cursor::new(input);
    std::iter::from_fn(move || {
        if cursor.is_eof() {
            None
        } else {
            cursor.reset_len_consumed();
            Some(cursor.advance_token())
        }
    })
}

fn is_whitespace(c: char) -> bool {
    matches!(
        c,
        '\u{0009}'   // \t
        | '\u{000A}' // \n
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // \r
        | '\u{0020}' // space
    )
}

/// checks if `c` is valid as a non-first character of an identifier.
fn is_ident_continue(c: char) -> bool {
    !c.is_ascii_control()
        && !is_whitespace(c)
        && !matches!(
            c,
            '+' | '-'
                | '*'
                | '/'
                | '%'
                | '^'
                | '#'
                | '&'
                | '~'
                | '|'
                | '<'
                | '>'
                | '='
                | '('
                | ')'
                | '{'
                | '}'
                | '['
                | ']'
                | ';'
                | ':'
                | ','
                | '.'
                | '\\'
                | '\''
                | '"'
        )
}

/// checks if `c` is valid as a first character of an identifier.
fn is_ident_start(c: char) -> bool {
    !c.is_ascii_digit() && is_ident_continue(c)
}

impl Cursor<'_> {
    /// Parses a token from the input string.
    fn advance_token(&mut self) -> Token {
        let first_char = self.consume().unwrap();
        let token_kind = match first_char {
            // Whitespace sequence.
            c if is_whitespace(c) => self.whitespace(),

            // Minus or comment
            '-' => match self.peek() {
                '-' => self.comment(),
                _ => Minus,
            },

            // Numeric literal.
            c @ '0'..='9' => self.number(c),

            // String literal.
            '\'' | '"' => self.short_string(first_char),

            '[' => match self.peek() {
                '[' | '=' => self.long_string(),
                _ => OpenBracket,
            },

            // One-symbol tokens.
            ';' => Semi,
            ',' => Comma,
            '.' => Dot,
            '(' => OpenParen,
            ')' => CloseParen,
            '{' => OpenBrace,
            '}' => CloseBrace,
            ']' => CloseBracket,
            '#' => Hash,
            '~' => Tilde,
            ':' => Colon,
            '=' => Eq,
            '<' => Lt,
            '>' => Gt,
            '+' => Plus,
            '*' => Star,
            '/' => Slash,
            '^' => Caret,
            '%' => Percent,

            // Identifier.
            c if is_ident_start(c) => {
                self.consume_while(is_ident_continue);
                Ident
            }

            _ => Unknown,
        };
        Token::new(token_kind, self.len_consumed())
    }

    fn comment(&mut self) -> TokenKind {
        debug_assert!(self.prev() == '-' && self.peek() == '-');
        self.consume();

        match self.peek() {
            '[' => {
                self.consume();
                let open_level = self.count_and_consume_while(|c| c == '=');
                match self.peek() {
                    '[' => {
                        while let Some(c) = self.consume() {
                            match c {
                                ']' => {
                                    let close_level = self.count_and_consume_while(|c| c == '=');
                                    if open_level == close_level && self.peek() == ']' {
                                        self.consume();
                                        return LongComment { terminated: true };
                                    }
                                }
                                _ => (),
                            }
                        }
                        LongComment { terminated: false }
                    }
                    _ => {
                        self.consume_while(|c| c != '\n');
                        ShortComment
                    }
                }
            }
            _ => {
                self.consume_while(|c| c != '\n');
                ShortComment
            }
        }
    }

    fn consume_decimal_digits(&mut self) -> bool {
        let mut has_digits = false;
        loop {
            match self.peek() {
                '0'..='9' => {
                    has_digits = true;
                    self.consume();
                }
                _ => break,
            }
        }
        has_digits
    }

    fn consume_hexadecimal_digits(&mut self) -> bool {
        let mut has_digits = false;
        loop {
            match self.peek() {
                '0'..='9' | 'a'..='f' | 'A'..='F' => {
                    has_digits = true;
                    self.consume();
                }
                _ => break,
            }
        }
        has_digits
    }

    /// Consumes the number exponent. Returns `true` if at least one digit was met,
    /// and returns `false` otherwise.
    fn consume_number_exponent(&mut self) -> bool {
        debug_assert!(
            self.prev() == 'e' || self.prev() == 'E' || self.prev() == 'p' || self.prev() == 'P'
        );
        if self.peek() == '-' || self.peek() == '+' {
            self.consume();
        }
        self.consume_decimal_digits()
    }

    fn number(&mut self, first_digit: char) -> TokenKind {
        debug_assert!(self.prev() == first_digit && '0' <= self.prev() && self.prev() <= '9');
        let mut base = NumberBase::Decimal;
        if first_digit == '0' {
            // Attempt to parse encoding base.
            let has_digits = match self.peek() {
                'x' | 'X' => {
                    base = NumberBase::Hexadecimal;
                    self.consume();
                    self.consume_hexadecimal_digits()
                }
                // Not a base prefix.
                '0'..='9' | '.' | 'e' | 'E' => {
                    self.consume_decimal_digits();
                    true
                }
                // Just a `0`.
                _ => {
                    return Literal {
                        kind: Number {
                            base,
                            empty_exponent: true,
                            empty_number: false,
                        },
                    }
                }
            };
            // Base prefix was provided, but there were no digits
            // after it, e.g. `0x`.
            if !has_digits {
                return Literal {
                    kind: Number {
                        base,
                        empty_exponent: false,
                        empty_number: true,
                    },
                };
            }
        } else {
            // No base prefix, parse number in the usual way.
            self.consume_decimal_digits();
        };

        let empty_number = false;

        let kind = match self.peek() {
            '.' => {
                self.consume();
                let mut empty_exponent = false;
                match base {
                    NumberBase::Decimal => {
                        self.consume_decimal_digits();
                        match self.peek() {
                            'e' | 'E' => {
                                self.consume();
                                empty_exponent = !self.consume_number_exponent();
                            }
                            _ => (),
                        }
                    }
                    NumberBase::Hexadecimal => {
                        self.consume_hexadecimal_digits();
                        match self.peek() {
                            'p' | 'P' => {
                                self.consume();
                                empty_exponent = !self.consume_number_exponent();
                            }
                            _ => (),
                        }
                    }
                }
                Number {
                    base,
                    empty_exponent,
                    empty_number,
                }
            }
            'e' | 'E' if base == NumberBase::Decimal => {
                self.consume();
                let empty_exponent = !self.consume_number_exponent();
                Number {
                    base,
                    empty_exponent,
                    empty_number,
                }
            }
            'p' | 'P' if base == NumberBase::Hexadecimal => {
                self.consume();
                let empty_exponent = !self.consume_number_exponent();
                Number {
                    base,
                    empty_exponent,
                    empty_number,
                }
            }
            _ => Number {
                base,
                empty_exponent: false,
                empty_number,
            },
        };
        Literal { kind }
    }

    fn short_string(&mut self, quote: char) -> TokenKind {
        debug_assert!(self.prev() == quote);
        let terminated = loop {
            match self.peek() {
                c if c == quote => {
                    self.consume();
                    break true;
                }
                '\\' => {
                    self.consume();
                    // consume escaped character.
                    if self.consume() == Some('z') {
                        // consume whitespaces after `\z`.
                        self.consume_while(is_whitespace);
                    }
                }
                '\n' | EOF_CHAR => {
                    break false;
                }
                _ => {
                    self.consume();
                }
            }
        };
        Literal {
            kind: ShortString { quote, terminated },
        }
    }

    fn consume_long_string_content(&mut self, level: usize) -> bool {
        debug_assert!(self.prev() == '[');
        let mut terminated = false;
        while let Some(c) = self.consume() {
            match c {
                ']' => {
                    let close_level = self.count_and_consume_while(|c| c == '=');
                    if close_level == level && self.peek() == ']' {
                        self.consume();
                        terminated = true;
                        break;
                    }
                }
                _ => (),
            }
        }
        terminated
    }

    fn long_string(&mut self) -> TokenKind {
        debug_assert!(self.prev() == '[');
        let kind = match self.peek() {
            '[' => {
                self.consume();
                LongString {
                    level: 0,
                    terminated: self.consume_long_string_content(0),
                }
            }
            '=' => {
                let level = self.count_and_consume_while(|c| c == '=');
                if self.peek() == '[' {
                    self.consume();
                    LongString {
                        level,
                        terminated: self.consume_long_string_content(level),
                    }
                } else {
                    LongString {
                        level,
                        terminated: false,
                    }
                }
            }
            _ => unreachable!(),
        };
        Literal { kind }
    }

    fn whitespace(&mut self) -> TokenKind {
        debug_assert!(is_whitespace(self.prev()));
        self.consume_while(is_whitespace);
        Whitespace
    }
}
