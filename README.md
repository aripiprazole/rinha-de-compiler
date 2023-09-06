# Rinha de Compilador

Ou melhor rinha de interpretadores. O ideal da rinha de compiladores é fazer um
interpretador que rode em condições de um Ubuntu x86, com 2 núcleos e 2G de RAM.

## Para executar

Depois será adicionado como executar

## Requisitos

Você tem que fazer um PR, alterando o arquivo [participantes.txt](participantes.txt),
com uma nova linha e seu repositório. Talvez isso seja mudado depois (fique atento).

Seu repositório terá que ter uma imagem no root do repositório, e buildaremos a imagem
no rankeamento.

## Specs

A linguagem terá que rodar com base em algum arquivo, que é o JSON da AST da
rinha, ou a própria linguagem.

1. O arquivo terá que ser lido de `/var/rinha/source.rinha.json`
2. Poderá também ser lido de `/var/rinha/source.rinha`, se você quiser ler a AST
na mão.

A linguagem é uma linguagem de programação dinâmica, como JavaScript, Ruby, etc.

O projeto da rinha de compilador, tem um "interpretador" do json, que retorna
um AST, e o código terá que ser testado de diferentes formas, como outros
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

O prazo para mandar os PRs, é até o dia 23/09, depois disso serão negados o
projeto.

Será liberado para ajustes até o dia 25/09, você poderá arrumar sua implementação,
depois da publicação dos testes.
