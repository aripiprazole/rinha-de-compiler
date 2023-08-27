use super::*;


#[derive(Debug, Clone, serde::Serialize)]
pub struct Block<S: State> {
    pub statements: Vec<Stmt<S>>,
    pub location: Location,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct If<S: State> {
    pub condition: Term<S>,
    pub then: Block<S>,
    pub otherwise: Option<Block<S>>,
    pub location: Location,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Let<S: State> {
    pub name: S::Definition,
    pub value: Term<S>,
    pub location: Location,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Return<S: State> {
    pub value: Option<Term<S>>,
    pub location: Location,
}

/// A statement. It can be an inductive type, or a downgrade.
#[derive(Clone, serde::Serialize)]
#[serde(tag = "kind")]
pub enum Stmt<S: State> {
    Error(Error),
    Term(Term<S>),
    If(If<S>),
    Let(Let<S>),
    Return(Return<S>),
    Function(Function<S>),
}

impl<S: State> Debug for Stmt<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Term(arg0) => arg0.fmt(f),
            Self::Error(arg0) => arg0.fmt(f),
            Self::Let(arg0) => arg0.fmt(f),
            Self::If(arg0) => arg0.fmt(f),
            Self::Return(arg0) => arg0.fmt(f),
            Self::Function(arg0) => arg0.fmt(f),
        }
    }
}

impl<S: State> Element for Stmt<S> {
    fn location(&self) -> &Location {
        match self {
            Self::Term(arg0) => arg0.location(),
            Stmt::Error(arg0) => &arg0.location,
            Stmt::Let(arg0) => &arg0.location,
            Self::If(arg0) => &arg0.location,
            Stmt::Return(arg0) => &arg0.location,
            Stmt::Function(arg0) => &arg0.location,
        }
    }
}
