# ADR-022 — OpenCore Builder: triagem, composição e empacotamento

**Versão desta ADR:** 0.1  
**Status:** Proposto (proposta formalmente revisada em 2026-07-24), condicionado aos Spikes 14–18  
**Data:** 2026-07-24  
**Base normativa:** Manifesto OpenCore v1.2  
**Documento relacionado:** Arquitetura OpenCore v1.3 · Roadmap OpenCore v2.3 · Especificação OpenCore Builder v0 · [`docs/REVISAO_FORMAL_ETAPA0_2026-07-24.md`](docs/REVISAO_FORMAL_ETAPA0_2026-07-24.md)  
**Numeração:** ADR-022  
**Histórico de conteúdo:** proposta inicial de Builder externo ao OpenCore Runtime, com IA opcional subordinada a motor de regras e validação determinística.  
**Substitui / altera:** complementa ADR-015 (matriz), ADR-016 (portabilidade), ADR-017 (confiança), ADR-018 (atualização), ADR-019 (sincronização), ADR-020 (testes) e ADR-021 (módulos nativos/processo); não autoriza compilação arbitrária, marketplace, sandbox completa nem IA como autoridade de compatibilidade  
**Nota de revisão formal:** o texto da proposta foi revisado e aceito como hipótese oficial; a decisão arquitetural permanece **Proposto** até evidências dos Spikes 14–18. A implementação completa do Builder **não** está autorizada.

---

## 0. Glossário

| Termo | Significado |
|---|---|
| **OpenCore Runtime** | Processo principal em Rust: inicialização, ciclo de vida, eventos, storage estrutural, segurança e Module Host |
| **OpenCore Builder** | Ferramenta/serviço auxiliar externo ao runtime que transforma necessidades do usuário em composição válida |
| **Distribuição** | Composição testada e verificável de runtime, módulos e modos operacionais para um público |
| **Perfil** | Variante verificada de uma distribuição (ex.: Essencial, Completo, Multiestação) |
| **Composição** | Conjunto exato escolhido (runtime, módulos, versões, adaptadores e configurações estruturais) |
| **Manifesto** | Declaração de intenção e faixas aceitas da distribuição ou módulo |
| **Lockfile** | Registro das versões exatas, hashes e origem dos artefatos instalados |
| **Catálogo de capacidades** | Tradução de necessidades de negócio em capacidades funcionais e módulos candidatos |
| **Motor de regras** | Autoridade determinística de recomendação e compatibilidade |
| **Preview** | Visualização estrutural da composição recomendada, sem execução completa do produto |

---

## 1. Contexto

O Manifesto OpenCore v1.2 estabelece acessibilidade operacional e descoberta guiada: usuários e organizações devem poder obter uma instalação funcional sem montar manualmente runtime, banco de dados, containers ou dependências, e sem serem obrigados a conhecer nomes de módulos.

A Arquitetura OpenCore v1.3 e o Roadmap OpenCore v2.3 preveem composição guiada, manifesto e lockfile de distribuição, catálogo de capacidades, instalação e onboarding, além de uma ferramenta externa denominada **OpenCore Builder**.

Sem um componente dedicado à triagem e à composição, o ecossistema tende a:

- exigir que o usuário escolha módulos por nome técnico;
- produzir combinações inválidas ou não testadas;
- confundir catálogo com garantia de compatibilidade;
- depender de portal ou nuvem para continuar operando;
- tratar IA como autoridade de segurança ou compatibilidade.

Esta ADR formaliza o Builder como decisão arquitetural proposta, condicionada aos Spikes 14–18, sem autorizar implementação completa nesta etapa documental.

## 2. Problema

Software profissional open source frequentemente falha na última milha: descoberta, instalação, configuração inicial e continuidade. No OpenCore, o problema específico é:

1. **Descoberta:** o usuário descreve necessidades de negócio, não arquitetura.
2. **Composição:** nem toda combinação de módulos é válida, suportada ou segura.
3. **Transparência:** recomendações não podem ocultar permissões, rede, componentes externos ou limitações.
4. **Reproduzibilidade:** a instalação precisa ser auditável via manifesto e lockfile.
5. **Empacotamento:** builds arbitrários e ilimitados são caros, inseguros e prematuros.
6. **Soberania:** o portal não pode ser requisito para operar, restaurar ou exportar dados após a instalação.

O OpenCore precisa de um caminho em que a pessoa conte o que precisa, receba uma recomendação explicada, visualize a composição, ajuste opções compatíveis, baixe um pacote instalável e comece a operar sem montar infraestrutura manualmente.

## 3. Decisão

### 3.1 Builder externo ao OpenCore Runtime

O OpenCore terá um **OpenCore Builder** externo ao OpenCore Runtime. O Builder é ferramenta/serviço auxiliar que transforma necessidades expressas pelo usuário em uma composição válida de distribuição, perfil e módulos.

O Builder **não** faz parte do runtime mínimo. A instalação resultante deve operar offline após o download, sem dependência contínua do portal ou do Builder.

### 3.2 IA opcional subordinada ao motor de regras

A IA é **opcional** e **subordinada** a:

- catálogo de capacidades;
- motor de regras;
- validador determinístico de compatibilidade;
- níveis de confiança (ADR-017);
- políticas de permissões, conflitos e distribuição.

A IA pode interpretar linguagem natural, ordenar perguntas, explicar recomendações e sugerir opcionais. A IA **não** decide compatibilidade, **não** ignora conflitos ou confiança, **não** gera comandos de instalação não validados e **não** inclui módulo não verificado silenciosamente.

O sistema deve funcionar em modo sem IA, com questionário e regras.

### 3.3 Autoridade da validação determinística

Toda saída do fluxo de recomendação — inclusive saída assistida por IA — deve passar pelo validador determinístico antes de gerar manifesto, lockfile, preview ou pacote.

### 3.4 Empacotamento progressivo

O MVP e o alpha inicial usam seleção de pacotes pré-construídos ou montagem a partir de artefatos assinados conhecidos. Compilação arbitrária de código enviado por usuários permanece fora do escopo até matriz robusta, custo controlado, assinatura, SBOM e auditoria (Estágio C).

### 3.5 Licenciamento do Builder

O OpenCore Builder, seus templates, protocolo de composição (se houver) e artefatos de ferramenta associados serão licenciados sob **Apache 2.0**, alinhados à política de SDK, protocolo e ferramentas. O núcleo e módulos oficiais permanecem sob MPL 2.0.

## 4. Componentes

O Builder é composto, conceitualmente, por:

| Componente | Função |
|---|---|
| **Questionário** | Coleta necessidades em linguagem não técnica, com “não sei” e “decidir depois” |
| **Interpretador de necessidades** | Traduz respostas (ou texto livre, se houver IA) em capacidades e restrições |
| **Catálogo de capacidades** | Mapeia necessidade → capability → módulos candidatos → perfis |
| **Perfis de negócio** | Variantes verificadas por distribuição |
| **Motor de recomendação** | Seleciona distribuição/perfil dentro de limites testados |
| **Validador** | Compatibilidade, dependências, conflitos, confiança, permissões e SO |
| **Preview** | Visualização estrutural da composição |
| **Gerador de manifesto** | Intenção e faixas aceitas da composição |
| **Gerador de lockfile** | Versões exatas, hashes, origem e identificador da composição |
| **Seletor / empacotador** | Escolhe pacote pré-construído ou monta a partir de artefatos conhecidos |
| **Catálogo de artefatos** | Runtime, módulos e instaladores assináveis/conhecidos |
| **Ficha da instalação** | Resumo auditável entregue ao usuário e embutível na instalação |

Fluxo normativo:

```text
Respostas do usuário
→ interpretação das necessidades
→ catálogo de capacidades
→ motor de regras
→ composições válidas
→ explicação da recomendação
→ personalização dentro dos limites
→ validação final
→ geração do manifesto/lockfile
→ seleção ou geração do pacote
```

## 5. Papel da IA

### Permitido (condicionado ao Spike 17)

- interpretar linguagem natural;
- explicar recomendações;
- ordenar ou adaptar perguntas;
- sugerir módulos opcionais já elegíveis pelo motor de regras;
- ajudar a redigir textos de onboarding e suporte.

### Proibido

- decidir compatibilidade;
- contornar permissões, conflitos ou níveis de confiança;
- gerar comandos de instalação não validados;
- incluir módulo experimental ou não verificado por padrão ou em silêncio;
- substituir o questionário + regras no MVP;
- tornar o uso do Builder inviável sem serviço de IA.

### Requisitos

- modo sem IA sempre disponível;
- informar quando respostas forem enviadas a serviço de IA;
- retenção mínima;
- saída da IA tratada como hipótese até validação determinística.

## 6. Validação determinística

O validador é a autoridade técnica. Deve verificar, no mínimo:

- dependências e conflitos de módulos;
- compatibilidade com versão do OpenCore Runtime;
- `execution.mode` e requisitos de runtime da linguagem empacotado, quando aplicável;
- sistemas operacionais e hardware mínimos;
- requisitos de rede e transmissões externas;
- níveis de confiança (preferência: T3 Oficial → T2 Verificado; T1 Comunitário apenas no modo avançado com aceite explícito; T0 Experimental nunca por padrão);
- permissões e categorias de dados;
- consistência entre preview, manifesto, lockfile e pacote;
- política de atualização e canal.

Regras de comportamento:

- mesma entrada produz resultado determinístico, salvo escolha explícita do usuário;
- falhas geram diagnóstico compreensível e acionável;
- combinação inválida nunca é recomendada nem empacotada;
- personalização livre além da matriz verificada é adiada.

## 7. Distribuições e perfis

- Uma **distribuição** não é combinação arbitrária; é composição testada.
- Um **perfil** é variante verificada de uma distribuição (ex.: Essencial, Completo, Multiestação).
- Personalização inicial ocorre dentro de limites verificados.
- Combinações livres só crescem com evidência, matriz de testes e governança.
- A Portaria permanece primeira distribuição de referência; não é identidade da plataforma.
- Exemplos de segmento (padaria, oficina etc.) são de UX e marketing; não contaminam o OpenCore Runtime com regras de negócio específicas.

## 8. Manifesto e lockfile

### Manifesto da distribuição / composição

Declara intenção e faixas aceitas, incluindo no mínimo:

- identificador, nome, versão;
- público e perfil de negócio;
- modos operacionais (monoposto, rede local, sincronizado, integração externa);
- OpenCore Runtime compatível;
- módulos obrigatórios e opcionais;
- capacidades;
- sistemas suportados e requisitos mínimos;
- política de atualização;
- formatos de backup e exportação;
- documentação, licença e identidade.

### Lockfile

Registra a composição exata:

- versão exata do OpenCore Runtime;
- módulos e versões;
- hashes;
- origem dos artefatos;
- runtime da linguagem empacotado, se houver;
- configurações estruturais;
- adaptadores;
- canal de atualização;
- data de geração;
- identificador da composição;
- assinatura futura.

O lockfile deve permitir reprodução, auditoria, diagnóstico e rollback. Manifesto e lockfile são artefatos distintos.

## 9. Preview

O preview inicial é **estrutural**, não execução completa do produto. Pode mostrar:

- navegação e dashboard representativos;
- módulos ativos;
- fluxos principais;
- permissões;
- alertas sobre rede e dados externos.

O preview não pode:

- prometer telas ou funcionalidades não implementadas;
- substituir testes;
- ocultar dependências;
- atuar como construtor low-code genérico na primeira versão.

Deve corresponder à composição validada (critério de aceitação).

## 10. Empacotamento progressivo

### Estágio A — seleção de pacote pré-construído (MVP / alpha)

- usuário responde;
- Builder escolhe distribuição/perfil verificado;
- entrega instalador pronto;
- gera configuração, manifesto e lockfile.

### Estágio B — montagem a partir de artefatos assinados

- OpenCore Runtime e módulos já compilados;
- pipeline monta pacote sem executar código arbitrário do usuário;
- valida hashes e compatibilidade;
- assinatura futura.

### Estágio C — geração avançada (condicionado)

- somente após matriz robusta;
- limites de custo, fila e cache;
- assinatura, SBOM, builds reproduzíveis;
- isolamento de pipeline e auditoria.

**Não autorizar** compilação arbitrária no MVP nem no alpha público inicial.

## 11. Privacidade

- triagem anônima por padrão;
- cadastro não obrigatório para explicar recomendação ou baixar composição;
- não solicitar faturamento, CPF, CNPJ ou dados sensíveis sem necessidade operacional;
- informar uso de IA, quando houver;
- retenção mínima das respostas;
- não usar respostas para anúncios;
- telemetria desativada por padrão e separada, somente com consentimento;
- dados da triagem não são requisito para operação local posterior.

## 12. Segurança

- preferir módulos T3 Oficiais e T2 Verificados (ADR-017);
- módulos T1 Comunitários apenas em modo avançado com aceite explícito de risco;
- nunca recomendar T0 Experimental por padrão;
- isolamento por processo (ADR-021) não equivale a sandbox;
- pacotes iniciais usam artefatos conhecidos;
- hashes e, futuramente, assinatura e SBOM;
- não baixar e executar código externo automaticamente na primeira versão;
- falhas e incompatibilidades devem ser explícitas, não silenciosas;
- o Builder não eleva privilégios do OpenCore Runtime nem altera a fronteira de permissões do manifesto.

## 13. Licenciamento

| Artefato | Licença proposta |
|---|---|
| OpenCore Builder (ferramenta/serviço auxiliar) | Apache 2.0 |
| Catálogo de capacidades, templates de triagem, geradores de manifesto/lockfile | Apache 2.0 |
| OpenCore Runtime e módulos oficiais | MPL 2.0 (inalterado) |
| Distribuições oficiais | integralmente open source, conforme Manifesto |

Certificação comercial, se existir no futuro, não altera direitos concedidos pela licença. Componentes GPL/AGPL/LGPL ou de licença customizada não entram no monorepo sem análise explícita por componente.

## 14. Consequências

### Positivas

- acessibilidade operacional sem exigir arquitetura do usuário;
- recomendações explicáveis e auditáveis;
- composição reproduzível via manifesto e lockfile;
- IA útil sem autoridade indevida;
- caminho de empacotamento progressivo com custo e risco controlados;
- soberania: operação, restauração e exportação independentes do portal;
- alinhamento com direitos do usuário no Manifesto 1.2.

### Negativas / custos

- manutenção de catálogo de capacidades, perfis e matriz de compatibilidade;
- risco de explosão combinatória se personalização livre for liberada cedo;
- preview pode divergir do produto se não for disciplinado;
- custo operacional de artefatos e, no futuro, de builds;
- necessidade de spikes (14–18) antes de Aceito;
- linguagem de marketing por segmento pode gerar expectativa além dos módulos existentes.

### Obrigações de execução

1. Não migrar status para **Aceito** antes dos Spikes 14–18 relevantes.
2. Especificação funcional `06_Especificacao_OpenCore_Builder_v0.md` permanece subordinada a esta ADR.
3. Arquitetura ≥ 1.3 e Roadmap ≥ 2.3 devem refletir Builder externo, IA subordinada e empacotamento progressivo.
4. Não implementar marketplace, compilação arbitrária ou IA obrigatória no MVP.
5. Não tornar o portal necessário para continuar usando a instalação.

## 15. Alternativas consideradas

| Alternativa | Veredito | Motivo |
|---|---|---|
| Catálogo manual sem recomendação | **Rejeitada** como única experiência | Exige conhecimento técnico incompatível com acessibilidade operacional |
| IA controlando toda a composição | **Rejeitada** | Incompatível com validação determinística, confiança e soberania |
| Download de módulos individualmente pelo usuário leigo | **Adiada / rejeitada** como fluxo principal | Aumenta erro de composição; catálogo avançado pode coexistir depois |
| Build totalmente dinâmico desde o início | **Adiada** (Estágio C) | Custo, segurança e matriz insuficientes no MVP |
| Um ERP monolítico por segmento | **Rejeitada** | Contradiz modularidade, Portaria como referência e anti-lock-in |
| Portal obrigatório durante a operação | **Rejeitada** | Viola offline-first e soberania dos dados |
| Marketplace como primeira etapa | **Adiada** | Prematuro sem confiança, assinatura e conformidade |

## 16. Critérios de aceitação

Status só poderá migrar para **Aceito** quando demonstrado, nos Spikes 14–18 aplicáveis:

1. Funciona sem IA.
2. Suporta a opção “não sei”.
3. Não recomenda combinação inválida.
4. Módulos experimentais não entram por padrão.
5. Mesma entrada produz resultado determinístico, salvo escolha explícita do usuário.
6. Preview corresponde à composição.
7. Pacote corresponde ao lockfile.
8. Instalação funciona offline após download.
9. O usuário pode exportar a composição.
10. Triagem não exige cadastro obrigatório.
11. Dados coletados são mínimos.
12. Não compila código arbitrário.
13. Pacote inicial usa artefatos conhecidos.
14. Falhas geram diagnóstico compreensível.
15. O portal não é necessário para continuar usando, restaurar ou exportar dados.

## 17. Fora do escopo

- compilação arbitrária de código enviado por usuários;
- marketplace comercial;
- sandbox completa de sistema operacional;
- geração ilimitada de combinações não testadas;
- IA como autoridade de segurança ou compatibilidade;
- construtor low-code genérico;
- emissão fiscal ou regras regulatórias universais no OpenCore Runtime;
- obrigatoriedade de cadastro, nuvem ou prestador;
- transformar exemplos de segmento (padaria, oficina) em módulos hardcodados no runtime;
- aceitar ADR-021 sem Spike 10 (decisão independente e prévia).

## 18. Relação com outras ADRs

| ADR | Relação |
|---|---|
| **ADR-015** — Matriz de classificação arquitetural | Composições e perfis devem respeitar a matriz; Builder não inventa classificação |
| **ADR-016** — Portabilidade e exclusão de módulos | Exportação, remoção e continuidade após exclusão permanecem obrigações; Builder declara contratos no manifesto/lockfile |
| **ADR-017** — Níveis de confiança | Preferência T3/T2; T1 só avançado; T0 nunca por padrão |
| **ADR-018** — Atualização estrutural e canais | Lockfile e canal de atualização alinhados; Builder não redefine política de update sozinho |
| **ADR-019** — Sincronização como adaptador | Modo sincronizado é opção declarada, não núcleo; Builder não trata sync como requisito |
| **ADR-020** — Testes, arquitetura e CI | Exige testes de composição, experiência, Builder e soberania |
| **ADR-021** — Módulos nativos / processo | Empacotamento deve respeitar `execution.mode`; processo ≠ sandbox; runtime da linguagem empacotado quando necessário |

## 19. Changelog desta ADR

| Versão | Descrição |
|---|---|
| 0.1 | Proposta inicial: Builder externo ao OpenCore Runtime; IA opcional subordinada a motor de regras; manifesto/lockfile; empacotamento progressivo; critérios condicionados aos Spikes 14–18. Proposta formalmente revisada em 2026-07-24; status permanece Proposto até Spikes 14–18. |
