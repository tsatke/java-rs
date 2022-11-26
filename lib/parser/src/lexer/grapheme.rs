use core::ops::{Add, AddAssign, Sub};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct GraphemeIndex(usize);

impl core::fmt::Debug for GraphemeIndex {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("GraphemeIndex({})", self.0))
    }
}

impl From<usize> for GraphemeIndex {
    fn from(index: usize) -> Self {
        Self(index)
    }
}

impl From<GraphemeIndex> for usize {
    fn from(index: GraphemeIndex) -> Self {
        index.0
    }
}

impl Add for GraphemeIndex {
    type Output = GraphemeIndex;

    fn add(self, rhs: GraphemeIndex) -> Self::Output {
        GraphemeIndex(self.0 + rhs.0)
    }
}

impl Add<usize> for GraphemeIndex {
    type Output = GraphemeIndex;

    fn add(self, rhs: usize) -> Self::Output {
        GraphemeIndex(self.0 + rhs)
    }
}

impl Sub for GraphemeIndex {
    type Output = GraphemeIndex;

    fn sub(self, rhs: GraphemeIndex) -> Self::Output {
        GraphemeIndex(self.0 - rhs.0)
    }
}

impl Sub<usize> for GraphemeIndex {
    type Output = GraphemeIndex;

    fn sub(self, rhs: usize) -> Self::Output {
        GraphemeIndex(self.0 - rhs)
    }
}

impl AddAssign for GraphemeIndex {
    fn add_assign(&mut self, rhs: GraphemeIndex) {
        self.0 += rhs.0;
    }
}

impl AddAssign<usize> for GraphemeIndex {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}
