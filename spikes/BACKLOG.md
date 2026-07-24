# Backlog de spikes — Etapa 1

**Atualizado:** 2026-07-24  
**Orçamento inicial:** 20 pessoa-dias (Spikes 01–09) + 3 (Spike 10)  
**Spikes 12–18:** time-box a definir; 14/16/17/18 adiados nesta data

## Prioridade imediata

| Ordem | Spike | Tema | Limite | Status |
|---:|---|---|---:|---|
| 1 | 01 | Aplicação mínima multiplataforma (Rust + Slint) | 3 pd | **em andamento** (A/B OK nos 3 SO; C macOS parcial; C Win/Linux pendente) |
| 2 | 02 | Registro de módulos | 2 pd | pendente |
| 3 | 03 | Eventos locais | 2 pd | pendente |
| 4 | 04 | SQLite e migrações por módulo | 3 pd | pendente |
| 5 | 05 | Backup e restauração | 2 pd | pendente |
| 6 | 06 | Exportação portátil | 2 pd | pendente |
| 7 | 07 | Duas composições | 2 pd | pendente |
| 8 | 08 | Licenças e CI arquitetural | 1 pd | pendente |
| 9 | 09 | Onboarding externo | 3 pd | pendente |
| 10 | 10 | Módulo em processo headless (ADR-021, preferência Python) | 3 pd | pendente |

## Adoção / DX (time-boxed, sem Builder completo)

| Spike | Tema | Status | Nota |
|---|---|---|---|
| 12 | CLI e scaffolding | autorizado | após evidências mínimas da fatia 01–04 |
| 13 | Manifesto e lockfile de distribuição | autorizado | |
| 15 | Instalador e onboarding monoposto | autorizado | numeração da Arquitetura (não agrupar como “12–13”) |

## Adiados (Builder / IA / preview avançado)

| Spike | Tema | Motivo |
|---|---|---|
| 14 | Builder baseado em regras | Builder completo não autorizado |
| 16 | Preview estrutural | depende de caminho Builder |
| 17 | Camada conversacional (IA) | pós-MVP / opcional |
| 18 | Montagem de pacote com artefatos pré-construídos | Builder não autorizado |

## Futuro

| Spike | Tema | Pré-condição |
|---|---|---|
| 11 | UI declarativa para módulos em processo | Spike 10 aceito |

## Critério de saída da Etapa 1 (resumo)

Conforme Roadmap 2.3 / Arquitetura 1.3: stack nos três SO; dois módulos sem acoplamento indevido; Spike 10 decidido com evidência; Spikes 12 e 13 executados com evidência; catálogo estático mínimo, perfil simples e protótipo instalador/`doctor` testáveis. **Aceite do ADR-022 não é critério de saída.**
