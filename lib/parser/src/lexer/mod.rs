use unicode_segmentation::UnicodeSegmentation;

use crate::lexer::source::Source;
use crate::lexer::span::Span;
pub use grapheme::*;

use crate::lexer::token::{Ident, Keyword, Literal, Separator, Token};

mod grapheme;
pub mod source;
pub mod span;
pub mod token;

fn is_java_whitespace(c: char) -> bool {
    c.is_whitespace()
        || c == ' '
        || c == '\t'
        || c == '\n'
        || c == '\r'
        || c == '\u{000C}'
        || c == '\u{000B}'
        || c == '\u{2007}'
        || c == '\u{202F}'
        || c == '\u{001C}'
        || c == '\u{001D}'
        || c == '\u{001E}'
        || c == '\u{001F}'
}

fn is_java_identifier_start(c: char) -> bool {
    c.is_alphabetic() || c == '_' || c == '$'
}

fn is_java_identifier_part(c: char) -> bool {
    is_java_identifier_start(c) || c.is_ascii_digit()
}

pub struct Lexer<'a> {
    source: Source<'a>,
}

impl<'a> From<&'a str> for Lexer<'a> {
    fn from(input: &'a str) -> Self {
        Self {
            source: Source::from(input),
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn tokens(&'a self) -> TokenIterator<'a> {
        TokenIterator::new(self)
    }

    pub fn source(&'a self) -> &'a Source<'a> {
        &self.source
    }

    #[inline]
    pub fn matches(&self, offset: GraphemeIndex, s: &str) -> bool {
        self.source.matches(offset, s)
    }

    /// Returns the unicode grapheme at the given index as a char.
    /// If the index is out of bounds, None is returned.
    #[inline]
    pub fn char_at(&self, index: GraphemeIndex) -> Option<char> {
        self.source.char_at(index)
    }

    pub fn count_consecutive_matches<F>(&self, offset: GraphemeIndex, f: F) -> usize
    where
        F: Fn(char) -> bool,
    {
        self.source
            .grapheme_indices()
            .iter()
            .skip(offset.into())
            .take_while(|(_, c)| f(*c))
            .count()
    }
}

pub struct TokenIterator<'a> {
    lexer: &'a Lexer<'a>,
    char_index: GraphemeIndex,
}

impl<'a> TokenIterator<'a> {
    fn new(lexer: &'a Lexer) -> Self {
        Self {
            lexer,
            char_index: 0.into(),
        }
    }

    fn advance_while<F>(&mut self, f: F)
    where
        F: Fn(char) -> bool,
    {
        self.char_index += self.lexer.count_consecutive_matches(self.char_index, f);
    }

    fn skip_whitespace(&mut self) {
        self.advance_while(is_java_whitespace);
    }

    fn next_keyword(&mut self) -> Option<Keyword> {
        for &keyword in token::KEYWORDS.iter() {
            if self.lexer.matches(self.char_index, keyword) {
                let start_index = self.char_index;
                self.char_index += UnicodeSegmentation::graphemes(keyword, true).count(); // technically this could be .len() since the keywords only consist of 1byte characters

                let span = Span::new(start_index, self.char_index);
                let keyword = Keyword::try_from_str(keyword, span).unwrap(); // never fails because we just matched it
                return Some(keyword);
            }
        }
        None
    }

    fn next_separator(&mut self) -> Option<Separator> {
        for &separator in token::SEPARATORS.iter() {
            if self.lexer.matches(self.char_index, separator) {
                let start_index = self.char_index;
                self.char_index += UnicodeSegmentation::graphemes(separator, true).count(); // technically this could be .len() since the keywords only consist of 1byte characters
                let span = Span::new(start_index, self.char_index);
                let separator = Separator::try_from_str(separator, span).unwrap(); // never fails because we just matched it
                return Some(separator);
            }
        }
        None
    }

    fn next_identifier(&mut self) -> Option<Ident> {
        let current_char = match self.lexer.char_at(self.char_index) {
            Some(c) => c,
            None => {
                // TODO: return a proper error
                panic!("unexpected end of input");
            }
        };
        if is_java_identifier_start(current_char) {
            let start_index = self.char_index;
            self.advance_while(is_java_identifier_part);
            let span = Span::new(start_index, self.char_index);
            let identifier = Ident::new(span);
            return Some(identifier);
        }
        None
    }

    fn next_literal(&mut self) -> Option<Literal> {
        // is it a string?
        if let Some(string_literal) = self.next_string_literal() {
            return Some(string_literal);
        }

        // is it a boolean?
        if let Some(boolean_literal) = self.next_boolean_literal() {
            return Some(boolean_literal);
        }

        None
    }

    fn next_boolean_literal(&mut self) -> Option<Literal> {
        for &boolean_value in token::BOOLEAN_VALUES.iter() {
            if self.lexer.matches(self.char_index, boolean_value) {
                let start_index = self.char_index;
                self.char_index += UnicodeSegmentation::graphemes(boolean_value, true).count(); // technically this could be .len() since the keywords only consist of 1byte characters
                let span = Span::new(start_index, self.char_index);
                let boolean = Literal::new_boolean(span);
                return Some(boolean);
            }
        }
        None
    }

    fn next_string_literal(&mut self) -> Option<Literal> {
        if self.lexer.char_at(self.char_index) == Some('"') {
            let start_index = self.char_index;
            self.char_index += 1;
            let mut end_index = self.char_index;
            let mut escaped = false;
            while self.char_index < self.lexer.source.grapheme_indices().len().into() {
                let c = self.lexer.char_at(self.char_index).unwrap();
                if escaped {
                    escaped = false;
                } else if c == '"' {
                    self.char_index += 1;
                    end_index = self.char_index;
                    break;
                } else if c == '\\' {
                    escaped = true;
                }
                self.char_index += 1;
            }
            let span = Span::new(start_index, end_index);
            let literal = Literal::new_string(span);
            return Some(literal);
        }
        None
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        // check for end of input
        if self.char_index >= self.lexer.source.grapheme_indices().len().into() {
            return None;
        }

        // check for keyword
        if let Some(keyword) = self.next_keyword() {
            return Some(Token::Keyword(keyword));
        }

        // check for separator
        if let Some(separator) = self.next_separator() {
            return Some(Token::Separator(separator));
        }

        // check for literal
        if let Some(literal) = self.next_literal() {
            return Some(Token::Literal(literal));
        }

        // literal needs to be checked before identifier, since a boolean literal like "true" would
        // otherwise also be a valid identifier

        // check for identifier
        if let Some(identifier) = self.next_identifier() {
            return Some(Token::Ident(identifier));
        }

        // no more tokens found or unknown token

        // TODO: handle unknown/invalid token

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::span::Span;
    use crate::lexer::token::Keyword::*;
    use crate::lexer::token::Separator::{
        Dot, LeftBracket, LeftCurly, LeftPar, RightBracket, RightCurly, RightPar, Semicolon,
    };
    use crate::lexer::token::{Ident, Literal, Token};
    use crate::lexer::{is_java_whitespace, Lexer};

    #[test]
    fn test_ident_between_other() {
        let input = "public abstract class Foo void transient";
        let lexer = Lexer::from(input);
        let expected = vec![
            Token::Keyword(Public(Span::new(0, 6))),
            Token::Keyword(Abstract(Span::new(7, 15))),
            Token::Keyword(Class(Span::new(16, 21))),
            Token::Ident(Ident::new(Span::new(22, 25))),
            Token::Keyword(Void(Span::new(26, 30))),
            Token::Keyword(Transient(Span::new(31, 40))),
        ];
        assert_eq!(lexer.tokens().collect::<Vec<Token>>(), expected);
    }

    #[test]
    fn test_whitespace_definition() {
        assert!(is_java_whitespace(' '));
        assert!(is_java_whitespace('\r'));
        assert!(is_java_whitespace('\n'));
        assert!(is_java_whitespace('\t'));
        assert!(is_java_whitespace('\u{000C}'));
        assert!(is_java_whitespace('\u{000B}'));
        assert!(is_java_whitespace('\u{2007}'));
        assert!(is_java_whitespace('\u{202F}'));
        assert!(is_java_whitespace('\u{001C}'));
        assert!(is_java_whitespace('\u{001D}'));
        assert!(is_java_whitespace('\u{001E}'));
        assert!(is_java_whitespace('\u{001F}'));
        assert!(!is_java_whitespace('a'));
        assert!(!is_java_whitespace('0'));
    }

    #[test]
    fn test_keywords() {
        let input = r#"
abstract assert boolean break byte case catch char
class const continue default do double else enum
extends final finally float for goto if implements
import instanceof int interface long native new package
private protected public return short static strictfp super
switch synchronized this throw throws transient try void
volatile while
"#;
        let lexer = Lexer::from(input);
        let expected = vec![
            Token::Keyword(Abstract(Span::new(1, 9))),
            Token::Keyword(Assert(Span::new(10, 16))),
            Token::Keyword(Boolean(Span::new(17, 24))),
            Token::Keyword(Break(Span::new(25, 30))),
            Token::Keyword(Byte(Span::new(31, 35))),
            Token::Keyword(Case(Span::new(36, 40))),
            Token::Keyword(Catch(Span::new(41, 46))),
            Token::Keyword(Char(Span::new(47, 51))),
            Token::Keyword(Class(Span::new(52, 57))),
            Token::Keyword(Const(Span::new(58, 63))),
            Token::Keyword(Continue(Span::new(64, 72))),
            Token::Keyword(Default(Span::new(73, 80))),
            Token::Keyword(Do(Span::new(81, 83))),
            Token::Keyword(Double(Span::new(84, 90))),
            Token::Keyword(Else(Span::new(91, 95))),
            Token::Keyword(Enum(Span::new(96, 100))),
            Token::Keyword(Extends(Span::new(101, 108))),
            Token::Keyword(Final(Span::new(109, 114))),
            Token::Keyword(Finally(Span::new(115, 122))),
            Token::Keyword(Float(Span::new(123, 128))),
            Token::Keyword(For(Span::new(129, 132))),
            Token::Keyword(Goto(Span::new(133, 137))),
            Token::Keyword(If(Span::new(138, 140))),
            Token::Keyword(Implements(Span::new(141, 151))),
            Token::Keyword(Import(Span::new(152, 158))),
            Token::Keyword(InstanceOf(Span::new(159, 169))),
            Token::Keyword(Int(Span::new(170, 173))),
            Token::Keyword(Interface(Span::new(174, 183))),
            Token::Keyword(Long(Span::new(184, 188))),
            Token::Keyword(Native(Span::new(189, 195))),
            Token::Keyword(New(Span::new(196, 199))),
            Token::Keyword(Package(Span::new(200, 207))),
            Token::Keyword(Private(Span::new(208, 215))),
            Token::Keyword(Protected(Span::new(216, 225))),
            Token::Keyword(Public(Span::new(226, 232))),
            Token::Keyword(Return(Span::new(233, 239))),
            Token::Keyword(Short(Span::new(240, 245))),
            Token::Keyword(Static(Span::new(246, 252))),
            Token::Keyword(Strictfp(Span::new(253, 261))),
            Token::Keyword(Super(Span::new(262, 267))),
            Token::Keyword(Switch(Span::new(268, 274))),
            Token::Keyword(Synchronized(Span::new(275, 287))),
            Token::Keyword(This(Span::new(288, 292))),
            Token::Keyword(Throw(Span::new(293, 298))),
            Token::Keyword(Throws(Span::new(299, 305))),
            Token::Keyword(Transient(Span::new(306, 315))),
            Token::Keyword(Try(Span::new(316, 319))),
            Token::Keyword(Void(Span::new(320, 324))),
            Token::Keyword(Volatile(Span::new(325, 333))),
            Token::Keyword(While(Span::new(334, 339))),
        ];
        assert_eq!(lexer.tokens().collect::<Vec<Token>>(), expected);
    }

    #[test]
    fn test_boolean_literals() {
        let input = "true false \"true\" false true";
        let lexer = Lexer::from(input);
        let expected = vec![
            Token::Literal(Literal::new_boolean(Span::new(0, 4))),
            Token::Literal(Literal::new_boolean(Span::new(5, 10))),
            Token::Literal(Literal::new_string(Span::new(11, 17))),
            Token::Literal(Literal::new_boolean(Span::new(18, 23))),
            Token::Literal(Literal::new_boolean(Span::new(24, 28))),
        ];
        assert_eq!(lexer.tokens().collect::<Vec<Token>>(), expected);
    }

    #[test]
    fn test_tokens_simple() {
        let input = r#"
public static void main(String[] args) {
    System.out.println("Hello, World");
}
        "#;
        let lexer = Lexer::from(input);
        let expected = vec![
            Token::Keyword(Public(Span::new(1, 7))),
            Token::Keyword(Static(Span::new(8, 14))),
            Token::Keyword(Void(Span::new(15, 19))),
            Token::Ident(Ident::new(Span::new(20, 24))),
            Token::Separator(LeftPar(Span::new(24, 25))),
            Token::Ident(Ident::new(Span::new(25, 31))),
            Token::Separator(LeftBracket(Span::new(31, 32))),
            Token::Separator(RightBracket(Span::new(32, 33))),
            Token::Ident(Ident::new(Span::new(34, 38))),
            Token::Separator(RightPar(Span::new(38, 39))),
            Token::Separator(LeftCurly(Span::new(40, 41))),
            Token::Ident(Ident::new(Span::new(46, 52))),
            Token::Separator(Dot(Span::new(52, 53))),
            Token::Ident(Ident::new(Span::new(53, 56))),
            Token::Separator(Dot(Span::new(56, 57))),
            Token::Ident(Ident::new(Span::new(57, 64))),
            Token::Separator(LeftPar(Span::new(64, 65))),
            Token::Literal(Literal::String(Span::new(65, 79))),
            Token::Separator(RightPar(Span::new(79, 80))),
            Token::Separator(Semicolon(Span::new(80, 81))),
            Token::Separator(RightCurly(Span::new(82, 83))),
        ];
        assert_eq!(lexer.tokens().collect::<Vec<Token>>(), expected);
    }
}
