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

<h6 align="right">19/04/2026</h6>

## Estrutura das entidades

A seguir temos a estrura das entidades usadas no projeto **(até
então)**:

<img src="../images/diagram.svg">
<div align="center">
  <i>
    Representação das entidades e seus relacionamentos (diagrama).
    Consider abrir a imagem
    <a href="../images/diagram.svg">
      svg original
    </a>.
  </i>
</div>

### `users` (usuários)

A tabela de `users` tem como finalidade armazenar um usuário
reconhecido pelo sistema. Essa tabela é constituída pelas colunas:

- `pkId`: campo do tipo `int`, usado como chave primária para
  identificar este usuário como único
- `userName`: campo do tipo `varchar(50)` não nulo e único, usado
  para armazenar o nome deste usuário
- `userMail`: campo do tipo `varchar(50)` não nulo e único, usado
  para armazenar o _e-mail_ deste usuário
- `fkCategory`: campo do tipo `int` não nulo, usado como chave
  estrangeira para associar o usuário com uma
  [categoria](#usercategories-categorias-de-usuário)
- `fkLatestCategoryChange`: campo do tipo `int` único, usado como
  chave estrangeira para associar o usuário com a última
  [troca de categoria](#usercategorychanges-trocas-de-categoria-para-usuário)

### `userCategories` (categorias de usuário)

A tabela de `userCategories` tem como finalidade armazenar as
possíveis categorias para os [usuários](#users-usuários). Essa tabela
é constituída pelas colunas:

- `pkId`: campo do tipo `int`, usado como chave primária para
  identificar esta categoria como única
- `categoryName`: campo do tipo `varchar(50)` não nulo e único, usado
  para expressar o nome da categoria em questão

### `userCategoryChanges` (trocas de categoria para usuário)

A tabela de `userCategoryChanges` tem como finalidade armazenar as
trocas de [categorias](#usercategories-categorias-de-usuário) feitas
pelos [usuários](#users-usuários). Essa tabela é constituída pelas
colunas:

- `pkId`: campo do tipo `int`, usado como chave primária para
  identificar esta troca como única
- `fkFrom`: campo do tipo `int` não nulo, usado como chave
  estrangeira para relacionar esta troca com a
  [categoria de origem](#usercategories-categorias-de-usuário)
- `fkTo`: campo do tipo `int` não nulo, usado como chave estrangeira
  para relacionar esta troca com a
  [categoria de destino](#usercategories-categorias-de-usuário)
- `atTimeStamp`: campo do tipo `timestamp`, usado para expressar
  quando a troca foi feita

### `reports`

A tabela `reports` tem como finalidade armazenar os reports
registrados em sistema. Essa tabela é constituída pelas colunas:

- `pkId`: campo do tipo `int`, usado como chave primária para
  identificar este report como único
- `fkUserOwner`: campo do tipo `int` não nulo, usado para associar
  este report com um [usuário](#users-usuários)
- `openedAt`: campo do tipo `timestamp` não nulo, usado para
  expressar quando este report foi feito
- `fkRecord`: campo do tipo `int` não nulo, usado para associar este
  report com uma [ficha](#records-fichas-de-descarte)
- `fkStatus`: campo do tipo `int` não nulo, usado para associar este
  report com um [status](#reportstatuses-status-de-report)
- `fkLatestStatusChange`: campo do tipo `int` não nulo e único, usado
  para associar este report à uma
  [troca de status](#reportstatuschanges-troca-de-status-de-report)

### `reportStatuses` (status de report)

A tabela `reportStatuses` tem como finalidade armazenar os possíveis
status para os [reports](#reports). Essa tabela é
constituída pelas colunas:

- `pkId`: campo do tipo `int`, usado como chave primária para
  identificar este status como único
- `statusName`: campo do tipo `varchar(50)` não nulo e único, usado
  para expressar o nome deste status

### `reportStatusChanges` (troca de status de report)

A tabela `reportStatusChanges` tem como finalidade armazenar as
trocas de status para os [reports](#reports). Essa tabela
é constituída pelas colunas:

- `pkId`: campo do tipo `int`, usado como chave primária para
  identificar esta troca como única
- `fkFrom`: campo do tipo `int` não nulo, usado como chave
  estrangeira para associar esta troca com um
  [status de origem](#reportstatuses-status-de-report)
- `fkTo`: campo do tipo `int` não nulo, usado como chave estrangeira
  para associar esta troca com um
  [status de destino](#reportstatuses-status-de-report)
- `atTimeStamp`: campo do tipo `timestamp` não nulo, usado para
  expressar quando a troca foi feita

### `records` (fichas de descarte)

A tabela `records` tem como finalidade armazenar as fichas de pontos
de descarte. Essa tabela é constituída pelas colunas:

- `pkId`: campo do tipo `int`, usado como chave primária para
  identificar esta ficha como única
- `openedAt`: campo do tipo `timestamp` não nulo, usado para expressar
  quando a ficha foi aberta
- `fkStatus`: campo do tipo `int` não nulo, usado como chave
  estrangeira para associar a ficha com um
  [status](#recordstatuses-status-de-ficha)
- `fkLatestStatusChange`: campo do tipo `int`, usado como chave
  estrangeira para associar a ficha à uma
  [troca de status](#recordstatuschanges-troca-de-status-de-ficha)
- `fkPlace`: campo do tipo `int`, usado como chave estrangeira para
  associar a ficha à um
  [local](#places-lugares-de-descarte-irregular)

### `recordStatuses` (status de ficha)

A tabela `recordStatuses` tem como finalidade armazenar os possíveis
status para as [fichas de descarte](#records-fichas-de-descarte).
Essa tabela é constituída pelas colunas:

- `pkId`: campo do tipo `int`, usado como chave primária para
  identificar este status como único
- `statusName`: campo do tipo `varchar(50)` não nulo e único, usado
  para expressar o nome do status

### `recordStatusChanges` (troca de status de ficha)

A tabela `recordStatusChanges` tem como finalidade armazenar as
trocas de status feitas pelas
[fichas de descarte](#records-fichas-de-descarte). Essa tabela é
constituída pelas colunas:

- `pkId`: campo do tipo `int`, usado como chave primária para
  identificar esta troca como única
- `fkFrom`: campo do tipo `int` não nulo, usado como chave
  estrangeira para associar a troca com um
  [status de origem](#recordstatuses-status-de-ficha)
- `fkTo`: campo do tipo `int` não nulo, usado como chave estrangeira
  para associar a troca com um
  [status de destino](#recordstatuses-status-de-ficha)
- `atTimeStamp`: campo do tipo `timestamp` não nulo, usado para
  expressar quando a troca foi feita

### `places` (lugares de descarte irregular)

A tabela `places` tem como finalidade armazenar os lugares de
descarte irregular registrados no sistema. Esta tabela é constituída
pelas colunas:

- `pkId`: campo do tipo `int`, usado como chave primária para
  identificar o lugar como único
- `CEP`: campo do tipo `char(9)` não nulo e único, usado para
  expressar o _CEP_ do lugar
- `streetName`: campo do tipo `varchar(35)` não nulo, usado para
  expressar o nome da rua
- `fkNeighborhood`: campo do tipo `int` não nulo, chave estrangeira
  usada para associar o lugar à um [bairro](#neighborhoods-bairros)
  registrado em sistema

### `neighborhoods` (bairros)

A tabela `neighborhoods` tem como finalidade armazenar os bairros
registrados em sistema. Esta tabela é constituída pelas colunas:

- `pkId`: campo do tipo `int`, usado como chave primária para
  identificar este bairro como único
- `nbhName`: campo do tipo `varchar(30)` não nulo, usado para
  expressar o nome do bairro em questão
- `fkCity`: campo do tipo `int` não nulo, chave estrangeira usada para
  associar o bairro com um [município](#cities-municípios)

### `cities` (municípios)

A tabela `cities` tem como finalidade armazenar os municípios
registrados em sistema. Esta tabela é constituída pelas colunas:

- `pkId`: campo do tipo `int`, chave primária usada para identificar
  este município como único
- `cityName`: campo do tipo `varchar(50)` não nulo, usado para
  expressar o nome do município
- `fkState`: campo do tipo `char(2)` não nulo, chave estrangeira
  usada para associar o município com um [estado](#states-estados)

### `states` (estados)

A tabela `estados` tem como finalidade armazenar os estados
registrados em sistema. Esta tabela é constituída pelas colunas:

- `pkUfId`: campo do tipo `char(2)`, chave primária (como _unidade
  federativa_) usada para para identificar o estado como único
- `stateName`: campo do tipo `varchar(50)` não nulo e único, usado
  para expressar o nome do estado em questão
- `fkRegion`: campo do tipo `int` não nulo, usado para associar o
  estado à uma [região](#regions-regiões)

### `regions` (regiões)

A tabela `regions` tem como finalidade armazenar as regiões
registradas em sistema. Esta tabela é constituída pelas colunas:

- `pkId`: campo do tipo `int`, chave primária usada para identificar
  esta região como única
- `regionName`: campo do tipo `varchar(25)` não nulo e único, usado
  para expressar o nome da região
