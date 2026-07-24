# Smoke test nativo — Spike 01 (Nível C)

Usar **artefato limpo** do GitHub Actions (ou build nativo local no mesmo SO).  
Preencher uma linha por execução e copiar o resumo para `EVIDENCIAS.md`.

Artefatos de referência: [run 30112186838](https://github.com/duvallemusic/open-core/actions/runs/30112186838)  
Cópia local (gitignored): `spikes/01-multiplataforma/artifacts/`

## Checklist

| # | Passo | macOS | Linux x64 | Windows x64 |
|---|---|---|---|---|
| 1 | Baixar artefato / build nativo (anotar hash) | ☑ | ☑ (baixado; teste pendente) | ☑ (baixado; teste pendente) |
| 2 | Executar sem instalar toolchain Rust | ☑ parcial | ☐ | ☐ |
| 3 | Janela Slint abre e renderiza | ☐ confirmar visual | ☐ | ☐ |
| 4 | Textos/fontes/escala legíveis | ☐ | ☐ | ☐ |
| 5 | Interagir (foco, redimensionar ou fechar pela UI) | ☐ | ☐ | ☐ |
| 6 | Encerrar sem travar | ☑ SIGTERM limpo | ☐ | ☐ |
| 7 | Sem console inesperado (Windows) / libs ausentes (Linux) | — | ☐ | ☐ (PE marcado console) |
| 8 | Captura de tela + versão SO/arch anexadas | ☐ (falhou no agente) | ☐ | ☐ |

## Como copiar para a máquina dual-boot

Os binários já estão em `spikes/01-multiplataforma/artifacts/` neste clone. Alternativa:

```bash
gh run download 30112186838 --repo duvallemusic/open-core --dir ./spike01-artifacts
```

- Linux: `artifacts/spike01-ubuntu-latest/spike01-multiplataforma`
- Windows: `artifacts/spike01-windows-latest/spike01-multiplataforma.exe`

## Registro rápido

```text
Data:
SO / versão / arch:
Origem do binário: Actions run 30112186838
SHA-256:
Resultado: OK / FALHA
Notas:
```

## Notas de arquitetura

- VM Linux/Windows **ARM64** no Mac M4 = evidência preliminar útil; **não** substitui Linux/Windows **x64** nativo.
- Executar x64 sob emulação em Windows ARM = útil; **não** equivale a Windows x64 nativo.
- Aceite integral do Spike 01 exige Nível C em macOS + Linux x64 + Windows x64 (ou bloqueio documentado ≤ 1 SO).
