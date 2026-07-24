# Revisão formal — pacote documental Etapa 0

**Data:** 2026-07-24  
**Autoridade:** Lead Maintainer interino (Estágio F) — Bruno Costa (`@duvallemusic`)  
**Escopo:** Manifesto 1.2 · Arquitetura 1.3 · Roadmap 2.3 · ADR-022 · ratificação das alterações de 2026-07-24 em Comunidade/Governança e Plano Institucional

---

## Veredito

**Aprovar com ressalvas menores** (nenhum bloqueador normativo).

| Documento | Decisão |
|---|---|
| Manifesto OpenCore v1.2 | **Aprovado** |
| Arquitetura OpenCore v1.3 | **Aprovado** (hipóteses tecnológicas permanecem condicionadas a spikes) |
| Roadmap OpenCore v2.3 | **Aprovado** |
| ADR-022 | **Proposta formalmente revisada**; status permanece **Proposto**, condicionado aos Spikes 14–18 |
| Comunidade e Governança v1.0 — alterações 2026-07-24 | **Ratificadas** |
| Plano Institucional v1.0 — alterações 2026-07-24 | **Ratificadas** |

---

## Pontos de controle verificados

- OpenCore Builder é externo ao runtime e **não** está autorizado para implementação completa nesta data
- ADR-022 permanece proposto e condicionado aos Spikes 14–18
- ADR-021 permanece proposto e condicionado ao Spike 10
- `trust_level` T0–T3 alinhado entre canônicos
- Etapa 1 autorizada apenas para spikes técnicos reversíveis, documentados e time-boxed
- RFC-0001 permanece em consulta pública até 2026-08-22 ([#2](https://github.com/duvallemusic/open-core/issues/2))
- Manifesto 1.2 ↔ Arquitetura 1.3 ↔ Roadmap 2.3 ↔ ADR-022 coerentes em descoberta guiada, validação determinística e composição

---

## Ressalvas menores (não bloqueadoras)

1. Esclarecer na Arquitetura que Spikes **12–13** (CLI/lockfile) não condicionam o ADR-022; apenas **14–18** o fazem.
2. Critério de saída da Etapa 1 na Arquitetura não deve exigir aceite do ADR-022.
3. Numeração: instalador/onboarding é Spike **15** na Arquitetura; o Roadmap menciona instalador junto a 12–13 como agrupamento prático — priorizar a numeração da Arquitetura no backlog.
4. ADR-021 ainda cita versões anteriores nos metadados; atualização editorial posterior é desejável, sem impacto no status condicionado.

Ressalvas 1–2 são aplicadas nesta mesma data nos canônicos. A ressalva 3 é refletida em `spikes/BACKLOG.md`. A ressalva 4 fica para manutenção documental futura.

---

## Decisões operacionais imediatas

1. **Não** iniciar implementação completa do OpenCore Builder.
2. Iniciar a Etapa 1 pelo backlog em `spikes/`, priorizando Spikes 01–09, depois Spike 10, depois Spikes 12–13 e 15 (instalador) de forma time-boxed.
3. Spikes 14, 16, 17 e 18 (Builder/preview/IA/montagem avançada) permanecem **adiados** até autorização explícita.
4. Manter a consulta da RFC-0001 aberta até 2026-08-22; não marcar como aceita antes do encerramento formal.

---

## Registro

Esta revisão formal encerra o estado “revisão formal pendente” / “pendente de ratificação” do pacote listado acima. O índice canônico (`00_Indice_Versoes.md`) e os cabeçalhos dos documentos devem refletir este registro.
