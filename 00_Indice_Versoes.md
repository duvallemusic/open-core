# Índice de versões — decisões OpenCore

Este arquivo aponta as versões **canônicas** (em uso) e os **snapshots históricos** preservados.

**Última consolidação:** 2026-07-24 — revisão formal da Etapa 0 concluída
([`docs/REVISAO_FORMAL_ETAPA0_2026-07-24.md`](docs/REVISAO_FORMAL_ETAPA0_2026-07-24.md));
Manifesto 1.2, Arquitetura 1.3 e Roadmap 2.3 aprovados;
ADR-022 proposta formalmente revisada (permanece Proposto, Spikes 14–18);
alterações de Comunidade/Plano de 2026-07-24 ratificadas;
Etapa 1 iniciada em `spikes/`; RFC-0001 em consulta pública até 2026-08-22;
OpenCore Builder completo **não** autorizado para implementação.

---

## Canônicos (usar estes)

| Documento | Arquivo | Versão / estado |
|---|---|---|
| Manifesto | `01_Manifesto_OpenCore_v1.2.md` | 1.2 · **Aprovado** |
| Arquitetura | `02_Arquitetura_OpenCore_v1.3.md` | 1.3 · **Aprovado** (hipóteses condicionadas a spikes) |
| Comunidade e Governança | `03_Comunidade_Governanca_OpenCore_v1.0.md` | 1.0 · Aprovado; alterações 2026-07-24 **ratificadas** |
| Plano Institucional | `04_Plano_Institucional_OpenCore_v1.0.md` | 1.0 · Aprovado; alterações 2026-07-24 **ratificadas** |
| Roadmap | `05_Roadmap_OpenCore_v2.3.md` | 2.3 · **Aprovado** |
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
| ADR-022 | `ADR-022_OpenCore_Builder_Triagem_Composicao_Empacotamento.md` | Proposto (proposta revisada), condicionado aos Spikes 14–18 |

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
| Spikes (Etapa 1) | `spikes/` |
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
| Revisão formal Etapa 0 | `docs/REVISAO_FORMAL_ETAPA0_2026-07-24.md` | registro de aprovação/ratificação |
| Instruções Cursor — Builder | `docs/history/INSTRUCOES_CURSOR_BUILDER_2026-07-24.md` | roteiro executado; não normativo |
| Instruções Cursor — Correções pós-revisão | `docs/history/INSTRUCOES_CURSOR_CORRECOES_2026-07-24.md` | roteiro executado; não normativo |
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

A documentação essencial da Etapa 0 foi **aprovada** na revisão formal de 2026-07-24.

1. Manifesto 1.2 — **aprovado**.
2. Arquitetura 1.3 — **aprovada** (hipóteses tecnológicas condicionadas a spikes).
3. Comunidade e Governança 1.0 — aprovada; alterações de 2026-07-24 **ratificadas**.
4. Plano Institucional 1.0 — aprovado; alterações de 2026-07-24 **ratificadas**.
5. Roadmap 2.3 — **aprovado**.
6. ADR-022 — proposta formalmente revisada; permanece Proposto e condicionado aos Spikes 14–18.
7. Especificação Builder v0 e Benchmarks v1.0 foram criados.
8. Licença documental publicada; RFC-0001 em consulta até 2026-08-22.

A Etapa 1 foi **iniciada** apenas para spikes técnicos reversíveis, documentados e time-boxed (`spikes/`).

O OpenCore Builder completo **não** está autorizado para implementação.

---

## Próxima etapa

1. Executar o backlog em `spikes/` (Spikes 01–09 → Spike 10 → Spikes 12–13 e 15).
2. Manter Spikes 14, 16, 17 e 18 **adiados** (Builder/preview/IA/montagem avançada).
3. Não iniciar implementação completa do OpenCore Builder.
4. Priorizar a fatia vertical comum (Portaria como distribuição de referência) após evidências mínimas da stack.
5. Documentar decisões de cada spike (aceito / rejeitado / adiado / inconclusivo).
6. Encerrar formalmente a RFC-0001 após 2026-08-22, sem marcar como aceita antes disso.
