use core::iter::Peekable;

use crate::lexer::span::{Span, Spanned};
use crate::lexer::token::{Keyword, Operator, Separator, Token};
use crate::lexer::Lexer;
use crate::parser::error::Error;
use crate::parser::tree::{CompilationUnit, Identifier, QualifiedName};
use crate::ImportDeclaration;

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
    pub fn parse(&self) -> Result<CompilationUnit> {
        let mut tokens = self.tokens();
        self.compilation_unit(&mut tokens)
    }

    pub fn resolve_span(&'a self, span: Span) -> Option<&'a str> {
        self.lexer.source().resolve_span(span)
    }

    pub fn resolve_spanned(&'a self, spanned: &impl Spanned) -> Option<&'a str> {
        spanned.span().and_then(|span| self.resolve_span(span))
    }
}

impl Parser<'_> {
    /// Returns the token iterator that this parser will use.
    ///
    /// The result will not yield any comment tokens.
    fn tokens(&self) -> Peekable<impl Iterator<Item = Token> + '_> {
        self.lexer
            .tokens()
            .filter(|t| !matches!(t, Token::Comment(_)))
            .peekable()
    }

    /// Peeks one token, and consumes it if it is a semicolon.
    ///
    /// If the token is not a semicolon, an error is added to the compilation unit.
    fn expect_semicolon(
        tokens: &mut Peekable<impl Iterator<Item = Token> + Sized>,
        compilation_unit: &mut CompilationUnit,
    ) {
        match tokens.next_if(|t| matches!(t, Token::Separator(Separator::Semicolon(_)))) {
            Some(_) => (),
            None => compilation_unit.add_error(Error::UnexpectedToken {
                expected: &[";"],
                found: tokens.peek().cloned(),
            }),
        }
    }

    fn compilation_unit(
        &self,
        tokens: &mut Peekable<impl Iterator<Item = Token>>,
    ) -> Result<CompilationUnit> {
        let mut compilation_unit = CompilationUnit::new();

        loop {
            if let Some(token) = tokens.peek() {
                match token {
                    Token::Keyword(Keyword::Package(_)) => {
                        match self.package_declaration(tokens) {
                            Ok(name) => compilation_unit.set_package(name),
                            Err(error) => compilation_unit.add_error(error),
                        }
                        Self::expect_semicolon(tokens, &mut compilation_unit);
                    }
                    Token::Keyword(Keyword::Import(_)) => {
                        match self.import_declaration(tokens) {
                            Ok(import) => compilation_unit.add_import(import),
                            Err(error) => compilation_unit.add_error(error),
                        }
                        Self::expect_semicolon(tokens, &mut compilation_unit);
                    }
                    _ => {
                        compilation_unit.add_error(Error::UnexpectedToken {
                            expected: &["<unknown>"],
                            found: tokens.next(),
                        });
                    }
                }
            } else {
                break; // no (more) tokens
            }
        }

        Ok(compilation_unit)
    }

    fn package_declaration(
        &self,
        tokens: &mut Peekable<impl Iterator<Item = Token>>,
    ) -> Result<QualifiedName> {
        let package_token = tokens.next().unwrap(); // skip the package token
        debug_assert!(matches!(package_token, Token::Keyword(Keyword::Package(_))));

        self.qualified_name(tokens)
    }

    fn import_declaration(
        &self,
        tokens: &mut Peekable<impl Iterator<Item = Token>>,
    ) -> Result<ImportDeclaration> {
        let import_token = tokens.next().unwrap(); // skip the import token
        debug_assert!(matches!(import_token, Token::Keyword(Keyword::Import(_))));

        let static_import = tokens
            .next_if(|t| matches!(t, Token::Keyword(Keyword::Static(_))))
            .is_some();

        let name = self.qualified_name(tokens)?;

        let last_segment_span = name
            .segments()
            .last()
            .expect("qualified name must have at least one segment")
            .span();
        let last_segment = self
            .resolve_span(*last_segment_span)
            .expect("span of last segment must be valid");
        let is_on_demand = last_segment == "*";

        Ok(match (static_import, is_on_demand) {
            (true, true) => ImportDeclaration::StaticOnDemand(name),
            (true, false) => ImportDeclaration::StaticSingleType(name),
            (false, true) => ImportDeclaration::OnDemand(name),
            (false, false) => ImportDeclaration::SingleType(name),
        })
    }

    fn qualified_name(
        &self,
        tokens: &mut Peekable<impl Iterator<Item = Token>>,
    ) -> Result<QualifiedName> {
        let mut qualified_name = QualifiedName::new();

        loop {
            // expect an identifier as first element
            match tokens.next_if(|t| {
                matches!(t, Token::Ident(_))
                    || matches!(t, Token::Operator(Operator::Arithmetic(_)))
            }) {
                Some(Token::Ident(id)) => qualified_name.push(Identifier::from(id)),
                Some(Token::Operator(Operator::Arithmetic(op))) => {
                    let text = self.resolve_span(op);
                    if text == Some("*") {
                        qualified_name.push(Identifier::from(op))
                    } else {
                        return Err(Error::UnexpectedToken {
                            expected: &["*"],
                            found: tokens.peek().cloned(),
                        });
                    }
                }
                _ => {
                    return Err(Error::UnexpectedToken {
                        expected: &["Ident"],
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
    use crate::lexer::span::Span;
    use crate::lexer::Lexer;

    use super::*;

    macro_rules! apply_rule {
        ($rule:expr, $input:expr) => {{
            let input = $input; // evaluate once
            let lexer = $crate::lexer::Lexer::from(input);
            let p = $crate::parser::Parser::from(lexer);
            let mut tokens = p.tokens();
            $rule(&p, &mut tokens)
        }};
    }

    #[test]
    fn test_imports() {
        let lexer = Lexer::from(
            r#"
import foo.bar.Baz;
import static foo.bar.Baz.snafu;
import foo.bar.*;
import static foo.bar.Baz.*;
"#,
        );
        let parser = Parser::from(lexer);
        let tree = parser.parse().unwrap();
        assert!(!tree.has_errors());
        assert_eq!(
            tree.imports(),
            &[
                ImportDeclaration::SingleType(QualifiedName::from(vec![
                    (8, 11),
                    (12, 15),
                    (16, 19),
                ])),
                ImportDeclaration::StaticSingleType(QualifiedName::from(vec![
                    (35, 38),
                    (39, 42),
                    (43, 46),
                    (47, 52),
                ])),
                ImportDeclaration::OnDemand(QualifiedName::from(vec![
                    (61, 64),
                    (65, 68),
                    (69, 70),
                ])),
                ImportDeclaration::StaticOnDemand(QualifiedName::from(vec![
                    (86, 89),
                    (90, 93),
                    (94, 97),
                    (98, 99),
                ])),
            ]
        );
    }

    #[test]
    fn test_erroneous_package_decl() {
        /*
        Tests a simple case, in which after one rule
        produces an error, parsing must continue.
         */
        let lexer = Lexer::from(
            r#"
package foo.bar.;

import foo;"#,
        );
        let parser = Parser::from(lexer);
        let tree = parser.parse().unwrap();
        assert!(tree.has_errors());
        assert_eq!(
            tree.errors(),
            &[Error::UnexpectedToken {
                expected: &["Ident"],
                found: Some(Token::Separator(Separator::Semicolon(Span::new(17, 18)))),
            }]
        );
        assert!(tree.package().is_none());
        assert_eq!(
            tree.imports(),
            &[ImportDeclaration::SingleType(QualifiedName::from(vec![(
                27, 30
            )]))]
        );
    }

    #[test]
    fn test_incomplete_qualified_name_eof() {
        let result = apply_rule!(Parser::qualified_name, "a.b.");
        assert_eq!(
            result,
            Err(Error::UnexpectedToken {
                expected: &["Ident"],
                found: None,
            })
        );
    }

    #[test]
    fn test_incomplete_qualified_name() {
        let result = apply_rule!(Parser::qualified_name, "a.b.;");
        assert_eq!(
            result,
            Err(Error::UnexpectedToken {
                expected: &["Ident"],
                found: Some(Token::Separator(Separator::Semicolon(Span::new(4, 5)))),
            })
        );
    }

    #[test]
    fn test_qualified_names() {
        for (input, expected) in &[
            ("a.b.c", QualifiedName::from(vec![(0, 1), (2, 3), (4, 5)])),
            ("a.b.*", QualifiedName::from(vec![(0, 1), (2, 3), (4, 5)])),
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
            let output = apply_rule!(Parser::qualified_name, *input).unwrap();
            assert_eq!(output, *expected);
        }
    }

    #[test]
    fn test_qualified_name_not_consume_after() {
        let lexer = Lexer::from("a.b.c;");
        let parser = Parser::from(lexer);
        let mut tokens = parser.tokens();
        let qualified_name = Parser::qualified_name(&parser, &mut tokens).unwrap();
        assert_eq!(
            qualified_name,
            QualifiedName::from(vec![Span::new(0, 1), Span::new(2, 3), Span::new(4, 5)]),
        );

        // ParserImpl::qualified_name must not consume the token after the qualified name
        assert_eq!(
            tokens.next().expect("expected the semicolon at the end"),
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

        assert_eq!(
            tree.imports(),
            &[ImportDeclaration::SingleType(QualifiedName::from(vec![
                (26, 29),
                (30, 33),
                (34, 37),
            ]))]
        );

        // TODO: assert the rest of the tree

        println!("{:#?}", tree);
    }
}
