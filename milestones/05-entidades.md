<h6 align="right">15/04/2026</h6>

# Entidades

_Entidades_ referem-se a:

> objeto ou conceito que possui uma identidade única e é reconhecido
> dentro de um sistema, representando elementos que podem ser
> manipulados, armazenados e recuperados.

Resumindo, é uma palavra comumente usada para expressar tabelas em
banco de dados.

## Qual a necessidade?

Estabelecer previamente as entidades contribui com o desenvolvimento
do fluxo do projeto.

Por termos ciência das entidades, sabemos exatamente:

- Quais dados são necessários para adicionar um novo item em uma
  tabela
- Quais dados são retornados ao consultar um item em uma tabela
- Quais dados devem ser avaliados (antes de/durante) sua inserção na
  tabela

<h6 align="right">18/04/2026</h6>

## Estrutura das entidades

A seguir temos a estrura das entidades usadas no projeto **(até
então)**:

> [!NOTE]
>
> Para evitar ambiguidade de linguagem, os termos serão tratados em
> inglês, já que essa é uma restrição não só da maioria dos bancos de
> dados mas também de boa parte das tecnologias utilizadas no
> projeto.

### Usuários e categoria de usuários

<div align="center"> <img src="../images/diag-users.svg"> </div>

A(s) entidade(s):

- `users`: usuários registrados no sistema. São constituídos por
  - `id`: **int como chave primária**, identifica este usuário como
    único
  - `username`: **string não nula e única**, usada como nome de
    usuário
  - `email`: **string não nula e única**, usada como email de usuário
  - `category`: **int não nulo como chave estrangeira (refere-se à
    `userCategories.id`)**, identifica a categoria do usuário
  - `latestReport`: **int estrangeiro (refere-se à `reports.id`) e
    único**, identifica o último report feito por este usuário
- `userCategories`: categorias de usuário registradas no sistema. São
  contituídas por
  - `id`: **int como chave primária**, identifica esta categoria como
    única
  - `name`: **string não nula e único**, usada como nome da categoria
