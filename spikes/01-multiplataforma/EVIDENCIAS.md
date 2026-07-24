# Evidências — Spike 01

**Status do spike:** Níveis **A e B concluídos** nos três SO (Actions run `30112186838`); Nível **C macOS parcial** (artefato nativo ARM64 inicia e permanece vivo); Nível **C Linux/Windows x64 pendente** na máquina dual-boot  
**Estratégia:** Mac = desenvolvimento; GitHub Actions = build/teste nativo x64 Linux/Windows; smoke GUI = Nível C manual  
**Aceite integral:** somente após Nível C nos três sistemas (ver `SMOKE_TEST.md`)

## Matriz por nível

| SO / alvo | A — Build | B — Automatizado | C — Smoke nativo |
|---|---|---|---|
| macOS ARM64 (artefato CI + local) | **OK** | **OK** (CI) | **parcial** — processo GUI vivo ~3s; captura de tela bloqueada neste ambiente |
| macOS (`macos-latest` CI) | **OK** | **OK** | — (CI não substitui C visual) |
| Linux x64 (`ubuntu-latest`) | **OK** | **OK** | **pendente** — executar artefato na máquina Linux x64 |
| Windows x64 (`windows-latest`) | **OK** | **OK** | **pendente** — executar artefato na máquina Windows x64 |

## Toolchain / run de referência

- Actions: [run 30112186838](https://github.com/duvallemusic/open-core/actions/runs/30112186838) (branch `cursor/archive-post-review-instructions`, sucesso)
- `rustc` / `cargo` **1.97.1** nos três runners
- `slint` **1.17**
- Artefatos baixados em `spikes/01-multiplataforma/artifacts/` (gitignored)

## Artefatos CI (Nível A)

| OS | Arch | Binário | Tamanho | SHA-256 |
|---|---|---|---:|---|
| macOS | ARM64 | `spike01-multiplataforma` | 12 591 792 | `51df2bc5dd4da81d482f0c1d977cf95ee0d4bc9a12e39489a7935f832a48c85f` |
| Linux | X64 | `spike01-multiplataforma` | 26 212 928 | `47cd16cda5e80b2d6749232f1851d39f0d5b57baee4a135f70a83322513f7cce` |
| Windows | X64 | `spike01-multiplataforma.exe` | 11 552 256 | `968613d85d1dc5668d87cac355c796c8cd60b34f59834125a56deac878f869e5` |

Verificação local `file(1)`:

- macOS: `Mach-O 64-bit executable arm64`
- Linux: `ELF 64-bit LSB pie executable, x86-64` (dinâmico)
- Windows: `PE32+ executable (console) x86-64` — **nota:** marcado como *console*; no Nível C Windows verificar se abre console inesperado

Hashes locais conferem com `BUILD_INFO.txt` de cada artefato.

## Nível B — Automatizado

Workflow `spike01-multiplataforma` em cada OS:

- `cargo check` — OK
- `cargo test` — OK
- `cargo build --release` — OK
- upload de artefato + `BUILD_INFO.txt` — OK

## Nível C — Smoke nativo

### macOS ARM64 (2026-07-24, ambiente do agente)

```text
Data: 2026-07-24
SO / versão / arch: macOS (darwin) / ARM64 (Apple Silicon)
Origem do binário: Actions run 30112186838 · spike01-macos-latest
SHA-256: 51df2bc5dd4da81d482f0c1d977cf95ee0d4bc9a12e39489a7935f832a48c85f
Resultado: PARCIAL
Notas:
- Executado sem toolchain no PATH do processo (binário release do artefato).
- Processo permaneceu vivo após 3s (janela presumida); encerrado com SIGTERM limpo.
- screencapture falhou neste ambiente ("could not create image from display").
- Confirmar visualmente no Mac do maintainer: texto, escala, fechar pela UI.
```

### Linux x64 — pendente (máquina dual-boot / VM x64)

```text
Data:
SO / versão / arch: Linux … / x86_64
Origem do binário: Actions run 30112186838 · spike01-ubuntu-latest
SHA-256: 47cd16cda5e80b2d6749232f1851d39f0d5b57baee4a135f70a83322513f7cce
Resultado: pendente
Notas: copiar spikes/01-multiplataforma/artifacts/spike01-ubuntu-latest/ para a máquina Linux.
Comando sugerido:
  chmod +x spike01-multiplataforma
  ./spike01-multiplataforma
Se faltar lib: anotar ldd / mensagem de erro (libxcb, libxkbcommon, fontconfig…).
```

### Windows x64 — pendente (máquina dual-boot / VM x64)

```text
Data:
SO / versão / arch: Windows … / x64
Origem do binário: Actions run 30112186838 · spike01-windows-latest
SHA-256: 968613d85d1dc5668d87cac355c796c8cd60b34f59834125a56deac878f869e5
Resultado: pendente
Notas: copiar spike01-multiplataforma.exe; executar por Explorer ou:
  .\spike01-multiplataforma.exe
Verificar: janela Slint, console inesperado (PE console), fechar sem travar.
```

## Memória / startup

_A preencher após smoke GUI completo nos três SO._

## Bloqueios / decisões

- Cross-compile a partir do Mac **não** foi usado como critério.
- A/B no Actions **OK**; C visual Linux/Windows exige execução nativa x64.
- Artefato Windows é PE *console* — possível ajuste futuro (`windows_subsystem = "windows"`), fora do aceite mínimo se a GUI abrir.

## Decisão final

_Ainda não encerrado_ — falta Nível C completo (macOS visual confirmado + Linux x64 + Windows x64).  
**Próximo passo humano:** na máquina dual-boot, rodar os dois artefatos x64 e colar os registros acima preenchidos.
