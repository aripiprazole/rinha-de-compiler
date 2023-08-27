use super::*;

/// A documentation string. It has a list of strings.
///
/// It's used to document declarations.
#[derive(Debug, Clone, serde::Serialize)]
pub struct DocString {
    pub full_text: String,
    pub text: String,
    pub location: Location,
}

impl Element for DocString {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Function<S: State> {
    pub doc_strings: Vec<DocString>,
    pub is_external: bool,
    pub name: S::Definition,
    pub parameters: Vec<S::Definition>,
    pub block: Option<Block<S>>,
    pub location: Location,
}

impl<S: State> Element for Function<S> {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "kind")]
pub enum Decl<S: State> {
    Function(Function<S>),
}

impl<S: State> Element for Decl<S> {
    fn location(&self) -> &Location {
        match self {
            Decl::Function(function) => &function.location,
        }
    }
}