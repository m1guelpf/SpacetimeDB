use spacetimedb_table::table::RowRef;

use crate::rel_ops::RelOps;
use crate::relation::RelValue;

/// Turns an iterator over [`RelValue<'_>`]s into a `RelOps`.
#[derive(Debug)]
pub struct RelIter<I> {
    pub iter: I,
}

impl<I> RelIter<I> {
    pub fn new(iter: impl IntoIterator<IntoIter = I>) -> Self {
        let iter = iter.into_iter();
        Self { iter }
    }
}

impl<'a, I: Iterator<Item = RelValue<'a>>> RelOps<'a> for RelIter<I> {
    fn next(&mut self) -> Option<RelValue<'a>> {
        self.iter.next()
    }
}

/// Turns an iterator over [`RelValue<'_>`]s into a `RelOps`.
#[derive(Debug)]
pub struct RowRefIter<I> {
    pub iter: I,
}

impl<I> RowRefIter<I> {
    pub fn new(iter: impl IntoIterator<IntoIter = I>) -> Self {
        let iter = iter.into_iter();
        Self { iter }
    }
}

impl<'a, I: Iterator<Item = RowRef<'a>>> RelOps<'a> for RowRefIter<I> {
    fn next(&mut self) -> Option<RelValue<'a>> {
        self.iter.next().map(RelValue::Row)
    }
}
