# Documentação

Aqui temos a documentação sobre o uso da API.

## Endpoints

Note que as informações apresentadas aqui seguem as mesmas já citadas
em [milestones/endpoints](../docs/milestones/06-endpoints.md), mas
alguns endpoints precisaram ser adicionados/modificados para o
funcionamento eficiente e adequado do projeto.

### `GET /` - Home

Requisição no endpoint raíz do projeto. Nenhum processamento,
ou alteração é feita, apenas retornado `200 OK` com um texto contento
os endpoints disponíveis (sem métodos http).

### `GET /users` - Lista de usuários

Requisição acessa a `connection pool` do `Postgresql`, retornando
`200 OK` onde o `body` é um json contendo um array de usuários (vazio
ou não) responsável por armazenar todos os usuários encontrados no
banco de dados.

A requisição ainda assim pode falhar, retornando `500 INTERNAL SERVER
ERROR` caso a falha seja durante a query no banco de dados, ou `422
UNPROCESSABLE ENTITY` caso a falha ocorra durante o `parsing` de
de `PgRow` para `User::json`.

### `GET /users/{userId}` - Usuário por id

Requisição acessa a `connection pool` do `Postgresql`, retornando
`200 OK` onde o `body` é um json contendo todos os campos da entidade
usuário.

Caso o usuário não exista (`não encontrado usuário com id ...`), é
retornado `404 NOT FOUND` com uma mensagem apropriada.

Espera-se que `{userId}` seja uma chave primária numérica (`integer
de 64 bits`), caso contrário é retornado `400 BAD REQUEST`.

Durante a query/parsing via backend - banco de dados, os erros
anteriormente mencionados em [`get users`](#get-users---lista-de-usuários)
também podem ocorrer, retornando os respectivos outputs.

### `POST /users` - Adicionar usuário

Requisição recebe um `body`, faz parsing de `String` para um
`CreateUserModel`, que pode ser representado pelo seguinte exemplo:

```json
{
  "user_name": "Um Nome de Exemplo",
  "user_mail": "algum.email@exemplo.com",
}
```

Qualquer estrura de `body` que não siga o modelo irá fazer com que o
endpoint retorne `400 BAD REQUEST`.

Antes de fazer envio ao banco de dados, os campos de `user_name` e
`user_mail` são testados, não de maneira severa, mas apenas se seguem
as convenções básicas:

- `user_name`:
  - não deve ser vazio
  - conter ao menos um caractere alfabético
- `user_mail`:
  - possuir estritamente um arroba (`@`)

Caso já exista um usuário com o nome e/ou o e-mail fornecido, a
operação irá falhar e retornar `500 INTERNAL SERVER ERROR`.
