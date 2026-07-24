# Índice de versões — decisões OpenCore

Este arquivo aponta as versões **canônicas** (em uso) e os **snapshots históricos** preservados.

**Última consolidação:** 2026-07-23 — Plano Institucional v1.0 aprovado;
artefatos operacionais publicados; licença documental CC BY 4.0 em vigor;
RFC-0001 em consulta pública até 2026-08-22; Etapa 1 autorizada a iniciar
com spikes controlados.

---

## Canônicos (usar estes)

| Documento | Arquivo | Versão / estado |
|---|---|---|
| Manifesto | `01_Manifesto_OpenCore_v1.1_licenciamento.md` | 1.1 |
| Arquitetura | `02_Arquitetura_OpenCore_v1.2.md` | 1.2 |
| Comunidade e Governança | `03_Comunidade_Governanca_OpenCore_v1.0.md` | 1.0 · **Aprovado** |
| Plano Institucional | `04_Plano_Institucional_OpenCore_v1.0.md` | 1.0 · **Aprovado** |
| Roadmap | `05_Roadmap_OpenCore_v2.2.md` | 2.2 |
| Licença documental | `LICENSE` · `rfcs/0001-licenca-documentacao.md` | CC BY 4.0 em vigor · RFC-0001 em consulta |
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
5. Licença documental publicada; RFC-0001 em consulta até 2026-08-22.

**Próximo:** Etapa 1 — spikes técnicos controlados. A consulta da RFC-0001
prossegue em paralelo e deverá ser encerrada formalmente antes de a decisão
ser marcada como aceita.
