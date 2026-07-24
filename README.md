# OpenCore

Plataforma open source para sistemas desktop modulares e multiplataforma (Windows, Linux e macOS), voltada a organizações de qualquer porte.

O OpenCore não é um único aplicativo: é um **runtime mínimo** sobre o qual se montam **distribuições** (como o OpenCore Portaria) a partir de módulos reutilizáveis — com soberania de dados, operação offline-first e licenciamento transparente.

> **Status atual:** fase documental (Etapa 0). Ainda não há código de produto. A próxima entrega prevista é *Plano Institucional OpenCore v1.0*.

---

## Princípios

- Offline-first e execução local sem assinatura obrigatória
- Dados sob controle do usuário (backup técnico ≠ exportação portátil)
- Modularidade com fronteiras claras (runtime × módulos × distribuições)
- Documentação como parte do produto
- Educação por trabalho real, sem reduzir o padrão profissional
- Sustentabilidade sem aprisionamento (lock-in)

## Licenciamento (previsto)

| Camada | Licença |
|---|---|
| Runtime, ciclo de vida de módulos e patrimônio estrutural | **MPL 2.0** |
| Protocolo público, SDKs, templates e materiais educacionais | **Apache 2.0** |

Detalhes: [`01_Manifesto_OpenCore_v1.1_licenciamento.md`](01_Manifesto_OpenCore_v1.1_licenciamento.md).

## Documentos canônicos

Comece pelo índice: [`00_Indice_Versoes.md`](00_Indice_Versoes.md).

| Documento | Arquivo | Versão |
|---|---|---|
| Manifesto | [`01_Manifesto_OpenCore_v1.1_licenciamento.md`](01_Manifesto_OpenCore_v1.1_licenciamento.md) | 1.1 |
| Arquitetura | [`02_Arquitetura_OpenCore_v1.2.md`](02_Arquitetura_OpenCore_v1.2.md) | 1.2 |
| Comunidade e Governança | [`03_Comunidade_Governanca_OpenCore_v1.0.md`](03_Comunidade_Governanca_OpenCore_v1.0.md) | 1.0 |
| Plano Institucional | [`04_Plano_Institucional_OpenCore.md`](04_Plano_Institucional_OpenCore.md) | rascunho → v1.0 |
| Roadmap | [`05_Roadmap_OpenCore_v2.2.md`](05_Roadmap_OpenCore_v2.2.md) | 2.2 |

### ADRs arquiteturais (015–021)

| ADR | Tema |
|---|---|
| [ADR-015](ADR-015_Matriz_Classificacao_Arquitetural.md) | Matriz runtime × módulo-base × adaptador |
| [ADR-016](ADR-016_Portabilidade_Exclusao_Modulos.md) | Portabilidade e exclusão por módulo |
| [ADR-017](ADR-017_Niveis_Confianca_Modulos.md) | Níveis de confiança |
| [ADR-018](ADR-018_Atualizacao_Estrutural_Canais.md) | Atualização estrutural vs canais |
| [ADR-019](ADR-019_Sincronizacao_Como_Adaptador.md) | Sincronização como adaptador |
| [ADR-020](ADR-020_Testes_Arquitetura_CI.md) | Testes de arquitetura no CI |
| [ADR-021](ADR-021_Modulos_Nativos_Processo_Protocolo_v1.1.md) | Módulos nativos / em processo e protocolo neutro |

> Arquivos `ADR-015_Modulos_Nativos_*` são **errata de numeração** (não canônicos). O conteúdo correto está no **ADR-021**.

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

1. Fechar Etapa 0 (Comunidade/Governança + Plano Institucional)
2. Spikes técnicos (Etapa 1), incluindo Spike 10 se ADR-021 for validado
3. Fatia vertical: OpenCore Portaria
4. Extrair SDK v0 a partir de contratos reais
5. Pilotos comunitário e educacional

Detalhes: [`05_Roadmap_OpenCore_v2.2.md`](05_Roadmap_OpenCore_v2.2.md).

## Como contribuir agora

Ainda não há repositório de código do runtime. Contribuições neste estágio são **documentais**:

1. Leia o Manifesto e a Arquitetura v1.2
2. Consulte o índice de versões antes de propor mudanças
3. Decisões relevantes devem virar RFC/ADR (processo a formalizar em Comunidade e Governança v1.0)

## Aviso sobre histórico

Snapshots (`v1.0.1`, `v1.0.2`, roadmaps anteriores, rascunhos) foram preservados de propósito. Em caso de dúvida, siga apenas os **canônicos** listados no índice.

## Licença deste repositório documental

Os textos deste repositório descrevem a política de licenciamento do software futuro. Até a publicação do código, trate o conteúdo documental como material de projeto sob direitos autorais dos autores, salvo indicação explícita em contrário nos arquivos individuais.
