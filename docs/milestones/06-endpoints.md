<h6 align="right">03/05/2026</h6>

# Endpoints

A fim de obter um projeto mas simples e produtivo, a API escolhida
para desenvolvimento será
[`Rest`](https://www.redhat.com/pt-br/topics/api/what-is-a-rest-api).

Essa interface se baseia em um serviço disposto em rede cujo qual
pode ser acessado via
[`protocolos HTTP`](https://www.alura.com.br/artigos/http) por meio
de uma [`URL`](https://tecnoblog.net/responde/o-que-e-url/).

A partir dos requisitos e características previamente definidas,
podemos definir quais serão os endpoints suportados pelo serviço.

## Relacionamento com casos de uso

Muitos dos [casos de uso](./04-casos-de-uso-e-regras-de-negócio.md)
mencionados estão diretamente relacionados com os endpoints
disponibilizados, sendo eles:

### Consultar usuário

O caso de uso `Consultar usuário` refere-se aos endpoints:

- `GET /users`: retorna uma lista com todos os usuários
- `GET /users/{userId}`: recebe um `userId` e retorna as informações
  referentes ao usuário vinculado ao id

### Atualizar usuário

O caso de uso `Atualizar usuário` refere-se aos endpoints:

- `PATCH users/{userId}`: atualiza os campos do usuário vinculado ao
  id mencionado

### Realizar report

O caso de uso `Realizar report` refere-se ao endpoint:

- `POST /reports`: adiciona um report à tabela de reports

### Consultar report

O caso de uso `Consultar report` refere-se aos endpoints:

- `GET /reports`: retorna uma lista com todos os reports
- `GET /reports/{reportId}`: recebe um `reportId` e retorna as
  informações referentes ao report vinculado

### Atualizar report

O caso de uso `Atualizar report` refere-se ao endpoint:

- `PATCH /reports/{reportId}`: recebe um `reportId` e atualiza os
  campos do registro vinculado ao id

### Consultar ficha

O caso de uso `Consultar ficha` refere-se aos endpoints:

- `GET /records`: retorna uma lista com todas as fichas
- `GET /records/{recordId}`: recebe um `recordId` e retorna as
  informações da ficha vinculada ao id

### Atualizar ficha

O caso de uso `Atualizar ficha` refere-se ao endpoint:

- `PATCH /records/{recordId}`: recebe um `recordId` e atualiza
  os campos da ficha vinculada ao id

## Endpoints extras

Outro(s) endpoint(s) adicionados a fim de agilizar as fases de
testes:

- `POST /users`: adicionar novos usuários
- `POST /moderators`: adicionar novo moderador a partir de um usuário
  existente
- `DELETE /users/{userId}`: recebe um `userId` e remove-o da
  tabela de usuários
- `DELETE /moderators/{moderatorId}`: recebe um `moderatorId` e
  remove-o da tabela de moderadores
