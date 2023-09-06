# Rinha de Compilador

Ou melhor rinha de interpretadores. O ideal da rinha de compiladores é fazer um
interpretador que rode em condições de um Ubuntu x86, com 2 núcleos e 2G de RAM.

## Specs

A linguagem terá que rodar com base em algum arquivo, que poderá ser um arquivo
JSON, ou a linguagem diretamente, seu programa deve ler o arquivo, e específicar
se a linguagem usa o arquivo JSON ou a linguagem diretamente. Ambos os exemplos
podem ser encontrados no root desse projeto: [Main.rinha](Main.rinha), e
[e.json](e.json).

A linguagem é uma linguagem de programação dinâmica, como JavaScript, Ruby, etc.

O projeto da rinha de compilador, tem um "interpretador" do json, que retorna
um json, e o código terá que ser testado de diferentes formas, como outros
algorítimos além de Fibonacci.

## Exemplo

Exemplo com fibonacci

```javascript
function fib(n) {
  if (n < 2) {
    n
  } else {
    fib(n - 1) + fib(n - 2)
  }
};

print("fib: " ++ fib(100000))
```

# Competição

O prazo para mandar os PRs, é até o dia 24/07, depois disso serão negados o
projeto