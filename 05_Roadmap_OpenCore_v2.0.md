# OpenCore — Roadmap Revisado v2.0

**Status:** histórico — supersedido pelo Roadmap v2.1  
**Nota:** snapshot preservado para histórico de decisões. Inclui a primeira menção parcial ao ADR-015 v1.

---

## 1. Direção estratégica

O OpenCore é uma plataforma open source para criação de sistemas desktop modulares e multiplataforma. O Sistema de Portaria será a primeira distribuição de referência, mas não o produto central.

A evolução deve ocorrer em três trilhas paralelas:

1. **Produto e arquitetura:** núcleo, módulos, SDK, interface e distribuições.
2. **Comunidade e educação:** documentação, trilhas de contribuição, mentorias e formação de mantenedores.
3. **Institucional e sustentabilidade:** parceiros, governança, métricas, patrocínio e proteção da independência do projeto.

Plataformas de ensino não devem aparecer apenas no final do roadmap. A possibilidade de adoção por DIO, EBAC, universidades e bootcamps precisa influenciar desde o início a documentação, a separação das tarefas, os critérios de revisão e a forma de demonstrar contribuições.

Ao mesmo tempo, o núcleo técnico deve continuar sendo projetado como software profissional. A camada educacional organiza a participação ao redor do produto; ela não deve reduzir os padrões de arquitetura, testes, segurança ou manutenção.

---

## 2. Modelo arquitetural de referência

Antes de implementar o SDK, o projeto deve formalizar quatro níveis:

### 2.1 Runtime do OpenCore

Parte mínima e não removível da plataforma:

- inicialização da aplicação;
- configuração;
- registro e ciclo de vida de módulos;
- banco e migrações;
- barramento de eventos;
- logs e diagnóstico;
- contratos mínimos de segurança e integridade.

### 2.2 Módulos-base

Serviços reutilizáveis que podem ser exigidos por uma distribuição:

- autenticação;
- permissões;
- backup e restauração;
- relatórios;
- internacionalização;
- atualização;
- sincronização opcional.

### 2.3 Módulos de domínio

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

### 2.4 Distribuições

Combinações testadas e empacotadas de módulos. Exemplos:

- OpenCore Portaria;
- OpenCore Academia;
- OpenCore Oficina;
- OpenCore Biblioteca;
- OpenCore Estacionamento.

Esse modelo impede que o Sistema de Portaria contamine o núcleo com regras específicas e permite que instituições de ensino trabalhem em módulos isolados sem precisar compreender toda a plataforma.

---

# 3. Roadmap por etapas

## Etapa 0 — Alinhamento e regras fundamentais

### Objetivo

Transformar os documentos atuais em uma base de decisão suficientemente clara para iniciar o repositório principal.

### Entregas

- manifesto v1;
- definição explícita do objetivo principal e dos objetivos secundários;
- licença do núcleo e política para módulos;
- modelo inicial de governança;
- código de conduta;
- política de contribuição;
- política de segurança;
- processo de RFC e ADR;
- definição das quatro camadas arquiteturais;
- critérios iniciais para aceitar módulos e distribuições;
- princípios de independência em relação a patrocinadores e parceiros educacionais.

### Decisões obrigatórias

- licença do núcleo;
- possibilidade ou não de módulos proprietários;
- quem pode aprovar mudanças no núcleo;
- como novos mantenedores serão formados;
- qual é a relação entre impacto social, educação e adoção comercial;
- quais decisões exigem RFC pública.

### Critério de saída

Uma pessoa externa consegue ler os documentos e explicar o que é o OpenCore, o que ele não é, quem decide e como contribuir.

---

## Etapa 1 — Spike técnico e estrutura de contribuição

### Objetivo

Validar as decisões técnicas de maior risco antes de construir o produto.

### Produto e arquitetura

Criar pequenos protótipos descartáveis ou isolados para comprovar:

- aplicação Rust com interface Slint em Windows, Linux e macOS;
- persistência SQLite e migrações;
- carregamento e registro de dois módulos simples;
- comunicação por eventos;
- empacotamento e atualização de esquema;
- exportação de dados em formato aberto;
- testes automatizados do runtime e dos módulos;
- (ADR-015) um módulo externo em processo isolado, em Python **ou** TypeScript, via protocolo local versionado — sem exigir segundo SDK completo nesta etapa.

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
- primeiro módulo de exemplo;
- trilha “primeira contribuição”.

### Institucional

Iniciar conversas exploratórias com pessoas ligadas a plataformas de ensino e comunidades, sem apresentar ainda uma proposta de adoção formal. O objetivo é entender requisitos de integração, formato de desafio, duração de coortes, mentoria e avaliação.

### Critério de saída

A stack funciona nos três sistemas operacionais, dois módulos conseguem operar sem acoplamento indevido e uma pessoa externa consegue executar o projeto seguindo apenas a documentação.

---

## Etapa 2 — Primeira fatia vertical: OpenCore Portaria

### Objetivo

Validar o OpenCore como plataforma através de uma distribuição real, e não construir um sistema de portaria isolado.

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
- instaladores de teste para os três sistemas operacionais.

### Regras

- nenhuma regra de portaria deve entrar no runtime;
- cada módulo deve declarar dependências;
- o banco deve permitir migrações por módulo;
- desativar um módulo não pode corromper dados de outro;
- a distribuição precisa funcionar offline;
- toda funcionalidade crítica deve possuir teste;
- decisões arquiteturais relevantes devem gerar ADR.

### Critério de saída

A mesma infraestrutura consegue executar ao menos uma segunda combinação experimental de módulos, mesmo que simples. Isso demonstra que foi criada uma plataforma e não apenas um aplicativo monolítico com pastas separadas.

---

## Etapa 3 — Contrato de módulos e SDK v0

### Objetivo

Extrair um SDK a partir dos problemas reais encontrados na primeira distribuição.

### Entregas

- manifesto de módulo;
- identificação e versão;
- modo de execução (`native` | `process`) e protocolo neutro de linguagem;
- dependências obrigatórias e opcionais;
- ciclo de vida;
- contratos de interface;
- eventos publicados e consumidos;
- permissões solicitadas;
- migrações de banco;
- configuração;
- testes de compatibilidade;
- template de módulo;
- ferramenta ou comando para criar um módulo;
- documentação de erros e diagnóstico;
- política de compatibilidade e depreciação;
- bindings oficiais do protocolo (Rust + a primeira linguagem externa aceita no Spike 10).

### Limite desta etapa

Não congelar uma ABI de plugins prematuramente. O SDK v0 é a especificação do protocolo mais bindings suficientes para experimentação. Distribuição dinâmica de plugins in-process, interpretador embutido, assinatura e marketplace devem esperar evidências de necessidade e maturidade. Um segundo SDK de linguagem externa só deve entrar após o primeiro host estabilizar.

### Critério de saída

Um desenvolvedor externo consegue criar um módulo simples, testá-lo e adicioná-lo a uma distribuição sem alterar o runtime.

---

## Etapa 4 — Piloto fechado de comunidade

### Objetivo

Validar se o projeto realmente funciona como ambiente de aprendizado e colaboração.

### Formato

Executar uma coorte pequena com participantes independentes, alunos ou membros de comunidades. A coorte deve incluir tarefas de níveis diferentes:

- documentação;
- testes;
- correção de bugs;
- interface;
- módulo simples;
- observabilidade e CI;
- revisão de código assistida.

### Estrutura

- onboarding documentado;
- encontros de mentoria;
- responsáveis por revisão;
- prazos e escopo limitados;
- registro público das decisões;
- retrospectiva;
- critérios para promoção a colaborador recorrente ou mantenedor em formação.

### Métricas

- tempo até executar o projeto localmente;
- tempo até a primeira contribuição;
- percentual de PRs concluídos;
- principais bloqueios de documentação;
- retrabalho por falhas arquiteturais;
- retenção após a primeira contribuição;
- quantidade de tarefas que exigiram intervenção direta do criador.

### Critério de saída

O projeto recebe contribuições úteis sem depender de explicações privadas ou correções manuais constantes.

---

## Etapa 5 — Piloto institucional educacional

### Objetivo

Apresentar uma proposta comprovada a uma plataforma de ensino, universidade ou bootcamp.

### Proposta de adoção

O parceiro pode utilizar o OpenCore em três formatos:

1. **Desafio de contribuição:** correções, testes, documentação e pequenos módulos.
2. **Projeto final ou capstone:** construção de um módulo de domínio completo.
3. **Trilha de mantenedores:** revisão, arquitetura, segurança, DevOps e mentoria.

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

Uma coorte institucional completa um ciclo de contribuição e produz resultados mensuráveis para alunos, projeto e usuários-piloto.

---

## Etapa 6 — Alpha público e portal mínimo

### Objetivo

Permitir instalação, documentação e descoberta pública sem construir um marketplace prematuro.

### Portal inicial

- documentação;
- downloads;
- guia de contribuição;
- catálogo de distribuições oficiais;
- catálogo de módulos verificados;
- showcase de casos reais;
- changelog;
- roadmap público;
- página de segurança e suporte.

### O que fica fora inicialmente

- marketplace comercial;
- geração arbitrária de builds no servidor;
- instalação de plugins não verificados;
- sistema complexo de pagamentos;
- infraestrutura de nuvem obrigatória.

O gerador de distribuições pode começar como ferramenta local ou pipeline controlado. A automação pública de builds deve surgir somente quando compatibilidade, segurança e custo operacional estiverem compreendidos.

### Critério de saída

Um usuário consegue encontrar, instalar, atualizar, fazer backup e entender os limites de uma distribuição sem assistência direta.

---

## Etapa 7 — Beta, ecossistema e sustentabilidade

### Objetivo

Ampliar módulos, distribuições, mantenedores e parceiros sem perder governança.

### Entregas possíveis

- processo formal de certificação de módulos;
- assinatura e verificação de artefatos;
- matriz de compatibilidade;
- política de suporte de versões;
- segunda distribuição oficial completa;
- programa de mantenedores;
- conselho ou comitê de governança;
- modelo transparente de patrocínio;
- GitHub Sponsors ou OpenCollective;
- suporte e treinamento pagos sem limitar o acesso ao núcleo;
- marketplace somente depois da política de confiança e compatibilidade.

### Critério de saída

O OpenCore possui mais de uma distribuição real, módulos reutilizados entre produtos, mantenedores além do fundador e fontes de sustentabilidade que não condicionam o controle do projeto.

---

# 4. Ordem prática de execução

A sequência imediata recomendada é:

1. consolidar manifesto, arquitetura, comunidade e plano institucional em versões v1;
2. registrar as decisões pendentes em RFCs ou ADRs;
3. criar a estrutura oficial do repositório e da documentação;
4. executar os spikes Rust, Slint, SQLite, módulos e empacotamento;
5. definir o runtime mínimo com base nos resultados;
6. implementar a fatia vertical do OpenCore Portaria;
7. extrair o SDK v0 da experiência real;
8. executar um piloto fechado de contribuição;
9. corrigir documentação, arquitetura e processo com base nas métricas;
10. preparar a proposta institucional para DIO, EBAC, universidade ou bootcamp;
11. executar uma coorte educacional piloto;
12. publicar o alpha e o portal mínimo;
13. expandir para a segunda distribuição e para o modelo de sustentabilidade.

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

---

# 6. Próxima decisão de projeto

Antes de iniciar código novo, o OpenCore precisa fechar a Etapa 0. O próximo trabalho documental deve revisar, nesta ordem:

1. Manifesto;
2. Arquitetura;
3. Comunidade e governança;
4. Plano institucional;
5. Roadmap v2 consolidado.

A decisão mais importante é definir a prioridade institucional do OpenCore:

- **principal:** infraestrutura aberta e utilizável por organizações reais;
- **meio estratégico:** educação e formação de desenvolvedores através de contribuições reais;
- **sustentação:** adoção comercial, patrocínio, suporte e treinamento sem lock-in.

Essa hierarquia mantém o produto útil fora do contexto educacional e, ao mesmo tempo, torna plataformas de ensino uma alavanca estrutural de crescimento.
