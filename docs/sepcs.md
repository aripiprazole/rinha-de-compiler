## Estruturas Principais

### 1. File

- `name`: Nome do arquivo.
- `expression`: A expressão principal do arquivo.
- `location`: Informações de localização no código fonte.

### 2. Loc

- `start`: Posição de início.
- `end`: Posição de fim.
- `filename`: Nome do arquivo.

## Tipos Literais

### 3. Str (Texto)

- `value`: Valor literal de texto.

### 4. Bool (Booleano)

- `value`: Valor booleano.

### 5. Int (Inteiro)

- `value`: Valor numérico inteiro.

## Operadores Binários

### 6. BinaryOp (Operador Binário)

- Enumerador que representa operações binárias, incluindo adição, subtração, multiplicação, divisão, etc.

## Operações Binárias

### 7. Binary (Operação Binária)

- `lhs`: O termo à esquerda da operação.
- `op`: O operador binário.
- `rhs`: O termo à direita da operação.

## Estruturas de Controle

### 8. If

- `condition`: A condição do bloco if/else.
- `then`: A expressão a ser executada se a condição for verdadeira.
- `otherwise`: A expressão a ser executada se a condição for falsa.

### 9. Let

- `name`: Nome do parâmetro.
- `value`: O valor associado ao parâmetro.
- `next`: A próxima estrutura após o `let`.

## Chamadas de Funções

### 10. Call (Aplicação de função)

- `callee`: A função a ser chamada.
- `arguments`: Lista de argumentos da função.

### 11. Function (Função anônima)

- `parameters`: Lista de parâmetros da função.
- `value`: O corpo da função.

## Funções Integradas

### 12. Print (Função de print)

- `value`: O valor a ser impresso.

### 13. First (Função de pegar o primeiro elemento de uma tupla)

- `value`: A tupla da qual o primeiro elemento será obtido.

### 14. Second (Função de pegar o segundo elemento de uma tupla)

- `value`: A tupla da qual o segundo elemento será obtido.

## Estruturas Compostas

### 15. Tuple (Criação de uma 2-Tuple)

- `first`: O primeiro elemento da tupla.
- `second`: O segundo elemento da tupla.

## Outras Estruturas

### 16. Parameter

- `text`: O nome do parâmetro.

### 17. Var (Nome de uma variável)

- `text`: O nome da variável.

## Term

Qualquer termo pode ser um dos seguintes:

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

Esta organização deve ajudá-lo a entender melhor a estrutura da AST da linguagem Rinha para implementar um compilador ou interpretador.
