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
      └── 1
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

`If` é uma estrutura que representa um bloco if/else dentro da linguagem. Ele é usado para tomar decisões com base em uma condição. O formato da estrutura é semelhante ao seguinte exemplo:

```javascript
if true { a } else { b }
```

| Nome       | Tipo     |
| --------   | -------- |
| kind       | String   |
| condition  | Term     |
| then       | Term     |
| otherwise  | Term     |
| location   | Location |

### Let

`Let` é uma estrutura que representa um `let in`, ou seja, além de ela conter um let, ela especifica a proxima estrutura

| Nome       | Tipo     |
| --------   | -------- |
| kind       | String   |
| name       | Var      |
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

`Int` é uma estrutura que representa um literal de número inteiro signed que tem tamanho de 32 bits, ou seza um Int32. Ela é representada por:

| Nome       | Tipo     |
| --------   | -------- |
| kind       | String   |
| value      | Number   |
| location   | Location |

### BinaryOp (Operador Binário)

Um `BinaryOp` é um enumerador que representa uma operação binária. Essas são as variantes disponiveis

| Nome       | Descrição        |
| --------   | ------------     |
| Add        | Soma             |
| Sub        | Subtração        |
| Mul        | Multiplicação    |
| Div        | Divisão          |
| Rem        | Resto da divisão |
| Eq         | Igualdade        |
| Neq        | Diferente        |
| Lt         | Menor            |
| Gt         | Maior            |
| Lte        | Menor ou igual   |
| Gte        | Maior ou igual   |
| And        | Conjunção        |
| Or         | Disjunção        |
| Not        | Negação          |

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
| parameters  | [Var]    |
| value       | Term     |
| location    | Location |

### Print (Função de printar para o standard output)

`Print` é a chamada da função de printar para o standard output. Ela é definida por:

| Nome        | Tipo     |
| --------    | -------- |
| kind        | String   |
| value       | Term     |
| location    | Location |

### First (Função de pegar o primeiro elemento de uma tupla)

`First` é uma chamada de função que pega o primeiro elemento de uma tupla. Ela é definida por:

| Nome        | Tipo     |
| --------    | -------- |
| kind        | String   |
| value       | Term     |
| location    | Location |


### Second (Função de pegar o segundo elemento de uma tupla)

`Second` é uma chamada de função que pega o segundo elemento de uma tupla. Ela é definida por:

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
