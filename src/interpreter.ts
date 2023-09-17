import { Term, If, Let, Str, Bool, Int, BinaryOp, Binary, Call, Function, Print, First, Second, Tuple, Var, MyFile } from './ast_types';


type Env = Record<string, any>;

interface Closure {
  body: Term;
  parameters: string[];
  env: Env;
}

interface BaseValue {
  kind: string;
}

interface StringValue extends BaseValue {
  value: string;
}

interface BooleanValue extends BaseValue {
  value: boolean;
}

interface IntValue extends BaseValue {
  value: number;
}

interface ClosureValue extends BaseValue {
  value: Closure;
}

interface TupleValue extends BaseValue {
    first: Value;
    second: Value;
}

type Value = TupleValue | ClosureValue | BooleanValue | IntValue | StringValue;


function interpret_int(term: Int, _): IntValue {
  return { kind: 'int', value: term.value };
}

function interpret_str(term: Str, _): StringValue {
  return { kind: 'string', value: term.value };
}

function interpret_bool(term: Bool, _): BooleanValue {
  return { kind: 'boolean', value: term.value };
}


function interpret_if(term: If, env: Env): Value {
    let condition = interpret(term.condition, env);
    if (isBooleanValue(condition) && condition.value) {
        return interpret(term.then, env);
    } else {
        return interpret(term.otherwise, env);
    }
}

function isBooleanValue(value: Value): value is BooleanValue {
    return value.kind === "boolean";
}

function interpret_tuple(term: Tuple, env: Env): TupleValue {
    let first = interpret(term.first, env);
    let second = interpret(term.second, env);
    return { kind: 'tuple', first, second };
}


function interpret_first(term: First, env: Env): Value {
    let tupleValue = interpret(term, env);

    if (isTupleValue(tupleValue)) {
        return tupleValue.first;
    } else {
        throw new Error('Expected a tuple');
    }
}

function interpret_second(term: Second, env: Env): Value {
    let tupleValue = interpret(term, env);

    if (isTupleValue(tupleValue)) {
        return tupleValue.second;
    } else {
        throw new Error('Expected a tuple');
    }
}

function isTupleValue(value: Value): value is TupleValue {
    return value.kind === 'tuple';
}

function convertValueToPrint(value: Value): string {
    if (value.kind === 'boolean') {
        if ((value as BooleanValue).value) {
            return 'true';
        }
        return 'false';
    } else if (value.kind === 'int') {
        return (value as IntValue).value.toString();
    } else if (value.kind === 'string') {
        return (value as StringValue).value;
    } else if (value.kind === 'tuple') {
        let first = (value as TupleValue).first as Value;
        let second = (value as TupleValue).second as Value;
        return `(${convertValueToPrint(first)}, ${convertValueToPrint(second)})`;
    } else if (value.kind === 'closure') {
        return '<closure>'; // Handle closure values as needed
    }
    return ''; // Handle other Value types or return an empty string
}

function interpret_print(term: Print, env: Env): Value {
    let result = interpret(term.value, env)
    console.log(convertValueToPrint(result));
    return result;
}

function interpret_var(term: Var, env: Env): Value {
    let val = env[term.text];
    if (val === undefined) {
        throw new Error(`Undefined variable ${term.text}`);
    }
    return val;
}

function interpret_let(term: Let, env: Env): Value {
    let newEnv = { ...env };
    let value = interpret(term.value, env);
    newEnv[term.name.text] = value;
    return interpret(term.next, newEnv);
}

function assertClosure(value: Value): ClosureValue {
    if (value.kind !== 'closure') {
        throw new Error('Expected a closure');
    }
    return value as unknown as ClosureValue;
}

function interpret_call(term: Call, env: Env) : Value {
    let func = interpret(term.callee, env);
    let closure = assertClosure(func);
    if(closure.value.parameters.length != term.arguments.length) {
        throw new Error('Wrong number of arguments');
    }

    let funEnv = {...env};
    let params = closure.value.parameters;
    for (let i = 0; i < params.length; i++) {
        let param = params[i];
        let arg = interpret(term.arguments[i], env);
        funEnv[param] = arg;
    }
    return interpret(closure.value.body, funEnv);
}

function addValues(left: Value, right: Value): Value {
    let types = ['closure', 'tuple', 'boolean'];
    if (types.includes(left.kind) || types.includes(right.kind)) {
        throw new Error(`Cannot add ${left.kind} and ${right.kind}`);
    }
    if (left.kind === "string" && right.kind === "string") {
        return { kind: 'string', value: (left as StringValue).value + (right as StringValue).value };
    }
    else if (left.kind == "int" && right.kind == "int") {
        return { kind: 'int', value: (left as IntValue).value + (right as IntValue).value };
    }
    else {
        return { kind: "string", value : (left as StringValue).value.toString() + (right as StringValue).value.toString() };
    }
}

type BinaryOperator = (l: any, r: any) => boolean | number;

const binary_op_dict: Record<string, BinaryOperator> = {
    'Sub': (l, r) => l - r,
    'Mul': (l, r) => l * r,
    'Div': (l, r) => l / r, // Use `/` for division, as `//` doesn't exist in JavaScript
    'Rem': (l, r) => l % r,
    'Eq': (l, r) => l === r,
    'Neq': (l, r) => l !== r,
    'Lt': (l, r) => l < r,
    'Gt': (l, r) => l > r,
    'Lte': (l, r) => l <= r,
    'Gte': (l, r) => l >= r,
    'And': (l, r) => l && r,
    'Or': (l, r) => l || r,
};

function interpret_binary_op(left: Value, right: Value, op: BinaryOp): Value {
    if (op == 'Add') {
        return addValues(left, right);
    }

    if (left.kind !== right.kind) {
        throw new Error(`Invalid operator ${op} for arguments with types ${left.kind} and ${right.kind}`);
    }

    if (['Sub', 'Mul', 'Div', 'Rem'].includes(op) && left.kind !== 'int') {
        throw new Error(`Invalid operator ${op} for arguments with types ${left.kind} and ${right.kind}`);
    }

    if (['Eq', 'Neq'].includes(op) && !['string', 'int'].includes(left.kind)) {
        throw new Error(`Invalid operator ${op} for arguments with types ${left.kind} and ${right.kind}`);
    }

    if (['Lt', 'Gt', 'Lte', 'Gte'].includes(op) && left.kind !== 'int') {
        throw new Error(`Invalid operator ${op} for arguments with types ${left.kind} and ${right.kind}`);
    }

    if (['And', 'Or'].includes(op) && left.kind !== 'boolean') {
        throw new Error(`Invalid operator ${op} for arguments with types ${left.kind} and ${right.kind}`);
    }

    const operation = binary_op_dict[op];

    if (operation) {
        let resultValue: any = null; // Default value

        if (left.kind === right.kind) {
            switch (left.kind) {
                case 'int':
                    resultValue = operation((left as IntValue).value as number, (right as IntValue).value as number);
                    break;
                case 'boolean':
                    resultValue = operation((left as BooleanValue).value as boolean, (right as BooleanValue).value as boolean);
                    break;
                case 'string':
                    resultValue = operation((left as StringValue).value as string, (right as StringValue).value as string);
                    break;
                default:
                    throw new Error(`Unsupported types for operation: ${left.kind}`);
            }

            let kind: string = '';

            if (typeof resultValue === 'number') {
                kind = 'int';
            } else if (typeof resultValue === 'boolean') {
                kind = 'boolean';
            } else if (typeof resultValue === 'string') {
                kind = 'string';
            }

            return {
                kind,
                value: resultValue,
            };
        } else {
            throw new Error(`Invalid operator ${op} for arguments with types ${left.kind} and ${right.kind}`);
        }
    } else {
        throw new Error(`Operator ${op} is not supported`);
    }
}

function interpret_binary(term : Binary, env : Env) : Value {
    let left = interpret(term.lhs, env);
    let right = interpret(term.rhs, env);
    return interpret_binary_op(left, right, term.op);
}

function interpret_function(term: Function, env: Env): Value {
    return {
        kind: 'closure',
        value: {
            body: term.value,
            parameters: term.parameters.map(p => p.text),
            env: env
        }
    };
}



let interpreter_dict = {
    'Int': interpret_int,
    'Str': interpret_str,
    'Call': interpret_call,
    'Binary': interpret_binary,
    'Function': interpret_function,
    'Let': interpret_let,
    'If': interpret_if,
    'Print': interpret_print,
    'First': interpret_first,
    'Second': interpret_second,
    'Bool': interpret_bool,
    'Tuple': interpret_tuple,
    'Var': interpret_var
}

function interpret(term: Term, env: Env): Value {
    let kind = term['kind'] as keyof typeof interpreter_dict;
    let func = interpreter_dict[kind];
    return func(term as any, env);
}

export class Interpreter {
    file: MyFile;
    constructor(file : MyFile) {
        this.file = file;
    }

    run() {
        let env = {};
        return interpret(this.file.expression, env);
    }
}
    
