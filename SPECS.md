# Especificação

Essa é a especificação da arvore sintática abstrata. Uma árvore sintática abstrata é uma estrutura
feita para ser lida por um computador que expressa um programa. Por exemplo, se você tem um programa que diz
"Some dois números e mostre o resultado", a Árvore Sintática Abstrata irá mostrar que há uma ação
principal (somar dois números e mostrar o resultado) e que essa ação é composta por duas partes
(somar e mostrar). Isso torna mais fácil para o computador entender e executar o programa corretamente.

Uma representação da árvore abstrata de `1 + 2` seria:

```
└ Add
  ├── Literal
  │   └── 1
  └── Literal
      └── 2
```

Ou em JSON da linguagem Rinha

```
{
  "name": "ata.rinha",
  "expression": {
    "kind": "Binary",
    "lhs": {
      "kind": "Int",
      "value": 1,
      "location": ..
    },
    "op": "Add",
    "rhs": {
      "kind": "Int",
      "value": 2,
      "location": ..
    },
    "location": ..
  },
  "location": ..
}
```

Onde `..` é um location node que foi ocultado por brevidade.

## Nodes

### File

`File` é uma estrutura que tem dados do arquivo inteiro e que contém os seguintes campos:

| Nome       | Tipo    |
| --------   | ------- |
| name       | String  |
| expression | Term    |
| location   | Loc     |

### Loc

`Loc` é uma estrutura que contém campos para localização de um pedaço da árvore dentro do código fonte

| Nome       | Tipo    |
| --------   | ------- |
| start      | Int     |
| end        | Int     |
| filename   | String  |

### If

`If` é uma estrutura que representa um bloco if/else dentro da linguagem. Ele é usado para tomar decisões com base em uma condição e sempre retorna um valor, é como se fosse um ternário de JS. O formato da estrutura é semelhante ao seguinte exemplo:

A condição do if deve ser sempre um boolean.

```javascript
if (true) { a } else { b }
```

| Nome       | Tipo     |
| --------   | -------- |
| kind       | String   |
| condition  | Term     |
| then       | Term     |
| otherwise  | Term     |
| location   | Location |

### Let

`Let` é uma estrutura que representa um `let in`, ou seja, além de ela conter um let, ela especifica a proxima estrutura. Todo let pode fazer *shadowing*, ou seja, usar o mesmo nome de outra variável e "ocultar" o valor da variável antiga. 

| Nome       | Tipo     |
| --------   | -------- |
| kind       | String   |
| name       | Parameter      |
| value      | Term     |
| next       | Term     |
| location   | Location |

### Str (Texto)

`Str` é uma estrutura que representa um literal de texto. Ela é representada por:

| Nome       | Tipo     |
| --------   | -------- |
| kind       | String   |
| value      | String   |
| location   | Location |

### Bool (Booleano)

`Bool` é uma estrutura que representa um literal de boolean. Ela é representada por:

| Nome       | Tipo     |
| --------   | -------- |
| kind       | String   |
| value      | Bool     |
| location   | Location |

### Int (Inteiro)

`Int` é uma estrutura que representa um literal de número inteiro signed que tem tamanho de 32 bits, ou seja um Int32. Ela é representada por:

| Nome       | Tipo     |
| --------   | -------- |
| kind       | String   |
| value      | Number   |
| location   | Location |

### BinaryOp (Operador Binário)

Um `BinaryOp` é um enumerador que representa uma operação binária. Essas são as variantes disponiveis

| Nome       | Descrição        | Exemplos que devem ser válidos                    |
| --------   | ------------     | ------------------------------------------------- |
| Add        | Soma             | `3 + 5 = 8`, `"a" + 2 = "a2"`, `2 + "a" = "2a"`, `"a" + "b" = "ab"` |
| Sub        | Subtração        | `0 - 1 = -1` |
| Mul        | Multiplicação    | `2 * 2 = 4`  |
| Div        | Divisão          | `3 / 2 = 1`  |
| Rem        | Resto da divisão | `4 % 2 = 0`  |
| Eq         | Igualdade        | `"a" == "a"`, `2 == 1 + 1`, `true == true` |
| Neq        | Diferente        | `"a" != "b"`, `3 != 1 + 1`, `true != false` |
| Lt         | Menor            | `1 < 2` |
| Gt         | Maior            | `2 > 3` |
| Lte        | Menor ou igual   | `1 <= 2` | 
| Gte        | Maior ou igual   | `1 >= 2` |
| And        | Conjunção        | `true && false` |
| Or         | Disjunção        | `false \|\| true` |

### Binary (Operação Binária)

`Binary` é uma operação binária entre dois termos sendo representada por:

| Nome        | Tipo     |
| --------    | -------- |
| kind        | String   |
| lhs         | Term     |
| op          | BinaryOp |
| rhs         | Term     |
| location    | Location |



### Call (Aplicação de função)

`Call` é uma aplicação de funçao entre um termo e varios outros termos chamados de argumentos. Essa estrutura é representada por:

| Nome        | Tipo     |
| --------    | -------- |
| kind        | String   |
| callee      | Term     |
| arguments   | [Term]   |
| location    | Location |

### Function (Função anônima)

`Function` é a criação de uma função anônima que pode capturar o ambiente, ela é representada por:

| Nome        | Tipo     |
| --------    | -------- |
| kind        | String   |
| parameters  | [Parameter]    |
| value       | Term     |
| location    | Location |

### Print (Função de printar para o standard output)

`Print` é a chamada da função de printar para o standard output. Ela é definida por:

Exemplos que devem ser válidos: `print(a)`, `print("a")`, `print(2)`, `print(true)`, `print((1, 2))`

| Nome        | Tipo     |
| --------    | -------- |
| kind        | String   |
| value       | Term     |
| location    | Location |

Os valores devem ser printados como: 

| Tipo    | Como deve ser printado |
| -----   | ---------------------- |
| String  | a string sem aspas duplas ex `a` |
| Number  | o literal de número ex `0` |
| Boolean | `true` ou `false` |
| Closure | `<#closure>`    |
| Tuple   | `(term, term)`  |

### First (Função de pegar o primeiro elemento de uma tupla)

`First` é uma chamada de função que pega o primeiro elemento de uma tupla. Ela é definida por:

```
first((1, 2))
```

| Nome        | Tipo     |
| --------    | -------- |
| kind        | String   |
| value       | Term     |
| location    | Location |

Quando o first for chamado com algo que não é uma tupla ele deve dar um erro de runtime.

### Second (Função de pegar o segundo elemento de uma tupla)

`Second` é uma chamada de função que pega o segundo elemento de uma tupla. Ela é definida por:

```
second((1, 2))
```

| Nome        | Tipo     |
| --------    | -------- |
| kind        | String   |
| value       | Term     |
| location    | Location |

### Tuple (Criação de uma 2-Tuple)

`Tuple` é um elemento que descreve a criação de uma tupla com a sintaxe:

```
(x, y)
```

Ela tem os seguintes elementos:

| Nome        | Tipo     |
| --------    | -------- |
| kind        | String   |
| first       | Term     |
| second      | Term     |
| location    | Location |

Quando o second for chamado com algo que não é uma tupla ele deve dar um erro de runtime.

### Parameter

`Parameter` representa o nome de uma parâmetro. É definida por:

| Nome        | Tipo     |
| --------    | -------- |
| text        | String   |
| location    | Location |

### Var (Nome de uma variável)

`Var` representa o nome de uma variável. É definida por:

| Nome        | Tipo     |
| --------    | -------- |
| kind        | String   |
| text        | String   |
| location    | Location | 

### Term

Um termo pode ser qualquer uma das seguintes estruturas:

- Int
- Str
- Call
- Binary
- Function
- Let
- If
- Print
- First
- Second
- Bool
- Tuple
- Var
