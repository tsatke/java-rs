use crate::lexer::span::Span;
use crate::lexer::GraphemeIndex;
use core::str::FromStr;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Source<'a> {
    input: &'a str,
    graphemes: Vec<(usize, char)>,
}

impl<'a> Source<'a> {
    pub fn resolve_span(&'a self, span: Span) -> Option<&'a str> {
        self.translate_indices(span.start(), span.end())
    }

    pub(in crate::lexer) fn grapheme_indices(&self) -> &[(usize, char)] {
        &self.graphemes
    }

    pub(in crate::lexer) fn translate_index(&self, index: GraphemeIndex) -> Option<usize> {
        self.graphemes
            .get(Into::<usize>::into(index))
            .map(|(i, _)| *i)
    }

    pub(in crate::lexer) fn translate_indices(
        &self,
        start: GraphemeIndex,
        end: GraphemeIndex,
    ) -> Option<&str> {
        let start = self.translate_index(start)?;
        let end = self.translate_index(end - 1_usize)?;
        self.input.get(start..=end)
    }

    pub(in crate::lexer) fn matches(&self, offset: GraphemeIndex, s: &str) -> bool {
        let mut graphemes = to_graphemes(s);
        for c in self.graphemes.iter().skip(offset.into()).map(|(_, c)| *c) {
            let next = graphemes.next();
            match next {
                Some(n) if n == c => continue,
                Some(_) => return false,
                None => return true,
            }
        }

        graphemes.next().is_none()
    }

    pub(in crate::lexer) fn char_at(&self, index: GraphemeIndex) -> Option<char> {
        self.graphemes
            .get(Into::<usize>::into(index))
            .map(|(_, c)| *c)
    }
}

impl<'a> From<&'a str> for Source<'a> {
    fn from(input: &'a str) -> Self {
        Self {
            input,
            graphemes: to_grapheme_indices(input),
        }
    }
}

fn to_graphemes(s: &str) -> impl Iterator<Item = char> + '_ {
    UnicodeSegmentation::graphemes(s, true).map(|s| char::from_str(s).unwrap())
}

fn to_grapheme_indices(s: &str) -> Vec<(usize, char)> {
    UnicodeSegmentation::grapheme_indices(s, true)
        .map(|(i, s)| (i, char::from_str(s).unwrap()))
        .collect()
}
