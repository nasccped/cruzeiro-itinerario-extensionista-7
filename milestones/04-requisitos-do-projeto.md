<h6 align="right">08/04/2026</h6>

# Requisitos do projeto

No contexto de arquitetura e engenharia de software, o termo
_"requisito"_ é muitas vezes usado para definir as obrigações de um
sistema, subdividindo-se em duas modalidades:

- requisitos **funcionais**
- requisitos **não funcionais**

## Requisitos funcionais

Requisitos funcionais referem-se ao conjunto de operações
**explícitas** (notadas sem necessidade de estudo aprofundado) que um
sistema deve ter para que se comporte da maneira esperada a fim de
resolver o problema em questão.

## Requisitos não funcionais

Requisitos não funcionais referem-se aos atributos **implícitos** de
um sistema e/ou suas funcionalidades para que certo critério seja
atendido (confiabilidade, desempenho, segurança, etc...).

## Linguagem de domínio

Antes de serem citados os requisitos referentes ao projeto, considere
as seguintes palavras da linguagem de domínio (palavras comuns para
referir aos termos específicos do projeto):

- `rf`: requisito funcional
- `rnf`: requisito não funcional
- `pdli`: ponto de descarte de lixo irregular
- `report`: ato resultante ao de reportar um `pdli` _(exemplo: ele
  reportou o descarte `->` ele realizou o report)_

<h6 align="right">10/04/2026</h6>

## Projeto em questão

> [!TIP]
>
> Considere:
>
> 1. `rf`
>    - `rnf` _(relacionado ao `rf` pai)_
>
> - `rnf` _(relacionado ao sistema no geral)_

Para o projeto trabalhado, aplicam-se os seguintes `rf`s e `rnf`s:

1. os usuários devem ser capazes realizar um `report`, gerando assim
   uma ficha de `pdli`
   - o usuário precisa estar _logado_[^logado] para que consiga iniciar um
     `report`
   - o usuário só consegue realizar o `report` caso:
     - seja o seu primeiro `report`
     -  ou tenha um intervalo de tempo superior à 1h30min desde o último
       `report` deste mesmo usuário
     - ou a ficha do `pdli` a ser reportado tenha sido fechada desde o
       último report deste mesmo usuário ao `pdli` em questão
2. os usuários devem ser capazes de acompanhar a situação do seu
   próprio `report` ou de um `pdli` específico.
3. os usuários devem ser capazes de atualizar a ficha de um `pdli`
4. os usuários devem ser capazes de consultar informações referentes
   a um `pdli` em aberto ou já encerrado

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

[^logado]: Logado é o particípio do verbo logar. Significa que um
  usuário acessou, ingressou ou conectou-se a um sistema digital,
  site ou programa de computador utilizando credenciais de
  autenticação, como nome de usuário e senha.
[^deterministico]: Algo determinístico é quando, dadas as mesmas
  condições inicias (dados de entrada), produzirá sempre o mesmo
  resultado.
[^CEP]: O CEP (Código de Endereçamento Postal) é um conjunto numérico
  constituído de oito algarismos, que orienta e acelera o
  encaminhamento, o tratamento e a distribuição de objetos de
  correspondência, por meio da sua atribuição a localidades,
  logradouros, unidades dos Correios, serviços, órgãos públicos,
  empresas e edifícios.
