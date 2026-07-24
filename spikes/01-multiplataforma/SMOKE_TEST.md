# Smoke test nativo — Spike 01 (Nível C)

Usar **artefato limpo** do GitHub Actions (ou build nativo local no mesmo SO).  
Preencher uma linha por execução e copiar o resumo para `EVIDENCIAS.md`.

## Checklist

| # | Passo | macOS | Linux x64 | Windows x64 |
|---|---|---|---|---|
| 1 | Baixar artefato / build nativo (anotar hash) | ☐ | ☐ | ☐ |
| 2 | Executar sem instalar toolchain Rust | ☐ | ☐ | ☐ |
| 3 | Janela Slint abre e renderiza | ☐ | ☐ | ☐ |
| 4 | Textos/fontes/escala legíveis | ☐ | ☐ | ☐ |
| 5 | Interagir (foco, redimensionar ou fechar pela UI) | ☐ | ☐ | ☐ |
| 6 | Encerrar sem travar | ☐ | ☐ | ☐ |
| 7 | Sem console inesperado (Windows) / libs ausentes (Linux) | ☐ | ☐ | ☐ |
| 8 | Captura de tela + versão SO/arch anexadas | ☐ | ☐ | ☐ |

## Registro rápido

```text
Data:
SO / versão / arch:
Origem do binário: (Actions run # / build local)
SHA-256:
Resultado: OK / FALHA
Notas:
```

## Notas de arquitetura

- VM Linux/Windows **ARM64** no Mac M4 = evidência preliminar útil; **não** substitui Linux/Windows **x64** nativo.
- Executar x64 sob emulação em Windows ARM = útil; **não** equivale a Windows x64 nativo.
- Aceite integral do Spike 01 exige Nível C em macOS + Linux x64 + Windows x64 (ou bloqueio documentado ≤ 1 SO).
