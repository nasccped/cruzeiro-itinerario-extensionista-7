// conteúdo referente à entidade de usuários.

Table users {
  id           int         [pk, increment]
  username     varchar(50) [not null, unique]
  email        varchar(50) [not null, unique]
  category     int         [not null]
  latestReport int         [unique]
}

Table userCategories {
  id   int         [pk, increment]
  name varchar(50) [not null, unique]
}

Ref: users.category > userCategories.id
