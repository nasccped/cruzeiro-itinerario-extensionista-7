# Código fonte

Todo o código fonte do projeto pode ser encontrado em `src` e
`db-templates` (lembrando os arquivos locais também são importantes).

## Sobre

O projeto aqui presente refere-se à uma API REST que possibilita a
interação com a lógica do domínio e banco de dados por meio de
requisições `HTTP`.

## Requisitos

Uma vez que o projeto é construído em [rust](https://rust-lang.org/pt-BR/)
e [postgres](https://www.postgresql.org/), é esperado que a máquina
local tenha as ferramentas e suas respectivas dependências para o
funcionamento do projeto.

Deixo o guia de instalação sob responsabilidade das fontes oficiais.

## Antes de executar:

Antes de experimentar o projeto, é necessário realizar a preparação
do ambiente:

1. banco de dados e entidades: deve-se criar o banco de dados +
   entidades, views e functions para o funcionamento do projeto. Os
   registros podem ser encontrados em
   [`db-templates/default-schema.sql`](./db-templates/default-schema.sql),
   basta copiar e colar no [`REPL`](https://pt.wikipedia.org/wiki/REPL)
   do postgres!
2. variáveis de ambiente: o programa funciona a partir de variáveis
   de ambiente que são interceptadas em tempo de execução. Por design
   do projeto, basta inserí-las em um arquivo `.env` no diretório
   local (caso não tenha certeza, consulte valores padrões para cada
   um dos campos):
   ```environment
   SERVER_URL=        ...
   SERVER_PORT=       ...
   POSTGRES_USER=     ...
   POSTGRES_PASSWORD= ...
   POSTGRES_HOST=     ...
   POSTGRES_PORT=     ...
   POSTGRES_DATABASE= ...
   ```

## Executando

Considerando que tenha seguido lista de [requisitos](#requisitos),
você pode-rá executar o projeto usando um único comando do
`package manager` do rust:
```sh
cargo run
```
Após a execução, é esperado visualizar a seguinte mensagem no
terminal:
```txt
[<TIMEZONE> INFO  project] Servidor rodando em `<SERVER_URL>:<SERVER_PORT>`
```

Com isso, a aplicação fica disponível para uso. Basta acessar os
endpoints por meio de algum navegador ou aplicativo (sugerido:
[postman](https://www.postman.com/)).

## Outros

Mais informações de uso (endpoints e relacionados) podem ser
encontradas no arquivo [`docs.md`](./docs.md).

## Licença

O código fonte disposto está sob a mesma licença que todo o resto do
projeto. O arquivo original pode ser encontrado na
[raíz do repositório](../LICENSE).
