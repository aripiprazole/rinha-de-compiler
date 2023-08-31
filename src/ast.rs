use std::{fmt::Debug, rc::Rc};

/// File definition, it contains all the statements,
/// the module name, and a base location for it as anchor
/// for the statements.
#[derive(Debug, Clone, serde::Serialize)]
pub struct File {
    pub name: String,
    pub expression: Term,
    pub location: Location,
}

impl<T: Element> Element for Rc<T> {
    fn location(&self) -> &Location {
        self.as_ref().location()
    }
}

impl<T: Element> Element for Box<T> {
    fn location(&self) -> &Location {
        self.as_ref().location()
    }
}

/// A definition. It has a text, and a location.
#[derive(Default, Debug, Clone, Hash, serde::Serialize)]
pub struct Definition {
    pub text: String,
    pub location: Location,
}

impl Definition {
    /// Creates a new instance of [`Definition`].
    pub fn new(text: String) -> Self {
        Self {
            text,
            location: Location::default(),
        }
    }
}

impl Element for Definition {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Default, Hash, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Location {
    pub start: usize,
    pub end: usize,
    pub filename: String,
}

impl Location {
    /// Creates a new instance of [`Location`].
    pub fn new(start: usize, end: usize, filename: &str) -> Self {
        Self {
            start,
            end,
            filename: filename.into(),
        }
    }
}

impl Debug for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Location")
    }
}

impl From<Location> for miette::SourceSpan {
    fn from(value: Location) -> Self {
        Self::from(value.start..value.end)
    }
}

/// An element. It can be a declaration, or a term.
pub trait Element {
    fn location(&self) -> &Location;
}

/// Error node, it does contains an error.
#[derive(Debug, Clone, serde::Serialize)]
pub struct Error {
    /// The error message.
    pub message: String,

    /// The original text that originated the error.
    pub full_text: String,

    /// The location of the error.
    pub location: Location,
}

impl Element for Error {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct If {
    pub condition: Box<Term>,
    pub then: Box<Term>,
    pub otherwise: Box<Term>,
    pub location: Location,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Let {
    pub name: crate::parser::Reference,
    pub value: Box<Term>,
    pub next: Box<Term>,
    pub location: Location,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Identifier {
    pub text: String,
    pub location: Location,
}

impl Element for Identifier {
    fn location(&self) -> &Location {
        &self.location
    }
}

/// Int is a integer value like `0`, `1`, `2`, etc.
#[derive(Default, Debug, Clone, serde::Serialize)]
pub struct Str {
    pub value: String,

    /// The location of the source in the source code.
    pub location: Location,
}

impl Element for Str {
    fn location(&self) -> &Location {
        &self.location
    }
}

/// Int is a integer value like `0`, `1`, `2`, etc.
#[derive(Default, Debug, Clone, serde::Serialize)]
pub struct Int {
    /// The value of the integer.
    pub value: isize,

    /// The location of the integer in the source code.
    pub location: Location,
}

impl Element for Int {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Binary {
    pub lhs: Box<Term>,
    pub op: String,
    pub rhs: Box<Term>,
    pub location: Location,
}

impl Element for Binary {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Call {
    pub callee: Box<Term>,
    pub arguments: Vec<Term>,
    pub location: Location,
}

impl Element for Call {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Fun {
    pub name: crate::parser::Reference,
    pub parameters: Vec<crate::parser::Reference>,
    pub is_external: bool,
    pub value: Option<Box<Term>>,
    pub next: Box<Term>,
    pub location: Location,
}

impl Element for Fun {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub enum Term {
    Error(Error),
    Int(Int),
    Str(Str),
    Call(Call),
    Binary(Binary),
    Fun(Fun),
    Let(Let),
    If(If),
    Group(Box<Term>),
    Reference(crate::parser::Reference),
}

impl Element for Term {
    fn location(&self) -> &Location {
        match self {
            Term::Error(arg0) => &arg0.location,
            Term::Int(arg0) => &arg0.location,
            Term::Str(arg0) => &arg0.location,
            Term::Group(arg0) => arg0.location(),
            Term::Fun(arg0) => &arg0.location,
            Term::Call(arg0) => arg0.location(),
            Term::Reference(arg0) => arg0.location(),
            Term::Binary(arg0) => &arg0.location,
            Term::Let(arg0) => &arg0.location,
            Term::If(arg0) => &arg0.location,
        }
    }
}
