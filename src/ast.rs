use std::{collections::HashMap, fmt::Debug, rc::Rc};

/// File definition, it contains all the statements,
/// the module name, and a base location for it as anchor
/// for the statements.
#[derive(Default, Debug, Clone, serde::Serialize)]
pub struct File<S: State> {
    pub name: String,
    pub declarations: Vec<Decl<S>>,
    pub definitions: HashMap<usize, Definition>,
    pub location: Location,
}

#[derive(Debug, Clone, serde::Serialize)]
pub enum Infallible {}

/// Represents the syntax state, if it's resolved, or just parsed, it's useful for not
/// having to redeclare the same types.
pub trait State: serde::Serialize + Default + Debug + Clone {
    type Definition: serde::Serialize + Debug + Clone;
    type Reference: serde::Serialize + Element + Debug + Clone;
}

impl Element for Infallible {
    fn location(&self) -> &Location {
        unreachable!()
    }
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

#[derive(Debug)]
pub enum DefinitionKind {
    Constructor,
    Inductive,
    Binding,
}

#[derive(Default, Debug, Clone, Hash)]
#[repr(transparent)]
pub struct Uniq {
    pub value: usize,
}

impl serde::Serialize for Uniq {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.value as u64)
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
pub struct Binary<S: State> {
    pub lhs: Box<Term<S>>,
    pub op: String,
    pub rhs: Box<Term<S>>,
    pub location: Location,
}

impl<S: State> Element for Binary<S> {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Call<S: State> {
    pub callee: Box<Term<S>>,
    pub arguments: Vec<Term<S>>,
    pub location: Location,
}

impl<S: State> Element for Call<S> {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Fun<S: State> {
    pub arguments: Vec<S::Definition>,
    pub value: Block<S>,
    pub location: Location,
}

impl<S: State> Element for Fun<S> {
    fn location(&self) -> &Location {
        &self.location
    }
}

pub enum Term<S: State> {
    Error(Error),
    Int(Int),
    Str(Str),
    Call(Call<S>),
    Binary(Binary<S>),
    Fun(Fun<S>),
    Group(Box<Term<S>>),
    Reference(S::Reference),
}

/// DERIVED FROM CARGO EXPAND
#[automatically_derived]
impl<S: State> serde::Serialize for Term<S> {
    fn serialize<Ser>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error>
    where
        Ser: serde::Serializer,
    {
        match *self {
            Term::Group(box ref arg0) => arg0.serialize(serializer),
            Term::Error(ref arg0) => {
                serialize_tagged_newtype(serializer, "Term", "Error", "kind", "Error", arg0)
            }
            Term::Int(ref arg0) => {
                serialize_tagged_newtype(serializer, "Term", "Int", "kind", "Int", arg0)
            }
            Term::Str(ref arg0) => {
                serialize_tagged_newtype(serializer, "Term", "Str", "kind", "Str", arg0)
            }
            Term::Call(ref arg0) => {
                serialize_tagged_newtype(serializer, "Term", "Call", "kind", "Call", arg0)
            }
            Term::Binary(ref arg0) => {
                serialize_tagged_newtype(serializer, "Term", "Binary", "kind", "Binary", arg0)
            }
            Term::Fun(ref arg0) => {
                serialize_tagged_newtype(serializer, "Term", "Fun", "kind", "Fun", arg0)
            }
            Term::Reference(ref arg0) => {
                serialize_tagged_newtype(serializer, "Term", "Reference", "kind", "Reference", arg0)
            }
        }
    }
}

impl<S: State> Clone for Term<S> {
    fn clone(&self) -> Self {
        match self {
            Self::Error(arg0) => Self::Error(arg0.clone()),
            Self::Int(arg0) => Self::Int(arg0.clone()),
            Self::Str(arg0) => Self::Str(arg0.clone()),
            Self::Group(arg0) => Self::Group(arg0.clone()),
            Self::Fun(arg0) => Self::Fun(arg0.clone()),
            Self::Call(arg0) => Self::Call(arg0.clone()),
            Self::Reference(arg0) => Self::Reference(arg0.clone()),
            Self::Binary(arg0) => Self::Binary(arg0.clone()),
        }
    }
}

impl<S: State> Debug for Term<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Error(arg0) => arg0.fmt(f),
            Self::Int(arg0) => arg0.fmt(f),
            Self::Str(arg0) => arg0.fmt(f),
            Self::Group(arg0) => arg0.fmt(f),
            Self::Fun(arg0) => arg0.fmt(f),
            Self::Call(arg0) => arg0.fmt(f),
            Self::Reference(arg0) => arg0.fmt(f),
            Self::Binary(arg0) => arg0.fmt(f),
        }
    }
}

impl<S: State> Element for Term<S> {
    fn location(&self) -> &Location {
        match self {
            Term::Error(arg0) => arg0.location(),
            Term::Int(arg0) => arg0.location(),
            Term::Str(arg0) => arg0.location(),
            Term::Group(arg0) => arg0.location(),
            Term::Fun(arg0) => arg0.location(),
            Term::Call(arg0) => arg0.location(),
            Term::Reference(arg0) => arg0.location(),
            Term::Binary(arg0) => arg0.location(),
        }
    }
}

use serde::__private::ser::serialize_tagged_newtype;
// SECTION: Modules
pub use decl::*;
pub mod decl;

pub use stmt::*;
mod stmt;
