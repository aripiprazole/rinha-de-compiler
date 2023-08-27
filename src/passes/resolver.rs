use std::rc::Rc;

use fxhash::FxBuildHasher;
use miette::{Context, IntoDiagnostic, NamedSource, SourceSpan};

use crate::ast::{
    Block, Call, Decl, Definition, DocString, Element, Error, File, Fun, Function, If, Int, Let,
    Location, Return, State, Stmt, Str, Term, Uniq, Binary,
};

use super::parser::{parse_or_report, Parsed};

/// Represents the resolved state, it's the state of the syntax tree when it's resolved.
#[derive(Default, Debug, Clone, serde::Serialize)]
pub struct Resolved;

impl State for Resolved {
    type Definition = Uniq;
    type Reference = Reference;
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Reference {
    pub definition: Uniq,
    pub location: Location,
}

impl Element for Reference {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(thiserror::Error, miette::Diagnostic, Debug)]
pub enum InnerError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    UnresolvedDefinition(#[from] UnresolvedDefinition),

    #[error(transparent)]
    #[diagnostic(transparent)]
    LaterUnresolvedDefinition(#[from] LaterUnresolvedDefinition),

    #[error(transparent)]
    #[diagnostic(transparent)]
    AlreadyDefinedSignature(#[from] AlreadyDefinedSignature),
}

#[derive(thiserror::Error, miette::Diagnostic, Debug)]
#[diagnostic(code(zu::resolution_failure), url(docsrs))]
#[error("can't resolve the file")]
pub struct ResolutionFailure {
    #[source_code]
    source_code: NamedSource,

    #[related]
    related: Vec<InnerError>,
}

#[derive(thiserror::Error, miette::Diagnostic, Debug)]
#[diagnostic(
    code(zu::unresolved_definition),
    url(docsrs),
    help("maybe add an import for it?")
)]
#[error("unresolved definition: {module}")]
pub struct UnresolvedDefinition {
    pub module: String,

    #[label = "here"]
    span: SourceSpan,
}

#[derive(thiserror::Error, miette::Diagnostic, Debug)]
#[diagnostic(
    code(zu::later_unresolved_declaration),
    url(docsrs),
    help("maybe move this declaration")
)]
#[error("unresolved definition: {module} in the current scope")]
pub struct LaterUnresolvedDefinition {
    pub module: String,

    #[label = "here is the reference"]
    span: SourceSpan,

    #[label("here is the declaration")]
    declaration_span: SourceSpan,
}

#[derive(thiserror::Error, miette::Diagnostic, Debug)]
#[diagnostic(
    code(zu::already_defined_signature),
    url(docsrs),
    help("remove this declaration")
)]
#[error("unresolved already defined: {module} signature in the current file")]
pub struct AlreadyDefinedSignature {
    pub module: String,

    #[label = "here is the duplicated"]
    span: SourceSpan,

    #[label("here is the already defined signature")]
    declaration_span: SourceSpan,
}

pub struct Resolver {
    pub code: String,
    pub errors: Vec<InnerError>,
    pub uniqs: im_rc::HashMap<usize, Definition, FxBuildHasher>,
    pub scope: im_rc::HashMap<usize, Uniq, FxBuildHasher>,
    pub index: im_rc::HashMap<String, Uniq, FxBuildHasher>,
    pub uniq: usize,
    pub file_scope: Scope,
    pub main: crate::ast::File<Parsed>,
}

/// Current file scope for the resolver. For error reporting
#[derive(Default)]
pub struct Scope {
    locations: im_rc::HashMap<String, Location, FxBuildHasher>,
    all_possible_names: im_rc::HashMap<String, Rc<Definition>, FxBuildHasher>,
}

impl Resolver {
    pub fn new(file: String) -> miette::Result<Resolver> {
        let code = std::fs::read_to_string(&file)
            .into_diagnostic()
            .wrap_err_with(|| format!("can't read file `{}`", file))?;
        Ok(Resolver {
            errors: vec![],
            uniqs: im_rc::HashMap::default(),
            scope: im_rc::HashMap::default(),
            index: im_rc::HashMap::default(),
            uniq: 0,
            file_scope: Default::default(),
            main: parse_or_report(&file, &code)?,
            code,
        })
    }

    pub fn resolve_and_import(mut self) -> miette::Result<File<Resolved>> {
        let file = std::mem::take(&mut self.main);

        let decls = file
            .declarations
            .into_iter()
            .inspect(|decl| self.define(decl))
            .collect::<Vec<_>>()
            .into_iter()
            .flat_map(|decl| self.decl(decl))
            .collect::<Vec<_>>();

        if !self.errors.is_empty() {
            return Err(ResolutionFailure {
                source_code: self.get_source_code(&file.location),
                related: self.errors,
            }
            .into());
        }

        Ok(File {
            name: file.name,
            declarations: decls,
            definitions: self.uniqs.into_iter().collect(),
            location: file.location,
        })
    }

    fn define(&mut self, stmt: &Decl<Parsed>) {
        let declaration = match stmt {
            Decl::Function(declaration) => &declaration.name,
        };

        self.file_scope.all_possible_names.insert(
            declaration.text.clone(),
            Rc::new(Definition {
                location: declaration.location.clone(),
                text: declaration.text.clone(),
            }),
        );
    }

    fn resolve(&mut self, stmt: Stmt<Parsed>) -> Stmt<Resolved> {
        match stmt {
            Stmt::Error(error) => Stmt::Error(Error { ..error }),
            Stmt::Let(let_stmt) => {
                let name = let_stmt.name.text.clone();
                let value = self.term(let_stmt.value);
                let location = let_stmt.name.location.clone();
                let definition = self.create_definition(&name, &location);

                Stmt::Let(Let {
                    name: definition,
                    value,
                    location,
                })
            }
            Stmt::Return(return_stmt) => Stmt::Return(Return {
                value: return_stmt.value.map(|value| self.term(value)),
                location: return_stmt.location,
            }),
            Stmt::Term(term) => Stmt::Term(self.term(term)),
            Stmt::Function(function) => {
                let name = function.name.text.clone();
                let location = function.name.location.clone();
                let definition = self.create_definition(&name, &location);
                let mut parameters = vec![];

                for parameter in function.parameters {
                    let name = parameter.text.clone();
                    let location = parameter.location.clone();
                    let definition = self.create_definition(&name, &location);
                    parameters.push(definition);
                }

                // Dont allow redefining a binding with a binding.
                if let Some(location) = self.file_scope.locations.get(&name) {
                    self.errors.push(InnerError::AlreadyDefinedSignature(
                        AlreadyDefinedSignature {
                            module: name.clone(),
                            span: function.name.location.clone().into(),
                            declaration_span: location.clone().into(),
                        },
                    ));
                }
                self.file_scope.locations.insert(name.clone(), location);

                let doc_strings = function
                    .doc_strings
                    .into_iter()
                    .map(|doc| DocString { ..doc })
                    .collect();

                Stmt::Function(Function {
                    doc_strings,
                    parameters,
                    is_external: function.is_external,
                    name: definition,
                    location: function.location,
                    block: function.block.map(|block| self.block(block)),
                })
            }
            Stmt::If(if_stmt) => Stmt::If(If {
                condition: self.term(if_stmt.condition),
                then: self.block(if_stmt.then),
                otherwise: if_stmt.otherwise.map(|block| self.block(block)),
                location: if_stmt.location,
            }),
        }
    }

    fn term(&mut self, term: Term<Parsed>) -> Term<Resolved> {
        match term {
            Term::Error(error) => Term::Error(Error { ..error }),
            Term::Int(int) => Term::Int(Int { ..int }),
            Term::Str(str) => Term::Str(Str { ..str }),
            Term::Group(group) => self.term(*group),
            Term::Binary(binary) => Term::Binary(Binary {
                lhs: self.term(*binary.lhs).into(),
                rhs: self.term(*binary.rhs).into(),
                op: binary.op,
                location: binary.location,
            }),
            Term::Fun(fun) => self.fork(|local| {
                let parameters = fun
                    .arguments
                    .into_iter()
                    .map(|argument| local.create_definition(&argument.text, &argument.location))
                    .collect::<Vec<_>>();

                Term::Fun(Fun {
                    arguments: parameters,
                    value: local.block(fun.value),
                    location: fun.location,
                })
            }),
            Term::Call(apply) => Term::Call(Call {
                callee: self.term(*apply.callee).into(),
                arguments: apply
                    .arguments
                    .into_iter()
                    .map(|argument| self.term(argument))
                    .collect(),
                location: apply.location,
            }),
            Term::Reference(reference) => self
                .find_reference(reference.clone())
                .map(|definition| {
                    Term::Reference(Reference {
                        definition,
                        location: reference.location.clone(),
                    })
                })
                .unwrap_or_else(|| {
                    Term::Error(Error {
                        message: format!("unresolved {}", reference.text),
                        full_text: "todo".into(),
                        location: reference.location,
                    })
                }),
        }
    }

    fn decl(&mut self, decl: Decl<Parsed>) -> Vec<Decl<Resolved>> {
        vec![match decl {
            Decl::Function(function) => {
                let name = function.name.text.clone();
                let location = function.name.location.clone();
                let definition = self.create_definition(&name, &location);
                let mut parameters = vec![];

                for parameter in function.parameters {
                    let name = parameter.text.clone();
                    let location = parameter.location.clone();
                    let definition = self.create_definition(&name, &location);
                    parameters.push(definition);
                }

                if let Some(location) = self.file_scope.locations.get(&name) {
                    self.errors.push(InnerError::AlreadyDefinedSignature(
                        AlreadyDefinedSignature {
                            module: name.clone(),
                            span: function.name.location.clone().into(),
                            declaration_span: location.clone().into(),
                        },
                    ));
                }
                self.file_scope.locations.insert(name.clone(), location);

                let doc_strings = function
                    .doc_strings
                    .into_iter()
                    .map(|doc| DocString { ..doc })
                    .collect();

                Decl::Function(Function {
                    doc_strings,
                    parameters,
                    is_external: function.is_external,
                    name: definition,
                    location: function.location,
                    block: function.block.map(|block| self.block(block)),
                })
            }
        }]
    }

    fn block(&mut self, block: Block<Parsed>) -> Block<Resolved> {
        self.fork(|local| {
            let mut statements = vec![];

            for stmt in block.statements {
                statements.push(local.resolve(stmt));
            }

            Block {
                statements,
                location: block.location,
            }
        })
    }

    fn create_definition(&mut self, name: &str, location: &Location) -> Uniq {
        let id = self.uniq;
        self.uniq += 1;
        let uniq = Uniq { value: id };

        self.scope.insert(id, uniq.clone());
        self.index.insert(name.to_string(), uniq.clone());
        self.uniqs.insert(id, Definition {
            text: name.to_string(),
            location: location.clone(),
        });

        uniq
    }

    // Find a reference and returns the definition. If it cant be found,
    // it will report an error.
    fn find_reference(&mut self, reference: crate::passes::parser::Reference) -> Option<Uniq> {
        match self.index.get(&reference.text) {
            Some(value) => value.clone().into(),
            None => {
                let is_later_defined = self.file_scope.all_possible_names.get(&reference.text);

                if let Some(is_later_defined) = is_later_defined {
                    // If the definition is later defined, it will report
                    // a possible definition.
                    self.report_possible_definition(&reference, is_later_defined.clone());
                } else {
                    // If can't find the definition, it will fallback to a hole.
                    self.report_unresolved(&reference);
                }

                None
            }
        }
    }

    /// Reports a possible definition for a reference.
    fn report_possible_definition(
        &mut self,
        reference: &crate::passes::parser::Reference,
        definition: Rc<Definition>,
    ) {
        self.errors.push(InnerError::LaterUnresolvedDefinition(
            LaterUnresolvedDefinition {
                module: reference.text.clone(),
                span: reference.location.clone().into(),
                declaration_span: definition.location.clone().into(),
            },
        ))
    }

    /// Reports an error for a reference.
    fn report_unresolved(&mut self, reference: &crate::passes::parser::Reference) {
        self.errors
            .push(InnerError::UnresolvedDefinition(UnresolvedDefinition {
                module: reference.text.clone(),
                span: reference.location.clone().into(),
            }))
    }

    fn get_source_code(&self, location: &Location) -> NamedSource {
        NamedSource::new(&location.filename, self.code.clone())
    }

    /// Creates a new fork of the current scope, with a new
    /// scope.
    fn fork<U, F: FnOnce(&mut Self) -> U>(&mut self, f: F) -> U {
        let new_scope = self.scope.clone();
        let scope = std::mem::replace(&mut self.scope, new_scope);
        let value = f(self);
        self.scope = scope;
        value
    }
}
