# Spikes técnicos — Etapa 1

Protótipos **descartáveis ou isolados** para validar hipóteses de maior risco antes do produto.

**Regras:**

1. Todo spike é time-boxed e deve terminar como **aceito**, **rejeitado**, **adiado** ou **inconclusivo** com evidências.
2. Código de spike **não** é runtime de produção só porque compilou.
3. O OpenCore Builder completo **não** está autorizado; Spikes 14/16/17/18 permanecem adiados.
4. Prioridade: ver [`BACKLOG.md`](BACKLOG.md).
5. Base normativa: Arquitetura v1.3 §33 · Roadmap v2.3 · [`docs/REVISAO_FORMAL_ETAPA0_2026-07-24.md`](../docs/REVISAO_FORMAL_ETAPA0_2026-07-24.md).

## Estado

| Item | Estado |
|---|---|
| Backlog Etapa 1 | ativo |
| Spike 01 — Multiplataforma | **em andamento** — Nível A macOS OK; CI A/B preparado; Nível C pendente |
| Spikes 02–09 | pendentes |
| Spike 10 (ADR-021) | pendente (após 01–09 ou em paralelo se capacity) |
| Spikes 12–13, 15 | autorizados time-boxed após base mínima |
| Spikes 14, 16–18 | **adiados** |
| Spike 11 | futuro (pré-condição: Spike 10 aceito) |

## Licença do código em spikes

Trechos e scaffolds sob **Apache 2.0**, salvo cabeçalho diferente (alinhado à política documental / RFC-0001).
