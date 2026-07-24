# Arquitetura OpenCore — Versão 1.3

**Status:** proposta consolidada para aprovação  
**Base estratégica:** Manifesto OpenCore v1.2  
**Roadmap relacionado:** OpenCore — Roadmap Revisado v2.3  
**ADR relacionada:** ADR-022 (OpenCore Builder — proposto, condicionado aos Spikes 14–18)  
**Finalidade:** definir as fronteiras arquiteturais iniciais do OpenCore, a experiência de composição e instalação para usuários não técnicos, e preparar a validação técnica da Etapa 1  
**Observação:** decisões tecnológicas marcadas como provisórias somente serão confirmadas após os spikes técnicos. Esta versão incorpora o OpenCore Builder como componente externo, experiência do desenvolvedor, manifesto/lockfile de distribuição, catálogo de capacidades, composição guiada, instalação e onboarding, interoperabilidade por adaptadores, e testes/métricas/riscos adicionais. Mantém ADR-021 condicionado ao Spike 10.  
**Supersede:** Arquitetura v1.2.

---

## 0. Glossário (ADR-021)

| Termo | Significado |
|---|---|
| **OpenCore Runtime** | Processo principal em Rust |
| **Module Host** | Componente do runtime que supervisiona módulos em processo |
| **Módulo nativo** | `execution.mode: native` — mesmo processo do runtime |
| **Módulo em processo** | `execution.mode: process` — processo filho via protocolo |
| **Interpretador / runtime da linguagem** | Python, Node etc. — distinto do OpenCore Runtime |
| **OpenCore Module Protocol** | Contrato IPC público versionado |
| **SDK nativo Rust** | API in-process para módulos nativos |
| **SDK de processo** | Binding do protocolo para linguagem externa |
| **Distribuição** | Composição testada de runtime, módulos e identidade de produto |
| **Perfil** | Variante verificada de uma distribuição |
| **Composição** | Conjunto exato de componentes escolhidos ou instalados |
| **Manifesto** | Declaração de intenção e faixas aceitas |
| **Lockfile** | Registro de versões, hashes e origem exatas |
| **OpenCore Builder** | Ferramenta/serviço externo de triagem, composição e empacotamento (não integra o runtime mínimo) |
| **Catálogo de capacidades** | Tradução de necessidades de negócio em capacidades e módulos candidatos |

---

## 1. Objetivo deste documento

Este documento traduz os compromissos do Manifesto OpenCore em uma arquitetura inicial verificável.

Seu objetivo não é definir antecipadamente todos os detalhes da plataforma, congelar um SDK público ou projetar infraestrutura para necessidades ainda não comprovadas. Seu objetivo é estabelecer:

- os requisitos arquiteturais obrigatórios;
- as fronteiras entre runtime, módulos e distribuições;
- as dependências permitidas e proibidas;
- o contrato interno inicial de módulos;
- os princípios de persistência, migração, backup e exportação;
- os mecanismos iniciais de comunicação entre componentes;
- as fronteiras de segurança e licenciamento;
- as hipóteses que deverão ser validadas antes da implementação definitiva.

A arquitetura deverá permanecer simples o suficiente para ser compreendida, testada e mantida por uma comunidade em formação, sem reduzir os padrões profissionais de segurança, documentação, qualidade e continuidade.

---

## 2. Escopo

Esta versão cobre a arquitetura inicial necessária para:

1. construir e validar o runtime mínimo do OpenCore;
2. registrar e executar módulos com fronteiras claras;
3. criar a primeira distribuição de referência, o OpenCore Portaria;
4. permitir uma segunda combinação experimental de módulos;
5. extrair futuramente um SDK v0 a partir de contratos reais;
6. preparar o projeto para contribuições externas e adoção educacional;
7. descrever a experiência de composição de distribuições para usuários não técnicos;
8. definir manifesto e lockfile de distribuições, distintos entre si;
9. estabelecer um catálogo de capacidades que traduza necessidades de negócio em módulos;
10. validar de forma determinística combinações, perfis e composições;
11. orientar instalação e onboarding guiados, incluindo diagnóstico da instalação;
12. especificar ferramentas de desenvolvimento, scaffolding e diagnóstico;
13. prever interoperabilidade por adaptadores, sem incorporar concorrentes ao runtime.

Esta versão não define de forma definitiva, e permanece fora do escopo inicial:

- ABI pública para plugins binários;
- carregamento arbitrário de bibliotecas dinâmicas;
- marketplace de módulos ou marketplace comercial;
- execução de código não confiável;
- sandbox completa de plugins;
- microserviços;
- infraestrutura obrigatória de nuvem;
- compilação arbitrária de código enviado por usuários;
- geração pública, ilimitada ou arbitrária de builds e combinações não testadas;
- IA tomando decisões de segurança ou compatibilidade;
- construtor low-code genérico;
- emissão fiscal ou regras regulatórias universais no runtime;
- sincronização distribuída entre múltiplas unidades;
- política final de certificação de módulos;
- suporte definitivo a múltiplos bancos de dados.

Esses recursos somente deverão ser projetados quando houver evidência técnica, operacional ou institucional de necessidade, e mediante ADR quando estruturais.

---

## 3. Princípios arquiteturais

### 3.1 Soberania do usuário

O funcionamento essencial da plataforma não poderá depender de assinatura, ativação recorrente, serviço central ou autorização remota da entidade OpenCore.

O usuário ou a organização responsável pela instalação deverá manter controle sobre:

- execução local;
- dados operacionais;
- backups;
- exportações;
- logs locais;
- configurações;
- atualizações;
- integrações externas habilitadas.

### 3.2 Offline-first

O runtime, as distribuições e os módulos instalados deverão executar localmente todas as funções que não dependam, por sua própria natureza, de um serviço externo.

A conexão poderá ser necessária para:

- baixar atualizações;
- instalar novos componentes;
- sincronizar dispositivos ou unidades;
- acessar integrações remotas escolhidas pelo usuário;
- utilizar serviços externos explicitamente contratados.

A ausência de conexão não poderá bloquear artificialmente funções locais já instaladas nem impedir o acesso aos dados armazenados localmente.

### 3.3 Modularidade com fronteiras claras

Uma capacidade somente deverá pertencer ao runtime quando for necessária à inicialização, integridade, segurança ou coordenação fundamental da plataforma.

Regras específicas de negócio deverão permanecer fora do runtime.

Cada componente deverá possuir:

- responsabilidade identificável;
- dependências declaradas;
- interfaces documentadas;
- testes proporcionais ao risco;
- política de evolução;
- licença explicitamente identificada.

### 3.4 Simplicidade proporcional

O OpenCore deverá preferir a solução mais simples que preserve corretamente:

- integridade de dados;
- segurança;
- extensibilidade necessária;
- portabilidade;
- testabilidade;
- manutenção.

Abstrações, serviços, filas, bancos adicionais e infraestrutura distribuída não deverão ser adicionados apenas por expectativa de crescimento futuro.

### 3.5 Robustez sem excesso

A arquitetura deverá permitir evolução para cenários maiores sem exigir que a primeira versão opere como um sistema distribuído complexo.

Escalabilidade deverá ser tratada como uma propriedade progressiva, guiada por medições e necessidades reais.

### 3.6 Contratos e formatos abertos

Os contratos públicos, os dados essenciais, os processos de migração e os formatos de exportação deverão ser documentados.

Nenhuma distribuição oficial poderá depender de conhecimento privado da entidade central para:

- instalar;
- operar;
- recuperar;
- exportar;
- migrar;
- reconstruir;
- auditar os componentes abertos.

### 3.7 Documentação como parte do produto

Uma funcionalidade não será considerada concluída sem documentação proporcional ao seu impacto.

Mudanças arquiteturais relevantes deverão ser registradas por ADR ou RFC, conforme a política de governança.

### 3.8 Segurança por padrão

Componentes deverão receber apenas o acesso necessário para sua função.

Operações sensíveis deverão possuir controles de autorização, rastreabilidade e tratamento explícito de falhas.

A arquitetura não deverá presumir que todo módulo é confiável apenas por estar instalado.

### 3.9 Complexidade invisível, não omitida

A complexidade técnica poderá ser encapsulada para o usuário final, mas não poderá ser omitida da auditoria, do diagnóstico ou da documentação.

Consequências:

- o instalador deverá carregar as dependências necessárias à operação da distribuição escolhida;
- o manifesto e o lockfile deverão registrar a composição real instalada;
- o usuário em modo simples receberá explicações em linguagem comum sobre módulos, permissões, rede e dados;
- o administrador ou técnico deverá conseguir inspecionar versões, hashes, origem, permissões, transmissões externas e estado dos módulos;
- simplificar a experiência não poderá esconder riscos, componentes proprietários, dependências ou limitações da configuração escolhida.

---

## 4. Requisitos arquiteturais obrigatórios

### 4.1 Portabilidade

A plataforma deverá ser validada em:

- Windows;
- Linux;
- macOS.

Os builds deverão buscar:

- comportamento consistente;
- instaladores reproduzíveis;
- baixa dependência de componentes exclusivos de um sistema operacional;
- diagnóstico claro quando uma funcionalidade não for suportada;
- funcionamento adequado em hardware modesto quando o domínio permitir.

### 4.2 Persistência local

SQLite será a hipótese inicial de banco local padrão.

A persistência deverá permitir:

- transações;
- migrações versionadas;
- backup consistente;
- restauração validada;
- identificação do módulo responsável por cada estrutura;
- exportação de dados em formato portátil;
- evolução futura para adaptadores alternativos, sem exigir essa abstração prematuramente.

### 4.3 Integridade e recuperação

Falhas em módulos, migrações ou atualizações não poderão corromper silenciosamente o estado da distribuição.

O sistema deverá registrar:

- etapa em execução;
- versão anterior;
- versão pretendida;
- módulo afetado;
- resultado da operação;
- erro recuperável ou não recuperável;
- instruções de diagnóstico quando aplicável.

### 4.4 Privacidade e telemetria

Logs técnicos locais poderão existir para diagnóstico, segurança e auditoria.

A transmissão de telemetria para servidores externos deverá permanecer desativada por padrão.

Quando uma integração externa exigir transmissão de dados, ela deverá possuir:

- finalidade documentada;
- dados transmitidos identificados;
- base de configuração explícita;
- possibilidade de revogação quando tecnicamente aplicável;
- tratamento proporcional de segurança e privacidade.

### 4.5 Portabilidade dos dados

A arquitetura deverá diferenciar:

1. **backup técnico**, destinado a restaurar integralmente uma instalação compatível;
2. **exportação portátil**, destinada a permitir leitura, migração e reutilização dos dados essenciais fora da instalação original.

Um backup opaco não substitui uma exportação portátil.

### 4.6 Privacidade, LGPD e responsabilidade da implantação

A arquitetura deverá oferecer mecanismos que permitam às distribuições e às organizações operadoras cumprir a LGPD e outras normas aplicáveis, sem presumir que o software, isoladamente, determine a base legal ou todas as obrigações de cada implantação.

Cada módulo que trate dados pessoais deverá declarar:

- categorias de dados tratados;
- finalidade funcional do tratamento;
- dados obrigatórios e opcionais;
- relações e dependências relevantes;
- regras configuráveis de retenção;
- mecanismos de consulta, correção, exportação e exclusão;
- registros de auditoria que possam limitar a exclusão imediata;
- integrações e transferências externas possíveis;
- dados sensíveis que não poderão aparecer em logs ou telemetria.

As distribuições oficiais deverão documentar quais responsabilidades pertencem ao software, ao operador da instalação e a eventuais serviços externos.

### 4.7 Contrato verificável por módulo

Os compromissos de soberania de dados não serão considerados atendidos apenas por documentação geral. Cada módulo persistente deverá fornecer um contrato verificável de portabilidade e exclusão, integrado ao manifesto, aos testes e à exportação da distribuição.

---

## 5. Modelo arquitetural de referência

O OpenCore será organizado inicialmente como um **monólito modular**, com quatro níveis principais:

1. runtime;
2. módulos-base;
3. módulos de domínio;
4. distribuições.

**Módulos de integração ou adaptadores** formam uma categoria transversal de módulo: conectam módulos-base ou de domínio a protocolos e serviços externos, mas não constituem uma nova camada obrigatória. SDK, templates e ferramentas apoiam esses níveis sem integrar o runtime mínimo.

O **OpenCore Builder** é componente externo auxiliar (proposta Apache 2.0): apoia triagem, recomendação, validação e empacotamento de distribuições. **Não faz parte do runtime mínimo** nem é requisito para operar, restaurar ou exportar uma instalação já baixada. Detalhes na seção dedicada e no ADR-022.

Esse modelo busca preservar simplicidade operacional, desempenho local e facilidade de empacotamento, mantendo fronteiras suficientes para evolução futura.

---

## 6. Runtime OpenCore

### 6.1 Definição

O runtime é a parte mínima, estrutural e não removível da plataforma.

Ele coordena a execução, mas não conhece regras específicas de portaria, academia, oficina, biblioteca ou qualquer outro domínio.

### 6.2 Responsabilidades

O runtime deverá ser responsável por:

- inicialização e encerramento da aplicação;
- carregamento e validação da configuração;
- descoberta ou registro dos módulos disponíveis;
- validação dos manifestos;
- resolução de dependências;
- ordenação do ciclo de vida;
- coordenação das migrações;
- acesso controlado à persistência;
- barramento local de eventos;
- logs e diagnóstico;
- contratos mínimos de integridade;
- infraestrutura comum de tratamento de erros;
- contratos estruturais de atualização, incluindo validação de versão, integridade, compatibilidade, migração e recuperação;
- disponibilização de contexto e serviços autorizados aos módulos.

### 6.3 O que não pertence ao runtime

O runtime não deverá conter:

- regras de visitantes;
- regras de moradores ou unidades;
- regras de entregas;
- regras de estoque;
- agendamentos;
- caixa e faturamento;
- relatórios específicos de um segmento;
- telas específicas de uma distribuição;
- integrações específicas de um cliente;
- fluxos educacionais;
- dependência obrigatória de serviços da entidade OpenCore;
- OpenCore Builder, portal de triagem ou pipeline de empacotamento.

### 6.4 Critério para entrada no runtime

Uma capacidade somente poderá entrar no runtime quando atender a pelo menos uma das seguintes condições:

- ser necessária para inicialização, integridade ou segurança da plataforma;
- precisar de comportamento uniforme entre distribuições;
- ser independente de um domínio de negócio;
- não poder ser implementada de forma segura e sustentável como módulo;
- demonstrar utilidade recorrente e estrutural para a maioria das distribuições.

A entrada de uma nova capacidade no runtime deverá exigir justificativa arquitetural registrada.

### 6.5 Matriz de classificação: runtime, módulo-base e adaptador

| Capacidade | Runtime estrutural | Módulo-base oficial | Integração ou adaptador | Justificativa inicial |
|---|---:|---:|---:|---|
| Inicialização e encerramento | Sim | Não | Não | Necessário para qualquer distribuição |
| Registro e ciclo de vida de módulos | Sim | Não | Não | Coordenação estrutural uniforme |
| Validação de compatibilidade | Sim | Não | Não | Protege integridade da distribuição |
| Configuração estrutural | Sim | Não | Não | Necessária antes da ativação dos módulos |
| Barramento local de eventos | Sim | Não | Não | Contrato comum entre módulos |
| Coordenação de migrações | Sim | Não | Não | Ordem e integridade globais |
| Persistência de domínio | Não | Sim ou domínio | Não | Dados pertencem ao módulo responsável |
| Autenticação | Não | Sim | Não | Reutilizável, mas removível em distribuições específicas |
| Permissões de negócio | Não | Sim | Não | Serviço comum consumido por módulos |
| Backup e restauração | Contratos e orquestração | Sim | Adaptadores de destino | Runtime preserva integridade; módulo implementa operação e provedores |
| Exportação portátil | Contrato e descoberta | Sim ou domínio | Adaptadores de formato quando necessário | Cada módulo exporta os próprios dados |
| Atualização | Validação, compatibilidade, migração e recuperação | Interface administrativa opcional | Feed, download e provedor de artefatos | Separa segurança estrutural de canais externos |
| Sincronização | Não | Não | Sim | Depende de protocolo, provedor e política externa |
| Telemetria | Consentimento e bloqueio por padrão | Não | Sim, sempre opcional | Transmissão externa nunca é capacidade estrutural obrigatória |
| Relatórios genéricos | Não | Sim | Exportadores externos opcionais | Capacidade reutilizável, não necessária ao boot |
| Integrações de terceiros | Não | Não | Sim | Dependem de serviços e credenciais externos |
| Interface específica | Não | Não | Não | Pertence à distribuição ou ao módulo correspondente |

A presença de contratos no runtime não transforma a implementação completa da capacidade em parte do runtime. O runtime poderá coordenar uma operação sem incorporar seus provedores, telas ou regras específicas.

### 6.6 Limiar para promoção de capacidades ao runtime

Uma capacidade modular somente poderá ser promovida ao runtime por uma RFC acompanhada de ADR e deverá atender a um dos caminhos abaixo:

1. **necessidade estrutural intrínseca:** é indispensável para inicialização, integridade, segurança ou coordenação uniforme; ou
2. **evidência de uso transversal:** foi utilizada por pelo menos duas distribuições de domínios distintos e é necessária em pelo menos 60% das distribuições oficiais mantidas, sem alternativa modular segura e sustentável.

Além disso, a proposta deverá demonstrar:

- redução mensurável de duplicação ou risco;
- ausência de regra de domínio;
- contrato estável e testado;
- impacto aceitável sobre tamanho, inicialização e superfície de segurança;
- plano de migração e compatibilidade;
- impossibilidade de resolver o problema apenas com contrato, serviço-base ou adaptador.

Antes de existirem duas distribuições distintas, nenhuma capacidade deverá ser promovida ao runtime apenas por conveniência, exceto quando se enquadrar claramente no caminho de necessidade estrutural intrínseca.

---

## 7. Módulos-base

### 7.1 Definição

Módulos-base oferecem capacidades reutilizáveis por diferentes distribuições, mas não são necessariamente obrigatórios em todas elas.

### 7.2 Exemplos iniciais

- autenticação;
- permissões;
- backup e restauração;
- relatórios genéricos;
- auditoria;
- internacionalização;
- notificações locais;
- gerenciamento de usuários.

### 7.3 Regras

Módulos-base:

- poderão depender de serviços do runtime;
- poderão depender de outros módulos-base quando a dependência for explícita e justificada;
- não poderão conhecer regras internas de módulos de domínio;
- deverão expor contratos estáveis o suficiente para uso pelas distribuições;
- deverão informar as permissões e recursos solicitados;
- deverão possuir migrações próprias quando armazenarem dados.

---

## 8. Módulos de domínio

### 8.1 Definição

Módulos de domínio implementam capacidades específicas de um tipo de operação ou negócio.

### 8.2 Exemplos

Para o OpenCore Portaria:

- unidades;
- moradores;
- visitantes;
- entregas;
- prestadores;
- ocorrências.

Para distribuições futuras:

- estoque;
- clientes;
- caixa;
- agenda;
- biblioteca;
- estacionamento;
- matrículas;
- ordens de serviço.

### 8.3 Regras

Módulos de domínio:

- poderão depender do runtime por meio de contratos autorizados;
- poderão depender de módulos-base;
- poderão declarar dependências de outros módulos de domínio quando necessário;
- não poderão alterar diretamente dados privados de outro módulo;
- não poderão exigir mudanças no runtime apenas para acomodar uma regra específica;
- deverão publicar ou consumir eventos quando a comunicação indireta reduzir acoplamento;
- deverão manter sua lógica de negócio testável sem depender da interface gráfica.

### 8.4 Módulos de integração e adaptadores

Módulos de integração conectam o OpenCore a protocolos, provedores ou serviços externos. Exemplos:

- sincronização entre unidades;
- armazenamento remoto opcional de backups;
- provedores de e-mail ou mensagens;
- APIs governamentais ou empresariais;
- importadores e exportadores específicos;
- feeds e canais de atualização;
- conectores para serviços de identidade externos.

#### Interoperabilidade por adaptadores

Concorrentes e sistemas externos **não** deverão ser incorporados como base do OpenCore Runtime. A interoperabilidade deverá ocorrer por adaptadores, como hipótese de produto condicionada a validação:

- importação de Odoo, ERPNext, Dolibarr, Tryton, OpenConcerto e formatos similares, quando houver demanda comprovada;
- CSV, JSON e SQLite documentado;
- integrações futuras substituíveis.

Os adaptadores deverão:

- preservar identificadores externos;
- mapear campos de forma documentada;
- gerar relatório de inconsistências;
- permitir dry-run antes de gravar;
- não alterar dados de origem sem pedido explícito;
- oferecer exportação reversa quando tecnicamente possível;
- declarar limitações e cobertura;
- manter a operação local independente da integração.

Não copiar código, estruturas protegidas ou documentação proprietária de terceiros. Cada adaptador exigirá análise de licença e fronteira antes de incorporação.

Esses módulos:

- não pertencem ao runtime mínimo;
- deverão declarar acesso à rede, destinos, credenciais e dados transmitidos;
- deverão preservar alternativa local ou exportação quando a função principal puder operar offline;
- não poderão transformar um provedor controlado pela OpenCore no único meio de acessar dados essenciais;
- deverão ser substituíveis por outros adaptadores compatíveis quando houver contrato público aplicável.

---

## 9. Distribuições

### 9.1 Definição

Uma distribuição é uma combinação testada, documentada e empacotada de runtime, módulos, configurações e identidade de produto.

A distribuição **não** é uma combinação arbitrária de módulos. Ela representa uma composição cuja compatibilidade foi verificada. A Portaria é a primeira distribuição de referência, não a identidade da plataforma.

### 9.2 Conteúdo mínimo de uma distribuição

Cada distribuição deverá declarar, no mínimo:

- identificador;
- nome;
- versão;
- público e perfil de negócio;
- modos operacionais (monoposto, rede local, sincronizado ou dependente de integração externa);
- runtime compatível;
- módulos obrigatórios e opcionais;
- capacidades fornecidas;
- dependências obrigatórias;
- configurações iniciais;
- identidade visual;
- sistemas operacionais suportados e requisitos mínimos de hardware;
- política de atualização;
- matriz de testes;
- documentação de instalação e operação;
- componentes e respectivas licenças;
- formatos de backup e exportação suportados.

### 9.3 Manifesto da distribuição

Cada distribuição deverá possuir um **manifesto** distinto do lockfile. O manifesto declara intenção e faixas aceitas:

- identificador, nome e versão;
- público e perfil de negócio;
- modos operacionais;
- faixa de OpenCore Runtime compatível;
- módulos obrigatórios e opcionais (faixas de versão);
- capacidades;
- sistemas suportados e requisitos mínimos;
- política de atualização;
- formatos de backup e exportação;
- documentação;
- licença e identidade.

O formato concreto (YAML, TOML ou outro) é hipótese; não será congelado antes dos spikes de manifesto/lockfile.

### 9.4 Lockfile da distribuição

O **lockfile** registra a composição exata instalada ou publicada:

- versão exata do OpenCore Runtime;
- módulos e versões exatas;
- hashes dos artefatos;
- origem dos artefatos;
- runtime da linguagem empacotado, quando houver módulos em processo;
- configurações estruturais resolvidas;
- adaptadores incluídos;
- canal de atualização;
- data de geração;
- identificador da composição;
- assinatura futura, quando disponível.

O lockfile deverá permitir reprodução, auditoria, diagnóstico e rollback. Manifesto e lockfile não se substituem.

### 9.5 Perfis e variantes verificadas

**Perfis** são variações testadas de uma distribuição, não combinações livres. Exemplos conceituais (não prescritivos de produto):

- Essencial;
- Completo;
- Multiestação.

A personalização inicial pelo usuário ou pelo OpenCore Builder deverá ocorrer **dentro de limites verificados**. Combinações livres somente poderão crescer com evidência, matriz de testes e governança de compatibilidade.

### 9.6 Distribuição de referência

O **OpenCore Portaria** será a primeira distribuição de referência.

Seu objetivo é validar o OpenCore como plataforma, e não transformar regras de portaria em capacidades do runtime. Termos do catálogo de capacidades deverão permanecer genéricos, evitando contaminação do núcleo por regras de um único segmento.

### 9.7 Segunda composição experimental

Antes de considerar a plataforma validada, a mesma infraestrutura deverá executar uma segunda composição experimental de módulos.

Essa composição poderá ser pequena, mas deverá provar que:

- o runtime não depende do domínio de portaria;
- módulos podem ser combinados de forma diferente;
- as configurações da distribuição não exigem alterações estruturais no núcleo;
- os contratos possuem reutilização real;
- manifesto e lockfile descrevem a composição de forma reproduzível.

---

## 10. Regras de dependência

### 10.1 Dependências permitidas

```text
Distribuição → Runtime
Distribuição → Módulos-base
Distribuição → Módulos de domínio
Módulo de domínio → Runtime, por contratos públicos
Módulo de domínio → Módulos-base, por contratos públicos
Módulo de domínio → Outro módulo de domínio, quando declarado
Módulo-base → Runtime, por contratos públicos
Módulo-base → Outro módulo-base, quando declarado
Adaptador → Runtime e módulos, somente por contratos públicos declarados
Distribuição → Adaptadores selecionados explicitamente
Runtime → bibliotecas internas estruturais
```

### 10.2 Dependências proibidas

```text
Runtime → regras ou tipos internos de um domínio específico
Runtime → uma distribuição específica
Módulo-base → implementação interna de módulo de domínio
Módulo A → tabelas privadas ou estado interno do Módulo B
Módulo → serviço externo não declarado
Runtime → provedor externo específico de sincronização, atualização ou telemetria
Módulo → acesso irrestrito ao sistema sem permissão explícita
Distribuição oficial → componente proprietário essencial
```

### 10.3 Dependências circulares

Dependências circulares entre módulos deverão ser rejeitadas durante a validação da distribuição.

Quando dois módulos parecerem depender mutuamente, a solução deverá considerar:

- extração de um contrato compartilhado;
- publicação de eventos;
- criação de um módulo-base apropriado;
- revisão da divisão de responsabilidades.

---

## 11. Contrato interno de módulos v0

### 11.1 Objetivo

O contrato interno v0 permitirá validar o modelo modular antes de publicar um SDK estável.

Ele poderá evoluir durante as primeiras distribuições.

### 11.2 Manifesto mínimo

Cada módulo deverá declarar, no mínimo:

```yaml
id: org.opencore.exemplo
name: Módulo de Exemplo
version: 0.1.0
license: MPL-2.0
type: base | domain | integration | tool
trust_level: T0 | T1 | T2 | T3
# T0 Experimental | T1 Comunitário | T2 Verificado | T3 Oficial
execution:
  mode: native            # native | process
  # apenas quando mode: process
  # command: python
  # args: ["main.py"]
runtime_requirement: ">=0.1.0 <0.2.0"
protocol: null | opencore-module-v1
dependencies:
  required: []
  optional: []
permissions: []
network:
  required: false
  destinations: []
events:
  publishes: []
  consumes: []
migrations: []
data_contract:
  stores_personal_data: false
  categories: []
  export_formats: []
  deletion_supported: true
  retention_policy: null
  external_transfers: []
configuration_schema: null
ui_schema: null            # opcional; fora do Spike 10
```

Exemplos ilustrativos:

```yaml
# Módulo nativo (Rust, in-process controlado)
execution:
  mode: native
protocol: null
```

```yaml
# Módulo em processo (sem string de shell)
execution:
  mode: process
  command: python
  args:
    - main.py
protocol: opencore-module-v1
```

O formato acima é ilustrativo. YAML, TOML ou outro formato somente será confirmado após validação técnica. A classe `process` e o protocolo neutro de linguagem estão condicionados ao ADR-021 e ao Spike 10.

Evitar `entrypoint` como string de shell (`"python main.py"`): usar `command` + `args` separados para previsibilidade multiplataforma.

### 11.3 Campos obrigatórios e campos conceituais do catálogo

O contrato deverá representar:

- identificador único;
- nome legível;
- versão;
- licença;
- tipo do módulo;
- bloco `execution` (`mode`, e se `process`: `command`, `args`);
- versão de OpenCore Runtime compatível;
- protocolo utilizado, quando `execution.mode: process`;
- dependências obrigatórias;
- dependências opcionais;
- permissões solicitadas;
- eventos publicados;
- eventos consumidos;
- migrações;
- esquema de configuração;
- `ui_schema` opcional (fora do Spike 10);
- ponto de inicialização / encerramento conforme o modo;
- estado de compatibilidade;
- nível de confiança (`trust_level`);
- necessidade de rede e destinos externos;
- contrato de portabilidade, retenção e exclusão de dados.

Além dos campos estruturais acima, o manifesto deverá representar, como **campos conceituais obrigatórios a modelar** (formato final não congelado antes dos spikes):

| Campo conceitual | Finalidade |
|---|---|
| `capabilities` | Capacidades funcionais fornecidas |
| `business_tags` | Termos de negócio usados pelo catálogo |
| `supported_os` | Sistemas operacionais suportados |
| `hardware_requirements` | Requisitos mínimos de hardware |
| `network_requirements` | Necessidade e destinos de rede |
| `data_categories` | Categorias de dados tratados |
| `external_transmissions` | Transmissões externas possíveis |
| `maintenance_status` | Status de manutenção |
| `trust_level` | Nível técnico T0–T3 conforme ADR-017; a interface pode exibir o nome traduzido |
| `owner` | Ownership / mantenedor declarado |
| `backup_contract` | Contrato de participação no backup |
| `export_contract` | Contrato de exportação portátil |
| `uninstall_policy` | Política de desinstalação e retenção |
| `demo_data` | Disponibilidade de dados de demonstração |
| `documentation_urls` | Documentação remota ou referências locais |
| `conflicts` | Conflitos com outros módulos |
| `replaces` | Módulos que este substitui |
| `recommended_with` | Combinações recomendadas |

Esses campos alimentam o catálogo de capacidades, o OpenCore Builder e o catálogo público. Sua serialização definitiva permanece hipótese até validação.

### 11.4 Ciclo de vida de módulos

O ciclo de vida consolidado deverá cobrir estados equivalentes a:

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

Transições intermediárias (por exemplo, “Migrações aplicadas” como etapa de “Migrado”) poderão ser detalhadas na implementação, desde que o estado observável permaneça auditável.

#### Estados de falha

Estados de falha deverão ser registrados de forma explícita, incluindo ao menos:

- incompatível;
- dependência ausente;
- migração falhou;
- crash loop;
- bloqueado por política;
- manutenção encerrada;
- quarentenado.

### 11.5 Falha de ativação

Sempre que a integridade da distribuição permitir, uma falha deverá impedir apenas a ativação do módulo afetado e de seus dependentes.

A aplicação deverá interromper completamente a inicialização quando a falha envolver:

- OpenCore Runtime incompatível;
- migração estrutural incompleta;
- corrupção detectada;
- módulo obrigatório ausente;
- violação de integridade;
- configuração essencial inválida;
- dependência circular não resolvida;
- política de confiança ou quarentena que bloqueie módulo obrigatório.

### 11.6 Estratégia inicial de carregamento

A primeira versão distinguirá duas classes técnicas de módulo, conforme ADR-021:

1. **Módulos nativos (`execution.mode: native`)** — componentes Rust conhecidos no momento da compilação ou registrados estaticamente na distribuição, executados no processo do OpenCore Runtime.
2. **Módulos em processo (`execution.mode: process`)** — processos isolados supervisionados pelo **Module Host**, conversando pelo **OpenCore Module Protocol**. Hipótese a validar no Spike 10; não será tratada como aceita antes das evidências.

Módulos nativos permanecerão a regra para o caminho crítico de segurança, integridade, autenticação e capacidades estruturais.

Módulos em processo serão a preferência inicial para domínio educacional e para módulos de domínio que não exijam integração profunda, desde que o spike seja aceito. A distribuição de referência deverá incluir pelo menos um módulo de domínio real em processo se essa classe for adotada — evitando relegar o modelo multilíngue a exemplos puramente didáticos.

#### Limite de segurança do isolamento por processo

Na v0, o isolamento por processo fornece **isolamento de falhas**, não uma **sandbox completa de segurança**.

Módulos em processo deverão ser oficiais, verificados ou explicitamente confiáveis. As permissões do manifesto controlam capacidades disponibilizadas pelas APIs do OpenCore: o Module Host nega **operações não autorizadas solicitadas pelo protocolo**. Isso não garante, por si só, que o processo filho — rodando com o mesmo usuário do OpenCore Runtime — não tente acessar diretamente arquivos, rede ou outros recursos do sistema operacional.

O módulo não recebe caminho, handle ou API de acesso direto ao SQLite da distribuição. Módulos oficiais utilizam exclusivamente o serviço de storage. A execução de código não confiável exigirá ADR e sandbox de SO específicos.

A primeira versão não deverá assumir:

- ABI binária estável;
- bibliotecas dinâmicas arbitrárias in-process;
- interpretador Python, JavaScript ou Wasm embutido no OpenCore Runtime;
- download e execução automática de módulos não verificados;
- compatibilidade ilimitada entre versões;
- sandbox completa de sistema operacional;
- suporte oficial simultâneo a múltiplas linguagens externas na Etapa 1.

Há dois contratos distintos:

- **SDK nativo Rust** — API in-process;
- **OpenCore Module Protocol** + **SDK de processo** — IPC para módulos externos.

O protocolo é o contrato público dos módulos em processo (Apache 2.0). O SDK nativo não deve misturar chamadas internas com mensagens IPC.

### 11.7 Contrato de portabilidade e exclusão por módulo

Todo módulo que persista dados deverá declarar e implementar um contrato mínimo contendo:

- entidades e categorias de dados sob sua responsabilidade;
- indicação de dados pessoais, sensíveis ou operacionais;
- relações necessárias para preservar significado na exportação;
- formatos de exportação suportados;
- versão do esquema exportado;
- procedimento de validação da exportação;
- operações de exclusão total e seletiva;
- dependências que podem impedir exclusão imediata;
- retenções legais ou de auditoria configuráveis;
- anexos e arquivos vinculados;
- transferências para adaptadores ou serviços externos;
- comportamento após desativação ou remoção do módulo.

#### Testes obrigatórios do contrato

Um módulo persistente deverá possuir testes que comprovem:

1. exportação completa de um conjunto de dados representativo;
2. leitura ou validação da exportação sem depender do banco interno;
3. exclusão dos registros elegíveis e de seus anexos;
4. preservação explícita dos registros sujeitos a retenção ou auditoria;
5. ausência de dados pertencentes a outros módulos na exportação privada;
6. relatório de dependências quando uma exclusão não puder ser concluída;
7. identificação de dados enviados a integrações externas configuradas.

Módulos proprietários candidatos a certificação deverão cumprir o mesmo contrato por interfaces documentadas. A proteção de segredos industriais não poderá eliminar a capacidade de exportar, excluir ou migrar dados essenciais do usuário.

### 11.8 Empacotamento de módulos em processo

| Estratégia | Uso recomendado |
|---|---|
| Interpretador no sistema (PATH) | Desenvolvimento e primeiros spikes |
| Interpretador empacotado na distribuição | Distribuições oficiais |
| Executável autônomo do módulo | Alternativa para distribuições oficiais |
| Equivalente nativo temporário | Transição, não solução definitiva |

**Distribuições oficiais não deverão exigir que o usuário final instale Python ou Node.** Runtime da linguagem no PATH não é estratégia final de produto.

Nenhuma estratégia poderá violar o offline-first: um módulo já instalado não dependerá de download de interpretador ou de serviço remoto para executar funções locais.

---

## 12. Comunicação entre módulos

A comunicação entre módulos deverá obedecer ao contrato conceitual:

```text
Dados pertencem ao módulo.
Serviços expõem operações.
Eventos notificam mudanças.
A interface consome serviços.
```

Na prática, a comunicação utilizará duas estratégias principais:

1. chamadas por contratos explícitos (comandos e consultas request/response), quando houver necessidade de resposta imediata;
2. eventos, quando a comunicação representar um fato ocorrido ou puder ser desacoplada.

### 12.1 Contratos explícitos e serviços

São apropriados para:

- consultar uma autorização;
- solicitar uma operação que precisa retornar resultado;
- validar uma dependência obrigatória;
- acessar um serviço-base documentado.

Exigências do contrato de serviços:

- contratos explícitos de entrada e saída;
- validação uniforme de payloads;
- erros tipados e observáveis;
- registro dos serviços disponíveis;
- versão do contrato;
- proibição de acesso à implementação interna de outro módulo;
- comandos request/response para operações síncronas;
- eventos para comunicação desacoplada;
- proteção contra ciclos e tempestades de eventos.

Contratos não deverão expor estruturas internas ou permitir mutação irrestrita do estado de outro módulo.

### 12.2 Barramento de eventos

A primeira versão deverá utilizar um barramento:

- local;
- em processo;
- sem serviço externo;
- observável;
- testável;
- com tipos ou esquemas identificáveis.

A implementação poderá ser síncrona ou assíncrona conforme a categoria do evento, desde que o comportamento seja documentado.

### 12.3 Categorias de eventos

#### Eventos de domínio

Representam fatos relevantes do negócio:

```text
VisitanteRegistrado
EntregaRecebida
MoradorAtualizado
OcorrenciaCriada
```

#### Eventos técnicos

Representam fatos operacionais:

```text
ModuloInicializado
MigracaoConcluida
BackupConcluido
FalhaDeMigracao
AtualizacaoDisponivel
```

### 12.4 Requisitos dos eventos

Cada evento deverá possuir:

- nome estável;
- versão ou esquema identificável;
- origem;
- horário;
- identificador de correlação quando necessário;
- payload documentado;
- política de falha;
- classificação como técnico ou de domínio.

### 12.5 Comunicação com módulos em processo

Quando `execution.mode: process` estiver habilitado, a comunicação entre Module Host e módulo externo deverá usar o mesmo modelo lógico de contratos e eventos, transportado pelo OpenCore Module Protocol.

Requisitos iniciais do Spike 10:

- transporte: stdio com framing por comprimento (não apenas quebra de linha);
- protocolo lógico: JSON-RPC 2.0 ou subconjunto documentado;
- handshake de registro e declaração de capacidades;
- request/response para comandos e consultas;
- publicação e consumo de eventos;
- encerramento e reinício controlados;
- correlação de requisições e diagnóstico de falhas;
- ausência de transação distribuída entre processos na v0;
- módulo headless (sem `ui_schema` no critério de aceitação).

A semântica de entrega deverá ser documentada. Consistência entre módulos continuará baseada em um proprietário claro dos dados e em orquestração por comandos/eventos.

Backlog obrigatório da especificação do protocolo (não bloqueia o spike conceitual, bloqueia SDK v0 estável):

- timeout de inicialização e de comandos;
- encerramento gracioso e cancelamento;
- heartbeat / health check;
- limite de reinícios, backoff e detecção de crash loop;
- limite de tamanho das mensagens;
- comportamento quando o módulo fica indisponível;
- propagação de logs e erros.

### 12.6 Limites iniciais

A primeira versão não exigirá:

- Kafka;
- RabbitMQ;
- broker externo;
- entrega distribuída;
- garantia global de ordenação;
- persistência obrigatória de todos os eventos;
- transações ACID atravessando processos de módulos.

Essas capacidades somente deverão ser consideradas mediante necessidade comprovada.

---

## 13. Persistência e arquitetura de dados

### 13.1 Banco inicial

SQLite será utilizado inicialmente como banco local padrão, condicionado à validação dos spikes.

### 13.2 Propriedade de dados por módulo

Cada tabela ou conjunto de dados deverá possuir um módulo responsável claramente identificável.

Um módulo não deverá:

- alterar diretamente tabelas privadas de outro módulo;
- depender de detalhes internos não documentados;
- executar migrações sobre estruturas que não controla.

Quando dados de outro módulo forem necessários, o acesso deverá ocorrer por:

- contrato público;
- projeção de leitura autorizada;
- evento;
- estrutura compartilhada formalmente definida.

### 13.3 Convenção inicial de identificação

Durante os spikes, deverá ser avaliado o uso de:

- prefixos por módulo;
- registro central de migrações;
- namespaces lógicos;
- metadados de propriedade de tabelas.

SQLite não oferece schemas independentes da mesma forma que outros bancos. Portanto, o isolamento inicial será principalmente contratual, estrutural e validado por testes.

### 13.4 Migrações

Cada módulo com persistência deverá fornecer suas próprias migrações.

As migrações deverão possuir:

- identificador único;
- versão de origem;
- versão de destino;
- ordem determinística;
- módulo proprietário;
- verificação de pré-condições;
- resultado registrado;
- teste automatizado;
- estratégia de recuperação.

### 13.5 Transações

Migrações deverão ser transacionais quando tecnicamente possível.

Quando uma operação não puder ser revertida integralmente, isso deverá ser documentado e acompanhado de:

- backup prévio;
- validação adicional;
- procedimento de recuperação;
- mensagem explícita ao operador.

### 13.6 Desativação e remoção de módulos

Desativar um módulo não deverá apagar automaticamente seus dados.

A remoção definitiva deverá ser uma operação separada e explícita, com:

- confirmação;
- backup recomendado;
- impacto documentado;
- verificação de dependentes;
- opção de exportação quando aplicável.

### 13.7 Banco compartilhado versus banco por módulo

A primeira versão utilizará, por simplicidade, um banco SQLite compartilhado pela distribuição, com propriedade lógica de estruturas por módulo.

Banco separado por módulo poderá ser reavaliado se os testes demonstrarem necessidade de:

- isolamento superior;
- portabilidade independente;
- redução de contenção;
- ciclo de vida separado;
- requisitos específicos de segurança.

### 13.8 Acesso a dados por módulos em processo

Módulos com `execution.mode: process` não recebem caminho, handle ou API de acesso direto ao SQLite da distribuição. Módulos oficiais utilizam exclusivamente o serviço de storage exposto pelo protocolo.

O Module Host nega operações de storage não autorizadas **solicitadas pelo protocolo**. Isso é política e arquitetura mediada — não uma sandbox de sistema operacional.

**SQL genérico irrestrito é proibido** (ex.: `storage.execute` com string SQL livre). O protocolo deverá expor operações de alto nível (put/get/query tipada, migração declarada).

O Spike 10 comparará:

1. **Opção A (inclinação inicial):** um arquivo SQLite por módulo em processo sob `data/modules/<id>.db`, mantendo `opencore.db` para o runtime/nativos;
2. **Opção B:** banco compartilhado com namespace controlado exclusivamente pelo runtime.

A Opção A oferece isolamento estrutural alinhado à ausência de transações entre processos. Módulos nativos podem permanecer sob ADR-013 (banco compartilhado + propriedade lógica) independentemente da escolha para processos.

---

## 14. Backup, restauração e exportação

### 14.1 Backup técnico

O backup técnico deverá preservar o estado necessário para restaurar uma instalação compatível.

Deverá incluir, conforme o caso:

- banco de dados;
- versão do runtime;
- versões dos módulos;
- configurações necessárias;
- metadados da distribuição;
- anexos e arquivos relacionados;
- checksum ou mecanismo de integridade;
- data e versão do formato de backup.

### 14.2 Restauração

Antes de restaurar, o sistema deverá validar:

- integridade do arquivo;
- versão do formato;
- compatibilidade do runtime;
- módulos exigidos;
- espaço disponível;
- impacto sobre a instalação atual.

A restauração deverá evitar sobrescrever silenciosamente dados existentes.

### 14.3 Exportação portátil

Dados essenciais deverão ser exportáveis em formatos documentados e legíveis, como:

- JSON;
- CSV;
- arquivos anexos acompanhados de metadados;
- outro formato aberto apropriado ao domínio.

A exportação deverá informar:

- esquema;
- codificação;
- relações entre entidades;
- data de geração;
- versão do exportador;
- limitações conhecidas.

### 14.4 Exclusão de dados

Módulos que armazenem dados pessoais ou operacionais deverão oferecer mecanismos documentados para exclusão quando o usuário autorizado possuir legitimidade para realizá-la.

A exclusão deverá considerar:

- dependências;
- auditoria;
- retenção legal aplicável;
- backups existentes;
- anexos;
- sincronizações externas configuradas.

A operação deverá produzir resultado verificável, indicando:

- dados excluídos;
- dados anonimizados ou pseudonimizados;
- dados mantidos e respectivo motivo;
- anexos processados;
- integrações externas que exigem ação adicional;
- backups que permanecem sujeitos à política de retenção.

### 14.5 Portabilidade em módulos proprietários

Para certificação ou presença em catálogo oficial, um módulo proprietário deverá fornecer:

- exportador documentado para os dados essenciais;
- esquema versionado e formato aberto ou amplamente interoperável;
- mecanismo documentado de exclusão;
- inventário de dados enviados a serviços externos;
- testes de conformidade executáveis pela entidade certificadora;
- documentação suficiente para auditoria de segurança sob termos adequados de confidencialidade quando necessário.

O contrato poderá ocultar detalhes internos do produto, mas não poderá tornar os dados essenciais dependentes de uma implementação privada insubstituível.

---

## 15. Configuração

### 15.1 Princípios

Configurações deverão ser:

- documentadas;
- validadas;
- versionadas quando necessário;
- separadas de segredos;
- exportáveis quando não sensíveis;
- recuperáveis por padrão seguro.

### 15.2 Escopos

A arquitetura deverá distinguir:

- configuração do runtime;
- configuração da distribuição;
- configuração do módulo;
- configuração por organização ou unidade;
- preferência individual do usuário;
- segredo ou credencial.

### 15.3 Segredos

Credenciais e tokens não deverão ser armazenados em texto simples quando houver mecanismo seguro disponível no sistema operacional.

A estratégia de armazenamento de segredos deverá ser validada em Windows, Linux e macOS.

---

## 16. Logs, diagnóstico e auditoria

### 16.1 Logs técnicos

O runtime deverá fornecer infraestrutura comum de logs com:

- níveis de severidade;
- origem identificada;
- horário;
- identificador de correlação quando necessário;
- proteção contra exposição indevida de dados sensíveis;
- rotação ou limite de armazenamento;
- exportação para suporte.

### 16.2 Auditoria de operações

Operações relevantes ao domínio poderão exigir trilha de auditoria própria, distinta dos logs técnicos.

A auditoria deverá registrar, quando aplicável:

- ator;
- operação;
- recurso afetado;
- data e hora;
- resultado;
- origem da solicitação;
- justificativa ou contexto.

### 16.3 Privacidade

Logs não deverão incluir automaticamente:

- senhas;
- tokens completos;
- documentos pessoais completos;
- conteúdo sensível desnecessário;
- dados enviados a integrações externas sem necessidade de diagnóstico.

---

## 17. Segurança e permissões

### 17.1 Princípio de menor privilégio

Cada módulo deverá declarar as capacidades de que necessita.

Exemplos de permissões futuras:

- leitura de configuração;
- gravação em armazenamento próprio;
- publicação de eventos;
- consumo de eventos específicos;
- acesso a arquivos selecionados;
- acesso à rede;
- uso de integração externa;
- execução de tarefa em segundo plano;
- acesso a dados de outro módulo por contrato.

### 17.2 Níveis de confiança de módulos

O nível de confiança descreve o grau de revisão, manutenção e distribuição do módulo. Ele não substitui permissões técnicas nem representa garantia absoluta de segurança.

| Nível | Denominação | Requisitos mínimos | Distribuição permitida |
|---|---|---|---|
| T0 | Experimental | Código local ou spike, manifesto parcial, sem garantia de manutenção | Apenas desenvolvimento; nunca em distribuição oficial |
| T1 | Comunitário | Código-fonte disponível, manifesto completo, licença identificada, testes mínimos e revisão inicial | Instalação manual ou catálogo comunitário com aviso |
| T2 | Verificado | Revisão técnica, CI completo, contrato de dados, SBOM, compatibilidade testada e artefato assinado quando distribuído | Catálogo verificado e distribuições não oficiais avaliadas |
| T3 | Oficial | Mantido sob governança OpenCore, licença compatível com módulo oficial, suporte de versão, auditoria e inclusão em matriz oficial | Distribuições oficiais |

A instalação automática de T0 será proibida fora do ambiente de desenvolvimento. Builds oficiais iniciais aceitarão apenas módulos T3 registrados estaticamente; spikes e ambientes de desenvolvimento poderão carregar T0 de forma explícita e isolada.

Módulos em processo na v0 deverão ser oficiais (T3), verificados (T2) ou explicitamente confiáveis, conforme ADR-017 e ADR-021. O isolamento por processo fornece **isolamento de falhas**, não sandbox de sistema operacional — permissões do manifesto controlam operações solicitadas pelo protocolo, mas não garantem bloqueio de acesso direto a recursos do SO pelo processo filho.

**Certificação comercial** será um atributo separado do nível de confiança. Um módulo ou edição comercial poderá ser certificado sem se tornar oficial ou open source, desde que cumpra os contratos públicos aplicáveis de segurança, portabilidade, marca e interoperabilidade.

Para alinhamento com ADR-017 e com o OpenCore Builder, a nomenclatura de produto poderá expor:

- experimental (T0);
- comunitário (T1);
- verificado (T2);
- certificado (atributo comercial distinto);
- oficial (T3).

O Builder deverá preferir, nesta ordem:

1. módulos T3 Oficiais;
2. módulos T2 Verificados compatíveis;
3. módulos T1 Comunitários somente quando o usuário entrar em modo avançado e aceitar o risco;
4. nunca recomendar T0 Experimental por padrão.

### 17.3 Código externo

Código externo não deverá ser baixado e executado automaticamente na primeira versão.

Um futuro sistema de módulos distribuídos deverá considerar:

- assinatura de artefatos;
- origem verificável;
- checksum;
- política de revogação;
- compatibilidade;
- permissões;
- isolamento;
- auditoria;
- resposta a vulnerabilidades.

### 17.4 Política de segurança

O repositório deverá possuir um processo separado para:

- relato privado de vulnerabilidades;
- triagem;
- correção;
- divulgação coordenada;
- versões afetadas;
- publicação de avisos de segurança.

---

## 18. Interface gráfica

### 18.1 Hipótese inicial

Slint será avaliado como tecnologia principal para a interface gráfica.

Essa escolha permanece provisória até a conclusão dos spikes em Windows, Linux e macOS.

### 18.2 Separação de responsabilidades

A lógica de negócio não deverá depender diretamente da interface.

A interface deverá consumir:

- casos de uso;
- contratos;
- estados de apresentação;
- eventos apropriados.

Módulos deverão evitar incorporar regras essenciais exclusivamente em componentes visuais.

### 18.3 Composição da interface

A Etapa 1 deverá avaliar como módulos contribuem com:

- rotas ou páginas;
- menus;
- permissões de acesso;
- formulários;
- componentes visuais;
- traduções;
- ações globais.

A solução deverá evitar que um módulo modifique livremente qualquer parte da interface sem contrato.

Módulos nativos poderão contribuir com componentes Slint sob contratos da distribuição.

`ui_schema` para módulos em processo é **opcional** e **fora do escopo de aceitação do Spike 10**. O primeiro módulo externo deverá ser headless (comandos, storage, eventos, consultas, falha/recuperação).

Um spike posterior (Spike 11) poderá validar UI declarativa (tabelas, formulários, rotas) renderizada por componentes oficiais, evitando transformar o protocolo em construtor low-code prematuro. Injeção arbitrária de código Slint a partir de linguagens externas permanecerá fora do escopo até ADR específico.

### 18.4 Acessibilidade e internacionalização

As distribuições oficiais deverão evoluir com suporte a:

- navegação por teclado;
- contraste adequado;
- textos escaláveis;
- mensagens de erro compreensíveis;
- tradução por recursos externos ao código quando possível;
- formatos locais de data, número e moeda.

---

## 19. Atualizações e canal de segurança

### 19.1 Responsabilidade estrutural do runtime

O Manifesto classifica atualização como capacidade estrutural. Isso significa que o runtime deverá fornecer os contratos e garantias necessários para atualizar com segurança:

- identificação das versões instaladas;
- validação de compatibilidade entre runtime, módulos e distribuição;
- verificação de integridade e autenticidade de metadados ou artefatos;
- coordenação de backup, migração e recuperação;
- bloqueio de downgrade incompatível;
- registro auditável do resultado;
- possibilidade de continuar utilizando a versão local quando a atualização não for obrigatória por necessidade técnica comprovada.

O runtime não deverá conter um provedor comercial, servidor ou feed específico.

### 19.2 Cliente, feed e obtenção de artefatos

A consulta a canais, download de pacotes e integração com provedores serão implementados por interface administrativa, ferramenta ou adaptador substituível.

O usuário poderá:

- desativar consultas automáticas;
- configurar canais compatíveis;
- importar manualmente metadados e pacotes assinados;
- consultar versões instaladas sem enviar telemetria;
- verificar avisos de segurança sem permitir atualização automática.

### 19.3 Canal mínimo de atualização de segurança

Antes de existir um atualizador automático, cada versão suportada deverá possuir um canal mínimo e publicamente documentado contendo:

- matriz de versões suportadas;
- avisos de segurança com identificador e severidade;
- versões afetadas e corrigidas;
- notas de versão;
- checksums e assinaturas dos artefatos publicados;
- instruções de atualização e recuperação;
- opção de download manual;
- formato legível por máquina para futura integração com adaptadores.

O formato deverá permitir espelhamento por terceiros e importação manual, evitando que um único servidor da entidade OpenCore seja requisito para continuidade.

### 19.4 Limite da primeira versão

Na Etapa 1, o objetivo será validar:

- identificação de componentes instalados;
- pacote assinado ou checksum verificável;
- compatibilidade e atualização de esquema;
- backup prévio e recuperação;
- leitura local ou remota de um aviso de segurança de teste.

Um atualizador automático completo poderá ser adiado até que o formato de distribuição e a política de compatibilidade estejam mais maduros.

---

## 20. Sincronização como adaptador

Sincronização será opcional e classificada como integração ou adaptador, não como módulo-base genérico nem como parte do runtime mínimo.

O runtime poderá oferecer contratos neutros de identificação, eventos, conflito e integridade somente quando esses contratos forem comprovadamente úteis a mais de um adaptador.

Uma futura solução deverá preservar:

- operação local;
- exportação independente;
- resolução documentada de conflitos;
- criptografia apropriada;
- controle do usuário;
- possibilidade de utilizar provedores alternativos quando viável;
- ausência de dependência obrigatória da entidade OpenCore.

A primeira distribuição poderá operar integralmente em uma única instalação local.

---

## 21. Licenciamento por fronteira arquitetural

### 21.1 MPL 2.0

Deverão utilizar MPL 2.0, conforme o Manifesto:

- runtime e kernel;
- registro e ciclo de vida de módulos;
- configuração estrutural;
- barramento de eventos;
- persistência e migrações estruturais;
- segurança e integridade;
- logs estruturais;
- atualização;
- módulos-base oficiais;
- módulos de domínio incorporados oficialmente;
- outros componentes classificados como patrimônio técnico central.

### 21.2 Apache 2.0

Deverão utilizar Apache 2.0, conforme o Manifesto:

- especificação pública do OpenCore Module Protocol;
- SDK de processo e bindings por linguagem;
- SDK nativo Rust publicado para autores de módulos (quando estabilizado), distinto do patrimônio MPL do runtime;
- bibliotecas cliente;
- bindings;
- adaptadores;
- templates;
- scaffolds;
- exemplos;
- suíte de testes de conformidade do protocolo (`conformance-tests`);
- ferramentas auxiliares que não componham o núcleo protegido;
- OpenCore Builder, scaffolds e CLI de desenvolvimento (proposta);
- materiais de código educacional não incorporados ao núcleo ou a módulos oficiais.

O protocolo é a especificação principal dos módulos em processo. O SDK nativo Rust e o SDK de processo são artefatos distintos e não devem misturar API in-process com mensagens IPC.

### 21.3 Módulos independentes

Módulos independentes poderão utilizar outras licenças compatíveis, desde que:

- a licença seja claramente informada;
- obrigações das dependências sejam respeitadas;
- os direitos sobre dados sejam preservados;
- não sejam apresentados como oficiais sem aprovação;
- não comprometam uma distribuição oficial aberta.

### 21.4 Distribuições oficiais

Distribuições oficiais deverão ser integralmente open source e informar de forma auditável:

- componentes;
- versões;
- licenças;
- avisos;
- código-fonte exigido;
- procedimento de reconstrução.

### 21.5 Verificação automatizada

O projeto deverá avaliar durante a Etapa 1:

- identificadores SPDX;
- cabeçalhos ou arquivos de licença por diretório;
- inventário de dependências;
- geração de SBOM;
- validação de compatibilidade de licenças no CI;
- documentação de componentes de terceiros.

---

## 22. Organização inicial do repositório

A hipótese inicial é um monorepo durante a fase de validação.

```text
opencore/
├── apps/
│   └── portaria/
├── runtime/
│   ├── core/
│   ├── config/
│   ├── events/
│   ├── storage/
│   ├── migrations/
│   ├── module_host/      # orquestração de módulos process (ADR-021)
│   └── diagnostics/
├── modules/
│   ├── base/
│   │   ├── auth/
│   │   ├── permissions/
│   │   ├── audit/
│   │   └── backup/
│   └── domain/
│       └── portaria/
│           ├── units/
│           ├── residents/
│           ├── visitors/
│           └── deliveries/
├── protocol/             # OpenCore Module Protocol (Apache 2.0)
├── sdk/
│   ├── native-rust/      # API in-process para módulos nativos
│   ├── process-python/   # após Spike 10, se Python for escolhido
│   ├── process-typescript/ # após estabilizar o primeiro host, se aplicável
│   └── conformance-tests/
├── tools/
├── examples/
├── docs/
│   ├── architecture/
│   ├── adr/
│   ├── rfc/
│   ├── security/
│   └── contributing/
├── licenses/
├── scripts/
└── tests/
```

### 22.1 Motivos para o monorepo inicial

- mudanças coordenadas entre runtime e módulos;
- testes integrados;
- revisão centralizada;
- visibilidade das fronteiras;
- simplificação do CI;
- onboarding mais direto;
- auditoria de licenças;
- menor custo operacional durante os spikes.

A divisão em múltiplos repositórios deverá ocorrer apenas quando houver motivos claros de ciclo de vida, acesso, publicação ou manutenção independente.

### 22.2 Trilha educacional sem redução do padrão técnico

A participação educacional será organizada principalmente em componentes Apache 2.0 e em tarefas de risco controlado, sem criar uma arquitetura paralela simplificada.

O monorepo deverá prever:

- templates de módulos e adaptadores sob Apache 2.0;
- exemplos mínimos executáveis;
- exercícios baseados em contratos reais;
- módulos de demonstração sem dados sensíveis;
- fixtures e ambientes descartáveis;
- documentação por nível de experiência;
- tarefas de documentação, testes, interface e domínio;
- validações automáticas iguais às exigidas das contribuições profissionais.

Contribuidores iniciantes poderão atuar em superfícies menores, mas qualquer código incorporado ao produto deverá cumprir os mesmos critérios de revisão, teste, segurança e documentação.

---


## 23. Ferramentas de desenvolvimento e experiência de contribuição

A arquitetura exige ferramentas que reduzam o custo de criar, validar e diagnosticar módulos e distribuições, sem substituir testes nem contratos.

### 23.1 CLI conceitual

A CLI abaixo é **hipótese de produto**, a validar em spike próprio (Spike 12) ou incorporada aos spikes existentes:

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

### 23.2 Requisitos de scaffolding e ambiente

As ferramentas deverão oferecer, no mínimo:

- scaffolding para módulo nativo;
- scaffolding para módulo em processo;
- scaffolding para adaptador;
- scaffolding para distribuição;
- manifesto inicial;
- licença adequada à fronteira (MPL ou Apache, conforme o caso);
- testes;
- documentação;
- exemplo de evento;
- migração inicial quando aplicável;
- fixtures e dados de demonstração;
- mensagens de erro acionáveis;
- ambiente reproduzível nos três sistemas operacionais;
- documentação parcialmente gerada a partir do manifesto;
- projeto de exemplo completo.

A CLI não faz parte do OpenCore Runtime mínimo. Ferramentas e templates deverão preferir Apache 2.0, conforme o Manifesto.

---

## 24. OpenCore Builder

### 24.1 Posição arquitetural

O **OpenCore Builder** é ferramenta ou serviço auxiliar externo ao runtime. Proposta de licenciamento: Apache 2.0. Detalhamento normativo: ADR-022 e especificação funcional dedicada.

O Builder **não**:

- integra o runtime mínimo;
- é requisito para operar uma instalação já baixada;
- substitui validação determinística de compatibilidade;
- autoriza compilação arbitrária de código de usuários no MVP.

### 24.2 Componentes

Composição conceitual:

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

### 24.3 Fluxo de composição guiada

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

### 24.4 Limite da IA

Formalização:

- a IA pode interpretar linguagem natural;
- a IA pode explicar recomendações;
- a IA pode ordenar perguntas;
- a IA pode sugerir módulos opcionais;
- a IA **não** decide compatibilidade;
- a IA **não** pode ignorar permissões, conflitos ou níveis de confiança;
- a IA **não** pode gerar comandos de instalação não validados;
- a IA **não** pode incluir módulo não verificado silenciosamente;
- toda saída deverá passar pelo motor determinístico;
- o sistema deverá funcionar em modo sem IA, usando questionário e regras.

A autoridade técnica permanece nos manifestos, lockfiles, níveis de confiança, matriz de compatibilidade e validador.

### 24.5 Catálogo de capacidades

O catálogo traduz linguagem de negócio em capacidades e módulos candidatos, sem hardcoded de tipo de negócio no OpenCore Runtime.

Exemplo conceitual:

```text
“Vender no balcão”
→ capability: point_of_sale
→ módulos candidatos: sales + pos

“Controlar produtos perecíveis”
→ capability: perishable_inventory
→ módulos candidatos: inventory + expiration_control
```

Separação obrigatória:

- necessidade do usuário;
- capacidade funcional;
- implementação por módulo;
- perfil de distribuição.

### 24.6 Preview estrutural

O preview inicial poderá ser **estrutural**, não uma execução completa do produto.

Poderá mostrar:

- navegação;
- dashboard;
- módulos ativos;
- fluxos;
- permissões;
- telas representativas;
- alertas sobre rede e dados externos.

O preview **não** poderá:

- prometer telas ou funcionalidades não implementadas;
- substituir testes;
- ocultar dependências;
- funcionar como construtor low-code genérico na primeira versão.

### 24.7 Empacotamento progressivo

#### Estágio A — seleção de pacote pré-construído

- o usuário responde à triagem;
- o Builder escolhe distribuição/perfil verificado;
- entrega instalador pronto;
- gera configuração e lockfile.

#### Estágio B — montagem a partir de artefatos assinados

- OpenCore Runtime e módulos já compilados;
- pipeline monta o pacote;
- não executa código arbitrário;
- valida hashes e compatibilidade.

#### Estágio C — geração avançada

Somente após matriz robusta:

- limites de custo;
- fila e cache;
- assinatura;
- SBOM;
- builds reproduzíveis;
- isolamento de pipeline;
- auditoria.

Compilação arbitrária **não** está autorizada no MVP.

---

## 25. Instalação, onboarding e ficha da instalação

### 25.1 Instalação e onboarding

A experiência inicial de uma distribuição oficial deverá prever:

- instalador por sistema operacional;
- ausência de Docker ou banco separado no modo monoposto;
- verificação de requisitos;
- configuração guiada;
- nome da organização;
- região e idioma;
- usuários iniciais;
- importação de planilhas quando houver formatos suportados;
- produtos ou cadastros iniciais;
- backup;
- impressoras e periféricos, quando aplicável;
- dados de demonstração opcionais;
- relatório final da configuração.

Migração progressiva de modos operacionais é hipótese a validar:

- monoposto;
- rede local;
- sincronização opcional (adaptador);
- múltiplas unidades.

A transição entre modos não deverá ser prometida sem spike e testes.

### 25.2 Ficha da instalação

A instalação deverá poder gerar uma ficha de diagnóstico contendo:

- distribuição;
- perfil;
- OpenCore Runtime;
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

A ficha apoia suporte, auditoria e reprodução sem tornar o portal obrigatório para continuidade.

---

## 26. Catálogo público de módulos

Antes de marketplace, o portal deverá oferecer um **catálogo verificado** de módulos.

Metadados mínimos:

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
- `execution.mode`;
- testes;
- status de manutenção;
- documentação;
- screenshots;
- dependências;
- conflitos;
- substitutos.

O catálogo não garante que qualquer combinação funcione. Compatibilidade continua sujeita a manifesto, lockfile, perfis verificados e validador determinístico.

---
## 27. Estratégia de testes

### 27.1 Camadas de teste

A arquitetura deverá suportar:

- testes unitários da lógica de negócio;
- testes de contrato entre runtime e módulos;
- testes de migração;
- testes de integração entre módulos;
- testes de backup e restauração;
- testes de exportação;
- testes de inicialização e falha;
- testes de empacotamento;
- testes multiplataforma;
- testes de segurança proporcionais ao risco.

### 27.2 Requisitos mínimos para módulos

Um módulo não deverá ser considerado concluído sem:

- testes de suas regras principais;
- testes de configuração inválida;
- testes de migração;
- testes de permissões relevantes;
- documentação de eventos e contratos;
- teste de ativação e encerramento.

### 27.3 CI inicial

O CI deverá evoluir para verificar:

- compilação;
- formatação;
- análise estática;
- testes;
- licenças;
- vulnerabilidades conhecidas;
- documentação básica;
- builds ou smoke tests nos sistemas suportados.

### 27.4 Testes de arquitetura no CI

As fronteiras não deverão depender apenas de revisão humana. O CI deverá, progressivamente, verificar:

- imports e dependências proibidas por camada;
- ciclos entre módulos;
- acesso direto a tabelas ou migrações pertencentes a outro módulo;
- presença e validade do manifesto;
- compatibilidade de versão declarada;
- divergência entre permissões declaradas e capacidades utilizadas;
- acesso à rede sem declaração;
- fronteiras MPL 2.0 e Apache 2.0 por diretório;
- presença do contrato de dados em módulos persistentes;
- testes de exportação e exclusão;
- proibição de dependência de domínio dentro do runtime;
- inclusão de componente proprietário essencial em distribuição oficial.

Quando uma regra não puder ser verificada estaticamente, deverá existir teste automatizado ou checklist obrigatório de revisão com responsável identificado.

### 27.5 Testes de composição

A suíte deverá cobrir, progressivamente:

- dependências;
- conflitos;
- compatibilidade;
- perfis;
- lockfile;
- reprodução de instalação a partir do lockfile;
- remoção de módulos;
- atualização.

### 27.6 Testes de experiência

- instalação limpa;
- primeira execução;
- backup;
- restauração;
- importação;
- desinstalação;
- funcionamento offline;
- hardware mínimo declarado.

### 27.7 Testes do OpenCore Builder

- mesma entrada gera composição determinística;
- IA não contorna regras do validador;
- opção “não sei” é suportada;
- sistema funciona sem IA;
- preview corresponde à composição;
- instalador corresponde ao lockfile;
- componentes não verificados não entram no modo padrão.

### 27.8 Testes de soberania

- exportação legível;
- restauração verificável;
- exclusão;
- transmissão externa declarada;
- telemetria desligada por padrão;
- instalação continua útil sem serviço central nem portal.

---

## 28. Desempenho e hardware modesto

A proposta de leveza deverá ser tratada como requisito mensurável, não apenas como intenção.

Os spikes deverão coletar ao menos:

- tamanho do executável ou pacote;
- tempo de inicialização;
- memória em repouso;
- memória durante operações comuns;
- tempo de leitura e gravação;
- tempo de migração em bases de teste;
- impacto de módulos adicionais;
- comportamento sem conexão;
- estabilidade em hardware mais simples disponível para teste.

Metas numéricas definitivas somente deverão ser estabelecidas após medições iniciais.

---

## 29. Observabilidade local

A observabilidade inicial deverá ser local e proporcional.

O projeto deverá permitir:

- identificar módulos carregados;
- visualizar versões;
- verificar migrações;
- consultar erros recentes;
- exportar diagnóstico;
- verificar integridade do banco e do backup;
- compreender por que um módulo não foi ativado.

Não será necessário criar uma plataforma remota de observabilidade na primeira versão.

---

## 30. Compatibilidade e versionamento

### 30.1 Versionamento

Runtime, módulos e distribuições deverão possuir versões independentes.

A estratégia inicial deverá avaliar versionamento semântico, levando em conta que versões anteriores a 1.0 podem sofrer mudanças frequentes.

### 30.2 Compatibilidade

O manifesto de módulo deverá declarar o intervalo de versões de runtime suportado.

A distribuição deverá possuir uma matriz resolvida de:

- runtime;
- módulos;
- migrações;
- sistema operacional;
- formato de backup.

### 30.3 Depreciação

Antes de remover um contrato público, o projeto deverá definir:

- aviso;
- alternativa;
- período de transição;
- impacto;
- migração necessária;
- versão em que ocorrerá a remoção.

Uma política completa será criada com o SDK v0.

---

## 31. ADRs iniciais

As seguintes decisões deverão ser registradas separadamente:

| ADR | Título | Estado inicial |
|---|---|---|
| ADR-001 | Monólito modular como arquitetura inicial | Proposto |
| ADR-002 | Separação entre runtime, módulos-base, módulos de domínio e distribuições | Proposto |
| ADR-003 | SQLite como persistência local inicial | Proposto, condicionado a spike |
| ADR-004 | Barramento local de eventos para comunicação desacoplada | Proposto |
| ADR-005 | Registro estático/nativo in-process antes de plugins dinâmicos; módulos em processo são classe distinta (ADR-021) | Proposto |
| ADR-006 | Rust como hipótese principal para o runtime | Proposto, condicionado a spike |
| ADR-007 | Slint como hipótese principal para a interface | Proposto, condicionado a spike |
| ADR-008 | Licenciamento definido por fronteira arquitetural | Aceito pelo Manifesto v1.1 |
| ADR-009 | Backup técnico separado de exportação portátil | Proposto |
| ADR-010 | Telemetria externa desativada por padrão | Aceito pelo Manifesto v1.1 |
| ADR-011 | Monorepo durante a fase inicial | Proposto |
| ADR-012 | Portaria como distribuição de referência, não parte do runtime | Aceito pela direção do projeto |
| ADR-013 | Banco compartilhado com propriedade lógica por módulo na primeira versão | Proposto, condicionado a spike |
| ADR-014 | Execução local sem broker ou infraestrutura externa | Proposto |
| ADR-015 | Matriz de classificação entre runtime, módulo-base e adaptador | Proposto |
| ADR-016 | Contrato de portabilidade e exclusão por módulo | Proposto |
| ADR-017 | Níveis de confiança de módulos | Proposto |
| ADR-018 | Separação entre atualização estrutural e canais de distribuição | Proposto |
| ADR-019 | Sincronização classificada como adaptador | Proposto |
| ADR-020 | Testes automatizados de arquitetura no CI | Proposto |
| ADR-021 | Módulos nativos, módulos em processo e protocolo neutro (isolamento ≠ sandbox; execution; storage tipado) | Proposto, condicionado ao Spike 10 |
| ADR-022 | OpenCore Builder — triagem, composição e empacotamento | Proposto, condicionado aos Spikes 14–18 |

ADRs condicionados deverão ser revisados após os resultados dos spikes correspondentes. ADR-021 permanece condicionado ao Spike 10; ADR-022 aos Spikes 14–18.

---

## 32. Hipóteses técnicas a validar na Etapa 1

### 32.1 Stack e multiplataforma

1. Rust compila, executa e pode ser empacotado adequadamente em Windows, Linux e macOS?
2. Slint oferece os componentes, acessibilidade, integração e desempenho necessários?
3. O tamanho dos pacotes e o consumo de memória são compatíveis com a proposta de leveza?
4. O fluxo de desenvolvimento é viável para contribuidores em formação?

### 32.2 Modularidade

5. Dois módulos simples podem ser registrados sem acoplamento indevido?
6. O runtime consegue resolver dependências e rejeitar ciclos?
7. Um módulo pode falhar sem derrubar desnecessariamente componentes independentes?
8. Uma segunda combinação experimental pode utilizar o mesmo runtime sem alterações específicas?

### 32.2.1 Módulos em processo (ADR-021)

8a. Um módulo em processo (Python preferencial) consegue registrar-se e conversar com o Module Host por protocolo local versionado (stdio + framing)?
8b. Crash do módulo deixa o OpenCore Runtime e demais módulos operacionais?
8c. Operações não autorizadas solicitadas pelo protocolo são negadas?
8d. O módulo não recebe caminho/handle/API direta ao SQLite e usa exclusivamente o serviço de storage (sem SQL genérico)?
8e. Opção A (SQLite por módulo) vs Opção B (namespace compartilhado) foi comparada com recomendação documentada?
8f. O empacotamento (PATH no spike; plano de runtime empacotado para distribuição oficial) permanece compatível com leveza e offline-first?
8g. Uma pessoa em formação consegue criar e executar um módulo headless só com documentação e template?

### 32.3 Dados e migrações

9. SQLite atende ao volume e à concorrência esperados para a primeira distribuição?
10. Cada módulo consegue controlar suas migrações com ordem determinística?
11. O banco compartilhado mantém fronteiras suficientemente claras?
12. Migrações com falha podem ser recuperadas com segurança?
13. A desativação de um módulo preserva seus dados sem afetar outros módulos?

### 32.4 Eventos e contratos

14. Eventos conectam módulos sem acesso direto às estruturas internas?
15. O modelo síncrono, assíncrono ou híbrido é compreensível e testável?
16. Erros em consumidores de eventos podem ser diagnosticados sem perda silenciosa?
17. Os contratos de consulta e comando evitam dependências excessivas?

### 32.5 Backup, exportação e continuidade

18. O backup captura banco, configuração e metadados suficientes?
19. A restauração valida compatibilidade antes de alterar a instalação?
20. A exportação portátil permite reconstruir os dados essenciais fora do OpenCore?
21. O formato de backup pode evoluir sem tornar versões antigas inutilizáveis?

### 32.6 Segurança e configuração

22. Permissões podem ser declaradas e verificadas de forma útil desde a primeira versão?
23. Segredos podem ser armazenados adequadamente nos três sistemas?
24. Logs evitam vazamento de dados sensíveis?
25. A ausência de conexão não altera a autorização para recursos locais?
26. O contrato de portabilidade e exclusão é implementável sem expor detalhes internos desnecessários?
27. Os metadados exigidos permitem documentar responsabilidades de LGPD por módulo e distribuição?

### 32.7 Empacotamento e manutenção

28. Instaladores podem ser reproduzidos para os três sistemas?
29. O projeto consegue registrar inventário e licenças dos componentes?
30. O CI consegue validar dependências, contratos de dados e conformidade sem infraestrutura excessiva?
31. Uma pessoa externa consegue executar o projeto seguindo apenas a documentação?
32. As decisões arquiteturais conseguem ser alteradas sem reescrever todo o protótipo?
33. Um canal mínimo de segurança pode ser consumido sem telemetria e sem dependência de servidor único?

---

## 33. Spikes recomendados

### 33.1 Fatia vertical experimental comum

Os spikes não serão nove protótipos desconectados. Eles deverão evoluir uma única **fatia vertical experimental descartável**, composta por:

```text
Aplicação mínima
→ runtime
→ módulo de unidades
→ módulo de visitantes
→ registro de visitante
→ persistência SQLite
→ evento VisitanteRegistrado
→ tela mínima
→ backup
→ exportação portátil
```

A fatia servirá para validar o caminho completo sem transformar o código do spike em implementação definitiva. Uma segunda composição experimental reutilizará o runtime sem o módulo de visitantes.

### 33.2 Time-box e política de interrupção

| Spike | Limite máximo | Resultado esperado no limite |
|---|---:|---|
| 01 — Multiplataforma | 3 pessoa-dias | Build e execução nos três sistemas ou relatório de bloqueio reproduzível |
| 02 — Registro de módulos | 2 pessoa-dias | Dois módulos estáticos, dependências e falha isolada |
| 03 — Eventos locais | 2 pessoa-dias | Publicação, consumo e falha observável |
| 04 — SQLite e migrações | 3 pessoa-dias | Propriedade lógica, migração e recuperação simulada |
| 05 — Backup e restauração | 2 pessoa-dias | Backup íntegro e restauração validada |
| 06 — Exportação portátil | 2 pessoa-dias | Exportação documentada e validação externa |
| 07 — Duas composições | 2 pessoa-dias | Dois manifests e builds sem regra de domínio no runtime |
| 08 — Licenças e CI arquitetural | 1 pessoa-dia | SPDX, inventário e pelo menos duas regras automatizadas |
| 09 — Onboarding externo | 3 pessoa-dias | Dois checkpoints com participantes externos e relatório |

**Limite total inicial:** 20 pessoa-dias para os Spikes 01–09. Para trabalho individual, a referência é quatro semanas úteis. Um time pequeno poderá executar partes em paralelo, mas deverá manter a fatia vertical compartilhada.

O **Spike 10** (ADR-021 — módulo em processo headless) possui orçamento adicional de **+3 pessoa-dias**, totalizando **23 pessoa-dias** quando incluído. O Spike 10 não faz parte do limite de 20 pessoa-dias dos Spikes 01–09.

O **Spike 11** (UI declarativa para módulos em processo) é futuro e **não está incluído** no orçamento inicial de validação técnica.

Os **Spikes 12–18** (CLI, lockfile, Builder, instalador, preview, IA opcional e montagem de pacote) são posteriores à sequência 01–11, **sem renumerá-la**, e possuem orçamento a definir no backlog; não elevam automaticamente o limite de 20+3 pessoa-dias da validação técnica inicial.

Ao atingir o limite sem evidência suficiente, o spike deverá ser encerrado como **inconclusivo** ou **rejeitado**. A resposta padrão não será aumentar silenciosamente o prazo, mas registrar o bloqueio, revisar a hipótese e decidir entre simplificação, alternativa tecnológica ou novo experimento explicitamente aprovado.

### 33.3 Critérios de reavaliação de Rust e Slint

Rust e Slint continuarão como hipóteses preferenciais, mas deverão ser comparados com alternativas se ocorrer qualquer um dos gatilhos abaixo:

- menos de dois de três participantes externos conseguem instalar, compilar, executar os testes e realizar uma alteração simples em até quatro horas, após uma rodada de correção da documentação;
- o build reproduzível permanece dependente de passos manuais não documentáveis em mais de um sistema operacional;
- a composição de uma tela simples de módulo exige alteração estrutural no runtime ou acoplamento direto ao domínio;
- acessibilidade, empacotamento ou integração essencial permanece bloqueada em dois sistemas após três pessoa-dias de investigação focada;
- o custo de contribuição observado impede dividir tarefas em unidades adequadas à trilha educacional sem concentrar todas as mudanças em poucos especialistas.

O acionamento de um gatilho não implica substituição automática. Ele exige pausar decisões irreversíveis, abrir um ADR comparativo e executar um spike curto com uma alternativa selecionada.

### Spike 01 — Aplicação mínima multiplataforma

**Objetivo:** compilar e executar uma aplicação Rust com Slint nos três sistemas.

**Evidências:**

- scripts de build;
- pacotes gerados;
- consumo de memória;
- tempo de inicialização;
- dificuldades por sistema;
- relatório comparativo.

### Spike 02 — Registro de módulos

**Objetivo:** registrar dois módulos estáticos com manifestos e ciclos de vida independentes.

**Evidências:**

- validação de identificadores;
- resolução de dependências;
- ordem de inicialização;
- falha isolada;
- logs de diagnóstico.

### Spike 03 — Eventos locais

**Objetivo:** permitir que um módulo publique um evento consumido por outro sem acesso ao estado interno.

**Evidências:**

- evento tipado ou validado;
- teste de consumidor;
- tratamento de falha;
- medição de acoplamento;
- decisão síncrona, assíncrona ou híbrida.

### Spike 04 — SQLite e migrações por módulo

**Objetivo:** criar tabelas pertencentes a dois módulos e aplicar migrações independentes.

**Evidências:**

- registro de migrações;
- rollback ou recuperação;
- falha simulada;
- teste de dependência;
- convenção de nomes.

### Spike 05 — Backup e restauração

**Objetivo:** gerar backup técnico, alterar dados e restaurar uma instalação compatível.

**Evidências:**

- formato do backup;
- checksum;
- metadados de versão;
- teste de corrupção;
- teste de incompatibilidade.

### Spike 06 — Exportação portátil

**Objetivo:** exportar dados dos dois módulos em formatos abertos e documentados.

**Evidências:**

- JSON ou CSV;
- esquema documentado;
- anexos quando aplicável;
- reimportação experimental ou validação externa;
- diferenças em relação ao backup.

### Spike 07 — Composição de distribuições

**Objetivo:** gerar duas aplicações ou composições usando o mesmo runtime e combinações diferentes de módulos.

**Evidências:**

- manifests das distribuições;
- builds separados;
- ausência de regras de domínio no runtime;
- testes de compatibilidade;
- comparação das dependências.

### Spike 08 — Licenças e inventário

**Objetivo:** validar a separação MPL 2.0 e Apache 2.0 no repositório.

**Evidências:**

- arquivos de licença;
- identificadores SPDX;
- inventário de dependências;
- verificação no CI;
- proposta de SBOM.

### Spike 09 — Onboarding externo

**Objetivo:** medir a acessibilidade real da stack e da arquitetura sem reduzir os critérios profissionais.

O spike terá dois checkpoints:

1. após a aplicação mínima multiplataforma;
2. após a fatia vertical integrada.

**Evidências:**

- perfil técnico do participante, sem coleta pessoal desnecessária;
- tempo até primeira execução;
- tempo até executar os testes;
- tempo até realizar uma alteração simples em módulo ou exemplo;
- bloqueios encontrados;
- passos não documentados;
- quantidade de orientação privada necessária;
- correções realizadas;
- resultado reproduzido em mais de um sistema;
- decisão sobre manutenção ou reavaliação de Rust e Slint.

### Spike 10 — Módulo em processo (ADR-021)

**Objetivo:** executar um módulo headless em processo isolado (**Python** preferencial), comunicando-se com o Module Host por OpenCore Module Protocol (stdio + framing por comprimento + JSON-RPC 2.0 ou subconjunto).

**Orçamento:** +3 pessoa-dias (fora do limite de 20 dos Spikes 01–09).

**Evidências:**

- manifesto com `execution.mode: process`, `command` + `args` (sem string de shell);
- handshake e registro;
- comando request/response;
- evento publicado e consumido;
- operações não autorizadas solicitadas pelo protocolo negadas;
- crash isolado e reinício;
- módulo sem caminho/handle/API direta ao SQLite; storage exclusivamente via serviço (sem SQL genérico);
- comparação documentada Opção A vs B de persistência, com recomendação;
- medição de memória e latência;
- PATH ok para o spike; plano de interpretador empacotado para distribuição oficial;
- template mínimo; teste em **macOS + Windows** (Linux desejável).

**Fora de escopo deste spike:** `ui_schema`, segundo SDK, interpretador embutido no OpenCore Runtime, UI Slint injetada, marketplace, sandbox de SO, SQL genérico mediado.

### Spike 11 — UI declarativa para módulos em processo (futuro)

**Objetivo:** validar `ui_schema` mínimo (rota, tabela, formulário) após o protocolo headless estar estável.

**Pré-condição:** Spike 10 aceito.

**Orçamento:** fora do limite inicial de validação técnica.

### Spike 12 — CLI e scaffolding

**Objetivo:** validar a hipótese de CLI e scaffolding de contribuição.

**Validar:**

- criação de módulo nativo;
- criação de módulo em processo;
- manifesto;
- testes;
- documentação;
- execução local;
- `opencore doctor`.

**Orçamento:** fora do limite de 20 pessoa-dias dos Spikes 01–09; time-box a definir no backlog da Etapa 1.

### Spike 13 — Manifesto e lockfile de distribuição

**Objetivo:** validar composição reproduzível de distribuições.

**Validar:**

- composição reproduzível;
- hashes;
- atualização;
- rollback;
- diagnóstico a partir do lockfile.

### Spike 14 — Builder baseado em regras

**Objetivo:** validar o motor determinístico do OpenCore Builder sem IA.

**Validar:**

- questionário;
- catálogo de capacidades;
- recomendação sem IA;
- seleção de perfil;
- composição válida;
- geração do lockfile.

### Spike 15 — Instalador e onboarding

**Objetivo:** validar instalação utilizável por pessoa não técnica no modo monoposto.

**Validar:**

- instalação limpa;
- configuração inicial;
- importação CSV;
- backup;
- funcionamento offline.

### Spike 16 — Preview estrutural

**Objetivo:** validar preview sem execução completa do produto.

**Validar:**

- menus;
- módulos ativos;
- telas representativas;
- ausência de promessas falsas.

### Spike 17 — Camada conversacional opcional

**Objetivo:** validar IA subordinada ao motor de regras.

**Validar:**

- IA interpreta respostas;
- motor determinístico mantém autoridade;
- fallback sem IA;
- privacidade e retenção mínima.

### Spike 18 — Montagem de pacote com artefatos pré-construídos

**Objetivo:** validar o Estágio B de empacotamento.

**Validar:**

- composição sem recompilar código;
- lockfile;
- hashes;
- assinatura futura;
- custo operacional.

Os Spikes 12–18 **não renumeram** os Spikes 01–11. São adicionados após a sequência atual e permanecem condicionantes do ADR-022 e da experiência de adoção.

---

## 34. Métricas iniciais

Os spikes deverão registrar dados comparáveis, sem impor metas arbitrárias antes da primeira medição.

### Produto e desempenho

- tempo de inicialização;
- memória em repouso;
- tamanho do pacote;
- tempo de operações comuns;
- tempo de migração;
- tempo de backup e restauração.

### Arquitetura

- dependências por módulo;
- falhas de isolamento;
- necessidade de acesso direto entre módulos;
- quantidade de mudanças no runtime exigidas por domínio;
- cobertura de testes dos contratos.

### Contribuição

- tempo para preparar o ambiente;
- quantidade de passos manuais;
- erros de documentação;
- tempo até executar um teste;
- necessidade de orientação privada.

### Portabilidade

- diferenças entre sistemas;
- dependências específicas;
- falhas de empacotamento;
- recursos indisponíveis;
- esforço de manutenção por plataforma.

### Dados, privacidade e conformidade

- percentual de módulos persistentes com contrato de dados válido;
- testes de exportação e exclusão aprovados;
- dados sem proprietário identificado;
- destinos externos não declarados;
- retenções sem justificativa documentada;
- falhas de remoção de anexos ou referências.

### Confiança e distribuição

- módulos por nível T0–T3;
- divergências entre nível declarado e evidências disponíveis;
- artefatos sem inventário ou assinatura exigida;
- componentes não abertos presentes em composições oficiais;
- tempo para revisar e promover um módulo entre níveis.

### Composição, Builder e adoção

- taxa de conclusão da triagem;
- tempo até recomendação;
- taxa de download;
- taxa de instalação concluída;
- tempo até primeiro uso útil;
- abandono por etapa;
- erros de compatibilidade;
- instalações reproduzidas pelo lockfile;
- módulos removidos após recomendação;
- porcentagem que escolhe o perfil recomendado;
- necessidade de suporte;
- sucesso na importação;
- retenção de uso;
- funcionamento em hardware mínimo.

Essas métricas de adoção somente deverão ser coletadas com consentimento, telemetria desativada por padrão e retenção mínima. Medições locais de spike não exigem telemetria remota.

---

## 35. Critérios de saída da Arquitetura v1.3

Esta etapa documental será considerada concluída quando:

- cada componente inicial possuir uma camada identificada;
- responsabilidades do OpenCore Runtime estiverem delimitadas por matriz de classificação;
- dependências permitidas e proibidas estiverem documentadas;
- contrato interno de módulo v0 estiver definido (incluindo `execution`, `protocol` e campos de confiança/portabilidade);
- campos conceituais do manifesto para catálogo e Builder estiverem listados sem congelar formato;
- contrato de portabilidade e exclusão por módulo estiver definido;
- níveis de confiança e condições de distribuição estiverem definidos;
- ciclo de vida de módulos e estados de falha estiverem descritos;
- manifesto e lockfile de distribuição estiverem diferenciados;
- perfis verificados estiverem definidos como variantes testadas;
- OpenCore Builder estiver posicionado como componente externo, com limite da IA;
- catálogo de capacidades, preview estrutural e empacotamento progressivo A/B/C estiverem descritos;
- instalação, onboarding e ficha da instalação estiverem especificados;
- catálogo público de módulos (pré-marketplace) estiver descrito;
- interoperabilidade por adaptadores estiver prevista;
- ferramentas de desenvolvimento e CLI conceitual estiverem documentadas;
- persistência e migrações tiverem regras iniciais;
- backup e exportação estiverem diferenciados;
- comunicação por dados/serviços/eventos/interface estiver definida;
- fronteiras de segurança estiverem registradas;
- responsabilidades estruturais de atualização estiverem separadas dos canais e provedores;
- obrigações arquiteturais relacionadas à LGPD estiverem explícitas;
- regras de arquitetura verificáveis pelo CI estiverem listadas;
- fronteiras de licenciamento estiverem mapeadas;
- ADRs iniciais tiverem sido criados, incluindo ADR-022 proposto;
- hipóteses técnicas tiverem sido convertidas em spikes mensuráveis (Spikes 01–11 preservados; Spikes 12–18 adicionados; ADR-021 condicionado ao Spike 10);
- nenhuma regra específica do OpenCore Portaria estiver localizada no runtime;
- uma pessoa leiga conseguir compreender como obter uma distribuição sem aprender a arquitetura interna.

---

## 36. Critérios de saída da validação técnica

A Etapa 1 poderá ser considerada concluída quando:

- a stack funcionar nos três sistemas operacionais;
- dois módulos operarem sem acoplamento indevido;
- dependências e ciclos forem validados;
- migrações por módulo funcionarem de forma verificável;
- eventos permitirem comunicação sem acesso interno direto;
- backup e restauração forem demonstrados;
- exportação portátil e exclusão verificável forem validadas;
- duas composições utilizarem o mesmo runtime;
- os builds puderem ser reproduzidos;
- o canal mínimo de atualização de segurança for demonstrado;
- o onboarding externo atender aos limites definidos ou gerar ADR comparativo de stack;
- o Spike 10 tiver aceito, rejeitado ou adiado com evidência registrada o modelo de módulo em processo (ADR-021);
- as fronteiras de licença puderem ser auditadas;
- uma pessoa externa executar o projeto seguindo apenas a documentação;
- os ADRs provisórios forem aceitos, alterados ou rejeitados com base nas evidências.

---

## 37. Riscos arquiteturais iniciais

### 37.1 Complexidade prematura do sistema de plugins

**Risco:** tentar oferecer plugins binários, marketplace e ABI estável antes de compreender os contratos reais.

**Mitigação:** iniciar com registro estático e extrair o SDK após a primeira fatia vertical; validar módulos em processo apenas via protocolo (ADR-021 / Spike 10); adiar interpretador embutido e ABI dinâmica.

### 37.2 Contaminação do runtime pelo domínio de portaria

**Risco:** acelerar a primeira distribuição incorporando regras específicas ao núcleo.

**Mitigação:** revisar dependências, exigir ADR e validar uma segunda composição experimental.

### 37.3 Isolamento apenas aparente entre módulos

**Risco:** módulos separados em diretórios, mas acoplados por tabelas, tipos internos ou chamadas diretas.

**Mitigação:** contratos, propriedade de dados, testes de arquitetura e eventos.

### 37.4 Stack pouco acessível à comunidade

**Risco:** Rust e Slint apresentarem barreira de contribuição maior que o benefício obtido.

**Mitigação:** medir onboarding, documentação e produtividade durante os spikes; oferecer caminho de contribuição via módulo em processo e template educacional (ADR-021); comparar alternativas caso os resultados sejam inadequados.

### 37.4.1 Custo oculto do empacotamento multilíngue

**Risco:** bundlar ou depender de Python/Node destruir a proposta de leveza, previsibilidade de instalador ou offline-first; ou distribuir oficialmente exigindo PATH do sistema.

**Mitigação:** Spike 10 medir tamanho; PATH só para desenvolvimento; distribuições oficiais exigem interpretador empacotado ou executável autônomo; não aceitar ADR-021 sem plano de empacotamento de produto.

### 37.4.2 Falsa confiança em "sandbox por processo"

**Risco:** documentação ou marketing apresentarem isolamento de processo como garantia de que o módulo não acessa SQLite, arquivos ou rede fora do protocolo.

**Mitigação:** declaração explícita no ADR-021 e nesta arquitetura; módulos em processo apenas oficiais/verificados/confiáveis na v0; sandbox de SO em ADR futura se houver necessidade de código não confiável.

### 37.5 Portabilidade inconsistente

**Risco:** diferenças de GUI, empacotamento, segredos ou sistema de arquivos entre plataformas.

**Mitigação:** validar os três sistemas desde o primeiro spike, não apenas ao final.

### 37.6 Backup sem portabilidade

**Risco:** considerar a cópia do banco suficiente para soberania de dados.

**Mitigação:** implementar e testar backup técnico e exportação portátil como entregas distintas.

### 37.7 Licenciamento difícil de auditar

**Risco:** misturar arquivos MPL 2.0 e Apache 2.0 sem fronteiras objetivas.

**Mitigação:** organização por diretório, SPDX, inventário e verificações automatizadas.

### 37.8 Dependência excessiva do fundador

**Risco:** decisões e conhecimento permanecerem privados.

**Mitigação:** ADRs, RFCs, documentação de setup, testes e onboarding externo desde a Etapa 1.

### 37.9 Spikes desconectados ou sem limite

**Risco:** consumir semanas em protótipos isolados sem demonstrar um fluxo completo de produto.

**Mitigação:** utilizar uma única fatia vertical experimental, aplicar limite total de 20 pessoa-dias e encerrar cada spike com decisão registrada.

### 37.10 Conformidade apenas declaratória

**Risco:** mencionar LGPD, portabilidade e exclusão sem mecanismos executáveis por módulo.

**Mitigação:** contrato de dados no manifesto, testes obrigatórios de exportação e exclusão e verificação no CI.

### 37.11 Explosão combinatória de módulos

**Risco:** combinações livres tornarem a matriz de compatibilidade inviável.

**Mitigação:** perfis verificados, personalização limitada, crescimento da matriz com evidência e rejeição de combinações não testadas no modo padrão.

### 37.12 Recomendação incorreta da IA

**Risco:** linguagem natural produzir composição inválida, insegura ou não solicitada.

**Mitigação:** IA subordinada ao motor determinístico; modo sem IA; proibição de instalar módulos não verificados silenciosamente.

### 37.13 Preview divergente do produto

**Risco:** preview estrutural prometer telas ou fluxos inexistentes.

**Mitigação:** Spike 16; preview limitado a composição real; testes de correspondência preview/composição.

### 37.14 Custo excessivo de builds

**Risco:** geração dinâmica de pacotes consumir recursos além do sustentável.

**Mitigação:** Estágios A/B antes de C; fila, cache e limites de custo; proibição de compilação arbitrária no MVP.

### 37.15 Distribuição não reproduzível

**Risco:** instalação não reconstruível a partir de artefatos documentados.

**Mitigação:** manifesto + lockfile, hashes, SBOM progressivo e testes de reprodução.

### 37.16 Onboarding mais complexo que a instalação

**Risco:** configuração inicial exigir conhecimento técnico incompatível com o público.

**Mitigação:** fluxos guiados, opção “não sei”, dados de demonstração e teste com usuário não técnico.

### 37.17 Excesso de perfis por negócio

**Risco:** proliferação de perfis sem manutenção nem evidência.

**Mitigação:** poucos perfis verificados por distribuição; governança de catálogo; SEO apenas para perfis reais.

### 37.18 Regras regulatórias locais em módulos gerais

**Risco:** contaminação do runtime ou de módulos genéricos por regras fiscais/regulatórias de um segmento.

**Mitigação:** regras de domínio fora do runtime; módulos específicos; Portaria ≠ identidade da plataforma.

### 37.19 Dependência do portal para reinstalação

**Risco:** usuário precisar do portal para restaurar ou recriar a instalação.

**Mitigação:** lockfile exportável, instaladores offline após download, ficha da instalação e operação sem serviço central.

### 37.20 Coleta indevida de dados da triagem

**Risco:** triagem coletar dados desnecessários ou usar respostas para fins não consentidos.

**Mitigação:** triagem anônima por padrão; telemetria off; retenção mínima; sem uso para anúncios.

### 37.21 SEO prometendo funcionalidades inexistentes

**Risco:** páginas de aquisição descreverem capacidades ainda não implementadas.

**Mitigação:** páginas apenas para distribuições/perfis reais; linguagem alinhada ao lockfile e à documentação.

### 37.22 Catálogo como marketplace inseguro

**Risco:** catálogo público evoluir prematuramente para marketplace sem confiança, assinatura e compatibilidade.

**Mitigação:** catálogo verificado antes de marketplace; ADR-017; Builder sem experimental por padrão.

### 37.23 Manutenção de módulos abandonados

**Risco:** módulos recomendados perderem mantenedor sem processo de abandono.

**Mitigação:** `maintenance_status` no manifesto; governança de abandono; remoção de distribuições oficiais quando necessário.

---

## 38. Questões ainda abertas

As seguintes decisões permanecem abertas até validação ou documento específico:

- formato definitivo do manifesto de módulo;
- mecanismo interno exato de registro de módulos;
- modelo síncrono, assíncrono ou híbrido do barramento de eventos;
- biblioteca de migrações SQLite;
- estratégia de reversão de migrações irreversíveis;
- armazenamento seguro de segredos em cada sistema;
- composição modular da interface Slint;
- formato final de backup;
- política de compatibilidade antes da versão 1.0;
- critérios para dividir o monorepo;
- necessidade e momento de plugins dinâmicos;
- estratégia futura de assinatura de módulos;
- contratos específicos para diferentes estratégias de sincronização;
- critérios técnicos detalhados para certificação comercial e uso da marca;
- momento em que um adaptador de banco alternativo se torna necessário;
- escolha confirmada da primeira linguagem externa (preferência: Python);
- framing e subconjunto JSON-RPC definitivos;
- Opção A vs B de storage para módulos em processo (recomendação pós-Spike 10);
- estratégia de empacotamento do interpretador em distribuições oficiais;
- parâmetros do ciclo operacional do Module Host (timeouts, heartbeat, crash loop);
- escape hatch futuro para UI além do esquema declarativo;
- necessidade e desenho de sandbox de SO para código não confiável;
- formato definitivo de manifesto e lockfile de distribuição;
- escopo exato da CLI e comandos estabilizados;
- critérios de promoção no catálogo de capacidades;
- limites de personalização de perfis no Builder;
- momento de ativar Estágio B e C de empacotamento;
- retenção e privacidade da triagem quando houver IA opcional;
- priorização de adaptadores de importação externos.

Questões abertas deverão ser transformadas em spikes, ADRs ou RFCs antes de gerar implementação estrutural definitiva.

---

## 39. Próximas entregas

Após aprovação desta arquitetura, a sequência recomendada é:

1. tratar esta Arquitetura v1.3, o Manifesto v1.2, o Roadmap v2.3 e os ADR-015 a ADR-022 como base documental da Etapa 0;
2. revisar e aprovar ADR-022 e a especificação funcional do OpenCore Builder v0, sem iniciar implementação completa do Builder;
3. concluir Comunidade e Governança OpenCore v1.0 e Plano Institucional OpenCore v1.0, se ainda pendentes;
4. extrair especificações normativas: `module-manifest-v0`, `data-portability-v0`, `module-trust-v0` e rascunho de `distribution-lockfile-v0`;
5. criar a matriz executável de dependências permitidas e proibidas e a matriz `native` vs `process`;
6. definir a convenção inicial de migrações, backup, restauração e exclusão;
7. preparar a estrutura mínima do monorepo (`module_host/`, `protocol/`, `native-rust`, `process-python`, `conformance-tests`, `tools/`) e os templates Apache 2.0 da trilha educacional;
8. converter os Spikes 01–09 em backlog time-boxed (20 pessoa-dias), o Spike 10 (+3 pessoa-dias), registrar Spike 11 como futuro e time-boxar Spikes 12–18 conforme capacidade;
9. a Etapa 1 pode iniciar com spikes técnicos reversíveis, documentados, time-boxed e isolados, enquanto a revisão formal da documentação prossegue; a fatia vertical definitiva permanece condicionada às decisões documentadas sobre stack e ADRs aplicáveis;
10. executar o Spike 10 (módulo em processo headless, ADR-021) com evidência registrada — ADR-021 permanece condicionado a esse spike;
11. priorizar CLI/lockfile/instalador de forma time-boxed após evidências mínimas da fatia vertical;
12. registrar resultados mensuráveis e revisar os ADRs condicionados;
13. iniciar a fatia vertical definitiva do OpenCore Portaria apenas após a decisão documentada sobre a stack e o resultado do Spike 10;
14. não antecipar marketplace, compilação arbitrária nem IA obrigatória.

---

## 40. Declaração arquitetural

O OpenCore será desenvolvido inicialmente como um monólito modular, local e multiplataforma, composto por um OpenCore Runtime mínimo em Rust, Module Host, módulos-base, módulos de domínio e distribuições testadas.

O OpenCore Runtime coordenará inicialização, configuração, módulos, dados, migrações, eventos, logs, integridade e garantias estruturais de atualização, sem incorporar regras específicas de negócio, provedores externos, canais comerciais ou o OpenCore Builder.

A primeira implementação deverá priorizar componentes T3 registrados estaticamente, SQLite como hipótese inicial, comunicação local, adaptadores opcionais e empacotamento nativo. Módulos em processo, comunicando-se pelo OpenCore Module Protocol, constituem hipótese oficial condicionada ao ADR-021 e ao Spike 10 — voltada a domínio, integrações e contribuição educacional, sem substituir Rust no núcleo e **sem** apresentar isolamento de processo como sandbox de segurança. Python permanece preferência inicial do primeiro SDK de processo, condicionada ao mesmo spike.

Distribuições oficiais deverão ser instaláveis e compreensíveis para usuários não técnicos, com manifesto, lockfile, perfis verificados e onboarding guiado. O OpenCore Builder é componente externo (ADR-022) que traduz necessidades em composição válida; a IA, quando existir, permanece subordinada a regras determinísticas.

Plugins dinâmicos in-process, interpretadores embutidos, marketplace, microserviços, sandbox completa de OS, compilação arbitrária de código de usuários e infraestrutura obrigatória de nuvem permanecerão fora do escopo até que necessidades reais justifiquem sua adoção.

A arquitetura deverá garantir que o OpenCore possa crescer sem abandonar os compromissos de soberania de dados, funcionamento offline, formatos abertos, licenciamento transparente, documentação, segurança, acessibilidade operacional e independência institucional definidos no Manifesto.

---

## Histórico

| Versão | Estado | Descrição |
|---|---|---|
| 1.0 | Proposta para revisão | Primeira consolidação das fronteiras arquiteturais, contrato modular, dados, eventos, segurança, licenciamento e plano de validação técnica. |
| 1.1 | Proposta revisada para aprovação | Resolve a classificação de atualização e sincronização, explicita LGPD, adiciona contrato de portabilidade e exclusão, níveis de confiança, trilha educacional, canal mínimo de segurança, testes arquiteturais e time-box dos spikes. |
| 1.2 | Proposta consolidada para aprovação | Consolida v1.1 com ADR-021 (módulos nativos/processo, protocolo, isolamento ≠ sandbox, empacotamento, storage tipado); supersede a linha divergente v1.0.1/v1.0.2. |
| 1.3 | Proposta consolidada para aprovação | Incorpora OpenCore Builder (externo ao runtime; ADR-022), experiência do desenvolvedor e CLI conceitual, manifesto/lockfile de distribuição, perfis verificados, catálogo de capacidades, preview estrutural, empacotamento progressivo A/B/C, instalação/onboarding/ficha, catálogo público de módulos, interoperabilidade por adaptadores, testes e métricas adicionais, riscos de composição/adoção e Spikes 12–18. Base: Manifesto 1.2 e Roadmap 2.3. Mantém ADR-021 condicionado ao Spike 10. Supersede Arquitetura v1.2. |
