# Evidências — Spike 01

**Status do spike:** Nível A parcial (macOS local OK); Linux/Windows A/B via CI; Nível C pendente em todos os SO  
**Estratégia:** Mac = desenvolvimento; GitHub Actions = build/teste nativo x64 Linux/Windows; smoke GUI = Nível C manual  
**Aceite integral:** somente após Nível C nos três sistemas (ver `SMOKE_TEST.md`)

## Matriz por nível

| SO / alvo | A — Build | B — Automatizado | C — Smoke nativo |
|---|---|---|---|
| macOS ARM64 (local M4) | OK (`cargo build`, 2026-07-24) | parcial (`cargo test` local) | pendente |
| macOS (`macos-latest` CI) | pendente (aguardar Actions) | pendente | — (CI não substitui C) |
| Linux x64 (`ubuntu-latest`) | pendente | pendente | pendente (máquina/VM x64) |
| Windows x64 (`windows-latest`) | pendente | pendente | pendente (máquina/VM x64) |
| Linux ARM64 VM no Mac | n/a como autoridade x64 | n/a | opcional preliminar |
| Windows 11 ARM VM no Mac | n/a como autoridade x64 | n/a | opcional preliminar |

## Toolchain (macOS local)

- `rustc 1.97.1 (8bab26f4f 2026-07-14)`
- `slint` **1.17**
- crate: `spikes/01-multiplataforma`
- workflow: `.github/workflows/spike01-multiplataforma.yml`

## Artefatos CI

| Run | OS | SHA-256 | Tamanho | Link |
|---|---|---|---|---|
| _preencher após primeiro workflow_ | | | | |

## Memória / startup (Nível C)

_A preencher após smoke GUI._

## Bloqueios / decisões

- Cross-compile a partir do Mac **não** é critério de aceite para Windows/Linux.
- Build no Actions prova Níveis A/B; **não** prova renderização, DLLs em máquina limpa nem instalador.
- ARM64 (VM no Mac) ≠ x64 desktop típico.

## Decisão final

_Ainda não encerrado_ — concluir A/B no Actions; depois C nativo (sua máquina Linux/Windows x64 é o caminho certo para fechar o spike).
