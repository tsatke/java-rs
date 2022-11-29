use crate::lexer::token::{Keyword, Operator, Separator, Token};
use crate::parser::error::Error;
use crate::parser::Result;
use crate::{
    CompilationUnit, Identifier, ImportDeclaration, Parser, QualifiedName, TypeDeclaration,
};
use std::iter::Peekable;

pub(in crate::parser) struct ParseContext<'a, I>
where
    I: Iterator<Item = Token>,
{
    parser: &'a Parser<'a>,
    compilation_unit: CompilationUnit,
    tokens: Peekable<I>,
}

impl<I> From<ParseContext<'_, I>> for CompilationUnit
where
    I: Iterator<Item = Token>,
{
    fn from(ctx: ParseContext<'_, I>) -> Self {
        ctx.compilation_unit
    }
}

impl<'a, I> ParseContext<'a, I>
where
    I: Iterator<Item = Token>,
{
    pub fn new(
        parser: &'a Parser<'a>,
        compilation_unit: CompilationUnit,
        tokens: Peekable<I>,
    ) -> Self {
        Self {
            parser,
            compilation_unit,
            tokens,
        }
    }

    pub fn parse(&mut self) {
        self.compilation_unit();
    }

    /// Peeks one token, and consumes it if it is a semicolon.
    ///
    /// If the token is not a semicolon, an error is added to the compilation unit.
    fn expect_semicolon(&mut self) {
        match self
            .tokens
            .next_if(|t| matches!(t, Token::Separator(Separator::Semicolon(_))))
        {
            Some(_) => (),
            None => self.compilation_unit.add_error(Error::UnexpectedToken {
                expected: &[";"],
                found: self.tokens.peek().cloned(),
            }),
        }
    }

    fn compilation_unit(&mut self) {
        while let Some(token) = self.tokens.peek() {
            match token {
                Token::Keyword(Keyword::Package(_)) => {
                    match self.package_declaration() {
                        Ok(name) => self.compilation_unit.set_package(name),
                        Err(error) => self.compilation_unit.add_error(error),
                    }
                    self.expect_semicolon();
                }
                Token::Keyword(Keyword::Import(_)) => {
                    match self.import_declaration() {
                        Ok(import) => self.compilation_unit.add_import(import),
                        Err(error) => self.compilation_unit.add_error(error),
                    }
                    self.expect_semicolon();
                }
                _ => match self.type_declaration() {
                    Ok(type_decl) => self.compilation_unit.add_type(type_decl),
                    Err(error) => self.compilation_unit.add_error(error),
                },
            }
        }
    }

    fn type_declaration(&mut self) -> Result<TypeDeclaration> {
        self.tokens.by_ref().for_each(|_| {}); // drain the iterator
        Err(Error::NotImplemented(None)) // TODO: implement
    }

    fn package_declaration(&mut self) -> Result<QualifiedName> {
        let package_token = self.tokens.next().unwrap(); // skip the package token
        debug_assert!(matches!(package_token, Token::Keyword(Keyword::Package(_))));

        self.qualified_name()
    }

    fn import_declaration(&mut self) -> Result<ImportDeclaration> {
        let import_token = self.tokens.next().unwrap(); // skip the import token
        debug_assert!(matches!(import_token, Token::Keyword(Keyword::Import(_))));

        let static_import = self
            .tokens
            .next_if(|t| matches!(t, Token::Keyword(Keyword::Static(_))))
            .is_some();

        let name = self.qualified_name()?;

        let last_segment_span = name
            .segments()
            .last()
            .expect("qualified name must have at least one segment")
            .span();
        let last_segment = self
            .parser
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

    fn qualified_name(&mut self) -> Result<QualifiedName> {
        let mut qualified_name = QualifiedName::new();

        loop {
            // expect an identifier as first element
            match self.tokens.next_if(|t| {
                matches!(t, Token::Ident(_))
                    || matches!(t, Token::Operator(Operator::Arithmetic(_)))
            }) {
                Some(Token::Ident(id)) => qualified_name.push(Identifier::from(id)),
                Some(Token::Operator(Operator::Arithmetic(op))) => {
                    let text = self.parser.resolve_span(op);
                    if text == Some("*") {
                        qualified_name.push(Identifier::from(op))
                    } else {
                        return Err(Error::UnexpectedToken {
                            expected: &["*"],
                            found: self.tokens.peek().cloned(),
                        });
                    }
                }
                _ => {
                    return Err(Error::UnexpectedToken {
                        expected: &["Ident"],
                        found: self.tokens.peek().cloned(), // as opposed to the pattern we're matching, peek returns the next token, which is what we want
                    });
                }
            }
            // after an identifier, expect a dot and then another identifier, or break
            match self
                .tokens
                .next_if(|t| matches!(t, Token::Separator(Separator::Dot(_))))
            {
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
            let parser = Parser::from($input);
            let tokens = parser.tokens();
            let mut ctx = ParseContext::new(&parser, CompilationUnit::new(), tokens);
            let result = $rule(&mut ctx);
            (parser.clone(), result) // TODO: can we get rid of the clone?
        }};
    }

    #[test]
    fn test_qualified_name() {
        let (parser, result) = apply_rule!(ParseContext::qualified_name, "a.b.c");
        let name = result.expect("qualified name must parse");
        assert_eq!(
            name.segments()
                .iter()
                .map(|s| parser.resolve_spanned(s))
                .map(|s| s.unwrap())
                .collect::<Vec<_>>()
                .as_slice(),
            &["a", "b", "c"]
        );
    }

    #[test]
    fn test_incomplete_qualified_name_eof() {
        let (_, result) = apply_rule!(ParseContext::qualified_name, "a.b.");
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
        let (_, result) = apply_rule!(ParseContext::qualified_name, "a.b.;");
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
            let (_, output) = apply_rule!(ParseContext::qualified_name, *input);
            assert_eq!(output.unwrap(), *expected);
        }
    }

    #[test]
    fn test_qualified_name_not_consume_after() {
        let lexer = Lexer::from("a.b.c;");
        let parser = Parser::from(lexer);
        let tokens = parser.tokens();
        let mut ctx = ParseContext::new(&parser, CompilationUnit::new(), tokens);
        let qualified_name = ctx.qualified_name().unwrap();
        assert_eq!(
            qualified_name,
            QualifiedName::from(vec![Span::new(0, 1), Span::new(2, 3), Span::new(4, 5)]),
        );

        // ParseContext::qualified_name must not consume the token after the qualified name
        assert_eq!(
            ctx.tokens
                .next()
                .expect("expected the semicolon at the end"),
            Token::Separator(Separator::Semicolon(Span::new(5, 6)))
        );
    }
}
