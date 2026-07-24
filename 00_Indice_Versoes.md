# Índice de versões — decisões OpenCore

Este arquivo aponta as versões **canônicas** (em uso) e os **snapshots históricos** preservados.

**Última consolidação:** 2026-07-24 — documentação essencial da Etapa 0 consolidada;
Manifesto 1.2, Arquitetura 1.3, Roadmap 2.3 e ADR-022 em revisão formal;
alterações de Comunidade/Plano de 2026-07-24 pendentes de ratificação;
RFC-0001 em consulta pública até 2026-08-22.

---

## Canônicos (usar estes)

| Documento | Arquivo | Versão / estado |
|---|---|---|
| Manifesto | `01_Manifesto_OpenCore_v1.2.md` | 1.2 · Revisão formal pendente |
| Arquitetura | `02_Arquitetura_OpenCore_v1.3.md` | 1.3 · Revisão formal pendente |
| Comunidade e Governança | `03_Comunidade_Governanca_OpenCore_v1.0.md` | 1.0 · Aprovado originalmente; alterações de 2026-07-24 pendentes de ratificação |
| Plano Institucional | `04_Plano_Institucional_OpenCore_v1.0.md` | 1.0 · Aprovado originalmente; alterações de 2026-07-24 pendentes de ratificação |
| Roadmap | `05_Roadmap_OpenCore_v2.3.md` | 2.3 · Revisão formal pendente |
| Especificação OpenCore Builder | `06_Especificacao_OpenCore_Builder_v0.md` | proposta v0 |
| Benchmarks do ecossistema | `07_Benchmarks_Ecossistema_OpenCore_v1.0.md` | 1.0 · referência **não normativa** |
| Licença documental | `LICENSE` · `rfcs/0001-licenca-documentacao.md` | CC BY 4.0 em vigor · RFC-0001 em consulta |
| ADR-015 | `ADR-015_Matriz_Classificacao_Arquitetural.md` | Proposto |
| ADR-016 | `ADR-016_Portabilidade_Exclusao_Modulos.md` | Proposto |
| ADR-017 | `ADR-017_Niveis_Confianca_Modulos.md` | Proposto |
| ADR-018 | `ADR-018_Atualizacao_Estrutural_Canais.md` | Proposto |
| ADR-019 | `ADR-019_Sincronizacao_Como_Adaptador.md` | Proposto |
| ADR-020 | `ADR-020_Testes_Arquitetura_CI.md` | Proposto |
| ADR-021 | `ADR-021_Modulos_Nativos_Processo_Protocolo_v1.1.md` | 1.1 · Proposto, condicionado a spike |
| ADR-022 | `ADR-022_OpenCore_Builder_Triagem_Composicao_Empacotamento.md` | Proposto, condicionado aos Spikes 14–18 |

### Artefatos operacionais (raiz)

| Artefato | Arquivo |
|---|---|
| Contribuição | `CONTRIBUTING.md` |
| Conduta | `CODE_OF_CONDUCT.md` |
| Segurança | `SECURITY.md` |
| Governança (entrada) | `GOVERNANCE.md` |
| Mantenedores | `MAINTAINERS.md` |
| DCO | `DCO.md` |
| Guias de ADR | `docs/adr/` |
| RFCs | `rfcs/` |
| Templates GitHub | `.github/` |

Espelhos de conveniência:

- `02_Arquitetura_OpenCore_v1.md` → espelha **1.3**
- `05_Roadmap_OpenCore_v2.md` → espelha **2.3**

---

## Histórico preservado

| Documento | Arquivo | Notas |
|---|---|---|
| Manifesto 1.1 | `01_Manifesto_OpenCore_v1.1_licenciamento.md` | supersedido pela 1.2 |
| Manifesto (rascunho) | `01_Manifesto_OpenCore.md` | pré-v1.1 |
| Arquitetura 1.2 | `02_Arquitetura_OpenCore_v1.2.md` | supersedida pela 1.3 |
| Arquitetura (rascunho) | `02_Arquitetura_OpenCore.md` | pré-v1 |
| Comunidade (rascunho) | `03_Comunidade_OpenCore_rascunho.md` | supersedido pela v1.0 |
| Plano institucional (rascunho) | `04_Plano_Institucional_OpenCore_rascunho.md` | supersedido pela v1.0 |
| Arquitetura 1.0.1 / 1.0.2 | `02_Arquitetura_OpenCore_v1.0.*.md` | linha divergente |
| Arquitetura 1.1 | `02_Arquitetura_OpenCore_v1.1.md` | base pré-consolidação 1.2 |
| Roadmap 2.2 | `05_Roadmap_OpenCore_v2.2.md` | supersedido pela 2.3 |
| Roadmaps anteriores | `05_Roadmap_OpenCore_v2.0.md`, `v2.1.md` | histórico |
| Pacote operacional (checklist) | `docs/PACOTE_REVISAO_historico.md` | itens pendentes resolvidos na integração |
| Instruções Cursor — Builder | `docs/history/INSTRUCOES_CURSOR_BUILDER_2026-07-24.md` | roteiro executado; não normativo |
| ADR-015 módulos* (errata) | `ADR-015_Modulos_Nativos_*` | conteúdo → ADR-021 |
| Legados | `OpenCore_Proposta.md`, `OpenCore_Roadmap_Arquitetura.md` | supersedidos |

---

## Numeração ADR

| ID | Tema |
|---|---|
| ADR-015 | Matriz runtime × módulo-base × adaptador |
| ADR-016 | Portabilidade e exclusão por módulo |
| ADR-017 | Níveis de confiança |
| ADR-018 | Atualização estrutural vs canais |
| ADR-019 | Sincronização como adaptador |
| ADR-020 | Testes de arquitetura no CI |
| ADR-021 | Módulos nativos, em processo e protocolo neutro |
| ADR-022 | OpenCore Builder — triagem, composição e empacotamento |

---

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

---

## Próxima etapa

1. Revisar formalmente Manifesto 1.2, Arquitetura 1.3, Roadmap 2.3 e ADR-022.
2. Ratificar ou ajustar as alterações de 2026-07-24 em Comunidade/Governança e Plano Institucional.
3. Não iniciar implementação completa do OpenCore Builder.
4. Iniciar a Etapa 1 apenas pelos spikes técnicos priorizados.
5. Priorizar a fatia vertical comum (Portaria como distribuição de referência).
6. Executar Spike 10 (módulos em processo) quando priorizado.
7. Executar CLI / lockfile / instalador de forma time-boxed (Spikes 12–15).
8. Documentar decisões e só então avançar o Builder baseado em regras.

A consulta da RFC-0001 prossegue em paralelo e deverá ser encerrada formalmente antes de a decisão ser marcada como aceita.
