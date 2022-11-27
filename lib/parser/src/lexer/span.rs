use crate::lexer::GraphemeIndex;

pub trait Spanned {
    fn span(&self) -> Option<Span>;
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Span {
    start: GraphemeIndex,
    end: GraphemeIndex,
}

impl core::fmt::Debug for Span {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Span")
            .field("start", &self.start)
            .field("end", &self.end)
            .finish()
    }
}

impl Span {
    pub fn new<I>(start: I, end: I) -> Self
    where
        I: Into<GraphemeIndex>,
    {
        Self {
            start: start.into(),
            end: end.into(),
        }
    }

    pub fn start(&self) -> GraphemeIndex {
        self.start
    }

    pub fn end(&self) -> GraphemeIndex {
        self.end
    }
}
