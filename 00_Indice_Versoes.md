# Índice de versões — decisões OpenCore

Este arquivo aponta as versões **canônicas** (em uso) e os **snapshots históricos** preservados.

**Última consolidação:** 2026-07-23 — Plano Institucional v1.0 aprovado; artefatos operacionais na raiz; RFC-0001 (CC BY 4.0) aceita; Etapa 0 documental essencial concluída.

---

## Canônicos (usar estes)

| Documento | Arquivo | Versão / estado |
|---|---|---|
| Manifesto | `01_Manifesto_OpenCore_v1.1_licenciamento.md` | 1.1 |
| Arquitetura | `02_Arquitetura_OpenCore_v1.2.md` | 1.2 |
| Comunidade e Governança | `03_Comunidade_Governanca_OpenCore_v1.0.md` | 1.0 · **Aprovado** |
| Plano Institucional | `04_Plano_Institucional_OpenCore_v1.0.md` | 1.0 · **Aprovado** |
| Roadmap | `05_Roadmap_OpenCore_v2.2.md` | 2.2 |
| Licença documental | `LICENSE` · `rfcs/0001-licenca-documentacao.md` | CC BY 4.0 (+ Apache 2.0 em código de docs) |
| ADR-015 | `ADR-015_Matriz_Classificacao_Arquitetural.md` | Proposto |
| ADR-016 | `ADR-016_Portabilidade_Exclusao_Modulos.md` | Proposto |
| ADR-017 | `ADR-017_Niveis_Confianca_Modulos.md` | Proposto |
| ADR-018 | `ADR-018_Atualizacao_Estrutural_Canais.md` | Proposto |
| ADR-019 | `ADR-019_Sincronizacao_Como_Adaptador.md` | Proposto |
| ADR-020 | `ADR-020_Testes_Arquitetura_CI.md` | Proposto |
| ADR-021 | `ADR-021_Modulos_Nativos_Processo_Protocolo_v1.1.md` | 1.1 · Proposto, condicionado a spike |

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

- `02_Arquitetura_OpenCore_v1.md` → espelha **1.2**
- `05_Roadmap_OpenCore_v2.md` → espelha **2.2**

---

## Histórico preservado

| Documento | Arquivo | Notas |
|---|---|---|
| Manifesto (rascunho) | `01_Manifesto_OpenCore.md` | pré-v1.1 |
| Arquitetura (rascunho) | `02_Arquitetura_OpenCore.md` | pré-v1 |
| Comunidade (rascunho) | `03_Comunidade_OpenCore_rascunho.md` | supersedido pela v1.0 |
| Plano institucional (rascunho) | `04_Plano_Institucional_OpenCore_rascunho.md` | supersedido pela v1.0 |
| Arquitetura 1.0.1 / 1.0.2 | `02_Arquitetura_OpenCore_v1.0.*.md` | linha divergente |
| Arquitetura 1.1 | `02_Arquitetura_OpenCore_v1.1.md` | base pré-consolidação 1.2 |
| Roadmaps anteriores | `05_Roadmap_OpenCore_v2.0.md`, `v2.1.md` | histórico |
| Pacote operacional (checklist) | `docs/PACOTE_REVISAO_historico.md` | itens pendentes resolvidos na integração |
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

---

## Etapa 0 — estado

1. ~~Manifesto v1.1~~
2. ~~Arquitetura v1.2 + ADR-015..021~~
3. ~~Comunidade e Governança v1.0~~
4. ~~Plano Institucional v1.0~~
5. ~~Artefatos operacionais + RFC-0001 (licença documental)~~

**Próximo:** Etapa 1 — spikes técnicos (incluindo Spike 10 / ADR-021 quando priorizado), sem antecipar runtime de produção antes das evidências.
