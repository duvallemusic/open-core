# Índice de versões — decisões OpenCore

Este arquivo aponta as versões **canônicas** (em uso) e os **snapshots históricos** preservados.

**Última consolidação:** 2026-07-23 — fusão Arquitetura v1.1 + linha multilíngue → v1.2; módulos em processo renumerados para **ADR-021**.

---

## Canônicos (usar estes)

| Documento | Arquivo | Versão |
|---|---|---|
| Manifesto | `01_Manifesto_OpenCore_v1.1_licenciamento.md` | 1.1 |
| Arquitetura | `02_Arquitetura_OpenCore_v1.2.md` | 1.2 |
| Comunidade e Governança | `03_Comunidade_OpenCore.md` | rascunho — **próxima entrega = v1.0** |
| Plano Institucional | `04_Plano_Institucional_OpenCore.md` | rascunho — próxima entrega = v1.0 |
| Roadmap | `05_Roadmap_OpenCore_v2.2.md` | 2.2 |
| ADR-015 | `ADR-015_Matriz_Classificacao_Arquitetural.md` | Proposto |
| ADR-016 | `ADR-016_Portabilidade_Exclusao_Modulos.md` | Proposto |
| ADR-017 | `ADR-017_Niveis_Confianca_Modulos.md` | Proposto |
| ADR-018 | `ADR-018_Atualizacao_Estrutural_Canais.md` | Proposto |
| ADR-019 | `ADR-019_Sincronizacao_Como_Adaptador.md` | Proposto |
| ADR-020 | `ADR-020_Testes_Arquitetura_CI.md` | Proposto |
| ADR-021 | `ADR-021_Modulos_Nativos_Processo_Protocolo_v1.1.md` | 1.1 · Proposto, condicionado a spike |

Espelhos de conveniência (apontam para o canônico mais recente quando sincronizados):

- `02_Arquitetura_OpenCore_v1.md` → espelha **1.2**
- `05_Roadmap_OpenCore_v2.md` → espelha **2.2**

---

## Histórico preservado

| Documento | Arquivo | Notas |
|---|---|---|
| Manifesto (rascunho) | `01_Manifesto_OpenCore.md` | pré-v1.1 |
| Arquitetura (rascunho) | `02_Arquitetura_OpenCore.md` | pré-v1 |
| Arquitetura 1.0.1 | `02_Arquitetura_OpenCore_v1.0.1.md` | linha divergente (só ADR multilíngue v1) |
| Arquitetura 1.0.2 | `02_Arquitetura_OpenCore_v1.0.2.md` | linha divergente (multilíngue P0–P2); **não substitui** a v1.1 |
| Arquitetura 1.1 | `02_Arquitetura_OpenCore_v1.1.md` | base correta pré-consolidação (LGPD, matriz, ADRs 015–020) |
| Roadmap v2.0 | `05_Roadmap_OpenCore_v2.0.md` | menção parcial multilíngue |
| Roadmap v2.1 | `05_Roadmap_OpenCore_v2.1.md` | multilíngue com numeração ADR incorreta (015) |
| ADR-015 módulos* (errata) | `ADR-015_Modulos_Nativos_*` | **não canônico** — conteúdo → ADR-021 |
| Legados | `OpenCore_Proposta.md`, `OpenCore_Roadmap_Arquitetura.md` | supersedidos |

---

## Numeração ADR (corrigida)

| ID | Tema |
|---|---|
| ADR-015 | Matriz runtime × módulo-base × adaptador |
| ADR-016 | Portabilidade e exclusão por módulo |
| ADR-017 | Níveis de confiança |
| ADR-018 | Atualização estrutural vs canais |
| ADR-019 | Sincronização como adaptador |
| ADR-020 | Testes de arquitetura no CI |
| ADR-021 | Módulos nativos, em processo e protocolo neutro |

Não renumerar ADR-015..020. O conteúdo multilíngue usa **ADR-021**.

---

## Próxima etapa (Etapa 0)

Não iniciar Spike 10 ainda.

1. ~~Manifesto v1.1~~
2. ~~Arquitetura v1.2 + ADR-015..021~~
3. **Comunidade e Governança OpenCore v1.0** ← próximo
4. Plano Institucional OpenCore v1.0
5. Roadmap v2.2 já consolidado; revisar após os itens 3–4 se necessário
