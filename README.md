<div align="center">

![banner]

</div>

# Introdução

O ideal da rinha é fazer um interpretador ou compilador que rode em uma maquina com 2 núcleos e 2G de RAM.

O seu interpretador ou compilador deve trabalhar com algo chamado "árvore sintática abstrata" que está armazenada no formato JSON. Essa árvore sintática abstrata será gerada por nós usando uma ferramenta específica disponível neste repositório.

Sua responsabilidade na tarefa é receber esse JSON que contém a árvore abstrata e, em seguida, interpretar ou compilar o programa de acordo com as informações fornecidas na árvore abstrata.

Simplificando:

1. Nós te damos um JSON com uma árvore dentro
2. Voce roda o JSON
3. Voce fica feliz que apareceu o resultado.

## Para executar

Depois será adicionado como executar

## Como testar

Para testar você pode usar o arquivo `files/fib.rinha` e gerar com o programa que disponibilizamos
aqui para um JSON ou você pode usar diretamente o JSON que está em `files/fib.json`.

Durante a rinha nós iremos adicionar outros testes :)

## Requisitos

Você tem que fazer um PR, alterando o arquivo [PARTICIPANTS.md](PARTICIPANTS.md),
com uma nova linha e seu repositório. Talvez isso seja mudado depois (fique atento).

Seu repositório terá que ter uma imagem no root do repositório, e buildaremos a imagem
no rankeamento.

## Especificação

A linguagem terá que rodar com base em algum arquivo, que é o JSON da AST da
rinha especificado [aqui](https://github.com/aripiprazole/rinha-de-compiler/blob/main/SPECS.md).

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
let fib = fn (n) => {
  if (n < 2) {
    n
  } else {
    fib(n - 1) + fib(n - 2)
  }
};

print("fib: " + fib(10))
```

# Competição

O prazo para mandar os PRs, é até o dia 23/09, depois disso serão negados o
projeto.

Será liberado para ajustes até o dia 25/09, você poderá arrumar sua implementação,
depois da publicação dos testes.

## Recursos

Alguns recursos úteis para aprender como fazer seu próprio interpretador ou compilador são:

- https://www.youtube.com/watch?v=t77ThZNCJGY
- https://www.youtube.com/watch?v=LCslqgM48D4
- https://ruslanspivak.com/lsbasi-part1/
- https://www.youtube.com/playlist?list=PLjcmNukBom6--0we1zrpoUE2GuRD-Me6W
- https://www.plai.org/

Fique ligado que alguns vídeos e posts úteis chegarão em breve.

[banner]: ./img/banner.png
