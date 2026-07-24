> **Status:** histórico — instruções executadas em 2026-07-24.
>
> Este arquivo registra o roteiro de correções pós-revisão (status da Etapa 0,
> ratificação, trust_level T0–T3, benchmarks, DCO e coerência documental).
>
> Não utilizar como fonte normativa. Consulte `00_Indice_Versoes.md`.

# Instruções para o Cursor — Correções pós-revisão do repositório OpenCore

**Repositório:** `duvallemusic/open-core`  
**Data da revisão:** 2026-07-24  
**Objetivo:** corrigir inconsistências documentais e normativas encontradas após a atualização para Manifesto 1.2, Arquitetura 1.3, Roadmap 2.3 e criação do OpenCore Builder.

## Natureza desta tarefa

Esta tarefa é exclusivamente documental e organizacional.

Não implementar:

- runtime;
- módulos;
- OpenCore Builder;
- IA;
- CLI;
- instaladores;
- pipelines;
- marketplace;
- adaptadores;
- SDKs.

A finalidade é reconciliar os documentos canônicos, corrigir inconsistências de status, atualizar links antigos, padronizar taxonomias e preservar a rastreabilidade da Etapa 0.

---

# 1. Resumo das correções obrigatórias

Executar estas correções:

1. atualizar o `CONTRIBUTING.md` para apontar para os documentos canônicos atuais;
2. unificar a descrição do estado da Etapa 0 no README, índice, roadmap e guia de contribuição;
3. corrigir o status das alterações de Comunidade/Governança e Plano Institucional;
4. padronizar `trust_level` para T0–T3 em todos os documentos;
5. corrigir o licenciamento de Frappe Framework e ERPNext no benchmark;
6. acrescentar estrutura de fontes e data de verificação ao benchmark;
7. registrar a data de início efetivo da exigência de DCO;
8. mover o arquivo de instruções do Cursor para o histórico;
9. corrigir pequenos erros de redação;
10. atualizar o índice e os espelhos apenas quando necessário;
11. executar uma varredura final de referências antigas.

---

# 2. Arquivos a alterar

Alterar:

- `README.md`
- `00_Indice_Versoes.md`
- `CONTRIBUTING.md`
- `01_Manifesto_OpenCore_v1.2.md` apenas se houver referência textual inconsistente encontrada na varredura
- `02_Arquitetura_OpenCore_v1.3.md`
- `02_Arquitetura_OpenCore_v1.md`
- `03_Comunidade_Governanca_OpenCore_v1.0.md`
- `04_Plano_Institucional_OpenCore_v1.0.md`
- `05_Roadmap_OpenCore_v2.3.md`
- `05_Roadmap_OpenCore_v2.md`
- `06_Especificacao_OpenCore_Builder_v0.md`
- `07_Benchmarks_Ecossistema_OpenCore_v1.0.md`
- `ADR-017_Niveis_Confianca_Modulos.md`
- `ADR-022_OpenCore_Builder_Triagem_Composicao_Empacotamento.md`
- `DCO.md`, caso necessário para registrar a regra de transição
- `.github/pull_request_template.md`, caso a data de corte do DCO seja mencionada no fluxo de contribuição

Mover:

- `INSTRUCOES_CURSOR_ATUALIZACAO_OPENCORE_BUILDER_E_ECOSSISTEMA.md`

Para:

- `docs/history/INSTRUCOES_CURSOR_BUILDER_2026-07-24.md`

Se `docs/history/` não existir, criar o diretório.

Não apagar o arquivo. Preservar como histórico.

---

# 3. Correção P0 — Atualizar o CONTRIBUTING.md

## Problema

O `CONTRIBUTING.md` ainda aponta para:

- Manifesto 1.1;
- Arquitetura 1.2.

Esses documentos foram supersedidos.

## Alteração

Na seção “Antes de começar”, substituir a lista atual por:

```markdown
Leia, nesta ordem:

1. [`README.md`](README.md)
2. [`00_Indice_Versoes.md`](00_Indice_Versoes.md)
3. [`01_Manifesto_OpenCore_v1.2.md`](01_Manifesto_OpenCore_v1.2.md)
4. [`02_Arquitetura_OpenCore_v1.3.md`](02_Arquitetura_OpenCore_v1.3.md)
5. [`03_Comunidade_Governanca_OpenCore_v1.0.md`](03_Comunidade_Governanca_OpenCore_v1.0.md)
6. [`05_Roadmap_OpenCore_v2.3.md`](05_Roadmap_OpenCore_v2.3.md)
7. [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md)

Para contribuições relacionadas ao OpenCore Builder, produto, UX, catálogo de capacidades ou composição guiada, leia também:

8. [`ADR-022_OpenCore_Builder_Triagem_Composicao_Empacotamento.md`](ADR-022_OpenCore_Builder_Triagem_Composicao_Empacotamento.md)
9. [`06_Especificacao_OpenCore_Builder_v0.md`](06_Especificacao_OpenCore_Builder_v0.md)

Use apenas as versões marcadas como canônicas no índice.
```

## Estado atual do projeto

Substituir a formulação da seção “Estado atual do projeto” por:

```markdown
## 2. Estado atual do projeto

A documentação essencial da Etapa 0 foi consolidada.

Manifesto 1.2, Arquitetura 1.3, Roadmap 2.3 e ADR-022 permanecem em revisão formal. A Etapa 1 está autorizada apenas para spikes técnicos reversíveis, documentados e time-boxed.

Ainda não existe runtime de produção, SDK público estável, OpenCore Builder implementado ou distribuição pronta para uso final.

Contribuições úteis agora incluem:

- revisão de consistência e documentação;
- pesquisa técnica;
- propostas e revisões de RFC;
- ADRs derivados de decisões aceitas;
- implementação de spikes explicitamente priorizados;
- testes de hipóteses arquiteturais;
- documentação de ambiente e reprodução;
- identificação de riscos, casos de uso e requisitos de pilotos.

Contribuições de código devem estar vinculadas a uma issue, RFC, ADR ou spike explicitamente aberto no roadmap.

A conclusão documental da Etapa 0 não autoriza a implementação antecipada de um runtime definitivo nem do OpenCore Builder completo. Código produzido nos spikes poderá ser descartado, refeito ou mantido isolado conforme as evidências obtidas.
```

---

# 4. Correção P0 — Unificar o estado da Etapa 0

## Problema

O repositório usa formulações diferentes:

- “Etapa 0 consolidada”;
- “Etapa 0 em evolução”;
- “Etapa 0 concluída”;
- “documentos pendentes de aprovação”.

Essas frases criam ambiguidade.

## Formulação canônica

Usar esta formulação em todos os arquivos públicos relevantes:

> **A documentação essencial da Etapa 0 foi consolidada. Manifesto 1.2, Arquitetura 1.3, Roadmap 2.3 e ADR-022 permanecem em revisão formal. A Etapa 1 está autorizada apenas para spikes técnicos reversíveis, documentados e time-boxed. O OpenCore Builder completo não está autorizado para implementação.**

## README.md

Substituir o bloco de status por:

```markdown
> **Status atual:** a documentação essencial da Etapa 0 foi consolidada.
> Manifesto 1.2, Arquitetura 1.3, Roadmap 2.3 e ADR-022 permanecem
> em revisão formal. A Etapa 1 está autorizada apenas para spikes técnicos
> reversíveis, documentados e time-boxed.
>
> O OpenCore Builder completo não está autorizado para implementação.
> A RFC-0001 permanece em consulta até 2026-08-22.
> Ainda não há código de produto.
```

## 00_Indice_Versoes.md

Na seção “Etapa 0 — estado”, substituir por:

```markdown
## Etapa 0 — estado

A documentação essencial da Etapa 0 foi consolidada.

1. Manifesto 1.2 criado — revisão formal pendente.
2. Arquitetura 1.3 criada — revisão formal pendente.
3. Comunidade e Governança 1.0 aprovada originalmente; alterações de 2026-07-24 pendentes de ratificação.
4. Plano Institucional 1.0 aprovado originalmente; alterações de 2026-07-24 pendentes de ratificação.
5. Roadmap 2.3 criado — revisão formal pendente.
6. ADR-022 permanece Proposto e condicionado aos Spikes 14–18.
7. Especificação Builder v0 e Benchmarks v1.0 foram criados.
8. Licença documental publicada; RFC-0001 em consulta até 2026-08-22.

A Etapa 1 está autorizada apenas para spikes técnicos reversíveis, documentados e time-boxed.

O OpenCore Builder completo não está autorizado para implementação.
```

Na seção “Próxima etapa”, preservar a ordem existente, mas iniciar com:

```markdown
1. Revisar formalmente Manifesto 1.2, Arquitetura 1.3, Roadmap 2.3 e ADR-022.
2. Ratificar ou ajustar as alterações de 2026-07-24 em Comunidade/Governança e Plano Institucional.
3. Não iniciar implementação completa do OpenCore Builder.
4. Iniciar a Etapa 1 apenas pelos spikes técnicos priorizados.
```

## 05_Roadmap_OpenCore_v2.3.md

Alterar o status do cabeçalho para:

```markdown
**Status:** documentação essencial da Etapa 0 consolidada; Manifesto 1.2, Arquitetura 1.3, Roadmap 2.3 e ADR-022 em revisão formal; RFC-0001 em consulta; Etapa 1 autorizada apenas para spikes técnicos reversíveis, documentados e time-boxed.
```

No restante do documento, substituir frases absolutas como:

- “Etapa 0 concluída”;
- “Etapa 0 fechada”;
- “Etapa 0 em evolução”;

pela formulação canônica, respeitando o contexto.

Atualizar o espelho `05_Roadmap_OpenCore_v2.md` para ficar idêntico à versão 2.3.

---

# 5. Correção P1 — Status de Comunidade/Governança

## Problema

O documento foi aprovado em 2026-07-23, mas recebeu em 2026-07-24 alterações que não são apenas editoriais:

- papéis adicionais;
- ownership de distribuições;
- relação com o Builder;
- novos tipos de contribuição;
- alinhamento com ADR-017.

Essas mudanças precisam ser ratificadas.

## Alteração no cabeçalho

Substituir:

```markdown
**Status:** Aprovado
```

por:

```markdown
**Status:** Aprovado originalmente em 2026-07-23; alterações substantivas de 2026-07-24 pendentes de ratificação
```

Substituir:

```markdown
**Última atualização editorial:**
```

por:

```markdown
**Última atualização:** 2026-07-24 — acréscimo da relação com o OpenCore Builder, papéis de distribuição e formação, ownership explícito, alinhamento à ADR-017 e formas adicionais de contribuição. As alterações de 2026-07-24 são substantivas e permanecem pendentes de ratificação.
```

Atualizar:

```markdown
**Base normativa:** Manifesto OpenCore v1.1
```

para:

```markdown
**Base normativa:** Manifesto OpenCore v1.2
```

Atualizar documentos relacionados para citar:

- Arquitetura 1.3;
- Roadmap 2.3;
- ADR-015..022;
- Especificação Builder v0.

## Histórico

Adicionar ao final:

```markdown
## Histórico de aprovação

| Data | Estado | Descrição |
|---|---|---|
| 2026-07-23 | Aprovado | Versão 1.0 original aprovada durante a consolidação da Etapa 0. |
| 2026-07-24 | Pendente de ratificação | Alterações substantivas relacionadas ao Builder, ownership, distribuição, formação e níveis de confiança. |
```

Não criar versão 1.1 nesta tarefa.

---

# 6. Correção P1 — Status do Plano Institucional

## Problema

O documento foi aprovado em 2026-07-23, mas recebeu em 2026-07-24 mudanças substantivas:

- Builder como canal de adoção;
- SEO;
- marketing por necessidade;
- rede de prestadores;
- métricas adicionais.

## Alteração no cabeçalho

Substituir:

```markdown
**Status:** Aprovado
```

por:

```markdown
**Status:** Aprovado originalmente em 2026-07-23; alterações substantivas de 2026-07-24 pendentes de ratificação
```

Substituir “Última atualização editorial” por:

```markdown
**Última atualização:** 2026-07-24 — acréscimo do OpenCore Builder como canal de adoção, SEO e marketing por necessidade, mensagens de referência, rede de prestadores e métricas. As alterações de 2026-07-24 são substantivas e permanecem pendentes de ratificação.
```

Atualizar:

```markdown
**Base normativa:** Manifesto OpenCore v1.1
```

para:

```markdown
**Base normativa:** Manifesto OpenCore v1.2
```

Atualizar documentos relacionados para citar:

- Arquitetura 1.3;
- Roadmap 2.3;
- ADR-015..022;
- Especificação Builder v0;
- Benchmarks v1.0.

## Histórico

Adicionar:

```markdown
## Histórico de aprovação

| Data | Estado | Descrição |
|---|---|---|
| 2026-07-23 | Aprovado | Versão 1.0 original aprovada durante a consolidação da Etapa 0. |
| 2026-07-24 | Pendente de ratificação | Alterações substantivas relacionadas ao Builder, SEO, marketing, prestadores e métricas. |
```

Não criar versão 1.1 nesta tarefa.

---

# 7. Correção P1 — Padronizar trust_level para T0–T3

## Decisão normativa

A ADR-017 é a referência canônica.

Valores técnicos:

```yaml
trust_level: T0 | T1 | T2 | T3
```

Correspondência:

| Valor | Nome de exibição |
|---|---|
| T0 | Experimental |
| T1 | Comunitário |
| T2 | Verificado |
| T3 | Oficial |

## Arquitetura 1.3

No manifesto ilustrativo, substituir:

```yaml
trust_level: experimental | community | verified | official
```

por:

```yaml
trust_level: T0 | T1 | T2 | T3
```

Adicionar comentário:

```yaml
# T0 Experimental | T1 Comunitário | T2 Verificado | T3 Oficial
```

Exemplo esperado:

```yaml
trust_level: T2
```

Na tabela de campos conceituais, alterar a descrição de `trust_level` para:

```markdown
| `trust_level` | Nível técnico T0–T3 conforme ADR-017; a interface pode exibir o nome traduzido |
```

## ADR-022

Onde houver nomes sem códigos, manter o nome para linguagem de produto, mas registrar também o código:

- oficial → T3;
- verificado → T2;
- comunitário → T1;
- experimental → T0.

Exemplo:

```markdown
preferência: T3 Oficial → T2 Verificado; T1 Comunitário apenas no modo avançado com aceite explícito; T0 Experimental nunca por padrão
```

## Especificação Builder

No modo simples:

```markdown
- apenas módulos T3 Oficiais e T2 Verificados;
```

No modo avançado:

```markdown
- eventual inclusão de T1 Comunitário com aceite explícito;
- T0 Experimental permanece bloqueado.
```

## Benchmarks e outros documentos

Executar busca global por:

- `trust_level: experimental`
- `trust_level: community`
- `trust_level: verified`
- `trust_level: official`
- `experimental | community | verified | official`

Substituir pelo padrão T0–T3 onde o campo técnico estiver sendo descrito.

Nomes em texto de UX podem permanecer traduzidos, desde que o código seja indicado quando necessário.

## ADR-017

Atualizar as referências de base normativa:

- Manifesto 1.2;
- Arquitetura 1.3.

Não alterar a decisão T0–T3.

---

# 8. Correção P1 — Benchmark Frappe / ERPNext

## Problema

O benchmark trata Frappe e ERPNext como se ambos fossem GPL.

## Alteração

Na linha de licenciamento de Frappe/ERPNext, usar:

```markdown
| **Licenciamento** | Frappe Framework: MIT. ERPNext: GPL-3.0. Outros produtos e componentes do ecossistema devem ser verificados individualmente. |
```

Na linha de risco jurídico:

```markdown
| **Risco jurídico de copiar código** | **Variável por componente:** baixo a médio no Frappe Framework MIT, desde que preservados aviso e licença; alto no ERPNext GPL-3.0 para incorporação ao monorepo. Preferir estudar padrões e reimplementar contratos. |
```

Na linha “Padrão a NÃO incorporar”:

```markdown
| **Padrão a NÃO incorporar** | Obrigatoriedade de stack web/servidor; importar código ERPNext GPL para o monorepo sem análise; tratar todo o ecossistema Frappe como se tivesse uma única licença. |
```

## Estrutura de fontes no benchmark

Adicionar ao início do documento:

```markdown
## Fontes e data de verificação

As informações deste documento devem ser verificadas em fontes primárias:

- repositório oficial;
- arquivo de licença;
- documentação oficial;
- site oficial;
- fundação ou organização mantenedora.

Cada seção deverá registrar:

- data de verificação;
- fonte principal;
- componente ou edição analisada;
- licença específica do componente.

Licenças e ofertas comerciais podem mudar. Este documento não substitui revisão jurídica por componente.
```

## Campo por projeto

Adicionar ao quadro de cada projeto:

```markdown
| **Verificação** | Data e fontes primárias usadas |
```

Para os projetos sem fonte já registrada, usar:

```markdown
| **Verificação** | Pendente de anexar fontes primárias e data de verificação |
```

Não inventar URLs ou datas de consulta.

Para Frappe/ERPNext, registrar:

```markdown
| **Verificação** | 2026-07-24 — arquivos de licença dos repositórios oficiais Frappe Framework e ERPNext |
```

## Aviso jurídico

Manter e reforçar:

- estudar padrões não autoriza copiar código;
- licenças devem ser verificadas por arquivo e componente;
- Apache/MIT/MPL também exigem cumprimento de avisos;
- GPL/AGPL/LGPL exigem análise de copyleft e fronteira.

---

# 9. Correção P1 — Política de DCO

## Problema

O projeto já instrui o uso de `Signed-off-by`, mas commits anteriores foram feitos sem assinatura DCO.

Não reescrever o histórico principal nesta tarefa.

## Decisão de transição

Adicionar ao `CONTRIBUTING.md`, após a seção de DCO:

```markdown
### Início da exigência operacional

A exigência operacional de DCO aplica-se a commits criados a partir de **2026-07-25**.

Commits anteriores a essa data são tratados como histórico legado da fase de consolidação documental e não serão reescritos apenas para acrescentar `Signed-off-by`.

Pull requests novas poderão ser bloqueadas quando seus commits não possuírem assinatura DCO válida.
```

Adicionar ao `DCO.md` uma nota equivalente:

```markdown
## Aplicação no OpenCore

A exigência operacional de `Signed-off-by` aplica-se a commits criados a partir de 2026-07-25.

O histórico anterior é tratado como legado da fase de consolidação documental. Essa regra de transição não elimina obrigações de autoria, licença ou direito de contribuição.
```

Se houver configuração simples de GitHub DCO já disponível no repositório, apenas documentar como ação futura:

```markdown
- [ ] configurar check automático de DCO no GitHub
```

Não instalar app, workflow ou dependência nesta tarefa sem solicitação explícita.

## Pull request template

A checklist pode permanecer como está.

Adicionar, se necessário:

```markdown
- [ ] Commits criados a partir de 2026-07-25 possuem `Signed-off-by`
```

---

# 10. Correção P2 — Mover instruções do Cursor para histórico

## Arquivo atual

`INSTRUCOES_CURSOR_ATUALIZACAO_OPENCORE_BUILDER_E_ECOSSISTEMA.md`

## Destino

`docs/history/INSTRUCOES_CURSOR_BUILDER_2026-07-24.md`

## Cabeçalho a adicionar

No topo do arquivo movido:

```markdown
> **Status:** histórico — instruções executadas em 2026-07-24.
>
> Este arquivo registra o roteiro utilizado para produzir Manifesto 1.2,
> Arquitetura 1.3, Roadmap 2.3, ADR-022, Especificação Builder v0 e
> Benchmarks v1.0.
>
> Não utilizar este documento como fonte normativa ou como lista atual de
> arquivos canônicos. Consulte `00_Indice_Versoes.md`.
```

## Índice

Adicionar na seção de históricos:

```markdown
| Instruções Cursor — Builder | `docs/history/INSTRUCOES_CURSOR_BUILDER_2026-07-24.md` | roteiro executado; não normativo |
```

Não listar o arquivo entre canônicos.

---

# 11. Correções editoriais obrigatórias

## ADR-022

Localizar:

```text
deve passar pelo validador determinística
```

Corrigir para:

```text
deve passar pelo validador determinístico
```

## Especificação Builder v0

Localizar:

```text
não introduce regras de padaria
```

Corrigir para:

```text
não introduz regras de padaria
```

## Varredura adicional

Buscar por:

- `v1.1` em referências normativas atuais;
- `v1.2` de Arquitetura em documentos canônicos atuais;
- `v2.2` de Roadmap em documentos canônicos atuais;
- `ADR-015..021` quando deveria incluir ADR-022;
- “Etapa 0 concluída”;
- “Etapa 0 em evolução”;
- “última atualização editorial” nos dois documentos ratificáveis;
- `trust_level` com nomes em vez de T0–T3;
- caminhos antigos de Comunidade e Plano Institucional.

Não alterar referências históricas legítimas.

---

# 12. Atualização do README

Além do status, revisar:

## Documentos canônicos

Confirmar que continuam apontando para:

- Manifesto 1.2;
- Arquitetura 1.3;
- Comunidade/Governança 1.0;
- Plano Institucional 1.0;
- Roadmap 2.3;
- Spec Builder v0;
- Benchmarks v1.0.

## Estado dos documentos

Acrescentar uma observação curta:

```markdown
> Comunidade/Governança 1.0 e Plano Institucional 1.0 foram aprovados originalmente em 2026-07-23. As alterações substantivas de 2026-07-24 permanecem pendentes de ratificação.
```

## Roadmap em uma linha

Manter:

1. revisão formal;
2. spikes;
3. Portaria;
4. SDK;
5. pilotos;
6. Builder MVP baseado em regras.

Não sugerir que o Builder já está sendo implementado.

---

# 13. Atualização do índice de versões

## Canônicos

Usar estados consistentes:

| Documento | Estado |
|---|---|
| Manifesto 1.2 | Revisão formal pendente |
| Arquitetura 1.3 | Revisão formal pendente |
| Comunidade/Governança 1.0 | Aprovado originalmente; alterações de 2026-07-24 pendentes de ratificação |
| Plano Institucional 1.0 | Aprovado originalmente; alterações de 2026-07-24 pendentes de ratificação |
| Roadmap 2.3 | Revisão formal pendente |
| ADR-022 | Proposto, condicionado aos Spikes 14–18 |
| Spec Builder v0 | Proposta |
| Benchmarks v1.0 | Não normativo |

## Histórico

Adicionar o arquivo de instruções movido.

## Próxima etapa

Usar a formulação unificada da Etapa 0 e manter a proibição de implementar o Builder completo.

---

# 14. Consistência entre documentos

Executar verificação cruzada:

| Tema | Resultado esperado |
|---|---|
| Estado da Etapa 0 | mesma formulação em README, índice, roadmap e CONTRIBUTING |
| Manifesto canônico | 1.2 |
| Arquitetura canônica | 1.3 |
| Roadmap canônico | 2.3 |
| ADRs | 015–022 |
| Builder | externo ao runtime |
| IA | opcional e subordinada ao motor determinístico |
| Perfis | variantes verificadas |
| Distribuição | composição testada |
| Manifesto | intenção/faixas |
| Lockfile | versões e hashes exatos |
| `trust_level` | T0–T3 |
| T0 | nunca em distribuição oficial |
| T1 | comunitário, aviso/aceite |
| T2 | verificado |
| T3 | oficial |
| DCO | exigido para commits a partir de 2026-07-25 |
| Comunidade/Plano | alterações de 2026-07-24 pendentes de ratificação |
| Benchmark | Frappe MIT; ERPNext GPL-3.0 |
| Instruções Cursor | histórico, não normativo |

---

# 15. Não fazer

- não alterar o conteúdo central do Manifesto;
- não aceitar ADR-022;
- não aceitar ADR-021;
- não iniciar spikes;
- não implementar código;
- não criar nova versão 1.1 de Comunidade ou Plano;
- não apagar histórico;
- não reescrever commits antigos;
- não configurar apps externos do GitHub;
- não inventar fontes;
- não alterar licenças;
- não transformar exemplos de padaria em compromisso de produto;
- não promover módulos T1/T2/T3 automaticamente;
- não declarar Etapa 0 integralmente aprovada.

---

# 16. Checklist final

## Links e versões

- [ ] `CONTRIBUTING.md` aponta para Manifesto 1.2.
- [ ] `CONTRIBUTING.md` aponta para Arquitetura 1.3.
- [ ] `CONTRIBUTING.md` inclui Roadmap 2.3.
- [ ] referências históricas não foram alteradas indevidamente.
- [ ] README e índice apontam para arquivos existentes.

## Estado do projeto

- [ ] README usa a formulação canônica da Etapa 0.
- [ ] índice usa a formulação canônica.
- [ ] roadmap usa a formulação canônica.
- [ ] CONTRIBUTING usa a formulação canônica.
- [ ] nenhum documento afirma que o Builder completo está autorizado.

## Governança

- [ ] Comunidade/Governança registra ratificação pendente.
- [ ] Plano Institucional registra ratificação pendente.
- [ ] ambos usam Manifesto 1.2 como base.
- [ ] histórico de aprovação foi adicionado.

## Confiança

- [ ] campo técnico usa T0–T3.
- [ ] nomes traduzidos permanecem apenas como exibição.
- [ ] T0 nunca é recomendado.
- [ ] T1 exige modo avançado/aceite.
- [ ] T2 e T3 são preferidos pelo Builder.

## Benchmarks

- [ ] Frappe Framework está como MIT.
- [ ] ERPNext está como GPL-3.0.
- [ ] documento possui seção de fontes.
- [ ] projetos sem fontes estão marcados como pendentes.
- [ ] nenhuma estatística foi inventada.

## DCO

- [ ] data de corte 2026-07-25 registrada.
- [ ] commits anteriores tratados como legado.
- [ ] template reflete a regra quando necessário.
- [ ] histórico não foi reescrito.

## Organização

- [ ] arquivo de instruções foi movido para `docs/history/`.
- [ ] cabeçalho histórico foi adicionado.
- [ ] índice registra o arquivo como histórico.
- [ ] não existe cópia duplicada na raiz.

## Redação

- [ ] “validador determinística” corrigido.
- [ ] “não introduce” corrigido.
- [ ] busca global por referências antigas concluída.

---

# 17. Validação local sugerida

Executar comandos equivalentes, adaptando ao ambiente:

```bash
git status --short

grep -R "01_Manifesto_OpenCore_v1.1_licenciamento.md" \
  --exclude-dir=.git .

grep -R "02_Arquitetura_OpenCore_v1.2.md" \
  --exclude-dir=.git .

grep -R "05_Roadmap_OpenCore_v2.2.md" \
  --exclude-dir=.git .

grep -R "trust_level: experimental" \
  --exclude-dir=.git .

grep -R "validador determinística" \
  --exclude-dir=.git .

grep -R "não introduce" \
  --exclude-dir=.git .

grep -R "Etapa 0 concluída\|Etapa 0 em evolução\|Etapa 0 consolidada" \
  --exclude-dir=.git .
```

Resultados em snapshots históricos são permitidos.

Revisar o diff:

```bash
git diff --check
git diff --stat
git diff
```

Confirmar que os espelhos continuam idênticos:

```bash
cmp -s 02_Arquitetura_OpenCore_v1.3.md 02_Arquitetura_OpenCore_v1.md
cmp -s 05_Roadmap_OpenCore_v2.3.md 05_Roadmap_OpenCore_v2.md
```

---

# 18. Commit recomendado

Criar um único commit focado:

```bash
git add README.md \
  00_Indice_Versoes.md \
  CONTRIBUTING.md \
  DCO.md \
  .github/pull_request_template.md \
  02_Arquitetura_OpenCore_v1.3.md \
  02_Arquitetura_OpenCore_v1.md \
  03_Comunidade_Governanca_OpenCore_v1.0.md \
  04_Plano_Institucional_OpenCore_v1.0.md \
  05_Roadmap_OpenCore_v2.3.md \
  05_Roadmap_OpenCore_v2.md \
  06_Especificacao_OpenCore_Builder_v0.md \
  07_Benchmarks_Ecossistema_OpenCore_v1.0.md \
  ADR-017_Niveis_Confianca_Modulos.md \
  ADR-022_OpenCore_Builder_Triagem_Composicao_Empacotamento.md \
  docs/history/INSTRUCOES_CURSOR_BUILDER_2026-07-24.md

git commit -s -m "docs: reconcile canonical status and review findings"
```

Se algum arquivo listado não precisar de alteração, não adicioná-lo artificialmente.

Não incluir mudanças não relacionadas.

---

# 19. Saída esperada do Cursor

Ao concluir, retornar:

1. arquivos alterados;
2. arquivo movido;
3. referências antigas encontradas;
4. referências antigas preservadas por serem históricas;
5. formulação final do estado da Etapa 0;
6. alterações de status em Comunidade e Plano;
7. locais em que `trust_level` foi padronizado;
8. correção de Frappe/ERPNext;
9. regra de transição do DCO;
10. erros editoriais corrigidos;
11. comandos de validação executados;
12. confirmação de que os espelhos são idênticos;
13. confirmação de que nenhuma implementação foi iniciada;
14. hash do commit criado.

---

# 20. Critério de conclusão

Esta tarefa estará concluída quando:

- as versões canônicas estiverem corretamente referenciadas;
- o estado da Etapa 0 for descrito da mesma forma em todos os documentos públicos;
- alterações normativas de 2026-07-24 estiverem marcadas como pendentes de ratificação;
- `trust_level` estiver tecnicamente padronizado em T0–T3;
- Frappe e ERPNext tiverem licenças separadas corretamente;
- a política de fontes do benchmark estiver registrada;
- a transição do DCO estiver clara;
- o arquivo de instruções executadas estiver arquivado;
- não houver erros editoriais conhecidos;
- o diff não iniciar implementação ou mudar decisões arquiteturais centrais.
