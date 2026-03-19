
Por padrão, as variaveis são imutaveis. Porém, voce ainda pode tornar suas variaveis mutaveis. Mas, Rust te encoraja a favorecer a imutabilidade.

Quando uma variavel é imutavel, uma vez que um valor está associado a um nome, voce nao pode mudar aquele valor.

## Constants
Como variaveis imutaveis, `constants` sao valores que sao valores que quando associados a um nome, nao podem mudar. Mas existem algumas diferencas entre constantes e variaveis.

Primeiro, não é possível usar `mut` com constantes. Constantes não são apenas imutaveis por padrao, elas são **sempre** imutáveis. Elas sao declaradas usando a keyword `const` ao inves da `let`, e o **tipo** do valor deve **sempre** estar anotado.

Constantes podem ser declaradas em qualquer escopo, incluindo o escopo global, o que fazem delas úteis para valores que diversas partes do código irão usar.

A última diferença é que constantes podem ser feitas apenas com uma expressão constante, não com o resultado de um valor que poderia ser computado em runtime.

Exemplo:
```rust
const THREE_HOURS_IN_SECONDS : u32 = 60*60*3;
```

O padrão do rust para constantes é sempre usar uppercase com underscore entre as palavras.

## Shadowing

É possível declarar uma nova variável coom o mesmo nome que uma variável anterior. A esse feito, falam que a primeira variável é `shadowed` pela segunda, o que significa que a segunda variável é o que o compilador irá ver quando voce usar o nome da variável.

Na prática, a segunda variavel overshadows a primeira, tomando quaisquer usos do nome da variabel para ela até que seja shadowed por outra ou o escopo acabe. Exemplo:

```rust
let x = 5;

let x = x + 1;

{
	let x = x + 2;
	println!("The value of x in the inner scope is: {x}");
}

println!("The value of x is: {x}");
```

## Shadowing != mutabilidade

Shadowing é dferente de marcar uma variável como mut, porque teremos um erro de compilação se acidentalmente tentarmos reatribuir um valor a aquela variavel sem usar o `let`. Ao usar o `let` (shadowing) podemos fazer algumas transformacoes no valor mas tendo a variavel como imutavel apos essas transformacoes terminarem.

Outra diferenca é que com o shadowing estamos efetivamente criando uma nova variavel ao usar o `let` novamente, podemos mudar o tipo do valor mas reusar o mesmo nome.

Se tentassemos fazer o mesmo com mut, haveria um erro de compilacao, pois nao é permitido mutar o tipo de uma variavel.


