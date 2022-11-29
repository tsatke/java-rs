use core::iter::Peekable;

use crate::lexer::span::{Span, Spanned};
use crate::lexer::token::Token;
use crate::lexer::Lexer;
use crate::parser::context::ParseContext;
use crate::parser::error::Error;
use crate::parser::tree::CompilationUnit;

mod context;
pub mod error;
pub mod tree;

pub type Result<'source, T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Eq, PartialEq)]
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
    pub fn parse(&self) -> CompilationUnit {
        let tokens = self.tokens();
        let mut context = ParseContext::new(self, CompilationUnit::new(), tokens);
        context.parse();
        context.into()
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
}

#[cfg(test)]
mod tests {
    use crate::lexer::span::Span;
    use crate::lexer::token::Separator;
    use crate::lexer::Lexer;
    use crate::{ImportDeclaration, QualifiedName};

    use super::*;

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
        let tree = parser.parse();
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
        let tree = parser.parse();
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
        let tree = parser.parse();
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
