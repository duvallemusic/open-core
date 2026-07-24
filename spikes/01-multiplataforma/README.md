# Spike 01 — Aplicação mínima multiplataforma

**Status:** em andamento — Nível A local (macOS) OK; Níveis A/B Linux/Windows via CI; Nível C pendente  
**Time-box:** 3 pessoa-dias  
**Hipótese:** Rust + Slint compilam e executam em Windows, Linux e macOS com esforço aceitável para contribuição.

## Objetivo

Compilar e executar uma aplicação Rust com interface Slint nos três sistemas operacionais de referência.

## Estratégia de validação

| Ambiente | Onde compilar | Onde testar |
|---|---|---|
| macOS ARM64 | Mac local (+ `macos-latest`) | Mac local |
| Linux x64 | GitHub Actions `ubuntu-latest` | Actions (A/B) + smoke nativo (C) |
| Windows x64 | GitHub Actions `windows-latest` | Actions (A/B) + smoke nativo (C) |

**Não** usar cross-compile a partir do Mac como validação principal de Windows/Linux (linker, backend gráfico, DLL/MSVC vs GNU). Cross-build ≠ teste nativo.

Workflow: [`.github/workflows/spike01-multiplataforma.yml`](../../.github/workflows/spike01-multiplataforma.yml)

### Níveis de evidência

| Nível | O que prova | Quem executa |
|---|---|---|
| **A — Build** | `cargo build --release` nos três SO; artefatos no Actions | Mac + GitHub Actions |
| **B — Automatizado** | `cargo check` / `cargo test`; `.slint` compila; metadados/hashes | GitHub Actions |
| **C — Smoke nativo** | Abrir janela, interagir, encerrar, máquina limpa | Pessoa em SO nativo (VM preliminar OK; x64 real antes do aceite integral) |

O spike só deve ser marcado **aceito integralmente** após o **Nível C** nos três sistemas. A e B podem (e devem) ser concluídos antes, só com Mac + Actions.

## Evidências esperadas

- builds e artefatos (CI + local);
- `EVIDENCIAS.md` com status A/B/C por SO;
- dificuldades por sistema / arquitetura (ARM64 vs x64);
- checklist de smoke test (`SMOKE_TEST.md`).

## Critérios de aceite / rejeição

| Resultado | Quando |
|---|---|
| Aceito | Níveis A+B+C nos três SO, ou bloqueio documentado em no máximo um SO com plano claro |
| Rejeitado | Dois ou mais SO bloqueados após o time-box sem caminho reproduzível |
| Inconclusivo | Time-box esgotado sem evidência suficiente — registrar e parar |

## Pré-requisitos locais (macOS)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cd spikes/01-multiplataforma
cargo build
cargo run
```

Documentação Slint: https://slint.dev/docs

## Escopo deste diretório

- protótipo descartável;
- **sem** Module Host, SQLite, Builder ou protocolo;
- UI mínima (janela + texto) basta;
- sem cadeia de cross-compilation Windows/Linux no Mac como critério de aceite.

## Próximas ações

1. Disparar o workflow `spike01-multiplataforma` (push deste diretório ou `workflow_dispatch`).
2. Baixar artefatos Linux/Windows do Actions e preencher Nível A/B em `EVIDENCIAS.md`.
3. Smoke preliminar em VM Linux/Windows no Mac (útil, não substitui x64 nativo).
4. Nível C em Linux x64 e Windows x64 reais (máquina dual-boot ou remota).
5. Só então decidir aceito / rejeitado / inconclusivo.

## Fora de escopo

- OpenCore Builder;
- módulos em processo;
- instalador de distribuição (quando entrar, entra no Nível C com install/uninstall);
- código definitivo do runtime.
