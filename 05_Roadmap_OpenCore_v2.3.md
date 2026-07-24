# OpenCore — Roadmap Revisado v2.3

**Status:** Etapa 0 documental em evolução (Manifesto 1.2, Arquitetura 1.3, Builder e benchmarks); RFC-0001 em consulta; Etapa 1 autorizada para spikes técnicos controlados.  
**Objetivo:** transformar a visão do OpenCore em uma sequência executável, validando arquitetura, produto, comunidade, adoção educacional e experiência do usuário final sem criar infraestrutura prematura.  
**Documentos relacionados:** Manifesto v1.2 · Arquitetura v1.3 · Comunidade e Governança v1.0 · Plano Institucional v1.0 · ADR-015..022 · Especificação Builder v0 · Benchmarks v1.0 · RFC-0001 ([consulta #2](https://github.com/duvallemusic/open-core/issues/2))  
**Histórico:** supersede Roadmap v2.2 (`05_Roadmap_OpenCore_v2.2.md`).

---

## 1. Direção estratégica

O OpenCore é uma plataforma open source para criação de sistemas desktop modulares e multiplataforma. O Sistema de Portaria será a primeira distribuição de referência, mas não o produto central.

A evolução deve ocorrer em três trilhas paralelas:

1. **Produto e arquitetura:** núcleo, módulos, SDK, interface e distribuições.
2. **Comunidade e educação:** documentação, trilhas de contribuição, mentorias e formação de mantenedores.
3. **Institucional e sustentabilidade:** parceiros, governança, métricas, patrocínio e proteção da independência do projeto.

Além das três trilhas, o roadmap aplica uma **lente transversal** (não uma quarta equipe obrigatória):

4. **Experiência e adoção do usuário final:** descoberta, triagem, instalação, onboarding, importação, suporte e continuidade — de modo que pessoas sem conhecimento técnico consigam obter e operar uma distribuição sem aprender a arquitetura da plataforma.

Plataformas de ensino não devem aparecer apenas no final do roadmap. A possibilidade de adoção por plataformas de ensino, universidades e bootcamps precisa influenciar desde o início a documentação, a separação das tarefas, os critérios de revisão e a forma de demonstrar contribuições.

Ao mesmo tempo, o núcleo técnico deve continuar sendo projetado como software profissional. A camada educacional organiza a participação ao redor do produto; ela não deve reduzir os padrões de arquitetura, testes, segurança ou manutenção.

---

## 2. Modelo arquitetural de referência

Antes de implementar o SDK, o projeto deve formalizar os níveis abaixo, alinhados à ADR-015 (matriz de classificação) e à Arquitetura v1.3:

### 2.1 OpenCore Runtime

Parte mínima e não removível da plataforma (Rust):

- inicialização da aplicação;
- configuração;
- registro e ciclo de vida de módulos;
- Module Host (supervisão de módulos em processo — hipótese ADR-021);
- banco e migrações;
- barramento de eventos;
- logs e diagnóstico;
- contratos mínimos de segurança e integridade;
- contratos estruturais de atualização (validação, compatibilidade, migração e recuperação — ADR-015 / ADR-018).

**Nota:** isolamento por processo = isolamento de falhas, não sandbox completa de SO (ADR-021 v1.1).

### 2.2 Módulos-base

Serviços reutilizáveis que podem ser exigidos por uma distribuição:

- autenticação;
- permissões;
- backup e restauração;
- relatórios;
- internacionalização;
- interface administrativa opcional de atualização.

**Nota (ADR-015 / ADR-018):** “atualização” não é um módulo-base monolítico. O runtime detém os contratos estruturais; a interface administrativa é opcional e modular; feeds, download e provedores ficam em adaptadores.

### 2.3 Adaptadores transversais

Categoria transversal (não é uma nova camada obrigatória do runtime). Exemplos:

- sincronização (sempre adaptador — ADR-015 / ADR-019; nunca módulo-base);
- feeds e provedores de atualização / obtenção de artefatos;
- telemetria opcional;
- integrações externas (e-mail, APIs, identidade, armazenamento remoto de backup, etc.);
- importação/exportação a partir de sistemas externos (CSV, JSON e adaptadores condicionados a evidência).

### 2.4 Módulos de domínio

Funcionalidades específicas de um tipo de sistema:

- visitantes;
- entregas;
- moradores;
- estoque;
- agendamento;
- caixa;
- clientes;
- biblioteca;
- estacionamento.

### 2.5 Distribuições, perfis e composição

Combinações testadas e empacotadas de módulos. Exemplos de distribuição:

- OpenCore Portaria;
- OpenCore Academia;
- OpenCore Oficina;
- OpenCore Biblioteca;
- OpenCore Estacionamento.

Conceitos distintos (Arquitetura v1.3 / ADR-022):

- **distribuição:** composição testada e empacotada para um público;
- **perfil:** variante verificada de uma distribuição (ex.: Essencial, Completo, Multiestação);
- **composição:** conjunto exato escolhido (manifesto + lockfile).

O **OpenCore Builder** é ferramenta/serviço auxiliar externo ao runtime (Apache 2.0; ADR-022), não parte do runtime mínimo. No MVP, entrega seleção de pacotes pré-construídos; montagem por artefatos e geração avançada ficam condicionadas a evidência.

Esse modelo impede que o Sistema de Portaria contamine o núcleo com regras específicas e permite que instituições de ensino trabalhem em módulos isolados sem precisar compreender toda a plataforma.

---

# 3. Roadmap por etapas

## Etapa 0 — Alinhamento e regras fundamentais

### Objetivo

Transformar os documentos atuais em uma base de decisão suficientemente clara para iniciar o repositório principal e orientar a experiência de obtenção de uma distribuição.

### Entregas

- manifesto v1.2 (acessibilidade operacional, descoberta guiada e reproduzibilidade);
- arquitetura v1.3 (Builder, manifesto/lockfile, catálogo, instalação e onboarding);
- Comunidade e Governança v1.0;
- Plano Institucional v1.0;
- ADR-022 (OpenCore Builder — triagem, composição e empacotamento);
- Especificação funcional do Builder v0;
- Benchmarks do ecossistema v1.0 (referência não normativa);
- política de uso de código externo (sem incorporar GPL/AGPL/LGPL sem análise);
- definição explícita do objetivo principal e dos objetivos secundários;
- licença do núcleo e política para módulos;
- modelo inicial de governança;
- código de conduta;
- política de contribuição;
- política de segurança;
- processo de RFC e ADR;
- definição das camadas arquiteturais e da distinção distribuição / perfil / composição;
- definição de manifesto e lockfile de distribuição;
- critérios iniciais para aceitar módulos e distribuições;
- princípios de independência em relação a patrocinadores e parceiros educacionais.

### Decisões obrigatórias

- licença do núcleo;
- possibilidade ou não de módulos proprietários;
- quem pode aprovar mudanças no núcleo;
- como novos mantenedores serão formados;
- qual é a relação entre impacto social, educação e adoção comercial;
- quais decisões exigem RFC pública;
- limites do Builder e da IA (IA opcional e subordinada a regras determinísticas).

### Critério de saída

Uma pessoa externa consegue ler os documentos e explicar o que é o OpenCore, o que ele não é, quem decide e como contribuir. **Uma pessoa leiga consegue entender como obter uma distribuição sem precisar aprender a arquitetura da plataforma.**

---

## Etapa 1 — Spike técnico e estrutura de contribuição

### Objetivo

Validar as decisões técnicas de maior risco antes de construir o produto, incluindo base mínima para instalação e composição.

### Produto e arquitetura

Criar pequenos protótipos descartáveis ou isolados para comprovar:

- aplicação Rust com interface Slint em Windows, Linux e macOS;
- persistência SQLite e migrações;
- carregamento e registro de dois módulos nativos simples;
- comunicação por eventos;
- empacotamento e atualização de esquema;
- exportação de dados em formato aberto;
- testes automatizados do runtime e dos módulos;
- **Spike 10 (ADR-021 v1.1):** módulo em processo **headless** em **Python** (preferência), via OpenCore Module Protocol (stdio + framing + JSON-RPC ou subconjunto); storage mediado sem SQL genérico; comparação Opção A vs B; sem `ui_schema`; sem segundo SDK;
- **Spike 12 — CLI e scaffolding:** `opencore new` / `new-module` / `validate` / `run` / `test` / `doctor` (hipótese de produto); criação de módulo nativo e em processo; manifesto; testes; documentação; execução local;
- **Spike 13 — Manifesto e lockfile de distribuição:** composição reproduzível; hashes; atualização; rollback; diagnóstico;
- primeira versão do **catálogo de capacidades** em arquivo estático;
- primeiro **perfil de negócio simples** (variante verificada mínima);
- protótipo de **instalador monoposto** (sem Docker nem banco separado);
- comando ou protótipo de **`opencore doctor`**;
- **dados de demonstração** para o primeiro módulo/distribuição de exemplo;
- **teste de instalação por pessoa externa** (sem assistência direta além da documentação).

**Fora do escopo desta etapa:** IA conversacional; portal completo; marketplace; compilação arbitrária de combinações.

Os protótipos devem responder se a stack escolhida atende aos requisitos. Eles não devem ser tratados como código definitivo apenas porque funcionaram.

### Comunidade e educação

Preparar o repositório para aprendizado guiado:

- README de entrada;
- guia de ambiente local;
- mapa da arquitetura;
- template de issue;
- template de pull request;
- níveis de dificuldade;
- critérios objetivos de revisão;
- primeiro módulo de exemplo (nativo e, se Spike 10 aceito, esboço do template de processo);
- trilha “primeira contribuição”.

### Institucional

Iniciar conversas exploratórias com pessoas ligadas a plataformas de ensino e comunidades, sem apresentar ainda uma proposta de adoção formal. O objetivo é entender requisitos de integração, formato de desafio, duração de coortes, mentoria e avaliação.

### Experiência do usuário final (lente transversal)

Validar, mesmo em protótipo:

- se a documentação descreve “como obter e instalar” em linguagem não técnica;
- se o instalador monoposto reduz passos manuais;
- se `doctor` e dados de demonstração aceleram o primeiro uso útil.

### Critério de saída

- a stack funciona nos três sistemas operacionais;
- dois módulos conseguem operar sem acoplamento indevido;
- uma pessoa externa consegue executar o projeto seguindo apenas a documentação;
- **o Spike 10 foi aceito, rejeitado ou adiado com evidências documentadas;**
- Spikes 12 e 13 foram executados com evidências (aceitos, rejeitados ou adiados);
- existe catálogo estático mínimo, perfil simples e protótipo de instalador/doctor testáveis.

---

## Etapa 2 — Primeira fatia vertical: OpenCore Portaria

### Objetivo

Validar o OpenCore como plataforma através de uma distribuição real, e não construir um sistema de portaria isolado — incluindo caminho instalável e utilizável por pessoa não técnica.

### Escopo mínimo recomendado

- runtime do OpenCore;
- módulo de autenticação e permissões;
- módulo de moradores ou unidades;
- módulo de visitantes;
- módulo de entregas;
- logs;
- backup e restauração;
- relatórios básicos;
- exportação de dados;
- instaladores de teste para os três sistemas operacionais;
- **instalador funcional** do modo monoposto;
- **onboarding** guiado (organização, usuários iniciais, região/idioma, backup);
- **importação CSV mínima**;
- **ficha da instalação** (distribuição, perfil, módulos, versões, modo operacional, lockfile, localização dos dados);
- **lockfile** da composição instalada;
- **perfil Essencial** da Portaria (variante verificada mínima);
- **preview estrutural interno** (navegação/módulos ativos — sem prometer telas inexistentes);
- **teste com usuário não técnico** (instalação → primeiro uso útil).

Embora a Portaria seja a primeira distribuição, o **catálogo de capacidades deve usar termos genéricos**, evitando contaminação do runtime ou do catálogo com regras exclusivas de portaria.

### Regras

- nenhuma regra de portaria deve entrar no runtime;
- cada módulo deve declarar dependências e `execution.mode`;
- o banco deve permitir migrações por módulo;
- desativar um módulo não pode corromper dados de outro;
- a distribuição precisa funcionar offline;
- toda funcionalidade crítica deve possuir teste;
- decisões arquiteturais relevantes devem gerar ADR;
- **caso o ADR-021 seja aceito, ao menos um módulo de domínio real da distribuição deverá utilizar `execution.mode: process`**;
- distribuições oficiais que incluam módulos em processo não devem exigir Python/Node instalado pelo usuário final (interpretador empacotado ou equivalente).

### Critério de saída

A mesma infraestrutura consegue executar ao menos uma segunda combinação experimental de módulos, mesmo que simples. Isso demonstra que foi criada uma plataforma e não apenas um aplicativo monolítico com pastas separadas. Um usuário não técnico consegue instalar o perfil Essencial, concluir o onboarding e executar backup/restauração básica sem assistência direta.

---

## Etapa 3 — Contrato de módulos e SDK v0

### Objetivo

Extrair um SDK a partir dos problemas reais encontrados na primeira distribuição, incluindo base para composição e contribuição via CLI.

### Entregas

- manifesto de módulo;
- identificação e versão;
- bloco `execution` (`native` | `process`) com `command`/`args` quando aplicável;
- especificação versionada do **OpenCore Module Protocol**;
- dependências obrigatórias e opcionais;
- ciclo de vida;
- contratos de interface;
- eventos publicados e consumidos;
- permissões solicitadas (capacidades do protocolo — não sandbox de OS);
- migrações de banco;
- configuração;
- testes de compatibilidade entre runtime e SDKs;
- política de negociação de versão do protocolo;
- template de módulo;
- ferramenta ou comando para criar um módulo;
- documentação de erros e diagnóstico;
- política de compatibilidade e depreciação;
- **SDK nativo Rust** (API in-process) — artefato distinto;
- **SDK de processo** da primeira linguagem aceita no Spike 10 (preferência: Python);
- suíte de **testes de conformidade** do protocolo;
- módulo de referência headless equivalente;
- ciclo operacional do Module Host (timeouts, heartbeat, reinícios, crash loop, limites de mensagem);
- **CLI inicial** e scaffolds (módulo nativo, em processo, adaptador, distribuição);
- **catálogo de metadados** alinhado ao Builder (capabilities, business_tags, trust_level, etc. — campos conceituais a estabilizar);
- **testes de composição** (dependências, conflitos, perfis, lockfile, remoção, atualização);
- **manifesto e lockfile de distribuição estabilizados experimentalmente**;
- **contrato de adaptadores** (importação/exportação com dry-run e relatório de inconsistências);
- **campos necessários ao Builder** no manifesto de módulo e de distribuição;
- suíte de conformidade ampliada para composição e metadados.

### Limite desta etapa

Não congelar uma ABI de plugins prematuramente. O SDK v0 é a especificação do protocolo mais bindings suficientes para experimentação. Não misturar SDK nativo com SDK de processo. Distribuição dinâmica de plugins in-process, interpretador embutido, assinatura, marketplace e sandbox de SO devem esperar evidências. Um segundo SDK de linguagem externa só deve entrar após o primeiro host estabilizar. `ui_schema` só após Spike 11. IA conversacional permanece fora desta etapa.

### Critério de saída

Um desenvolvedor externo consegue criar um módulo simples (nativo **ou** em processo, conforme trilha), testá-lo contra a suíte de conformidade e adicioná-lo a uma distribuição sem alterar o OpenCore Runtime. A composição da distribuição pode ser descrita por manifesto/lockfile e validada por testes de composição.

---

## Etapa 4 — Piloto fechado de comunidade

### Objetivo

Validar se o projeto realmente funciona como ambiente de aprendizado e colaboração, incluindo experiência de instalação e onboarding para usuários finais de uma organização piloto.

### Formato

Executar uma coorte pequena com participantes independentes, alunos ou membros de comunidades. A coorte deve incluir tarefas de níveis diferentes e expandir o perfil de participantes:

- desenvolvedores;
- documentadores;
- designers;
- usuários finais de uma organização piloto;
- documentação;
- testes;
- correção de bugs;
- interface;
- módulo simples **nativo** (Rust);
- módulo simples **em processo** na primeira linguagem externa aceita (preferência: Python), se ADR-021 estiver aceito;
- observabilidade e CI;
- revisão de código assistida;
- validação de UX (linguagem, triagem preliminar, onboarding, importação, suporte e restauração).

Se o Spike 10 / ADR-021 não tiver sido aceito, a trilha de processo permanece fora da coorte e o piloto documenta o adiamento.

### Estrutura

- onboarding documentado;
- encontros de mentoria;
- responsáveis por revisão;
- prazos e escopo limitados;
- registro público das decisões;
- retrospectiva;
- critérios para promoção a colaborador recorrente ou mantenedor em formação.

### Validações de UX (obrigatórias nesta etapa)

- instalação limpa;
- primeira execução;
- linguagem compreensível para leigos;
- triagem ou questionário mínimo (mesmo que estático);
- onboarding;
- importação;
- suporte (canais e expectativas);
- restauração a partir de backup.

### Métricas

- tempo até executar o projeto localmente;
- tempo até a primeira contribuição;
- percentual de PRs concluídos;
- principais bloqueios de documentação;
- retrabalho por falhas arquiteturais;
- retenção após a primeira contribuição;
- quantidade de tarefas que exigiram intervenção direta do criador;
- tempo até primeiro uso útil para o usuário final piloto;
- taxa de conclusão da instalação e do onboarding.

### Critério de saída

O projeto recebe contribuições úteis sem depender de explicações privadas ou correções manuais constantes. Ao menos uma organização piloto conclui instalação, onboarding e restauração com feedback documentado.

---

## Etapa 5 — Piloto institucional educacional

### Objetivo

Apresentar uma proposta comprovada a uma plataforma de ensino, universidade ou bootcamp.

### Proposta de adoção

O parceiro pode utilizar o OpenCore em três formatos:

1. **Desafio de contribuição:** correções, testes, documentação e pequenos módulos.
2. **Projeto final ou capstone:** construção de um módulo de domínio completo — preferencialmente via SDK de processo quando ADR-021 estiver aceito, para maximizar acessibilidade educacional sem reduzir padrões de teste, revisão e documentação.
3. **Trilha de mantenedores:** revisão, arquitetura, segurança, DevOps e mentoria (incluindo Module Host e conformidade do protocolo).

O pacote institucional deverá incluir explicitamente a trilha da primeira linguagem externa aceita (template, critérios de revisão, ambiente e limites de segurança: isolamento de falhas ≠ sandbox).

### Trilhas adicionais do pacote

- desenvolvimento de módulos;
- documentação;
- **catálogo de capacidades** (contribuição de termos e mapeamentos);
- **testes de instalação** e experiência;
- acessibilidade;
- **criação de perfis de negócio**;
- **adaptadores de migração** (CSV e, quando houver evidência, sistemas externos).

### Pacote institucional

- apresentação do problema;
- arquitetura e limites do projeto;
- catálogo de desafios;
- competências avaliadas;
- regras de contribuição;
- licença e propriedade intelectual;
- política de dados e segurança;
- modelo de mentoria;
- critérios de avaliação;
- evidências públicas para portfólio;
- métricas de impacto;
- regras que protegem a independência do OpenCore.

### Papel das empresas

Empresas podem fornecer problemas reais, financiar desafios ou testar distribuições, mas não devem controlar o roadmap do núcleo.

### Critério de saída

Uma coorte institucional completa um ciclo de contribuição e produz resultados mensuráveis para alunos, projeto e usuários-piloto — incluindo artefatos úteis ao catálogo, perfis ou adaptadores.

---

## Etapa 6 — Alpha público e Builder MVP

### Objetivo

Permitir descoberta, recomendação baseada em regras, instalação guiada e operação inicial sem construir marketplace, compilação arbitrária ou IA obrigatória.

### Portal e Builder MVP

- documentação;
- downloads;
- guia de contribuição;
- **páginas por distribuição** real (linguagem de necessidade, não só de módulos);
- **triagem baseada em regras** (questionário; opção **“não sei”**);
- **recomendação de perfis verificados**;
- **preview estrutural**;
- **personalização limitada** dentro de combinações testadas;
- **manifesto e lockfile** gerados/entregues com a composição;
- **download de pacote pré-construído** (Estágio A de empacotamento);
- catálogo de distribuições oficiais;
- **catálogo de módulos verificados**;
- **ficha da configuração**;
- **instalação guiada**;
- showcase de casos reais;
- changelog;
- roadmap público;
- página de segurança e suporte.

### O que fica fora do escopo desta etapa

- compilação arbitrária de código enviado por usuários;
- inclusão de qualquer módulo comunitário por padrão;
- marketplace comercial;
- IA obrigatória (modo sem IA deve funcionar; camada conversacional fica para depois);
- pagamentos complexos;
- builds ilimitados / geração pública arbitrária;
- infraestrutura de nuvem obrigatória;
- sandbox completa.

O gerador de distribuições no alpha seleciona pacotes pré-construídos. A montagem a partir de artefatos assinados e a geração avançada só avançam com matriz de compatibilidade, custo e segurança compreendidos.

### Critério de saída

Um usuário consegue encontrar uma distribuição adequada, compreender a recomendação, baixar um pacote pré-construído, instalar, concluir onboarding, fazer backup e entender os limites da composição **sem assistência direta** e **sem depender do portal para continuar operando** após o download.

---

## Etapa 7 — Beta, ecossistema e sustentabilidade

### Objetivo

Ampliar módulos, distribuições, mantenedores, prestadores e canais de adoção sem perder governança nem antecipar marketplace inseguro.

### Entregas possíveis

- **montagem por artefatos assinados** (Estágio B — composição sem recompilar código arbitrário);
- **rede de prestadores** (diretório, critérios, não exclusividade);
- **adaptadores para sistemas externos** (importação com dry-run e limitações declaradas);
- **perfis adicionais** de negócio verificados;
- segunda distribuição oficial completa;
- **IA conversacional opcional** (Spike 17; subordinada ao motor determinístico; fallback sem IA);
- catálogo ampliado;
- assinatura e verificação de artefatos;
- **SBOM**;
- processo formal de **certificação** de módulos e prestadores;
- matriz de compatibilidade;
- política de suporte de versões;
- programa de mantenedores;
- conselho ou comitê de governança;
- modelo transparente de patrocínio;
- GitHub Sponsors ou OpenCollective;
- suporte e treinamento pagos sem limitar o acesso ao núcleo.

### Critério de saída

O OpenCore possui mais de uma distribuição real, módulos reutilizados entre produtos, mantenedores além do fundador, caminho de prestadores opcionais e fontes de sustentabilidade que não condicionam o controle do projeto. Marketplace permanece **fora** desta etapa.

---

## Etapa futura condicionada (backlog posterior)

Itens **sem número fixo**, condicionados a evidência, ADR e maturidade da matriz de confiança:

- marketplace;
- geração avançada de builds (Estágio C);
- segundo SDK de linguagem externa;
- sandbox completa de SO;
- personalização livre de combinações;
- sincronização avançada;
- serviços de nuvem opcionais;
- compilação pública a partir de código arbitrário.

Nenhum destes itens é pré-requisito do alpha nem substitui a autoridade do motor de regras, dos lockfiles e dos níveis de confiança.

---

# 4. Ordem prática de execução

A sequência imediata recomendada é:

1. consolidar documentos (Manifesto 1.2, Arquitetura 1.3, Comunidade 1.0, Plano 1.0);
2. criar ADR-022 e a especificação do Builder v0 (e benchmarks v1.0);
3. executar os spikes técnicos existentes (Rust, Slint, SQLite, módulos, empacotamento, Spike 10);
4. validar CLI, lockfile e instalador (Spikes 12–13 e protótipos associados);
5. construir a Portaria (fatia vertical + instalador/onboarding);
6. testar com usuário leigo;
7. extrair o SDK v0;
8. piloto comunitário fechado (incluindo UX e usuários finais);
9. Builder baseado em regras (seleção de pacotes pré-construídos);
10. alpha público;
11. piloto institucional;
12. segunda distribuição;
13. adaptadores para sistemas externos;
14. IA opcional (subordinada a regras);
15. ecossistema comercial somente após confiança (marketplace e geração avançada condicionados).

---

# 5. Mudanças em relação ao roadmap anterior

1. **Comunidade deixa de ser uma fase tardia.** Ela começa junto com o primeiro repositório.
2. **Educação deixa de ser apenas uma parceria futura.** Seus requisitos passam a orientar documentação, tarefas, revisão e métricas desde o início.
3. **O kernel completo deixa de ser construído no escuro.** Primeiro são validados runtime, módulos e uma fatia vertical.
4. **O SDK nasce de contratos reais.** Ele não é definido apenas por previsão arquitetural.
5. **O Portal é reduzido ao essencial.** Marketplace e gerador público de builds ficam para depois da estabilidade.
6. **A Portaria permanece como primeiro produto, não como identidade do projeto.**
7. **A aproximação institucional ocorre em duas etapas.** Descoberta inicial em paralelo e proposta formal somente após evidências técnicas e comunitárias.
8. **Cada etapa possui critério de saída.** O avanço depende de evidência, não apenas de concluir uma lista de tarefas.

### Mudanças específicas da v2.1 (ADR-021 v1.1)

9. **Spike 10 na Etapa 1** com critério de saída explícito (aceito / rejeitado / adiado).
10. **Portaria (Etapa 2)** exige ≥ 1 módulo de domínio real em processo se ADR-021 for aceito.
11. **SDK v0 separa** SDK nativo Rust, protocolo, SDK de processo e testes de conformidade.
12. **Pilotos 4 e 5** incluem trilha da primeira linguagem externa.
13. **Isolamento de processo ≠ sandbox** passa a ser premissa explícita do roadmap.
14. **Interpretador no PATH** não é estratégia de distribuição oficial.

### Mudanças específicas da v2.2 (consolidação)

15. **Numeração corrigida:** módulos nativos/processo passam a ser **ADR-021**; ADR-015 permanece a matriz de classificação.
16. **Alinhamento à Arquitetura v1.2:** recupera obrigações da v1.1 (LGPD, portabilidade, confiança, atualização/sync, CI, time-box) + melhorias multilíngues.
17. **Próxima etapa documental:** Comunidade e Governança v1.0 — não iniciar Spike 10 antes de fechar a Etapa 0.
18. **Classificação de atualização e sincronização alinhada à ADR-015:** módulos-base não listam mais “atualização” nem “sincronização opcional”; sync e feeds/provedores ficam em adaptadores transversais.
19. **Etapa 0 documental essencial implementada**; licença CC BY 4.0 em vigor; RFC-0001 em consulta pública até 2026-08-22; Etapa 1 autorizada para spikes controlados.

### Mudanças específicas da v2.3 (Builder, UX e composição)

20. **Lente transversal** de experiência e adoção do usuário final, sem criar quarta equipe obrigatória.
21. **Etapa 0** incorpora Manifesto 1.2, Arquitetura 1.3, ADR-022, Especificação Builder v0, Benchmarks v1.0 e critério leigo de obtenção de distribuição.
22. **Etapa 1** acrescenta Spikes 12–13, catálogo estático, perfil simples, instalador monoposto, `doctor`, demo data e teste externo — sem IA nem portal completo.
23. **Etapa 2 (Portaria)** exige instalador, onboarding, CSV, backup, ficha, lockfile, perfil Essencial, preview interno e teste leigo; catálogo permanece genérico.
24. **Etapa 3 (SDK)** inclui CLI, scaffolds, metadados, testes de composição, lockfile experimental, adaptadores, campos do Builder e conformidade ampliada.
25. **Etapa 4** expande participantes (designers, documentadores, usuários finais) e validações de UX.
26. **Etapa 5** inclui trilhas de catálogo, perfis e adaptadores no pacote institucional.
27. **Etapa 6** renomeada para **alpha público e Builder MVP**; fora de escopo: marketplace, IA obrigatória, compilação arbitrária e builds ilimitados.
28. **Etapa 7** acrescenta montagem por artefatos, rede de prestadores, adaptadores, IA opcional, SBOM e certificação — marketplace permanece futuro.
29. **Etapa futura condicionada** registra marketplace, geração avançada, sandbox, segundo SDK e afins sem número fixo.
30. **Ordem prática revisada** em 15 itens, priorizando documentos → spikes → Portaria → leigo → SDK → Builder em regras → alpha → IA opcional → ecossistema comercial após confiança.
31. Documentos relacionados passam a citar Manifesto 1.2, Arquitetura 1.3, ADR-015..022, Spec Builder v0 e Benchmarks v1.0.

---

# 6. Próxima decisão de projeto

A consolidação documental da Etapa 0 avança para a linha 1.2/1.3/2.3:

1. Manifesto v1.2 (acessibilidade operacional e descoberta guiada)
2. Arquitetura v1.3 + ADR-015..022
3. Comunidade e Governança v1.0
4. Plano Institucional v1.0
5. Especificação Builder v0 + Benchmarks v1.0
6. Licença documental publicada; RFC-0001 em consulta até 2026-08-22 ([#2](https://github.com/duvallemusic/open-core/issues/2))

A hierarquia institucional permanece:

- **principal:** infraestrutura aberta e utilizável por organizações reais;
- **meio estratégico:** educação e formação de desenvolvedores através de contribuições reais;
- **sustentação:** adoção comercial, patrocínio, suporte e treinamento sem lock-in.

**Próximo passo:** revisar e aprovar os documentos 1.2/1.3/1.0/2.3 e a ADR-022; não iniciar implementação completa do Builder; iniciar a Etapa 1 pelos spikes técnicos controlados (incluindo Spike 10 quando priorizado, e Spikes 12–13 de CLI/lockfile/instalador de forma time-boxed). A consulta da RFC-0001 prossegue em paralelo até 2026-08-22. IA permanece opcional e pós-MVP; marketplace permanece etapa futura condicionada.
