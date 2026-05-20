# Documentação

Aqui temos a documentação sobre o uso da API.

## Endpoints

Note que as informações apresentadas aqui seguem as mesmas já citadas
em [milestones/endpoints](../docs/milestones/06-endpoints.md), mas
alguns endpoints precisaram ser adicionados/modificados para o
funcionamento eficiente e adequado do projeto.

### `GET /` - Home

Requisição no endpoint raíz do projeto. Nenhum processamento,
ou alteração é feita, apenas retornado status de sucesso exibindo os
endpoints disponíveis (sem métodos http).

### `GET /users` - Lista de usuários

Acessa a _connection pool_ do postgres e retorna a lista de usuários
em um corpo json:
```json
{
  "users": [
    {
      "id": 1,
      "user_name": "Walter White",
      "user_mail": "contact@goodman.accessory.com",
      "latest_change": null,
      "current_status": "suspended"
    },
    // outros...
  ]
}
```

O _array_ de usuários pode ou não estar vazio (ainda significando
status `200`).

Ainda pode ser retornado status `INTERNAL SERVER ERROR` ou
`UNPROCESSABLE ENTITY` caso ocorra um erro na execução da query ou
na (de)serialização dos dados, respectivamente.

> [!NOTE]
>
> Todas as requests que executam alguma query no banco de dados estão
> sujeitas ao erro mencionado anteriormente.
>
> Não espera-se que aconteçam!

### `GET /users/{userId}` - Usuário por id

Acessa a _connection pool_ do postgres e retorna os campos do usuário
em um formato json:
```json
{
  "user": {
    "id": 67,
    "user_name": "Jessie Pinkman",
    "user_mail": "heiseberg@industries.com",
    "latest_change": null,
    "current_status": "available"
  }
}
```

Caso o usuário de id especificado não exista, é retornado status
`NOT FOUND`.

Se o id fornecido não for válido (inteiro de `64 bits`), é retornado
status `BAD REQUEST`.

### `PATCH /users/{userId}` - Atualizar usuário

Função para modificar os campos do usuário. Ainda não implementada
(retorna `501 NOT IMPLEMENTED`).

### `POST /users` - Adicionar usuário

Faz _parsing_ do `body`, converte em um `model` comum, tenta inserir
na tabela e retorna o resultado obtido.

Espera-se que o body seja um json no seguinte formato:
```json
{
  "user_name": "Um Nome de Exemplo",
  "user_mail": "exemplo@mail.com"
}
```

Caso contrário, é retornado status `BAD REQUEST`. Além disso, é
necessário que `user_name` seja válido (não vazio/nulo, conter ao
menos um caractere alfabético) da mesma forma que `user_mail` (padrão
geral para formatação de endereços e-mail), retornando o mesmo erro
se negativo.

Vale lembrar que ambos `user_name` e `user_mail` tem _constraints_ de
`UNIQUE`. Ao tentar inserir um recurso já sendo utilizado por outro
usuário o status `CONFLICT` ou `INTERNAL SERVER ERROR` pode ser
retornado.

## `GET /moderators` - Lista de moderadores

Acessa a _connection pool_ do postgres e retorna uma lista de
`moderators view`:
```json
{
  "moderators": [
    {
      "moderator_id": 2,
      "user_id": 12,
      "name": "Gus Fring",
      "mail": "ceo@pollos.com",
      "since": "2026-05-17T17:49:53Z"
    },
    // outros...
  ]
}
```

> [!NOTE]
>
> A estrutura retornada refere-se ao `moderator view` e não ao
> `moderator` verdadeiro.
>
> O registro verdadeiro contém apenas os campos de `id` e `user_id`,
> que não são muito úteis para exibição.

## `POST /moderators` - Adicionar moderador

Assim como a requisição [`POST /users`](#post-users---adicionar-usuário),
é necessário convert o `body` para um `model` comum:
```json
{
  "user_id": 22
}
```

Já que o esperado de todo moderador é que seja um usuário registrado,
basta passar o campo `user_id` para referenciar o usuário alvo.

Note que algumas operações de validação interferir no resultado
final:
- **usuário de id mencionado não existe:** `NOT FOUND`
- **usuário já é moderador:** `CONFLICT`
- **usuário tem como status `suspended`:** `CONFLICT` (regra de
  negócio)

Sem contar outras checagens óbvias como (de)serialização...

## `DELETE /moderators/{userId}` - Remove um moderador

Aciona a _connection pool_ e remove o administrador vinculado ao id
de usuário. Note que o id checado é o do usuário, não o do moderador.

Dessa forma, caso você queira remover 'Paulo Alberto' que é
representado pela seguinte estrutura:
```json
{
  "moderator_id": 2,
  "user_id": 5,
  "name": "Paulo Alberto",
  // outros campos...
}
```
Deve-se usar a request `DELETE /moderators/5`, e não
`DELETE /moderators/2`.

É retornado erro caso:
- `{userId}` fornecido não seja reconhecido como válido
- não exista um usuário vinculado ao `{userId}` passado
- o usuário vinculado ao `{userId}` não seja um moderador
