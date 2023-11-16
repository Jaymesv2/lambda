//! Source location tracking helpers


use std::{str::Chars, fmt::Display};

/// this needs a manual ord impl
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Location {
    absolute: usize
}

//#[derive(Debug, Clone, Copy, Default)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct BytePos(usize);

impl From<Location> for BytePos {
    fn from(value: Location) -> Self {
        Self(value.absolute)
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Location: {{ absolut: {} }}", self.absolute)
    }
}

impl Location {
    pub fn new() -> Self {
        Self {
            absolute: 0
        }
    }

    /// Steps the location forward by the character
    pub fn step(&self,ch: char) -> Location {
        // increment the absolut position
        Self {
            absolute: self.absolute + ch.len_utf8()
        }
    }

    pub fn to_usize(&self) -> usize {
        self.absolute
    }
}


pub struct CharLocations<'a> {
    //source: &'a str,
    inner: Chars<'a>,
    location: Location,
}

impl<'a> CharLocations<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            //source,
            inner: source.chars(),
            location: Location::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Span<P> {
    pub start: P,
    pub end: P
}

#[derive(Debug, Clone)]
pub struct Spanned<T, P> {
    pub span: Span<P>,
    pub value: T
}
impl<T,P> Spanned<T,P> {
    pub fn to_triple() {
        
    }
}

pub fn spanned<T, P>(start: P, end: P, value: T) -> Spanned<T, P> {
    Spanned {
        span: Span {
            start, 
            end
        },
        value
    }
} 


impl<'a> Iterator for CharLocations<'a> {
    type Item = (Location, char);

    fn next(&mut self) -> Option<Self::Item> {
        let Some(ch) = self.inner.next() else {
            return None;
        };
        let location = self.location;
        self.location = self.location.step(ch);
        Some((location, ch))
    }
}