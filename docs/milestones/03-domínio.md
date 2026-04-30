<h6 align="right">28/04/2026</h6>

# Domínio

Chamamos de domínio o conhecimento ou área específica em que um
sistema opera, definida por um conjunto de características que
descrevem uma família de problemas para os quais a aplicação visa
oferecer solução.

Para o projeto em questão, temos os seguintes domínios:

<h6 align="right">29/04/2026</h6>

## Usuário e Moderador

`usuário` refere-se à um indivíduo com capacidade de interagir com o
sistema, sendo autor de eventos ([`reports`](#report) e atualizações) e
recebedor de dados processados.

`moderador` por sua vez refere-se à uma entidade com essas mesmas
capacidades, mas com maiores privilégios.

## Report

`report` refere-se a um registro gerado gerado ao reportar um ponto
de descarte irregular.

Está fortemente associado com outros campos, como
[`localidade`](#localidade), autor ([`usuário`](#usuario-e-moderador)),
timestamp, etc.

## Ficha

`ficha` refere-se ao registro que relaciona um ou mais
[`reports`](#report) a um [`local`](#localidade), funcionando de
maneira similar a um _'container'_ que armazena o conjunto de
reports.

## Status do usuário, do report e da ficha

Existem diferentes status para as diferentes entidades do sistema:

### Status do usuário

Determina se um usuário está suspenso de efetuar novos
[`reports`](#report).

### Status do report

Determina qual a condição atual de um dado report:

- aberto (aguardando ser atendido)
- indeterminadamente suspenso
- cancelado
- fechado (já atendido)

### Status da ficha

Similar ao anterior, mas destinando-se às [`fichas`](#ficha).

## Localidade

`localidade` refere-se ao conjunto de dados que expressam um lugar
(ao qual o [`report`](#report) diz respeito).
