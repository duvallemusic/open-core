> **Status:** histórico — instruções executadas em 2026-07-24.
>
> Este arquivo registra o roteiro utilizado para produzir Manifesto 1.2,
> Arquitetura 1.3, Roadmap 2.3, ADR-022, Especificação Builder v0 e
> Benchmarks v1.0.
>
> Não utilizar este documento como fonte normativa ou como lista atual de
> arquivos canônicos. Consulte `00_Indice_Versoes.md`.

# Instruções para o Cursor — Atualização documental do OpenCore

**Objetivo:** consolidar, nos arquivos do repositório OpenCore, as decisões e sugestões definidas durante a análise de concorrentes, experiência do desenvolvedor, governança de módulos, interoperabilidade, experiência do usuário final e criação do **OpenCore Builder**.

**Natureza desta tarefa:** atualização documental e arquitetural. Não implementar o runtime, o portal ou os módulos nesta etapa. O resultado deve deixar os documentos normativos claros o suficiente para orientar a implementação futura e os spikes técnicos.

**Princípio central da revisão:**

> O OpenCore não deve ser apenas uma plataforma modular tecnicamente correta. Ele deve transformar software profissional open source em algo instalável e utilizável por pessoas sem conhecimento técnico, preservando offline-first, soberania dos dados, distribuições verificadas e ausência de lock-in.

---

# 1. Regras obrigatórias para executar esta atualização

1. Ler integralmente os documentos canônicos antes de editar:
   - `00_Indice_Versoes.md`
   - `01_Manifesto_OpenCore_v1.1_licenciamento.md`
   - `02_Arquitetura_OpenCore_v1.2.md`
   - `03_Comunidade_OpenCore.md`
   - `04_Plano_Institucional_OpenCore.md`
   - `05_Roadmap_OpenCore_v2.2.md`
   - `ADR-015_Matriz_Classificacao_Arquitetural.md`
   - `ADR-016_Portabilidade_Exclusao_Modulos.md`
   - `ADR-017_Niveis_Confianca_Modulos.md`
   - `ADR-018_Atualizacao_Estrutural_Canais.md`
   - `ADR-019_Sincronizacao_Como_Adaptador.md`
   - `ADR-020_Testes_Arquitetura_CI.md`
   - `ADR-021_Modulos_Nativos_Processo_Protocolo_v1.1.md`

2. Preservar os arquivos históricos. Não substituir silenciosamente versões anteriores.

3. Criar novas versões canônicas:
   - Manifesto `1.2`
   - Arquitetura `1.3`
   - Comunidade e Governança `1.0`
   - Plano Institucional `1.0`
   - Roadmap `2.3`

4. Atualizar os espelhos de conveniência, caso existam:
   - `02_Arquitetura_OpenCore_v1.md` deve espelhar a versão `1.3`.
   - `05_Roadmap_OpenCore_v2.md` deve espelhar a versão `2.3`.

5. Criar uma ADR específica para o OpenCore Builder:
   - `ADR-022_OpenCore_Builder_Triagem_Composicao_Empacotamento.md`

6. Criar uma especificação funcional separada:
   - `06_Especificacao_OpenCore_Builder_v0.md`

7. Criar um documento não normativo de benchmarks:
   - `07_Benchmarks_Ecossistema_OpenCore_v1.0.md`

8. Atualizar `00_Indice_Versoes.md` ao final, apontando somente para os arquivos realmente criados.

9. Não introduzir como decisão definitiva algo que ainda depende de spike. Usar termos como:
   - hipótese;
   - proposta;
   - condicionado a validação;
   - fora do escopo inicial;
   - decisão futura mediante ADR.

10. Manter as decisões já aprovadas ou consolidadas:
    - runtime principal em Rust;
    - Slint como hipótese de GUI;
    - SQLite como hipótese inicial;
    - monólito modular;
    - módulos nativos e módulos em processo conforme ADR-021;
    - Python como preferência inicial do primeiro SDK em processo;
    - isolamento por processo não equivale a sandbox;
    - MPL 2.0 para núcleo e módulos oficiais;
    - Apache 2.0 para SDK, protocolo, templates e ferramentas;
    - distribuições oficiais integralmente open source;
    - telemetria desativada por padrão;
    - backup não equivale a exportação;
    - sincronização como adaptador;
    - anti-complexidade prematura;
    - Portaria como primeira distribuição de referência, não identidade da plataforma.

11. Não copiar código, textos extensos ou estruturas protegidas dos concorrentes. Os projetos externos devem servir como benchmarks de padrões, não como dependências automáticas.

12. Não adicionar código GPL, AGPL, LGPL ou de licença customizada ao monorepo sem análise explícita por componente. A documentação deve registrar essa restrição.

---

# 2. Resultado documental esperado

Ao final, o repositório deverá conter como canônicos:

| Documento | Arquivo esperado | Versão/status |
|---|---|---|
| Índice | `00_Indice_Versoes.md` | atualizado |
| Manifesto | `01_Manifesto_OpenCore_v1.2.md` | 1.2 |
| Arquitetura | `02_Arquitetura_OpenCore_v1.3.md` | 1.3 |
| Comunidade e Governança | `03_Comunidade_Governanca_OpenCore_v1.0.md` | 1.0 |
| Plano Institucional | `04_Plano_Institucional_OpenCore_v1.0.md` | 1.0 |
| Roadmap | `05_Roadmap_OpenCore_v2.3.md` | 2.3 |
| Especificação do Builder | `06_Especificacao_OpenCore_Builder_v0.md` | proposta v0 |
| Benchmarks | `07_Benchmarks_Ecossistema_OpenCore_v1.0.md` | referência não normativa |
| ADR Builder | `ADR-022_OpenCore_Builder_Triagem_Composicao_Empacotamento.md` | Proposto |

Os arquivos anteriores devem permanecer em histórico.

---

# 3. Alteração do Manifesto — versão 1.2

## Arquivo a criar

`01_Manifesto_OpenCore_v1.2.md`

## Base

Copiar a versão 1.1 e realizar uma evolução editorial, preservando licenciamento, direitos, governança, independência, soberania e offline-first.

## Mudanças obrigatórias

### 3.1 Adicionar o princípio de acessibilidade operacional

Criar uma subseção em “Princípios de produto e arquitetura”, após “Simplicidade proporcional”, com conteúdo equivalente a:

## Acessibilidade operacional

O direito de utilizar software aberto perde valor prático quando instalação, configuração, atualização, recuperação ou escolha de funcionalidades exigem conhecimento técnico incompatível com o público atendido.

O OpenCore deverá buscar uma experiência na qual usuários e organizações possam:

- encontrar uma distribuição adequada ao seu tipo de atividade;
- compreender quais capacidades estão incluídas;
- receber recomendações em linguagem não técnica;
- instalar o sistema sem configurar manualmente runtime, banco de dados, containers ou dependências;
- iniciar a operação por meio de configuração guiada;
- importar dados existentes sempre que houver formatos suportados;
- executar backup, restauração, atualização e exportação por fluxos compreensíveis;
- solicitar ajuda de qualquer prestador compatível, sem dependência obrigatória da entidade central.

A simplificação da experiência não poderá esconder riscos, permissões, transmissões externas, componentes proprietários ou limitações da configuração escolhida.

### 3.2 Acrescentar um direito do usuário

Na seção “Direitos dos usuários”, adicionar um item que preserve o direito de:

- obter uma instalação funcional das distribuições oficiais sem precisar montar manualmente dependências técnicas;
- conhecer previamente quais módulos, permissões, transmissões externas e requisitos compõem a instalação;
- reconstruir ou reproduzir a configuração instalada a partir de manifesto e lockfile documentados.

### 3.3 Reforçar distribuições oficiais utilizáveis

Na seção sobre marca, certificação e distribuições oficiais, acrescentar que uma distribuição oficial deve:

- possuir caminho de instalação compreensível para seu público;
- evitar dependências técnicas não empacotadas;
- fornecer configuração inicial guiada;
- declarar o modo de operação: monoposto, rede local, sincronizado ou dependente de integração externa;
- apresentar uma composição verificável e reproduzível;
- documentar recursos mínimos de hardware.

### 3.4 Adicionar compromisso com descoberta guiada

Criar uma subseção curta deixando claro que:

- usuários não devem ser obrigados a conhecer nomes de módulos;
- necessidades do negócio devem ser traduzidas para capacidades técnicas;
- recomendações podem utilizar automação ou IA, mas a composição final deve ser validada por regras determinísticas;
- a recomendação não pode instalar silenciosamente componentes incompatíveis, não verificados ou não solicitados.

### 3.5 Atualizar “O que não construiremos”

Adicionar explicitamente:

- não construiremos uma experiência em que o usuário precise compreender a arquitetura para instalar uma distribuição;
- não usaremos IA como substituta da validação de compatibilidade;
- não trataremos um catálogo de módulos como garantia de que qualquer combinação funciona;
- não criaremos geração pública arbitrária de builds antes de existir matriz de compatibilidade, assinatura e controle de custo;
- não esconderemos componentes comerciais ou transmissões externas dentro de recomendações automáticas.

### 3.6 Atualizar cabeçalho e histórico

O cabeçalho deve informar:

- versão 1.2;
- supersede a 1.1;
- principal mudança: acessibilidade operacional, descoberta guiada e reproduzibilidade das instalações;
- licenciamento permanece inalterado.

---

# 4. Alteração da Arquitetura — versão 1.3

## Arquivo a criar

`02_Arquitetura_OpenCore_v1.3.md`

## Base

Copiar integralmente a versão 1.2 e incorporar os itens abaixo de forma coerente, evitando repetição.

## 4.1 Atualizar o escopo

Acrescentar que a arquitetura também cobre:

- experiência de composição de distribuições para usuários não técnicos;
- manifesto e lockfile de distribuições;
- catálogo de capacidades;
- validação determinística de combinações;
- instalação e onboarding guiados;
- ferramentas de desenvolvimento e diagnóstico;
- interoperabilidade por adaptadores.

Deixar fora do escopo definitivo da versão:

- compilação arbitrária de código enviado por usuários;
- marketplace comercial;
- sandbox completa;
- geração ilimitada de combinações não testadas;
- IA tomando decisões de segurança ou compatibilidade;
- construtor low-code genérico;
- emissão fiscal ou regras regulatórias universais no runtime.

## 4.2 Criar princípio “complexidade invisível, não omitida”

Adicionar aos princípios arquiteturais:

- complexidade técnica pode ser encapsulada para o usuário;
- não pode ser escondida da auditoria, diagnóstico ou documentação;
- o instalador deve carregar dependências necessárias;
- o manifesto e lockfile devem registrar a composição real;
- o usuário simples recebe explicação em linguagem comum;
- o administrador ou técnico consegue inspecionar os detalhes.

## 4.3 Refinar o contrato de módulos

Incorporar ou reforçar no manifesto de módulo:

- `capabilities`: capacidades funcionais fornecidas;
- `business_tags`: termos de negócio usados pelo catálogo;
- `supported_os`;
- `hardware_requirements`;
- `network_requirements`;
- `data_categories`;
- `external_transmissions`;
- `maintenance_status`;
- `trust_level`;
- `owner`;
- `backup_contract`;
- `export_contract`;
- `uninstall_policy`;
- `demo_data`;
- `documentation_urls` ou referências locais;
- `conflicts`;
- `replaces`;
- `recommended_with`.

Não congelar o formato final antes dos spikes. Registrar como campos conceituais obrigatórios a representar.

## 4.4 Formalizar o ciclo de vida

A arquitetura deve consolidar estados equivalentes a:

```text
Descoberto
→ Manifesto validado
→ Compatibilidade validada
→ Dependências resolvidas
→ Permissões avaliadas
→ Instalado
→ Migrado
→ Inicializado
→ Ativo
→ Suspenso
→ Desativado
→ Removido
```

Também prever estados de falha:

- incompatível;
- dependência ausente;
- migração falhou;
- crash loop;
- bloqueado por política;
- manutenção encerrada;
- quarentenado.

## 4.5 Criar seção de experiência do desenvolvedor

Adicionar uma seção normativa chamada “Ferramentas de desenvolvimento e experiência de contribuição”.

Definir uma CLI conceitual:

```text
opencore new
opencore new-module
opencore new-distribution
opencore validate
opencore run
opencore test
opencore conformance
opencore inspect
opencore migrate
opencore package
opencore doctor
```

A seção deve exigir:

- scaffolding para módulo nativo;
- scaffolding para módulo em processo;
- scaffolding para adaptador;
- scaffolding para distribuição;
- manifesto inicial;
- licença adequada;
- testes;
- documentação;
- exemplo de evento;
- migração inicial quando aplicável;
- fixtures e dados de demonstração;
- mensagens de erro acionáveis;
- ambiente reproduzível nos três sistemas operacionais;
- documentação parcialmente gerada do manifesto;
- projeto de exemplo completo.

A CLI é hipótese de produto e deve ser validada em spike próprio ou incorporada aos spikes existentes.

## 4.6 Criar contrato de serviços

Adicionar uma seção ou reforçar comunicação entre módulos com a regra:

```text
Dados pertencem ao módulo.
Serviços expõem operações.
Eventos notificam mudanças.
A interface consome serviços.
```

Exigir:

- contratos explícitos de entrada e saída;
- validação uniforme;
- erros tipados;
- registro dos serviços disponíveis;
- versão do contrato;
- proibição de acesso à implementação interna;
- comandos request/response para operações síncronas;
- eventos para comunicação desacoplada;
- proteção contra ciclos e tempestades de eventos.

## 4.7 Manifesto e lockfile de distribuição

Expandir a seção de distribuições.

Cada distribuição deve possuir dois artefatos distintos:

### Manifesto da distribuição

Declara intenção e faixas aceitas:

- identificador;
- nome;
- versão;
- público;
- perfil de negócio;
- modos operacionais;
- runtime compatível;
- módulos obrigatórios;
- módulos opcionais;
- capacidades;
- sistemas suportados;
- requisitos mínimos;
- política de atualização;
- formatos de backup e exportação;
- documentação;
- licença e identidade.

### Lockfile da distribuição

Registra a composição exata instalada:

- versão exata do runtime;
- módulos e versões;
- hashes;
- origem dos artefatos;
- SDK/runtime de linguagem empacotado;
- configurações estruturais;
- adaptadores;
- canal de atualização;
- data de geração;
- identificador da composição;
- assinatura futura.

O lockfile deve permitir reprodução, auditoria, diagnóstico e rollback.

## 4.8 Perfis e variantes verificadas

Definir:

- uma distribuição não é uma combinação arbitrária;
- perfis de configuração são variações testadas de uma distribuição;
- exemplo conceitual:
  - Essencial;
  - Completo;
  - Multiestação;
- personalização inicial deve ocorrer dentro de limites verificados;
- combinações livres só podem crescer com evidência e matriz de testes.

## 4.9 OpenCore Builder como componente externo

Criar uma seção arquitetural específica.

O Builder não faz parte do runtime mínimo. Ele é uma ferramenta/serviço auxiliar Apache 2.0, composto por:

1. catálogo de capacidades;
2. perfis de negócio;
3. motor de recomendação;
4. validador determinístico;
5. gerador de preview;
6. gerador de manifesto e lockfile;
7. pipeline de seleção ou empacotamento;
8. catálogo de artefatos;
9. telemetria opcional e separada, sempre desativada sem consentimento;
10. documentação e fluxo de suporte.

A arquitetura deve registrar o fluxo:

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

## 4.10 Limite da IA

Formalizar:

- IA pode interpretar linguagem natural;
- IA pode explicar recomendações;
- IA pode ordenar perguntas;
- IA pode sugerir módulos opcionais;
- IA não decide compatibilidade;
- IA não pode ignorar permissões, conflitos ou níveis de confiança;
- IA não pode gerar comandos de instalação não validados;
- IA não pode incluir módulo não verificado silenciosamente;
- toda saída deve passar pelo motor determinístico;
- o sistema deve funcionar em modo sem IA usando questionário e regras.

## 4.11 Catálogo de capacidades

Definir um catálogo que traduza linguagem de negócio para módulos.

Exemplo:

```text
“Vender no balcão”
→ capability: point_of_sale
→ módulos candidatos: sales + pos

“Controlar produtos perecíveis”
→ capability: perishable_inventory
→ módulos candidatos: inventory + expiration_control
```

O catálogo deve separar:

- necessidade do usuário;
- capacidade funcional;
- implementação por módulo;
- perfil de distribuição.

Evitar hardcode do tipo de negócio no runtime.

## 4.12 Preview de interface

Definir que o preview inicial pode ser estrutural, não uma execução completa.

Pode mostrar:

- navegação;
- dashboard;
- módulos ativos;
- fluxos;
- permissões;
- telas representativas;
- alertas sobre rede e dados externos.

O preview não pode:

- prometer telas ou funcionalidades não implementadas;
- substituir testes;
- ocultar dependências;
- funcionar como construtor low-code genérico na primeira versão.

## 4.13 Estratégia de empacotamento progressiva

Documentar três estágios:

### Estágio A — seleção de pacote pré-construído

- usuário responde;
- Builder escolhe distribuição/perfil verificado;
- entrega instalador pronto;
- gera configuração e lockfile.

### Estágio B — montagem a partir de artefatos assinados

- runtime e módulos já compilados;
- pipeline monta pacote;
- não executa código arbitrário;
- valida hashes e compatibilidade.

### Estágio C — geração avançada

- somente após matriz robusta;
- limites de custo;
- fila e cache;
- assinatura;
- SBOM;
- builds reproduzíveis;
- isolamento de pipeline;
- auditoria.

Não autorizar compilação arbitrária no MVP.

## 4.14 Instalação e onboarding

Criar seção obrigatória para experiência inicial:

- instalador por sistema operacional;
- sem Docker ou banco separado no modo monoposto;
- verificação de requisitos;
- configuração guiada;
- nome da organização;
- região e idioma;
- usuários iniciais;
- importação de planilhas;
- produtos/cadastros;
- backup;
- impressoras e periféricos;
- dados de demonstração opcionais;
- relatório final da configuração.

Prever migração progressiva:

- monoposto;
- rede local;
- sincronização opcional;
- múltiplas unidades.

Não prometer a transição sem spike e testes.

## 4.15 Diagnóstico e ficha da instalação

A instalação deve poder gerar uma ficha contendo:

- distribuição;
- perfil;
- runtime;
- módulos;
- versões;
- modo operacional;
- necessidade de internet;
- localização dos dados;
- política de backup;
- componentes externos;
- permissões;
- lockfile;
- status de atualização;
- canal de suporte.

## 4.16 Catálogo público de módulos

Antes de marketplace, o portal deve oferecer catálogo verificado.

Metadados:

- nome;
- problema resolvido;
- licença;
- mantenedor;
- versão;
- compatibilidade;
- nível de confiança;
- permissões;
- dados tratados;
- uso de rede;
- transmissões externas;
- modo de execução;
- testes;
- status de manutenção;
- documentação;
- screenshots;
- dependências;
- conflitos;
- substitutos.

## 4.17 Interoperabilidade por adaptadores

Reforçar que concorrentes não devem ser incorporados como base do runtime.

Prever adaptadores como:

- importação de Odoo;
- importação de ERPNext;
- importação de Dolibarr;
- importação de Tryton;
- importação de OpenConcerto;
- CSV;
- JSON;
- SQLite documentado;
- integrações futuras.

Os adaptadores devem:

- preservar IDs externos;
- mapear campos;
- gerar relatório de inconsistências;
- permitir dry-run;
- não alterar dados de origem sem pedido;
- oferecer exportação reversa quando tecnicamente possível;
- declarar limitações;
- manter operação local independente da integração.

## 4.18 Segurança e confiança do catálogo

Relacionar com ADR-017:

- experimental;
- comunitário;
- verificado;
- certificado;
- oficial.

O Builder deve preferir:

1. módulos oficiais;
2. módulos verificados compatíveis;
3. módulos comunitários somente quando o usuário entrar em modo avançado e aceitar o risco;
4. nunca recomendar experimental por padrão.

## 4.19 Testes adicionais

Adicionar à estratégia de testes:

### Testes de composição

- dependências;
- conflitos;
- compatibilidade;
- perfis;
- lockfile;
- reprodução;
- remoção;
- atualização.

### Testes de experiência

- instalação limpa;
- primeira execução;
- backup;
- restauração;
- importação;
- desinstalação;
- funcionamento offline;
- hardware mínimo.

### Testes do Builder

- mesma entrada gera composição determinística;
- IA não contorna regras;
- opção “não sei” é suportada;
- sistema funciona sem IA;
- preview corresponde à composição;
- instalador corresponde ao lockfile;
- componentes não verificados não entram no modo padrão.

### Testes de soberania

- exportação legível;
- restauração verificável;
- exclusão;
- transmissão externa declarada;
- telemetria desligada;
- instalação continua útil sem serviço central.

## 4.20 Métricas adicionais

Adicionar:

- taxa de conclusão da triagem;
- tempo até recomendação;
- taxa de download;
- taxa de instalação concluída;
- tempo até primeiro uso útil;
- abandono por etapa;
- erros de compatibilidade;
- instalações reproduzidas pelo lockfile;
- módulos removidos após recomendação;
- porcentagem que escolhe perfil recomendado;
- necessidade de suporte;
- sucesso na importação;
- retenção de uso;
- funcionamento em hardware mínimo.

## 4.21 Novos riscos

Adicionar:

- explosão combinatória de módulos;
- recomendação incorreta da IA;
- preview divergente do produto;
- custo excessivo de builds;
- distribuição não reproduzível;
- onboarding mais complexo que a instalação;
- excesso de perfis por negócio;
- regras regulatórias locais contaminando módulos gerais;
- dependência do portal para reinstalação;
- coleta indevida de dados da triagem;
- SEO prometendo funcionalidades inexistentes;
- catálogo se transformando em marketplace inseguro;
- manutenção de módulos abandonados.

## 4.22 Novos spikes sugeridos

Adicionar após os spikes atuais, sem renumerar ADRs existentes:

### Spike 12 — CLI e scaffolding

Validar:
- criação de módulo nativo;
- criação de módulo em processo;
- manifesto;
- testes;
- documentação;
- execução local;
- `doctor`.

### Spike 13 — Manifesto e lockfile de distribuição

Validar:
- composição reproduzível;
- hashes;
- atualização;
- rollback;
- diagnóstico.

### Spike 14 — Builder baseado em regras

Validar:
- questionário;
- catálogo de capacidades;
- recomendação sem IA;
- seleção de perfil;
- composição válida;
- geração do lockfile.

### Spike 15 — Instalador e onboarding

Validar:
- instalação limpa;
- configuração inicial;
- importação CSV;
- backup;
- funcionamento offline.

### Spike 16 — Preview estrutural

Validar:
- menus;
- módulos ativos;
- telas representativas;
- ausência de promessas falsas.

### Spike 17 — Camada conversacional opcional

Validar:
- IA interpreta respostas;
- motor determinístico mantém autoridade;
- fallback sem IA;
- privacidade e retenção mínima.

### Spike 18 — Montagem de pacote com artefatos pré-construídos

Validar:
- composição sem recompilar código;
- lockfile;
- hashes;
- assinatura futura;
- custo operacional.

## 4.23 Atualizar histórico

Versão 1.3 deve registrar:

- incorpora OpenCore Builder;
- experiência do desenvolvedor;
- manifesto/lockfile;
- catálogo de capacidades;
- composição guiada;
- instalação e onboarding;
- adaptadores;
- testes e métricas adicionais;
- mantém ADR-021 condicionado ao Spike 10.

---

# 5. Comunidade e Governança — versão 1.0

## Arquivo a criar

`03_Comunidade_Governanca_OpenCore_v1.0.md`

## Objetivo

Substituir o rascunho atual por um documento completo e normativo.

## Estrutura mínima obrigatória

1. Objetivo e princípios.
2. Escopo da governança.
3. Participantes e formas de contribuição.
4. Código de conduta.
5. Papéis.
6. Ownership de componentes.
7. Formação de revisores e mantenedores.
8. Processo de decisão.
9. ADR e RFC.
10. Aprovação de pull requests.
11. Governança de módulos.
12. Níveis de confiança.
13. Abandono e transferência de manutenção.
14. Segurança e resposta emergencial.
15. Conflitos de interesse.
16. Patrocinadores e independência.
17. Reconhecimento de contribuições.
18. Programa educacional.
19. Métricas.
20. Processo de evolução da governança.

## Conteúdo obrigatório

### 5.1 Formas de contribuição

Reconhecer:

- código;
- testes;
- documentação;
- tradução;
- design;
- acessibilidade;
- segurança;
- pesquisa;
- suporte;
- triagem;
- mentoria;
- gestão comunitária;
- validação com usuários;
- criação de dados de demonstração.

### 5.2 Papéis

Definir no mínimo:

- usuário;
- contribuidor;
- colaborador recorrente;
- responsável por componente;
- revisor;
- mantenedor em formação;
- mantenedor;
- mantenedor de segurança;
- responsável por distribuição;
- conselho ou comitê futuro.

### 5.3 Progressão

Usar uma progressão equivalente a:

```text
Primeira contribuição
→ colaborador recorrente
→ responsável por componente
→ revisor
→ mantenedor em formação
→ mantenedor
```

Promoção deve considerar:

- qualidade;
- constância;
- conhecimento;
- comportamento;
- capacidade de revisão;
- documentação;
- responsabilidade;
- segurança;
- ausência de conflito grave.

Não basear poder apenas em volume de commits ou financiamento.

### 5.4 Aprovação

Definir:

- mudanças comuns: aprovação de responsável/revisor;
- mudanças em runtime: mantenedor do runtime + testes;
- segurança: mantenedores autorizados;
- arquitetura: ADR/RFC;
- manifesto, licença e governança: consulta ampliada;
- distribuições oficiais: responsável da distribuição + arquitetura + segurança;
- inclusão no catálogo: checklist e nível de confiança.

### 5.5 Ownership

Cada módulo deve declarar:

- mantenedor principal;
- substituto;
- canais de contato;
- status;
- prazo de resposta;
- política de sucessão.

### 5.6 Abandono

Definir estados:

- mantido;
- manutenção limitada;
- procurando mantenedor;
- órfão;
- arquivado;
- substituído;
- removido de distribuições oficiais.

Processo de adoção comunitária deve ser público.

### 5.7 Níveis de confiança

Integrar com ADR-017:

- experimental;
- comunitário;
- verificado;
- certificado;
- oficial.

Definir critérios e quem pode promover/rebaixar.

### 5.8 Programa educacional

Definir:

- good first issues;
- trilhas;
- mentorias;
- capstones;
- revisão assistida;
- projetos reais;
- critérios profissionais;
- evidências públicas;
- promoção de mantenedores;
- obrigação de não reduzir padrões técnicos.

### 5.9 Métricas

Incluir:

- tempo para configurar o ambiente;
- tempo até primeiro PR;
- taxa de conclusão;
- retenção;
- número de revisores;
- módulos sem substituto;
- tempo de revisão;
- dependência do fundador;
- diversidade de contribuição;
- documentação necessária para contribuição sem ajuda privada;
- transferências de manutenção bem-sucedidas.

### 5.10 Relação com o Builder

Definir contribuição também para:

- perfis de negócio;
- textos de triagem;
- catálogo de capacidades;
- validação com usuários;
- traduções;
- acessibilidade;
- templates;
- dados de demonstração.

Mudanças que afetem recomendações do Builder devem passar por revisão de produto e compatibilidade, não apenas revisão editorial.

---

# 6. Plano Institucional — versão 1.0

## Arquivo a criar

`04_Plano_Institucional_OpenCore_v1.0.md`

## Objetivo

Transformar o rascunho em plano completo de parcerias, sustentabilidade, proteção institucional e adoção.

## Estrutura mínima

1. Propósito institucional.
2. Hierarquia de prioridades.
3. Tipos de parceiros.
4. Regras de independência.
5. Plataformas de ensino.
6. Universidades e bootcamps.
7. Empresas usuárias e patrocinadoras.
8. Comunidades open source.
9. Rede de prestadores.
10. Certificação.
11. Sustentabilidade.
12. OpenCore Builder como canal de adoção.
13. Marketing e aquisição.
14. Métricas.
15. Riscos.
16. Etapas de implantação.

## Conteúdo obrigatório

### 6.1 Hierarquia

Preservar:

1. usuários e organizações;
2. continuidade e segurança;
3. mantenedores;
4. contribuidores;
5. parceiros e patrocinadores.

### 6.2 Formas de sustentabilidade

Prever:

- instalação;
- configuração;
- suporte;
- treinamento;
- manutenção;
- migração;
- integração;
- desenvolvimento sob demanda;
- hospedagem opcional;
- certificação;
- patrocínio;
- OpenCollective/GitHub Sponsors;
- materiais educacionais;
- programas institucionais.

Não cobrar licença por usuário para distribuições oficiais abertas.

### 6.3 Rede de prestadores

Planejar:

- diretório público;
- critérios;
- especialidades;
- regiões;
- idiomas;
- avaliações e contestação;
- renovação;
- conflito de interesse;
- suspensão;
- não exclusividade.

Usuário deve poder contratar terceiros fora da rede.

### 6.4 Certificação

Distinguir:

- distribuição oficial;
- módulo oficial;
- módulo verificado;
- módulo certificado;
- prestador certificado;
- edição comercial certificada.

Certificação não altera direitos concedidos pela licença.

### 6.5 Builder como canal de adoção

O Builder deve:

- orientar usuários leigos;
- apresentar prestadores opcionalmente;
- não condicionar download à contratação;
- não exigir cadastro para explicar a recomendação;
- minimizar coleta de dados;
- permitir download gratuito;
- deixar claro o que é suporte comunitário e comercial;
- permitir salvar ou exportar a composição.

### 6.6 SEO e marketing

Criar estratégia baseada em necessidades reais:

- sistema gratuito para padaria;
- controle de estoque offline;
- sistema para oficina sem mensalidade;
- sistema para condomínio;
- software de gestão local;
- sistema para associação;
- sistema para biblioteca.

Regras:

- não prometer distribuição inexistente;
- páginas por segmento devem apontar para perfis realmente suportados;
- linguagem de resultado antes de arquitetura;
- destacar:
  - sem mensalidade obrigatória;
  - funcionamento local;
  - dados sob controle do usuário;
  - instalação guiada;
  - módulos conforme necessidade;
  - código aberto;
  - suporte opcional.

### 6.7 Propostas de mensagem

Registrar como referência, não slogan definitivo:

- “Sistemas que pertencem a quem usa.”
- “Você explica como seu negócio funciona. O OpenCore prepara o sistema certo.”
- “Seu sistema pronto, sem mensalidade e sem nuvem obrigatória.”
- “Não procure módulos. Conte o que seu negócio precisa.”
- “Um sistema do tamanho da sua organização.”

### 6.8 Métricas institucionais

- organizações ativas;
- instalações concluídas;
- distribuições em uso;
- importações realizadas;
- prestadores ativos;
- receita por fonte;
- concentração de financiamento;
- usuários atendidos sem serviço pago;
- retenção;
- incidentes;
- satisfação;
- contribuições originadas de instituições;
- mantenedores formados;
- taxa de conversão do Builder.

---

# 7. Roadmap — versão 2.3

## Arquivo a criar

`05_Roadmap_OpenCore_v2.3.md`

## Base

Copiar o Roadmap 2.2 e incorporar as mudanças sem destruir a sequência já definida.

## Alterações estratégicas

As três trilhas permanecem:

1. produto e arquitetura;
2. comunidade e educação;
3. institucional e sustentabilidade.

Adicionar uma quarta perspectiva transversal:

4. **experiência e adoção do usuário final**: descoberta, triagem, instalação, onboarding, importação, suporte e continuidade.

Não tratar como quarta equipe obrigatória; é uma lente transversal.

## Mudanças por etapa

### Etapa 0 — alinhamento

Adicionar entregas:

- Manifesto 1.2;
- Arquitetura 1.3;
- Comunidade/Governança 1.0;
- Plano Institucional 1.0;
- ADR-022;
- Especificação Builder v0;
- Benchmarks v1.0;
- política de uso de código externo;
- definição de distribuição, perfil e composição;
- definição de manifesto e lockfile.

Critério adicional:

- pessoa leiga consegue entender como obter uma distribuição sem aprender arquitetura.

### Etapa 1 — spikes e estrutura de contribuição

Adicionar:

- Spike 12 CLI/scaffolding;
- Spike 13 manifesto/lockfile;
- primeira versão do catálogo de capacidades em arquivo estático;
- primeiro perfil de negócio simples;
- protótipo de instalador monoposto;
- `opencore doctor`;
- dados de demonstração;
- teste de instalação por pessoa externa.

Não construir IA ou portal completo nesta etapa.

### Etapa 2 — Portaria

Além da fatia vertical:

- instalador funcional;
- onboarding;
- importação CSV mínima;
- backup/restauração;
- ficha da instalação;
- lockfile;
- perfil Essencial;
- preview estrutural interno;
- teste com usuário não técnico.

Embora a Portaria seja a primeira distribuição, o catálogo de capacidades deve usar termos genéricos, evitando contaminação.

### Etapa 3 — SDK v0

Adicionar:

- CLI inicial;
- scaffolds;
- catálogo de metadados;
- testes de composição;
- manifesto/lockfile estabilizados experimentalmente;
- contrato de adaptadores;
- campos necessários ao Builder;
- suíte de conformidade.

### Etapa 4 — piloto fechado

Expandir participantes:

- desenvolvedores;
- documentadores;
- designers;
- usuários finais de uma organização piloto.

Validar:

- instalação;
- primeira execução;
- linguagem;
- triagem;
- onboarding;
- importação;
- suporte;
- restauração.

### Etapa 5 — piloto institucional

Incluir trilhas:

- desenvolvimento de módulos;
- documentação;
- catálogo de capacidades;
- testes de instalação;
- acessibilidade;
- criação de perfis de negócio;
- adaptadores de migração.

### Etapa 6 — alpha público e Builder MVP

Renomear a etapa para incluir explicitamente o Builder.

Entregas:

- páginas por distribuição;
- triagem baseada em regras;
- opção “não sei”;
- recomendação de perfis verificados;
- preview estrutural;
- personalização limitada;
- manifesto/lockfile;
- download de pacote pré-construído;
- documentação;
- catálogo de módulos verificados;
- ficha da configuração;
- instalação guiada.

Fora do escopo:

- compilação arbitrária;
- qualquer módulo comunitário por padrão;
- marketplace;
- IA obrigatória;
- pagamentos complexos;
- builds ilimitados.

### Etapa 7 — beta e ecossistema

Adicionar:

- montagem por artefatos assinados;
- rede de prestadores;
- adaptadores para sistemas externos;
- perfis adicionais;
- segunda distribuição completa;
- IA conversacional opcional;
- catálogo ampliado;
- assinatura;
- SBOM;
- certificação.

### Etapa futura condicionada

Registrar sem número fixo ou como backlog posterior:

- marketplace;
- geração avançada;
- segundo SDK;
- sandbox;
- personalização livre;
- sincronização avançada;
- serviços de nuvem.

## Ordem prática revisada

A ordem deve incluir:

1. consolidar documentos;
2. criar ADR-022 e especificação;
3. executar spikes técnicos existentes;
4. validar CLI, lockfile e instalador;
5. construir Portaria;
6. testar com usuário leigo;
7. extrair SDK;
8. piloto comunitário;
9. Builder baseado em regras;
10. alpha;
11. piloto institucional;
12. segunda distribuição;
13. adaptadores;
14. IA opcional;
15. ecossistema comercial somente após confiança.

---

# 8. ADR-022 — OpenCore Builder

## Arquivo a criar

`ADR-022_OpenCore_Builder_Triagem_Composicao_Empacotamento.md`

## Status

`Proposto, condicionado aos Spikes 14–18`

## Estrutura obrigatória

1. Contexto.
2. Problema.
3. Decisão.
4. Componentes.
5. Papel da IA.
6. Validação determinística.
7. Distribuições e perfis.
8. Manifesto e lockfile.
9. Preview.
10. Empacotamento progressivo.
11. Privacidade.
12. Segurança.
13. Licenciamento.
14. Consequências.
15. Alternativas consideradas.
16. Critérios de aceitação.
17. Fora do escopo.
18. Relação com outras ADRs.
19. Changelog.

## Decisão central

O OpenCore terá um Builder externo ao runtime, que transforma necessidades expressas pelo usuário em uma composição válida de distribuição, perfil e módulos.

A IA é opcional e subordinada a um motor de regras e ao validador de compatibilidade.

## Componentes

- questionário;
- interpretador de necessidades;
- catálogo de capacidades;
- perfis de negócio;
- motor de recomendação;
- validador;
- preview;
- gerador de manifesto;
- gerador de lockfile;
- seletor/empacotador;
- catálogo de artefatos;
- ficha da instalação.

## Critérios de aceitação

1. Funciona sem IA.
2. Suporta “não sei”.
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

## Alternativas

Avaliar e rejeitar/adiar:

- catálogo manual sem recomendação;
- IA controlando tudo;
- download de módulos individualmente;
- build totalmente dinâmico desde o início;
- um ERP monolítico por segmento;
- portal obrigatório durante a operação;
- marketplace como primeira etapa.

---

# 9. Especificação funcional do Builder v0

## Arquivo a criar

`06_Especificacao_OpenCore_Builder_v0.md`

## Natureza

Documento de produto e UX, subordinado ao Manifesto, Arquitetura e ADR-022.

## Estrutura obrigatória

1. Visão.
2. Problema do usuário.
3. Personas.
4. Princípios de UX.
5. Jornada.
6. Triagem.
7. Catálogo de capacidades.
8. Perfis.
9. Recomendação.
10. Preview.
11. Personalização.
12. Validação.
13. Download.
14. Instalação.
15. Onboarding.
16. Importação.
17. Backup e continuidade.
18. Suporte.
19. SEO e páginas de entrada.
20. Privacidade.
21. Acessibilidade.
22. Métricas.
23. MVP.
24. Evolução.
25. Critérios de aceite.

## Persona principal de referência

Pequeno empresário sem conhecimento técnico, por exemplo dono de padaria, oficina, academia ou pequeno comércio, que:

- precisa de estoque, caixa e gestão;
- procura opção gratuita;
- não sabe o que é runtime, módulo, Docker ou banco;
- utiliza principalmente Windows;
- precisa começar com um computador;
- pode evoluir para vários;
- possui dados em planilha;
- quer suporte opcional;
- não quer mensalidade obrigatória.

Não limitar o produto a essa persona. Usá-la como teste de simplicidade.

## Jornada

```text
Busca
→ página por necessidade
→ triagem
→ recomendação
→ explicação
→ preview
→ personalização limitada
→ validação
→ download
→ instalação
→ configuração inicial
→ importação
→ backup
→ primeiro uso
```

## Perguntas mínimas da triagem

- tipo de organização;
- capacidades necessárias;
- sistema operacional;
- quantidade de computadores;
- quantidade de usuários;
- uma ou mais unidades;
- necessidade de acesso remoto;
- necessidade de funcionamento sem internet;
- sistema atual;
- formato dos dados;
- equipamentos;
- idioma/região;
- nível de ajuda desejado.

Sempre oferecer:

- “não sei”;
- “decidir depois”;
- explicação curta;
- recomendação padrão segura.

## Exemplo de padaria

Necessidades possíveis:

- vendas no balcão;
- caixa;
- produtos;
- estoque;
- compras;
- fornecedores;
- clientes;
- contas;
- encomendas;
- validade;
- ficha técnica;
- integração com balança;
- impressora;
- múltiplos caixas;
- múltiplas unidades.

O documento deve mostrar um exemplo de resultado:

### Padaria Essencial

- vendas;
- caixa;
- produtos;
- estoque;
- fornecedores;
- backup;
- relatórios básicos.

### Padaria Completo

- Essencial;
- financeiro;
- clientes;
- compras;
- encomendas;
- validade;
- relatórios.

### Padaria Multiestação

- Completo;
- usuários;
- permissões;
- rede local;
- backup centralizado.

Deixar explícito que são exemplos e dependem de módulos realmente implementados.

## Explicabilidade

Cada recomendação deve dizer:

- o que foi incluído;
- por que;
- o que ficou opcional;
- o que exige internet;
- o que transmite dados;
- o que exige hardware;
- o que pode ser removido;
- o que é necessário por dependência.

## Modos de uso

### Simples

- perfil recomendado;
- poucas decisões;
- módulos verificados;
- linguagem comum.

### Avançado

- módulos;
- versões;
- permissões;
- adaptadores;
- requisitos;
- detalhes técnicos;
- riscos.

## Download

Mostrar:

- sistema operacional;
- tamanho aproximado;
- versão;
- composição;
- checksum;
- assinatura futura;
- licença;
- documentação;
- necessidade de internet;
- requisitos;
- última atualização.

## Instalação

Objetivo: “próximo, próximo, concluir” quando seguro.

Nunca exigir no modo monoposto:

- Docker;
- terminal;
- instalação manual de SQLite;
- instalação manual de Python/Node;
- edição manual de arquivos.

## Onboarding

Perguntar:

- organização;
- usuários;
- localização;
- moeda;
- backup;
- importação;
- equipamentos;
- dados de demonstração.

## Privacidade

- triagem anônima por padrão;
- não solicitar faturamento, CPF, CNPJ ou dados sensíveis sem necessidade;
- informar se respostas são enviadas à IA;
- permitir modo sem IA;
- retenção mínima;
- não usar respostas para anúncios;
- telemetria desligada por padrão.

## Acessibilidade

- linguagem simples;
- teclado;
- leitores de tela;
- contraste;
- explicações;
- não depender somente de cores;
- opção de voltar;
- salvar progresso localmente;
- compatibilidade móvel para a triagem, mesmo que o produto seja desktop.

## MVP

1. páginas por distribuição real;
2. questionário estático;
3. motor de regras;
4. três perfis;
5. preview de navegação;
6. personalização limitada;
7. lockfile;
8. instalador pré-construído;
9. onboarding;
10. importação CSV;
11. backup/restauração;
12. documentação.

IA fica fora do MVP obrigatório.

---

# 10. Benchmarks do ecossistema

## Arquivo a criar

`07_Benchmarks_Ecossistema_OpenCore_v1.0.md`

## Natureza

Documento não normativo. Deve registrar padrões observados, não copiar código.

## Projetos de referência

- Tryton;
- Frappe/ERPNext;
- Odoo;
- OpenConcerto;
- Dolibarr;
- NocoBase;
- Apache OFBiz;
- Moodle;
- OpenMRS;
- Appsmith;
- Axelor;
- Budibase, se relevante.

## Para cada projeto

Registrar:

- categoria;
- arquitetura;
- licenciamento;
- público;
- pontos fortes;
- pontos fracos para a proposta OpenCore;
- padrão que vale estudar;
- padrão que não deve ser incorporado;
- possibilidade de interoperabilidade;
- risco jurídico de copiar código;
- apelo de marketing.

## Síntese esperada

### Padrões a incorporar conceitualmente

- manifestos e dependências;
- ciclo de vida;
- CLI e scaffolding;
- contratos de serviços;
- módulos e perfis;
- operação local;
- catálogo;
- governança;
- formação de mantenedores;
- rede de prestadores;
- instalação guiada;
- descoberta por necessidade.

### Padrões a melhorar

- web/server como requisito;
- linguagem única;
- banco compartilhado sem fronteira;
- Community limitada;
- dependência de empresa central;
- instalação complexa;
- catálogo sem compatibilidade;
- usuário obrigado a entender módulos;
- dados presos;
- marketplace prematuro.

### Diferenciação OpenCore

- offline-first verificável;
- distribuições nativas instaláveis;
- soberania testada;
- manifesto/lockfile;
- composição guiada;
- IA subordinada a regras;
- módulos multilíngues;
- distribuições oficiais integralmente abertas;
- educação ligada a manutenção real;
- baixo custo operacional;
- migração por adaptadores.

Não incluir números atuais de usuários sem fonte e data verificáveis. Caso números sejam adicionados, marcar data, origem e natureza autodeclarada.

---

# 11. Atualização do índice de versões

## Arquivo

`00_Indice_Versoes.md`

## Mudanças

### Canônicos

Atualizar para:

- Manifesto 1.2;
- Arquitetura 1.3;
- Comunidade/Governança 1.0;
- Plano Institucional 1.0;
- Roadmap 2.3;
- ADR-022 proposto;
- especificação Builder v0;
- benchmarks v1.0 não normativo.

### Histórico

Mover versões anteriores para histórico:

- Manifesto 1.1;
- Arquitetura 1.2;
- rascunho de comunidade;
- rascunho institucional;
- Roadmap 2.2.

### ADRs

Manter:

- ADR-015 matriz;
- ADR-016 portabilidade;
- ADR-017 confiança;
- ADR-018 atualização;
- ADR-019 sincronização;
- ADR-020 testes;
- ADR-021 módulos em processo;
- ADR-022 Builder.

### Próxima etapa

A próxima etapa deve ser descrita como:

1. revisar e aprovar os documentos 1.2/1.3/1.0/2.3;
2. revisar ADR-022;
3. não iniciar implementação do Builder completo;
4. iniciar Etapa 1 pelos spikes técnicos;
5. priorizar fatia vertical comum;
6. executar Spike 10;
7. executar CLI/lockfile/instalador de forma time-boxed;
8. documentar decisões.

---

# 12. Consistência entre documentos

O Cursor deve verificar as seguintes relações:

| Tema | Manifesto | Arquitetura | Comunidade | Institucional | Roadmap | ADR |
|---|---|---|---|---|---|---|
| Acessibilidade operacional | princípio/direito | requisitos | validação com usuários | adoção | etapas | ADR-022 |
| Builder | compromisso | componentes | contribuições | canal | Etapas 6–7 | ADR-022 |
| IA | limite ético | subordinada a regras | revisão | privacidade | pós-MVP | ADR-022 |
| Distribuição | direito | manifesto/lockfile | ownership | certificação | etapas | ADR-022 |
| Catálogo | transparência | metadados | revisão | canal | alpha | ADR-017/022 |
| Instalação | direito | requisitos | testes | suporte | Etapa 2/6 | ADR-022 |
| CLI | — | ferramenta | onboarding | educação | Etapa 1/3 | spike |
| Adaptadores | portabilidade | integração | manutenção | serviços | beta | ADR futura se necessário |
| Licenças externas | liberdade | fronteira | revisão | risco | Etapa 0 | política |
| Perfis | simplicidade | composição testada | validação | marketing | alpha | ADR-022 |

Nenhum documento deve afirmar:

- que a IA monta qualquer combinação;
- que o portal compila tudo no MVP;
- que qualquer módulo pode entrar numa distribuição oficial;
- que módulos em processo são sandbox;
- que o Builder pertence ao runtime;
- que o usuário precisa do portal para continuar usando o sistema;
- que a nuvem é obrigatória;
- que a Portaria é o produto central;
- que todas as funcionalidades mencionadas já existem.

---

# 13. Convenções de escrita

1. Português do Brasil.
2. Linguagem técnica precisa.
3. Usar “OpenCore Runtime” para o processo principal.
4. Usar “runtime da linguagem” para Python/Node.
5. Usar `execution.mode`.
6. Usar “módulo em processo”, não “plugin Python dentro do runtime”.
7. Usar “distribuição” para composição testada.
8. Usar “perfil” para variante de uma distribuição.
9. Usar “composição” para conjunto exato escolhido.
10. Usar “manifesto” para intenção/faixas.
11. Usar “lockfile” para versões exatas.
12. Usar “OpenCore Builder” como nome provisório consistente.
13. Não usar “ERP OpenCore” como identidade geral.
14. Não chamar self-hosted de offline-first automaticamente.
15. Não prometer sandbox.
16. Não usar “100% seguro”.
17. Diferenciar:
    - gratuito;
    - open source;
    - oficial;
    - certificado;
    - comercial;
    - proprietário.

---

# 14. Não fazer nesta tarefa

- Não implementar código Rust, Python ou frontend.
- Não criar pipeline de build.
- Não adicionar dependências.
- Não criar marketplace.
- Não criar IA.
- Não alterar licenças.
- Não aceitar ADR-021 sem Spike 10.
- Não aceitar ADR-022 sem spikes.
- Não remover históricos.
- Não renumerar ADR-015 a ADR-021.
- Não alterar a Portaria para padaria; padaria é apenas exemplo de UX.
- Não colocar regras de negócio de padaria no runtime.
- Não criar promessa fiscal/contábil sem domínio e conformidade específicos.
- Não copiar documentação de concorrentes.
- Não inventar estatísticas de adoção.
- Não declarar tecnologia provisória como definitiva.

---

# 15. Checklist de validação

Antes de concluir, verificar:

## Arquivos

- [ ] Novas versões foram criadas.
- [ ] Históricos foram preservados.
- [ ] Índice aponta para arquivos existentes.
- [ ] Espelhos foram atualizados.
- [ ] ADR-022 existe.
- [ ] Especificação Builder existe.
- [ ] Benchmarks existem.

## Consistência

- [ ] Manifesto e arquitetura concordam.
- [ ] Roadmap não antecipa marketplace.
- [ ] IA é opcional.
- [ ] Motor de regras é autoridade.
- [ ] Builder não é runtime.
- [ ] Distribuições oficiais são abertas.
- [ ] Lockfile é distinto de manifesto.
- [ ] Perfil é distinto de distribuição.
- [ ] Portal não é requisito para operação.
- [ ] Telemetria continua desativada.
- [ ] Backup continua distinto de exportação.
- [ ] Sincronização continua adaptador.
- [ ] Módulo em processo não é chamado de sandbox.

## Usuário final

- [ ] Existe jornada completa da busca ao primeiro uso.
- [ ] Existe opção “não sei”.
- [ ] Instalação monoposto não exige terminal.
- [ ] Existe onboarding.
- [ ] Existe importação.
- [ ] Existe ficha da instalação.
- [ ] Existe política de suporte opcional.
- [ ] Existe explicação de módulos e transmissões.

## Desenvolvimento

- [ ] CLI foi descrita.
- [ ] Scaffolding foi descrito.
- [ ] Conformance tests foram preservados.
- [ ] Manifesto de módulo recebeu metadados para catálogo.
- [ ] Manifesto e lockfile de distribuição foram definidos.
- [ ] Spikes foram adicionados.
- [ ] Complexidade foi time-boxed.

## Governança

- [ ] Papéis foram definidos.
- [ ] Aprovação de PRs foi definida.
- [ ] Ownership foi definido.
- [ ] Abandono foi tratado.
- [ ] Confiança foi integrada.
- [ ] Contribuições não técnicas foram reconhecidas.
- [ ] Patrocinadores não controlam a missão.

---

# 16. Saída esperada do Cursor

Após realizar as alterações, retornar:

1. lista dos arquivos criados;
2. lista dos arquivos modificados;
3. versões novas;
4. resumo das decisões incorporadas;
5. decisões mantidas como hipóteses;
6. conflitos encontrados;
7. inconsistências corrigidas;
8. pontos que exigem validação humana;
9. diff ou commits organizados por documento;
10. confirmação de que nenhuma implementação foi iniciada.

Organizar preferencialmente em commits separados:

1. `docs: add manifesto 1.2`
2. `docs: add architecture 1.3`
3. `docs: add community governance 1.0`
4. `docs: add institutional plan 1.0`
5. `docs: add builder ADR and specification`
6. `docs: add ecosystem benchmarks`
7. `docs: update roadmap 2.3 and version index`

---

# 17. Resumo executivo da decisão

O OpenCore deve absorver padrões maduros de ecossistemas existentes sem se tornar uma cópia deles:

- do Tryton: manifesto, dependências e modularidade;
- do Frappe: CLI, scaffolding e experiência do desenvolvedor;
- do Odoo: descoberta por resultado e organização do catálogo;
- do OpenConcerto: operação local e sustentabilidade por serviços;
- do NocoBase: ciclo de vida claro e microkernel conceitual;
- do OFBiz: separação entre dados, serviços, eventos e interface;
- do Moodle: governança de plugins;
- do OpenMRS: comunidade orientada a problemas reais e formação de mantenedores.

O diferencial do OpenCore deve ser a combinação:

> **offline-first verificável + distribuições nativas instaláveis + soberania testada + módulos multilíngues + composição guiada para usuários leigos + distribuições oficiais integralmente abertas + formação real de mantenedores.**

A experiência do usuário final deve ser:

> **contar o que precisa, receber uma recomendação explicada, visualizar a composição, ajustar opções compatíveis, baixar um pacote instalável e começar a operar sem montar infraestrutura manualmente.**

A IA pode melhorar a conversa, mas o motor de compatibilidade, os manifestos, os lockfiles, os níveis de confiança e as distribuições verificadas permanecem como autoridade técnica.
