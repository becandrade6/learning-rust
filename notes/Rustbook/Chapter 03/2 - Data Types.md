Todo valor no Rust é de um determinado *data type*, o que conta para o Rust qual tipo de dado está sendo especificado, para que ele saiba como trabalhar com aquele dado. Olharemos para dois subsets de data types: *scalar* e *compound*

**Rust é uma linguagem staticamente tipada.** Ou seja, ela tem que saber os tipos de todas as variaveis no tempo de compilação.

O compilador consegue usualmente inferir o tipo que queremos usar, baseado no valor e como o usamos. Em casos que multiplos tipos sao possiveis, por exemplo quando convertemos uma String para um tipo numerico usando parse, devemos adicionar um type annotation:

```Rust
let guess : u32 = "42".parse().expect("Not a number!");
```

Se não adicionassemos o u32 type annotation, o compilador iria mostrar um erro que precisa de mais informacoes para trabalhar com a variavel.
## Scalar Types

Um tipo *scalar* representa um único valor. Temos 4 tipos primários: *integers*, *floating-point numbers*, *Booleans* e *characters*.

### Integer Types

Um *integer* é um número sem o componente fracional. Os tipos são:

| Length                 | Signed  | Unsigned |
| ---------------------- | ------- | -------- |
| 8-bit                  | `i8`    | `u8`     |
| 16-bit                 | `i16`   | `u16`    |
| 32-bit                 | `i32`   | `u32`    |
| 64-bit                 | `i64`   | `u64`    |
| 128-bit                | `i128`  | `u128`   |
| Architecture-dependent | `isize` | `usize`  |
Cada variante pode ser *signed* ou *unsigned* e tem um tamanho explícito. *Signed* e *unsigned* se referem a se é possível o número ser negativo. Em outras palavras, se o número precisa ter um sinal com ele (positivo ou negativo) ou se ele será apenas positivo sempre e portanto pode ser representado sem um sinal.

Números *signed* são armazenados usando a representação por [complemento de dois](https://en.wikipedia.org/wiki/Two%27s_complement) .

Complemento de dois é uma forma de representar números inteiros com sinal em binário, onde o bit mais significativo indica o sinal.  
Números negativos são obtidos invertendo os bits do valor positivo e somando 1, facilitando operações aritméticas.

Cada variante *signed* pode armazenar valores que vão de $−(2n − 1)$ a $2ˆ{(n − 1)} − 1$ onde n é o numero de bits que a variante usa.

Variantes *unsigned* podem armazenar valores de 0 a $2ˆn-1$

You can write integer literals in any of the forms shown in Table 3-2. Note that number literals that can be multiple numeric types allow a type suffix, such as `57u8`, to designate the type. Number literals can also use `_` as a visual separator to make the number easier to read, such as `1_000`, which will have the same value as if you had specified `1000`.

Table 3-2: Integer Literals in Rust

|Number literals|Example|
|---|---|
|Decimal|`98_222`|
|Hex|`0xff`|
|Octal|`0o77`|
|Binary|`0b1111_0000`|
|Byte (`u8` only)|`b'A'`|

So how do you know which type of integer to use? If you’re unsure, Rust’s defaults are generally good places to start: Integer types default to `i32`. The primary situation in which you’d use `isize` or `usize` is when indexing some sort of collection.

To explicitly handle the possibility of overflow, you can use these families of methods provided by the standard library for primitive numeric types:

- Wrap in all modes with the `wrapping_*` methods, such as `wrapping_add`.
- Return the `None` value if there is overflow with the `checked_*` methods.
- Return the value and a Boolean indicating whether there was overflow with the `overflowing_*` methods.
- Saturate at the value’s minimum or maximum values with the `saturating_*` methods.


### Boolean Type

Um valor booleano pode ser true ou false. Booleanos sao de tamanho 1 byte.

### Character Type

É o tipoalfabetico mais primitivo do Rust.

Especificamos literais *char* com notacao de aspas simples, oposto a literais string que usam notacao de aspas duplas. O tipo *char* do rust tem 4 bytes de tamanho e representa um valor escalar Unicode, ou seja pode representar muito mais que ASCII. 

## Compound Types

*Compound types* podem agrupar multiplos valores em um tipo. Rust tem dois tipos compound primitivos: *tuples* e *arrays*.

### Tuple Type

Uma tupla, é uma forma geral de agrupar junto um numero de valores com uma variedade de tipos em um tipo compound. Tuplas tem um tamanho fixo: uma vez declarada, elas não podem crescer ou diminuir em valor.

Criamos tuplas ao escrever uma lista de valores separados por virgula dentro de parenteses, cada posicao da tupla tem um tipo e os tipos de diferentes valores dentro de uma tupla nao precisam ser todos iguais. Exemplo:

```Rust
fn main() {
	let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

A variável *tup* fica linkada à tupla inteira, porque a tupla é considerada um único elemento compound. Para obter os valores individuais de uma tupla, podemos usar pattern matching para desestruturar um valor de uma tupla, tipo assim:

```Rust
fn main() {
	let tup = (500, 6.4, 1);
	
	let (x, y, z) = tup;
	
	println!("The value of y is {y}");
}
```

Podemos tambem acessar um elemento da tupla diretamente usando um `.` seguido pelo index desejado:

```Rust
fn main() {
	let x: (i32, f64, u8) = (500, 6.4, 1);
	
	let five_hundred = x.0;
	
	let six_point_four = x.1;
	
	let one = x.2;
}
```

Uma tupla sem nenhum valores tem um nome especial: `unit`. Esse valor e o tipo correspondente são ambos escritos `()` e representam um valor vazio ou um return type vazio. Expressões implicitamente retornam a o valor unit se nao retornam nenhum outro valor.

### Array Type

Outra forma de se agrupar valores é com o array. Diferente da tupla, cada elemento de um array tem que ter o mesmo tipo. Diferente de arrays em outras linguas, um array no Rust tem o tamanho fixo.

Escrevemos arrays em uma lista separada por virgula, dentro de colchetes:

```Rust
let a = [1, 2, 3, 4, 5];
```

Arrays são uteis quando queremos dados alocados na stack, assim como outros tipos que vimos, ao inves da heap, ou quando queremos garantir que teremos sempre um numero fixo de elementos. Um array não é tao flexivel quanto um `vector type`. Um vetor é uma colecao similar fornecida pela lib standard que permite crescer ou diminuir e tamanho, porque o conteudo dele vive na heap. Se nao tem certeza se precisa usar um array ou vetor, provavelmente um vetor.

Porem, arrays sao mais uteis quando voce sabe que o numero de elementos nao precisara mudar. Por exemplo se estivesse usando os nomes dos meses em um programa, provavelmente usaria um array, pois sao sempre 12 elementos.

Voce escreve o tipo do array usando colchetes, junto com o numero itens, por exemplo:

```Rust
let a: [i32, 5] = [1, 2, 3, 4, 5];
```

Podemos tambem inicializar um array para conter o mesmo valor para cada elemento, especificando o valor inicial, seguido de ; e o tamanho do array:

```Rust
// Cria um array de tamanho 5 com 3 em todos os indices
let a = [3; 5];
```

Acessando os valores:

```Rust
let a = [1, 2, 3, 4, 5];

let first = a[0];
let second = a[1];
```