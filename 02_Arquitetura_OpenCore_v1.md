# Arquitetura OpenCore — Versão 1.2

**Status:** proposta consolidada para aprovação  
**Base estratégica:** Manifesto OpenCore v1.1  
**Roadmap relacionado:** OpenCore — Roadmap Revisado v2.2  
**Finalidade:** definir as fronteiras arquiteturais iniciais do OpenCore e preparar a validação técnica da Etapa 1  
**Observação:** decisões tecnológicas marcadas como provisórias somente serão confirmadas após os spikes técnicos. Esta versão consolida a v1.1 (matriz de classificação, LGPD, portabilidade, níveis de confiança, separação atualização/sincronização, testes arquiteturais, time-box) com ADR-021 (módulos nativos, módulos em processo e protocolo neutro — conteúdo anteriormente mal-numerado).  
**Supersede:** Arquitetura v1.1 e a linha divergente v1.0.1/v1.0.2.

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
6. preparar o projeto para contribuições externas e adoção educacional.

Esta versão não define de forma definitiva:

- ABI pública para plugins binários;
- carregamento arbitrário de bibliotecas dinâmicas;
- marketplace de módulos;
- execução de código não confiável;
- sandbox completa de plugins;
- microserviços;
- infraestrutura obrigatória de nuvem;
- geração pública e arbitrária de builds;
- sincronização distribuída entre múltiplas unidades;
- política final de certificação de módulos;
- suporte definitivo a múltiplos bancos de dados.

Esses recursos somente deverão ser projetados quando houver evidência técnica, operacional ou institucional de necessidade.

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
- dependência obrigatória de serviços da entidade OpenCore.

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

A distribuição não é apenas uma lista livre de plugins. Ela representa uma composição cuja compatibilidade foi verificada.

### 9.2 Conteúdo mínimo de uma distribuição

Cada distribuição deverá declarar:

- identificador;
- nome;
- versão;
- runtime compatível;
- módulos incluídos;
- versões dos módulos;
- dependências obrigatórias;
- configurações iniciais;
- identidade visual;
- sistemas operacionais suportados;
- política de atualização;
- matriz de testes;
- documentação de instalação e operação;
- componentes e respectivas licenças;
- formato de backup e exportação suportado.

### 9.3 Distribuição de referência

O **OpenCore Portaria** será a primeira distribuição de referência.

Seu objetivo é validar o OpenCore como plataforma, e não transformar regras de portaria em capacidades do runtime.

### 9.4 Segunda composição experimental

Antes de considerar a plataforma validada, a mesma infraestrutura deverá executar uma segunda composição experimental de módulos.

Essa composição poderá ser pequena, mas deverá provar que:

- o runtime não depende do domínio de portaria;
- módulos podem ser combinados de forma diferente;
- as configurações da distribuição não exigem alterações estruturais no núcleo;
- os contratos possuem reutilização real.

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
trust_level: experimental | community | verified | official
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

### 11.3 Campos obrigatórios

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
- nível de confiança;
- necessidade de rede e destinos externos;
- contrato de portabilidade, retenção e exclusão de dados.

### 11.4 Ciclo de vida preliminar

```text
Descoberto
→ Manifesto validado
→ Compatibilidade validada
→ Dependências resolvidas
→ Permissões avaliadas
→ Migrações aplicadas
→ Inicializado
→ Ativo
→ Encerrado
```

Estados de falha deverão ser registrados de forma explícita.

### 11.5 Falha de ativação

Sempre que a integridade da distribuição permitir, uma falha deverá impedir apenas a ativação do módulo afetado e de seus dependentes.

A aplicação deverá interromper completamente a inicialização quando a falha envolver:

- runtime incompatível;
- migração estrutural incompleta;
- corrupção detectada;
- módulo obrigatório ausente;
- violação de integridade;
- configuração essencial inválida;
- dependência circular não resolvida.

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

A comunicação deverá utilizar duas estratégias principais:

1. chamadas por contratos explícitos, quando houver necessidade de resposta imediata;
2. eventos, quando a comunicação representar um fato ocorrido ou puder ser desacoplada.

### 12.1 Contratos explícitos

São apropriados para:

- consultar uma autorização;
- solicitar uma operação que precisa retornar resultado;
- validar uma dependência obrigatória;
- acessar um serviço-base documentado.

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

## 23. Estratégia de testes

### 23.1 Camadas de teste

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

### 23.2 Requisitos mínimos para módulos

Um módulo não deverá ser considerado concluído sem:

- testes de suas regras principais;
- testes de configuração inválida;
- testes de migração;
- testes de permissões relevantes;
- documentação de eventos e contratos;
- teste de ativação e encerramento.

### 23.3 CI inicial

O CI deverá evoluir para verificar:

- compilação;
- formatação;
- análise estática;
- testes;
- licenças;
- vulnerabilidades conhecidas;
- documentação básica;
- builds ou smoke tests nos sistemas suportados.

### 23.4 Testes de arquitetura no CI

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

---

## 24. Desempenho e hardware modesto

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

## 25. Observabilidade local

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

## 26. Compatibilidade e versionamento

### 26.1 Versionamento

Runtime, módulos e distribuições deverão possuir versões independentes.

A estratégia inicial deverá avaliar versionamento semântico, levando em conta que versões anteriores a 1.0 podem sofrer mudanças frequentes.

### 26.2 Compatibilidade

O manifesto de módulo deverá declarar o intervalo de versões de runtime suportado.

A distribuição deverá possuir uma matriz resolvida de:

- runtime;
- módulos;
- migrações;
- sistema operacional;
- formato de backup.

### 26.3 Depreciação

Antes de remover um contrato público, o projeto deverá definir:

- aviso;
- alternativa;
- período de transição;
- impacto;
- migração necessária;
- versão em que ocorrerá a remoção.

Uma política completa será criada com o SDK v0.

---

## 27. ADRs iniciais

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
| ADR-021 | Módulos nativos, módulos em processo e protocolo neutro (isolamento ≠ sandbox; execution; storage tipado) | Proposto, condicionado a spike |

ADRs condicionados deverão ser revisados após os resultados da Etapa 1.

---

## 28. Hipóteses técnicas a validar na Etapa 1

### 28.1 Stack e multiplataforma

1. Rust compila, executa e pode ser empacotado adequadamente em Windows, Linux e macOS?
2. Slint oferece os componentes, acessibilidade, integração e desempenho necessários?
3. O tamanho dos pacotes e o consumo de memória são compatíveis com a proposta de leveza?
4. O fluxo de desenvolvimento é viável para contribuidores em formação?

### 28.2 Modularidade

5. Dois módulos simples podem ser registrados sem acoplamento indevido?
6. O runtime consegue resolver dependências e rejeitar ciclos?
7. Um módulo pode falhar sem derrubar desnecessariamente componentes independentes?
8. Uma segunda combinação experimental pode utilizar o mesmo runtime sem alterações específicas?

### 28.2.1 Módulos em processo (ADR-021)

8a. Um módulo em processo (Python preferencial) consegue registrar-se e conversar com o Module Host por protocolo local versionado (stdio + framing)?
8b. Crash do módulo deixa o OpenCore Runtime e demais módulos operacionais?
8c. Operações não autorizadas solicitadas pelo protocolo são negadas?
8d. O módulo não recebe caminho/handle/API direta ao SQLite e usa exclusivamente o serviço de storage (sem SQL genérico)?
8e. Opção A (SQLite por módulo) vs Opção B (namespace compartilhado) foi comparada com recomendação documentada?
8f. O empacotamento (PATH no spike; plano de runtime empacotado para distribuição oficial) permanece compatível com leveza e offline-first?
8g. Uma pessoa em formação consegue criar e executar um módulo headless só com documentação e template?

### 28.3 Dados e migrações

9. SQLite atende ao volume e à concorrência esperados para a primeira distribuição?
10. Cada módulo consegue controlar suas migrações com ordem determinística?
11. O banco compartilhado mantém fronteiras suficientemente claras?
12. Migrações com falha podem ser recuperadas com segurança?
13. A desativação de um módulo preserva seus dados sem afetar outros módulos?

### 28.4 Eventos e contratos

14. Eventos conectam módulos sem acesso direto às estruturas internas?
15. O modelo síncrono, assíncrono ou híbrido é compreensível e testável?
16. Erros em consumidores de eventos podem ser diagnosticados sem perda silenciosa?
17. Os contratos de consulta e comando evitam dependências excessivas?

### 28.5 Backup, exportação e continuidade

18. O backup captura banco, configuração e metadados suficientes?
19. A restauração valida compatibilidade antes de alterar a instalação?
20. A exportação portátil permite reconstruir os dados essenciais fora do OpenCore?
21. O formato de backup pode evoluir sem tornar versões antigas inutilizáveis?

### 28.6 Segurança e configuração

22. Permissões podem ser declaradas e verificadas de forma útil desde a primeira versão?
23. Segredos podem ser armazenados adequadamente nos três sistemas?
24. Logs evitam vazamento de dados sensíveis?
25. A ausência de conexão não altera a autorização para recursos locais?
26. O contrato de portabilidade e exclusão é implementável sem expor detalhes internos desnecessários?
27. Os metadados exigidos permitem documentar responsabilidades de LGPD por módulo e distribuição?

### 28.7 Empacotamento e manutenção

28. Instaladores podem ser reproduzidos para os três sistemas?
29. O projeto consegue registrar inventário e licenças dos componentes?
30. O CI consegue validar dependências, contratos de dados e conformidade sem infraestrutura excessiva?
31. Uma pessoa externa consegue executar o projeto seguindo apenas a documentação?
32. As decisões arquiteturais conseguem ser alteradas sem reescrever todo o protótipo?
33. Um canal mínimo de segurança pode ser consumido sem telemetria e sem dependência de servidor único?

---

## 29. Spikes recomendados

### 29.1 Fatia vertical experimental comum

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

### 29.2 Time-box e política de interrupção

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

Ao atingir o limite sem evidência suficiente, o spike deverá ser encerrado como **inconclusivo** ou **rejeitado**. A resposta padrão não será aumentar silenciosamente o prazo, mas registrar o bloqueio, revisar a hipótese e decidir entre simplificação, alternativa tecnológica ou novo experimento explicitamente aprovado.

### 29.3 Critérios de reavaliação de Rust e Slint

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

---

## 30. Métricas iniciais

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

---

## 31. Critérios de saída da Arquitetura v1.2

Esta etapa documental será considerada concluída quando:

- cada componente inicial possuir uma camada identificada;
- responsabilidades do runtime estiverem delimitadas por matriz de classificação;
- dependências permitidas e proibidas estiverem documentadas;
- contrato interno de módulo v0 estiver definido (incluindo `execution`, `protocol` e campos de confiança/portabilidade);
- contrato de portabilidade e exclusão por módulo estiver definido;
- níveis de confiança e condições de distribuição estiverem definidos;
- ciclo de vida de módulos estiver descrito;
- persistência e migrações tiverem regras iniciais;
- backup e exportação estiverem diferenciados;
- comunicação por contratos e eventos estiver definida;
- fronteiras de segurança estiverem registradas;
- responsabilidades estruturais de atualização estiverem separadas dos canais e provedores;
- obrigações arquiteturais relacionadas à LGPD estiverem explícitas;
- regras de arquitetura verificáveis pelo CI estiverem listadas;
- fronteiras de licenciamento estiverem mapeadas;
- ADRs iniciais tiverem sido criados;
- hipóteses técnicas tiverem sido convertidas em spikes mensuráveis (incluindo Spike 10 condicionado);
- nenhuma regra específica do OpenCore Portaria estiver localizada no runtime;
- uma pessoa externa conseguir compreender como uma distribuição é composta.

---

## 32. Critérios de saída da validação técnica

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

## 33. Riscos arquiteturais iniciais

### 33.1 Complexidade prematura do sistema de plugins

**Risco:** tentar oferecer plugins binários, marketplace e ABI estável antes de compreender os contratos reais.

**Mitigação:** iniciar com registro estático e extrair o SDK após a primeira fatia vertical; validar módulos em processo apenas via protocolo (ADR-021 / Spike 10); adiar interpretador embutido e ABI dinâmica.

### 33.2 Contaminação do runtime pelo domínio de portaria

**Risco:** acelerar a primeira distribuição incorporando regras específicas ao núcleo.

**Mitigação:** revisar dependências, exigir ADR e validar uma segunda composição experimental.

### 33.3 Isolamento apenas aparente entre módulos

**Risco:** módulos separados em diretórios, mas acoplados por tabelas, tipos internos ou chamadas diretas.

**Mitigação:** contratos, propriedade de dados, testes de arquitetura e eventos.

### 33.4 Stack pouco acessível à comunidade

**Risco:** Rust e Slint apresentarem barreira de contribuição maior que o benefício obtido.

**Mitigação:** medir onboarding, documentação e produtividade durante os spikes; oferecer caminho de contribuição via módulo em processo e template educacional (ADR-021); comparar alternativas caso os resultados sejam inadequados.

### 33.4.1 Custo oculto do empacotamento multilíngue

**Risco:** bundlar ou depender de Python/Node destruir a proposta de leveza, previsibilidade de instalador ou offline-first; ou distribuir oficialmente exigindo PATH do sistema.

**Mitigação:** Spike 10 medir tamanho; PATH só para desenvolvimento; distribuições oficiais exigem interpretador empacotado ou executável autônomo; não aceitar ADR-021 sem plano de empacotamento de produto.

### 33.4.2 Falsa confiança em "sandbox por processo"

**Risco:** documentação ou marketing apresentarem isolamento de processo como garantia de que o módulo não acessa SQLite, arquivos ou rede fora do protocolo.

**Mitigação:** declaração explícita no ADR-021 e nesta arquitetura; módulos em processo apenas oficiais/verificados/confiáveis na v0; sandbox de SO em ADR futura se houver necessidade de código não confiável.

### 33.5 Portabilidade inconsistente

**Risco:** diferenças de GUI, empacotamento, segredos ou sistema de arquivos entre plataformas.

**Mitigação:** validar os três sistemas desde o primeiro spike, não apenas ao final.

### 33.6 Backup sem portabilidade

**Risco:** considerar a cópia do banco suficiente para soberania de dados.

**Mitigação:** implementar e testar backup técnico e exportação portátil como entregas distintas.

### 33.7 Licenciamento difícil de auditar

**Risco:** misturar arquivos MPL 2.0 e Apache 2.0 sem fronteiras objetivas.

**Mitigação:** organização por diretório, SPDX, inventário e verificações automatizadas.

### 33.8 Dependência excessiva do fundador

**Risco:** decisões e conhecimento permanecerem privados.

**Mitigação:** ADRs, RFCs, documentação de setup, testes e onboarding externo desde a Etapa 1.

### 33.9 Spikes desconectados ou sem limite

**Risco:** consumir semanas em protótipos isolados sem demonstrar um fluxo completo de produto.

**Mitigação:** utilizar uma única fatia vertical experimental, aplicar limite total de 20 pessoa-dias e encerrar cada spike com decisão registrada.

### 33.10 Conformidade apenas declaratória

**Risco:** mencionar LGPD, portabilidade e exclusão sem mecanismos executáveis por módulo.

**Mitigação:** contrato de dados no manifesto, testes obrigatórios de exportação e exclusão e verificação no CI.

---

## 34. Questões ainda abertas

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
- necessidade e desenho de sandbox de SO para código não confiável.

Questões abertas deverão ser transformadas em spikes, ADRs ou RFCs antes de gerar implementação estrutural definitiva.

---

## 35. Próximas entregas

Após aprovação desta arquitetura, a sequência recomendada é:

1. tratar esta Arquitetura v1.2 e os ADR-015 a ADR-021 como base documental da Etapa 0;
2. **próxima entrega:** Comunidade e Governança OpenCore v1.0 (autoridade, papéis, mantenedores, RFC/ADR, conflitos, segurança emergencial, reconhecimento, abandono de módulos);
3. em seguida: Plano Institucional OpenCore v1.0;
4. extrair três especificações normativas: `module-manifest-v0`, `data-portability-v0` e `module-trust-v0`;
5. criar a matriz executável de dependências permitidas e proibidas e a matriz `native` vs `process`;
6. definir a convenção inicial de migrações, backup, restauração e exclusão;
7. preparar a estrutura mínima do monorepo (`module_host/`, `protocol/`, `native-rust`, `process-python`, `conformance-tests`) e os templates Apache 2.0 da trilha educacional;
8. converter os Spikes 01–09 em backlog time-boxed (20 pessoa-dias) e o Spike 10 (+3 pessoa-dias); registrar Spike 11 como futuro;
9. somente após fechar a Etapa 0 documental, iniciar a fatia vertical experimental e os spikes;
10. executar o Spike 10 (módulo em processo headless, ADR-021) com evidência registrada;
11. registrar resultados mensuráveis e revisar os ADRs condicionados;
12. iniciar a fatia vertical definitiva do OpenCore Portaria apenas após a decisão documentada sobre a stack e o resultado do Spike 10.

---

## 36. Declaração arquitetural

O OpenCore será desenvolvido inicialmente como um monólito modular, local e multiplataforma, composto por um OpenCore Runtime mínimo em Rust, Module Host, módulos-base, módulos de domínio e distribuições testadas.

O runtime coordenará inicialização, configuração, módulos, dados, migrações, eventos, logs, integridade e garantias estruturais de atualização, sem incorporar regras específicas de negócio, provedores externos ou canais comerciais.

A primeira implementação deverá priorizar componentes T3 registrados estaticamente, SQLite, comunicação local, adaptadores opcionais e empacotamento nativo. Módulos em processo, comunicando-se pelo OpenCore Module Protocol, constituem hipótese oficial condicionada ao ADR-021 e ao Spike 10 — voltada a domínio, integrações e contribuição educacional, sem substituir Rust no núcleo e **sem** apresentar isolamento de processo como sandbox de segurança.

Plugins dinâmicos in-process, interpretadores embutidos, marketplace, microserviços, sandbox completa de OS e infraestrutura obrigatória de nuvem permanecerão fora do escopo até que necessidades reais justifiquem sua adoção.

A arquitetura deverá garantir que o OpenCore possa crescer sem abandonar os compromissos de soberania de dados, funcionamento offline, formatos abertos, licenciamento transparente, documentação, segurança e independência institucional definidos no Manifesto.

---

## Histórico

| Versão | Estado | Descrição |
|---|---|---|
| 1.0 | Proposta para revisão | Primeira consolidação das fronteiras arquiteturais, contrato modular, dados, eventos, segurança, licenciamento e plano de validação técnica. |
| 1.1 | Proposta revisada para aprovação | Resolve a classificação de atualização e sincronização, explicita LGPD, adiciona contrato de portabilidade e exclusão, níveis de confiança, trilha educacional, canal mínimo de segurança, testes arquiteturais e time-box dos spikes. |
| 1.2 | Proposta consolidada para aprovação | Consolida v1.1 com ADR-021 (módulos nativos/processo, protocolo, isolamento ≠ sandbox, empacotamento, storage tipado); supersede a linha divergente v1.0.1/v1.0.2. |
