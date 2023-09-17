export type TermKind =
  | 'Int'
  | 'Str'
  | 'Call'
  | 'Binary'
  | 'Function'
  | 'Let'
  | 'If'
  | 'Print'
  | 'First'
  | 'Second'
  | 'Bool'
  | 'Tuple'
  | 'Var';

export interface Term {
  kind: TermKind;
}

export interface Loc {
  start: number;
  end: number;
  filename: string;
}

export interface Parameter {
  text: string;
  location: Loc;
}

export interface MyFile {
  filename: string;
  expression: Term;
  location: Loc;
}

export interface If extends Term {
  kind: 'If';
  condition: Term;
  then: Term;
  otherwise: Term;
  location: Loc;
}

export interface Let extends Term {
  kind: 'Let';
  name: Parameter;
  value: Term;
  next: Term;
  location: Loc;
}

export interface Str extends Term {
  kind: 'Str';
  value: string;
  location: Loc;
}

export interface Bool extends Term {
  kind: 'Bool';
  value: boolean;
  location: Loc;
}

export interface Int extends Term {
  kind: 'Int';
  value: number;
  location: Loc;
}

export type BinaryOp =
  | 'Add'
  | 'Sub'
  | 'Mul'
  | 'Div'
  | 'Rem'
  | 'Eq'
  | 'Neq'
  | 'Lt'
  | 'Gt'
  | 'Lte'
  | 'Gte'
  | 'And'
  | 'Or';

export interface Binary extends Term {
  kind: 'Binary';
  lhs: Term;
  op: BinaryOp;
  rhs: Term;
  location: Loc;
}

export interface Call extends Term {
  kind: 'Call';
  callee: Term;
  arguments: Term[];
  location: Loc;
}

export interface Function extends Term {
  kind: 'Function';
  parameters: Parameter[];
  value: Term;
  location: Loc;
}

export interface Print extends Term {
  kind: 'Print';
  value: Term;
  location: Loc;
}

export interface First extends Term {
  kind: 'First';
  value: Term;
  location: Loc;
}

export interface Second extends Term {
  kind: 'Second';
  value: Term;
  location: Loc;
}

export interface Tuple extends Term {
  kind: 'Tuple';
  first: Term;
  second: Term;
  location: Loc;
}

export interface Var extends Term {
  kind: 'Var';
  text: string;
  location: Loc;
}
