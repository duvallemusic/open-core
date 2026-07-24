# OpenCore

Plataforma open source para sistemas desktop modulares e multiplataforma (Windows, Linux e macOS), voltada a organizações de qualquer porte.

O OpenCore não é um único aplicativo: é um **runtime mínimo** sobre o qual se montam **distribuições** (como o OpenCore Portaria) a partir de módulos reutilizáveis — com soberania de dados, operação offline-first e licenciamento transparente.

> **Status atual:** revisão formal da Etapa 0 concluída em 2026-07-24
> ([`docs/REVISAO_FORMAL_ETAPA0_2026-07-24.md`](docs/REVISAO_FORMAL_ETAPA0_2026-07-24.md)).
> Manifesto 1.2, Arquitetura 1.3 e Roadmap 2.3 estão **aprovados**.
> ADR-022 permanece **Proposto** (condicionado aos Spikes 14–18).
> A Etapa 1 foi iniciada em [`spikes/`](spikes/) apenas com protótipos
> reversíveis, documentados e time-boxed.
>
> O OpenCore Builder completo **não** está autorizado para implementação.
> A RFC-0001 permanece em consulta até 2026-08-22.
> Ainda não há runtime de produção.

---

## Princípios

- Offline-first e execução local sem assinatura obrigatória
- Dados sob controle do usuário (backup técnico ≠ exportação portátil)
- Modularidade com fronteiras claras (runtime × módulos × distribuições)
- Documentação como parte do produto
- Educação por trabalho real, sem reduzir o padrão profissional
- Sustentabilidade sem aprisionamento (lock-in)

## Licenciamento

| Material | Licença |
|---|---|
| Documentação textual deste repositório | **CC BY 4.0** — em vigor; RFC-0001 em consulta até 2026-08-22 ([#2](https://github.com/duvallemusic/open-core/issues/2)) |
| Exemplos / trechos de código na documentação | **Apache 2.0** |
| Runtime e módulos oficiais (quando publicados) | **MPL 2.0** (previsto no Manifesto) |
| Protocolo, SDKs e ferramentas (quando publicados) | **Apache 2.0** (previsto no Manifesto) |
| Marca e selos | direitos reservados |

Detalhes: [`LICENSE`](LICENSE) e [`01_Manifesto_OpenCore_v1.2.md`](01_Manifesto_OpenCore_v1.2.md).

## Documentos canônicos

Comece pelo índice: [`00_Indice_Versoes.md`](00_Indice_Versoes.md).

| Documento | Arquivo | Versão |
|---|---|---|
| Manifesto | [`01_Manifesto_OpenCore_v1.2.md`](01_Manifesto_OpenCore_v1.2.md) | 1.2 |
| Arquitetura | [`02_Arquitetura_OpenCore_v1.3.md`](02_Arquitetura_OpenCore_v1.3.md) | 1.3 |
| Comunidade e Governança | [`03_Comunidade_Governanca_OpenCore_v1.0.md`](03_Comunidade_Governanca_OpenCore_v1.0.md) | 1.0 |
| Plano Institucional | [`04_Plano_Institucional_OpenCore_v1.0.md`](04_Plano_Institucional_OpenCore_v1.0.md) | 1.0 |
| Roadmap | [`05_Roadmap_OpenCore_v2.3.md`](05_Roadmap_OpenCore_v2.3.md) | 2.3 |
| Especificação Builder | [`06_Especificacao_OpenCore_Builder_v0.md`](06_Especificacao_OpenCore_Builder_v0.md) | proposta v0 |
| Benchmarks | [`07_Benchmarks_Ecossistema_OpenCore_v1.0.md`](07_Benchmarks_Ecossistema_OpenCore_v1.0.md) | 1.0 · não normativo |

> Comunidade/Governança 1.0 e Plano Institucional 1.0: versão original aprovada em 2026-07-23; alterações substantivas de 2026-07-24 **ratificadas** na revisão formal.

### ADRs arquiteturais (015–022)

| ADR | Tema |
|---|---|
| [ADR-015](ADR-015_Matriz_Classificacao_Arquitetural.md) | Matriz runtime × módulo-base × adaptador |
| [ADR-016](ADR-016_Portabilidade_Exclusao_Modulos.md) | Portabilidade e exclusão por módulo |
| [ADR-017](ADR-017_Niveis_Confianca_Modulos.md) | Níveis de confiança |
| [ADR-018](ADR-018_Atualizacao_Estrutural_Canais.md) | Atualização estrutural vs canais |
| [ADR-019](ADR-019_Sincronizacao_Como_Adaptador.md) | Sincronização como adaptador |
| [ADR-020](ADR-020_Testes_Arquitetura_CI.md) | Testes de arquitetura no CI |
| [ADR-021](ADR-021_Modulos_Nativos_Processo_Protocolo_v1.1.md) | Módulos nativos / em processo e protocolo neutro |
| [ADR-022](ADR-022_OpenCore_Builder_Triagem_Composicao_Empacotamento.md) | OpenCore Builder — triagem, composição e empacotamento |

> Arquivos `ADR-015_Modulos_Nativos_*` são **errata de numeração** (não canônicos). O conteúdo correto está no **ADR-021**.

## Participação

| Tema | Arquivo |
|---|---|
| Como contribuir | [`CONTRIBUTING.md`](CONTRIBUTING.md) |
| Código de conduta | [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md) — relatos: `opencore.conduta@gmail.com` |
| Segurança | [`SECURITY.md`](SECURITY.md) — GitHub Private Vulnerability Reporting |
| Governança (entrada) | [`GOVERNANCE.md`](GOVERNANCE.md) |
| Mantenedores | [`MAINTAINERS.md`](MAINTAINERS.md) |
| RFCs | [`rfcs/`](rfcs/) |
| Guias de ADR | [`docs/adr/`](docs/adr/) |

## Modelo técnico (resumo)

```text
OpenCore GUI (Slint — hipótese)
        │
OpenCore Runtime (Rust)
  config · eventos · storage · migrações · Module Host · segurança
        │
   ┌────┴────┐
nativo     processo (ADR-021)
(Rust)     (Python preferencial no Spike 10, via protocolo IPC)
```

- **Runtime:** Rust, mínimo e previsível
- **Módulos críticos:** nativos (`execution.mode: native`)
- **Módulos de domínio / educacionais:** podem ser processos isolados (`execution.mode: process`) — isolamento de **falhas**, não sandbox de SO
- **Protocolo primeiro;** SDKs depois (nativo Rust ≠ SDK de processo)

## Roadmap em uma linha

1. ~~Revisão formal dos documentos 1.2/1.3/2.3 e ADR-022~~ (concluída 2026-07-24)
2. Spikes técnicos (Etapa 1) — em andamento em [`spikes/`](spikes/)
3. Fatia vertical: OpenCore Portaria (com instalação guiada)
4. Extrair SDK v0 a partir de contratos reais
5. Pilotos comunitário/institucional
6. Builder MVP baseado em regras (não autorizado para implementação completa agora)

Detalhes: [`05_Roadmap_OpenCore_v2.3.md`](05_Roadmap_OpenCore_v2.3.md).

## Aviso sobre histórico

Snapshots (`v1.0.1`, `v1.0.2`, roadmaps anteriores, rascunhos) foram preservados de propósito. Em caso de dúvida, siga apenas os **canônicos** listados no índice.
