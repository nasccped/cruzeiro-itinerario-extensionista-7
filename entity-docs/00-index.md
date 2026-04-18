<h6 align="right">18/04/2026</h6>

# Documentos de entidade

Aqui encontram-se os documentos relacionados às
_entidades_[^entidades]. Note que esses documentos não são código
_"hardcoded"_[^hardcoded] inseridos no projeto, mas sim uma
representação intermediária e/ou informações utilizadas para chegar
ao produto final.

## Arquivos `diag`

Arquivos com prefixo `diag` referem-se ao diagrama das entidades.

Quando montamos o `schema` do banco de dados, é preferível que haja
uma representação intermediária (e/ou visual) entre o conceito dos
dados e sua implementação em servidor.

Fazemos isso por meio de diagramas que representam visualmente as
entidades e seus relacionamentos.

Todos os arquivos começados com `diag` não são de fato SQL, na
verdade são arquivos em uma linguagem interpretada que é utilizada
pelo site [dbdiagram.io](https://dbdiagram.io/home).

Esse site foi usado para montar os diagramas do banco de dados
referente ao projeto em questão. _(você pode acompanhar os diagramas
na página [entidades](../milestones/05-entidades.md))_

[^entidades]: Entidade refere-se a um objeto ou conceito que possui
  identidade única e é reconhecido dentro de um sistema,
  representando elementos que podem ser manipulados, armazenados e
  recuperados (referentes ao banco de dados).
[^hardcoded]: Hardcoded refere-se à prática de desenvolvimento de
  software na qual valores, dados ou comportamentos são embutidos
  diretamente no código-fonte de um programa, em vez de serem obtidos
  de fontes externas ou gerados em tempo de execução.
