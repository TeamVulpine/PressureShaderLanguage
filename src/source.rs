use std::{
    fmt::{Debug, Display},
    iter::Sum,
    num::NonZeroU32,
    ops::Add,
};

use crate::module_cache::ModuleIndex;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SourcePos {
    line: NonZeroU32,
    column: NonZeroU32,
}

pub struct SourceCursor<'a> {
    source: &'a str,
    module_index: ModuleIndex,

    current_index: u32,
    current_pos: SourcePos,

    rollback_index: u32,
    rollback_pos: SourcePos,
}

#[derive(Clone, Copy, PartialEq)]
pub struct SourceSpan {
    module_index: ModuleIndex,
    span: (u32, u32),
    start_pos: SourcePos,
    end_pos: SourcePos,
}

#[derive(Debug, Clone, Copy)]
pub struct Spanned<T> {
    pub value: T,
    pub span: SourceSpan,
}

impl SourcePos {
    fn advance(&mut self, c: char) {
        if c != '\n' {
            self.column = self.column.checked_add(1).unwrap_or(self.column);
        } else {
            self.column = NonZeroU32::MIN;
            self.line = self.line.checked_add(1).unwrap_or(self.line);
        }
    }

    const fn new() -> Self {
        return Self {
            line: NonZeroU32::MIN,
            column: NonZeroU32::MIN,
        };
    }

    pub const fn line(&self) -> NonZeroU32 {
        return self.line;
    }

    pub const fn column(&self) -> NonZeroU32 {
        return self.column;
    }
}

impl Display for SourcePos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}:{}", self.line, self.column);
    }
}

impl<'a> SourceCursor<'a> {
    pub fn new(source: &'a str, module_index: ModuleIndex) -> Self {
        return Self {
            source,
            module_index,
            current_index: 0,
            current_pos: SourcePos::new(),
            rollback_index: 0,
            rollback_pos: SourcePos::new(),
        };
    }

    pub fn empty_span(&self) -> SourceSpan {
        return SourceSpan {
            module_index: self.module_index,
            span: (0, 0),
            start_pos: SourcePos::new(),
            end_pos: SourcePos::new(),
        };
    }

    fn current_slice(&self) -> &str {
        debug_assert!(self.source.is_char_boundary(self.current_index as usize));
        return &self.source[self.current_index as usize..];
    }

    pub fn current(&self) -> Option<char> {
        return self.current_slice().chars().next();
    }

    pub fn advance(&mut self) -> Option<char> {
        let c = self.current()?;

        self.current_index += c.len_utf8() as u32;
        self.current_pos.advance(c);

        return Some(c);
    }

    pub fn is_char(&self, c: char) -> bool {
        return self.current() == Some(c);
    }

    pub fn is_char_and_then_fn<F: Fn(char) -> bool>(&self, c: char, f: F) -> bool {
        if self.current() != Some(c) {
            return false;
        }

        let Some(c) = self.current_slice().chars().nth(1) else {
            return false;
        };

        return f(c);
    }

    pub fn is_str(&self, s: &str) -> bool {
        return self.current_slice().starts_with(s);
    }

    pub fn is_fn<F: Fn(char) -> bool>(&self, f: F) -> bool {
        return self.current().map_or(false, f);
    }

    pub fn consume_char(&mut self, c: char) -> bool {
        if !self.is_char(c) {
            return false;
        }

        let _ = self.advance();
        return true;
    }

    pub fn consume_str(&mut self, s: &str) -> bool {
        if !self.is_str(s) {
            return false;
        }

        for c in s.chars() {
            debug_assert_eq!(self.current(), Some(c));
            self.advance();
        }

        return true;
    }

    pub fn consume_fn<F: Fn(char) -> bool>(&mut self, f: F) -> bool {
        if !self.is_fn(f) {
            return false;
        }

        let _ = self.advance();
        return true;
    }

    pub fn while_char(&mut self, c: char) -> bool {
        let mut consumed = false;

        while self.consume_char(c) {
            consumed = true;
        }

        return consumed;
    }

    pub fn while_fn<F: Fn(char) -> bool>(&mut self, f: F) -> bool {
        let mut consumed = false;

        while self.consume_fn(&f) {
            consumed = true;
        }

        return consumed;
    }

    #[must_use]
    pub fn commit(&mut self) -> (SourceSpan, &'a str) {
        let span = (self.rollback_index, self.current_index);
        let start_pos = self.rollback_pos;
        let end_pos = self.current_pos;

        self.rollback_index = self.current_index;
        self.rollback_pos = self.current_pos;

        let source = &self.source[span.0 as usize..span.1 as usize];

        let span = SourceSpan {
            module_index: self.module_index,
            span,
            start_pos,
            end_pos,
        };

        return (span, source);
    }

    pub fn rollback(&mut self) {
        self.current_index = self.rollback_index;
        self.current_pos = self.rollback_pos;
    }

    pub fn is_eof(&self) -> bool {
        return self.current().is_none();
    }

    pub fn relative_offset(&self) -> u32 {
        return self.current_index - self.rollback_index;
    }
}

impl SourceSpan {
    pub fn start_pos(&self) -> SourcePos {
        return self.start_pos;
    }

    pub fn end_pos(&self) -> SourcePos {
        return self.end_pos;
    }

    pub fn module_index(&self) -> ModuleIndex {
        return self.module_index;
    }

    pub fn into_spanned<T>(&self, value: T) -> Spanned<T> {
        return Spanned { value, span: *self };
    }
}

impl Add for SourceSpan {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.module_index, rhs.module_index);

        return Self {
            module_index: self.module_index,
            span: (self.span.0.min(rhs.span.0), self.span.1.max(rhs.span.1)),
            start_pos: self.start_pos.min(rhs.start_pos),
            end_pos: self.end_pos.max(rhs.end_pos),
        };
    }
}

impl<'a> Sum<&'a SourceSpan> for Option<SourceSpan> {
    fn sum<I: Iterator<Item = &'a SourceSpan>>(iter: I) -> Option<SourceSpan> {
        return iter.fold(None, |a, b| a.map(|it| it + *b).or_else(|| Some(*b)));
    }
}

impl Debug for SourceSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "{:?}[{}..{}]",
            self.module_index, self.span.0, self.span.1
        );
    }
}

impl<T> Spanned<T> {
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Spanned<U> {
        return Spanned {
            value: f(self.value),
            span: self.span,
        };
    }
}
