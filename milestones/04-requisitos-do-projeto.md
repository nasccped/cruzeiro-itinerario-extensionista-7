<h6 align="right">08/04/2026</h6>

# Requisitos do projeto

No contexto de arquitetura e engenharia de software, o termo
_"requisito"_ é muitas vezes usado para definir as obrigações de um
sistema, subdividindo-se em duas modalidades:

- requisitos **funcionais**
- requisitos **não funcionais**

<h6 align="right">14/04/2026</h6>

## Linguagem de domínio

Antes de serem citados os requisitos referentes ao projeto, considere
as seguintes palavras da linguagem de domínio (palavras comuns para
referir aos termos específicos do projeto):

- `pdli`: ponto de descarte de lixo irregular
- `report`: ato resultante ao de reportar um `pdli` _(exemplo: ele
  reportou o descarte `->` ele realizou o report)_
- `ficha`: entidade[^entidade] gerada ao realizar o `report` de um
  `pdli`

## Requisitos funcionais

Requisitos funcionais referem-se ao conjunto de operações
**explícitas** (notadas sem necessidade de estudo aprofundado) que um
sistema deve ter para que se comporte da maneira esperada a fim de
resolver o problema em questão.

Dentre os requisitos funcionais, temos:

1. Os usuários devem ser capazes de realizar o `report` de um `pdli`
2. Os usuários devem ser capazes de acompanhar o status de seu(s)
   `report`(s)
3. Os usuários devem ser capazes de acompanhar a `ficha` de um `pdli`
4. Os usuários devem ser capazes de atualizar uma `ficha` de um
   `pdli` (adicionar `report`, adicionar comentário, atualizar status
   da `ficha`)

## Requisitos não funcionais

Requisitos não funcionais referem-se aos atributos **implícitos** de
um sistema e/ou suas funcionalidades para que certo critério seja
atendido.

Dentre os requisitos **não funcionais**, temos:

- O 'usuário' deve ser reconhecido como um aos _'olhos do sistema'_,
  ou seja, possuir:
  - nome de usuário
  - e-mail
  - _etc..._
- As condições para `report` são:
  - ser o `report` inicial para a `ficha` do `pdli`
  - **E** ter `+1hr30min` desde o último `report` feito por este
    mesmo usuário
- A `ficha` de um `pdli` deve estar associada ao menos com 1 `report`
- O status de uma `ficha` só pode ser atualizado por um usuário
  responsável (ex: _monitor_, _administrador_, etc.)
- o `pdli` deve ser indexado de maneira
  determinística[^deterministico], por exemplo **CEP**[^CEP] ou
  longitude e latitude, etc.

> [!WARNING]
>
> É importante lembrar que os requisitos mencionados anteriormente
> referem-se à idealização do projeto inicial _(08/04/2026)_ e que
> estão sujeitos à atualizações de acordo com as necessidades do
> projeto e dos critérios (tempo para desenvolvimento, orçamento,
> etc).

[^deterministico]: Algo determinístico é quando, dadas as mesmas
  condições inicias (dados de entrada), produzirá sempre o mesmo
  resultado.
[^CEP]: O CEP (Código de Endereçamento Postal) é um conjunto numérico
  constituído de oito algarismos, que orienta e acelera o
  encaminhamento, o tratamento e a distribuição de objetos de
  correspondência, por meio da sua atribuição a localidades,
  logradouros, unidades dos Correios, serviços, órgãos públicos,
  empresas e edifícios.
[^entidade]: Em bancos de dados e modelagem de dados, uma entidade é
  qualquer objeto ou conceito identificável (como um cliente ou
  produto) que pode ser armazenado e gerenciado.
