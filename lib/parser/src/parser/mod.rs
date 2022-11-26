use crate::lexer::span::Span;
use crate::lexer::token::{Keyword, Separator, Token};
use crate::lexer::Lexer;
use crate::parser::error::Error;
use crate::parser::tree::{CompilationUnit, Identifier, QualifiedName};
use std::iter::Peekable;

pub mod error;
pub mod tree;

pub type Result<'source, T> = core::result::Result<T, Error>;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> From<&'a str> for Parser<'a> {
    fn from(s: &'a str) -> Self {
        let lexer = Lexer::from(s);
        Self::from(lexer)
    }
}

impl<'a> From<Lexer<'a>> for Parser<'a> {
    fn from(lexer: Lexer<'a>) -> Self {
        Self { lexer }
    }
}

impl<'a> Parser<'a> {
    pub fn parse_str(input: &'a str) -> Result<CompilationUnit> {
        Self::from(input).parse()
    }

    pub fn parse(&self) -> Result<CompilationUnit> {
        let mut tokens = self.lexer.tokens();
        Self::parse_tokens(&mut tokens)
    }

    pub fn resolve_span(&'a self, span: Span) -> Option<&'a str> {
        self.lexer.source().resolve_span(span)
    }
}

impl Parser<'_> {
    fn parse_tokens(tokens: impl Iterator<Item = Token>) -> Result<'static, CompilationUnit> {
        let mut p = tokens.peekable();
        ParserImpl::compilation_unit(&mut p)
    }
}

struct ParserImpl;

impl ParserImpl {
    fn compilation_unit(
        tokens: &mut Peekable<impl Iterator<Item = Token>>,
    ) -> Result<CompilationUnit> {
        let mut compilation_unit = CompilationUnit::new();

        if let Some(token) = tokens.peek() {
            match token {
                Token::Keyword(Keyword::Package(_)) => {
                    tokens.next().unwrap(); // safe, we just peeked it. skip the package token
                    let name = match ParserImpl::qualified_name(tokens) {
                        Ok(name) => name,
                        Err(_) => todo!("add error to compilation unit"),
                    };
                    compilation_unit.set_package(name);
                    match tokens.next_if(|t| matches!(t, Token::Separator(Separator::Semicolon(_))))
                    {
                        Some(_) => (),
                        None => todo!("expect semicolon after package declaration"),
                    }
                }
                Token::Keyword(Keyword::Import(_)) => {
                    todo!("parse imports")
                }
                _ => todo!(),
            }
        } else {
            todo!("no tokens in the file")
        }

        Ok(compilation_unit)
    }

    fn qualified_name(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<QualifiedName> {
        let mut qualified_name = QualifiedName::new();

        loop {
            // expect an identifier as first element
            match tokens.next_if(|t| matches!(t, Token::Ident(_))) {
                Some(Token::Ident(id)) => qualified_name.push(Identifier::from(id)),
                _ => {
                    return Err(Error::UnexpectedToken {
                        expected: vec!["Ident"],
                        found: tokens.peek().cloned(), // as opposed to the pattern we're matching, peek returns the next token, which is what we want
                    });
                }
            }
            // after an identifier, expect a dot and then another identifier, or break
            match tokens.next_if(|t| matches!(t, Token::Separator(Separator::Dot(_)))) {
                Some(_) => {
                    // dot is consumed
                }
                None => {
                    // no dot, so we're done
                    return Ok(qualified_name);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::span::Span;
    use crate::lexer::Lexer;

    macro_rules! apply_rule {
        ($rule:expr, $input:expr) => {{
            let input = $input; // evaluate once
            let lexer = $crate::lexer::Lexer::from(input);
            let mut tokens = lexer.tokens();
            let p = &mut (&mut tokens).peekable();
            $rule(p)
        }};
    }

    #[test]
    fn test_incomplete_qualified_name_eof() {
        let result = apply_rule!(ParserImpl::qualified_name, "a.b.");
        assert_eq!(
            result,
            Err(Error::UnexpectedToken {
                expected: vec!["Ident"],
                found: None
            })
        );
    }

    #[test]
    fn test_incomplete_qualified_name() {
        let result = apply_rule!(ParserImpl::qualified_name, "a.b.;");
        assert_eq!(
            result,
            Err(Error::UnexpectedToken {
                expected: vec!["Ident"],
                found: Some(Token::Separator(Separator::Semicolon(Span::new(4, 5))))
            })
        );
    }

    #[test]
    fn test_qualified_names() {
        for (input, expected) in &[
            ("a.b.c", QualifiedName::from(vec![(0, 1), (2, 3), (4, 5)])),
            (
                "a .b . c",
                QualifiedName::from(vec![(0, 1), (3, 4), (7, 8)]),
            ),
            (
                "a.b.c hello world",
                QualifiedName::from(vec![(0, 1), (2, 3), (4, 5)]),
            ),
            (
                "hello.world.Foobar",
                QualifiedName::from(vec![(0, 5), (6, 11), (12, 18)]),
            ),
        ] {
            let output = apply_rule!(ParserImpl::qualified_name, *input).unwrap();
            assert_eq!(output, *expected);
        }
    }

    #[test]
    fn test_qualified_name_not_consume_after() {
        let lexer = Lexer::from("a.b.c;");
        let tokens = &mut lexer.tokens();
        let p = &mut tokens.peekable();
        let qualified_name = ParserImpl::qualified_name(p).unwrap();
        assert_eq!(
            qualified_name,
            QualifiedName::from(vec![Span::new(0, 1), Span::new(2, 3), Span::new(4, 5)]),
        );

        // ParserImpl::qualified_name must not consume the token after the qualified name
        assert_eq!(
            p.next().expect("expected the semicolon at the end"),
            Token::Separator(Separator::Semicolon(Span::new(5, 6)))
        );
    }

    #[test]
    fn test_small_example() {
        let lexer = Lexer::from(
            r#"
package foo.bar;

import foo.bar.Baz;

public class Main {
    public static void main(String[] args) {
        System.out.println("Hello, World!");
    }
}
        "#,
        );
        let parser = Parser::from(lexer);
        let tree = parser.parse().unwrap();
        let package_name = parser
            .resolve_span(
                tree.package()
                    .expect("tree must have a package declaration")
                    .span()
                    .expect("package declaration must have a span"),
            )
            .expect("package declaration span must be resolvable");
        assert_eq!("foo.bar", package_name);

        println!("{:#?}", tree);
    }
}
