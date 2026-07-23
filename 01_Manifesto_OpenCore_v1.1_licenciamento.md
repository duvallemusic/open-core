# Manifesto OpenCore — Versão 1.1

**Status:** revisado; arquitetura inicial de licenciamento definida

## 1. Por que existimos

O OpenCore existe para garantir que software profissional aberto seja acessível a organizações de qualquer porte: simples e leve o suficiente para pequenos negócios e profissionais independentes, e robusto, escalável e personalizável o suficiente para grandes instituições.

Software essencial não deveria obrigar uma organização a entregar o controle de suas operações, de seus dados ou de sua continuidade a um único fornecedor. O OpenCore busca oferecer uma base digital aberta, modular e duradoura sobre a qual diferentes sistemas profissionais possam ser construídos, adaptados e mantidos.

## 2. Para quem existimos

O OpenCore prioriza, nesta ordem:

1. usuários e organizações que dependem do software para suas atividades;
2. mantenedores responsáveis pela segurança, qualidade e continuidade do projeto;
3. desenvolvedores, estudantes e demais contribuidores que aprendem e crescem por meio do trabalho real;
4. parceiros, patrocinadores, fornecedores e instituições que apoiam o ecossistema.

Quando interesses entrarem em conflito, a proteção dos usuários, de seus dados e da continuidade do software terá precedência sobre conveniências comerciais, educacionais ou tecnológicas.

## 3. Nossa promessa

O OpenCore desenvolverá infraestrutura profissional aberta, modular, documentada e multiplataforma, capaz de atender diferentes tipos de organizações sem criar dependência obrigatória de nuvem, assinatura, fornecedor único ou serviço central.

A plataforma deverá ser utilizável fora de um contexto educacional, comercial ou institucional específico. Educação, patrocínio e serviços pagos são instrumentos para fortalecer o projeto, não condições para que o software permaneça útil.

## 4. Direitos dos usuários

Toda versão oficial aberta do OpenCore preservará o direito de qualquer pessoa ou organização de:

1. usar o software para qualquer finalidade legítima;
2. estudar, auditar e compreender como ele funciona;
3. modificar o código e redistribuir versões originais ou modificadas, nos termos da licença adotada;
4. acessar, exportar, migrar e apagar os próprios dados;
5. continuar utilizando localmente as funcionalidades instaladas sem assinatura obrigatória;
6. operar offline sempre que a funcionalidade não depender, por sua própria natureza, de um serviço externo;
7. criar ou contratar suporte sem depender da entidade central do OpenCore;
8. construir uma distribuição funcional a partir do código-fonte aberto.

O direito de modificar e redistribuir o código não dependerá de autorização prévia, negociação individual, doação obrigatória ou notificação à entidade OpenCore. Regras de autorização e contrapartida poderão existir para o uso da marca, certificação oficial, presença no marketplace, suporte institucional ou acordos comerciais, mas não para restringir as liberdades concedidas pela licença do código.

A entidade OpenCore poderá oferecer certificação oficial a empresas de suporte, consultorias, distribuidores e integradores que cumpram critérios públicos de competência, segurança, transparência e qualidade.

## 5. Soberania, privacidade e propriedade dos dados

Os dados pertencem a seus titulares e às organizações responsáveis por seu tratamento, nunca à plataforma por simples dependência tecnológica.

As distribuições oficiais deverão observar os seguintes compromissos:

- armazenamento local como padrão, salvo quando o usuário escolher outra arquitetura;
- exportação completa, documentada e legível dos dados;
- migração sem dependência obrigatória de serviços controlados pelo OpenCore;
- exclusão completa dos dados quando solicitada por quem possuir legitimidade para fazê-lo;
- backups em formatos documentados e restauráveis;
- coleta mínima de dados pessoais e técnicos;
- controles de acesso, rastreabilidade e segurança proporcionais ao risco;
- respeito à LGPD e às demais normas aplicáveis a cada implantação.

Logs técnicos locais poderão ser gerados para diagnóstico, segurança e auditoria. Esses logs continuam sob controle do usuário ou da organização que opera o sistema.

Telemetria significa a transmissão de informações técnicas sobre uso, desempenho, falhas ou ambiente para servidores externos. O envio de telemetria será desativado por padrão, salvo quando estritamente necessário a uma funcionalidade externa claramente contratada. Qualquer coleta adicional deverá ser informada, específica, proporcional, revogável e baseada em fundamento jurídico adequado. Sempre que possível, os dados deverão ser anonimizados ou pseudonimizados.

Módulos proprietários poderão utilizar formatos internos não públicos, mas não poderão impedir o usuário de acessar, exportar ou migrar os dados essenciais produzidos por sua operação. Para certificação ou distribuição pública no ecossistema OpenCore, o desenvolvedor deverá fornecer interfaces documentadas de portabilidade e informações suficientes para auditoria de segurança. Segredos industriais poderão ser protegidos por acordos de confidencialidade, sem eliminar os direitos do usuário sobre seus dados.

Serviços de sincronização, hospedagem ou integração poderão adicionar conveniência e recursos remotos, mas não poderão transformar o serviço online no único meio de acessar, recuperar ou exportar dados que também deveriam estar disponíveis localmente.

## 6. Offline-first

Offline-first significa que o núcleo, as distribuições e os módulos já instalados devem executar localmente todas as funções que não dependam necessariamente de comunicação externa.

A conexão poderá ser recomendada ou exigida para:

- baixar atualizações e correções de segurança;
- instalar novos módulos;
- acessar integrações remotas;
- sincronizar diferentes dispositivos ou unidades;
- utilizar serviços externos escolhidos pelo usuário.

A falta de conexão não poderá bloquear artificialmente o uso local de funcionalidades já instaladas, exigir reativação recorrente ou impedir o acesso aos dados locais.

## 7. Princípios de produto e arquitetura

### Simplicidade proporcional

O OpenCore deve ser simples para quem utiliza e administra o sistema, sem esconder riscos nem transferir complexidade desnecessária ao usuário. A plataforma deve funcionar em hardware modesto quando o domínio permitir, sem limitar sua evolução para cenários maiores.

### Robustez sem excesso

Escalabilidade, segurança e extensibilidade serão construídas a partir de necessidades comprovadas. O projeto evitará infraestrutura prematura, abstrações sem uso real e complexidade adotada apenas por tendência tecnológica.

### Modularidade com fronteiras claras

Uma capacidade deverá pertencer ao núcleo apenas quando:

- for necessária à inicialização, integridade ou segurança da plataforma;
- for independente de um domínio de negócio específico;
- precisar de comportamento uniforme entre distribuições;
- não puder ser implementada de forma segura e sustentável como módulo;
- ou demonstrar utilidade recorrente para a maioria das distribuições.

Necessidades específicas permanecerão em módulos-base, módulos de domínio ou distribuições próprias.

### Formatos e contratos abertos

Dados essenciais, interfaces públicas, contratos entre módulos e processos de migração deverão ser documentados. A substituição de componentes deve ser possível sem depender de conhecimento privado da entidade central.

### Documentação como parte do produto

Uma funcionalidade não estará completa sem documentação proporcional ao seu impacto. Instalação, operação, recuperação, contribuição, atualização e migração deverão ser compreensíveis sem depender de explicações privadas dos criadores.

## 8. Comunidade e educação

A tecnologia serve às pessoas, mas a abertura à participação não reduz os padrões de segurança, qualidade, acessibilidade, testes, documentação e manutenção.

A educação é um meio estratégico do OpenCore. O projeto busca preencher o espaço entre projetos genéricos reproduzidos apenas para fins didáticos e a experiência real de desenvolvimento encontrada em equipes profissionais.

Estudantes e novos contribuidores deverão trabalhar em problemas reais, com requisitos, revisão, testes, documentação, responsabilidade e impacto verificável. O projeto organizará tarefas de diferentes níveis, mentorias, decisões arquiteturais públicas e caminhos para formação de mantenedores.

Contribuições não relacionadas a código — como documentação, design, testes, pesquisa, tradução, segurança, suporte e organização comunitária — deverão ser reconhecidas como parte legítima da construção do projeto.

## 9. Licenciamento, núcleo aberto e edições comerciais

O OpenCore adotará uma arquitetura inicial de licenciamento por camadas, buscando proteger o patrimônio técnico compartilhado sem criar barreiras desnecessárias para integrações, ferramentas, adoção empresarial e desenvolvimento por terceiros.

### Núcleo e módulos oficiais — Mozilla Public License 2.0

Serão licenciados sob a **Mozilla Public License 2.0 (MPL 2.0)**:

- o runtime e o kernel do OpenCore;
- o sistema de carregamento, registro e ciclo de vida de módulos;
- os componentes estruturais responsáveis por configuração, eventos, persistência, migrações, segurança, integridade, logs e atualização;
- os módulos-base oficiais;
- os módulos de domínio mantidos ou incorporados oficialmente ao projeto;
- outros componentes que a governança classificar como parte do patrimônio técnico central.

A MPL 2.0 preserva o núcleo como bem comum no nível dos arquivos cobertos. Quando esses arquivos forem modificados e distribuídos, as versões modificadas deverão continuar disponíveis sob os termos da MPL 2.0.

Arquivos e módulos separados poderão utilizar outras licenças compatíveis. Essa separação permite a criação de integrações, extensões e edições comerciais sem autorizar o fechamento de modificações distribuídas diretamente nos arquivos oficiais cobertos pela MPL 2.0.

### SDK, integrações e ferramentas — Apache License 2.0

Serão licenciados sob a **Apache License 2.0**:

- o SDK público destinado à criação de módulos e integrações;
- bibliotecas cliente, bindings e adaptadores oficiais;
- interfaces e ferramentas de interoperabilidade;
- templates e scaffolds para novos projetos;
- exemplos de implementação;
- ferramentas auxiliares que não integrem o núcleo protegido;
- materiais de código voltados a ensino, experimentação e adoção institucional, quando não forem incorporados ao núcleo ou a módulos oficiais.

Essa camada permissiva busca facilitar adoção por empresas, universidades, plataformas de ensino e desenvolvedores independentes, inclusive em produtos que contenham componentes proprietários separados.

A classificação de um novo componente como MPL 2.0 ou Apache 2.0 deverá considerar sua função arquitetural, seu impacto sobre a independência da plataforma e sua necessidade de reutilização externa. A licença aplicável deverá estar claramente identificada no repositório, no diretório e, quando apropriado, nos próprios arquivos.

### Contribuições e distribuição combinada

Contribuições aceitas em um componente seguirão a licença já atribuída àquele componente. A política de contribuição deverá informar essa condição antes do envio de código.

Distribuições oficiais poderão combinar componentes MPL 2.0 e Apache 2.0, desde que permaneçam integralmente open source e preservem:

- os textos das licenças aplicáveis;
- avisos de copyright e atribuição;
- a identificação dos componentes e de suas respectivas licenças;
- o acesso ao código-fonte exigido pela MPL 2.0;
- as informações necessárias para reconstrução, auditoria e conformidade da distribuição.

A organização dos repositórios, dependências e pipelines deverá tornar as fronteiras de licenciamento objetivas e auditáveis. O projeto utilizará inventário de componentes, identificadores padronizados de licença e verificações automatizadas sempre que tecnicamente viável.

### Módulos proprietários e edições comerciais

Módulos proprietários serão permitidos em edições comerciais separadas ou como componentes opcionais claramente identificados. Eles não poderão substituir por completo funcionalidades essenciais necessárias para que uma distribuição oficial aberta continue útil e funcional.

Edições comerciais deverão:

- identificar claramente quais componentes são abertos e quais são proprietários;
- preservar as liberdades e direitos concedidos pelo núcleo e pelos demais componentes abertos;
- cumprir as obrigações da MPL 2.0, da Apache 2.0 e das demais licenças incorporadas;
- manter portabilidade e acesso aos dados;
- respeitar os requisitos de segurança, privacidade e interoperabilidade aplicáveis;
- não utilizar a marca oficial sem autorização ou certificação;
- não se apresentar como distribuição oficial aberta quando contiver componentes fechados.

A criação e comercialização de forks e distribuições independentes não dependerá de autorização da entidade OpenCore. Aprovação institucional será exigida apenas para uso da marca, participação em canais oficiais, obtenção de certificação ou apresentação da solução como reconhecida pela OpenCore.

## 10. Marca, certificação e distribuições oficiais

O código aberto poderá ser usado para criar forks e distribuições independentes nos termos da licença. Entretanto, o uso do nome, logotipo, selos e expressões que indiquem aprovação oficial dependerá da política de marca e de autorização da entidade OpenCore.

Uma distribuição será considerada oficial somente quando:

- for integralmente aberta;
- atender aos requisitos públicos de segurança, privacidade e qualidade;
- possuir utilidade e justificativa claras em relação às alternativas existentes;
- demonstrar manutenção responsável;
- preservar compatibilidade, documentação, portabilidade e funcionamento offline;
- passar por avaliação técnica e institucional independente;
- aceitar auditorias e processo público de revisão ou contestação.

Edições comerciais poderão receber certificação própria, distinta do status de distribuição oficial aberta.

Contrapartidas financeiras, técnicas ou comunitárias poderão ser negociadas para certificação, uso comercial da marca, suporte institucional, marketplace ou parcerias. Essas contrapartidas não serão condição para exercer os direitos concedidos pela licença do código.

## 11. Durabilidade

Durabilidade não significa apenas manter um repositório disponível. Significa permitir que software, dados, conhecimento e responsabilidades sobrevivam a mudanças de tecnologia, mercado, patrocinadores e lideranças.

O OpenCore buscará durabilidade em quatro dimensões.

### Durabilidade técnica

- formatos de dados documentados e migráveis;
- backups verificáveis e restauráveis;
- contratos estáveis entre módulos;
- versionamento e políticas de depreciação;
- migrações seguras;
- possibilidade de continuar utilizando versões locais;
- redução de dependências insubstituíveis;
- documentação das decisões arquiteturais;
- políticas de suporte definidas para cada versão.

### Durabilidade comunitária

- entrada contínua de novos contribuidores;
- formação progressiva de revisores e mantenedores;
- transferência documentada de conhecimento;
- responsabilidades distribuídas;
- sucessão de lideranças;
- reconhecimento de diferentes formas de contribuição;
- redução da dependência de qualquer indivíduo.

### Durabilidade institucional

- entidade independente;
- regras públicas de governança;
- transparência financeira e decisória;
- diversidade de fontes de financiamento;
- gestão de conflitos de interesse;
- proteção da marca e dos bens comuns do projeto;
- impossibilidade de captura do núcleo por um único patrocinador.

### Durabilidade pelo uso múltiplo

O núcleo não será subordinado a apenas um produto, empresa ou segmento. Sua validade deverá ser demonstrada pela capacidade de sustentar diferentes distribuições, módulos e tipos de organização.

## 12. Governança e pertencimento

O OpenCore deverá ser protegido por uma entidade independente, preferencialmente uma fundação ou estrutura equivalente, regida por uma constituição pública.

A entidade será responsável pela marca, bens institucionais, políticas de certificação e proteção da missão. Sua governança deverá ser democrática, transparente e baseada em responsabilidades comprovadas.

O conselho deverá possuir composição mista, com representação equilibrada de:

- fundadores;
- mantenedores e contribuidores técnicos;
- organizações e usuários da plataforma;
- apoiadores e patrocinadores.

Nenhum grupo poderá controlar sozinho as decisões fundamentais. Patrocinadores deverão declarar conflitos de interesse e não poderão adquirir poder permanente apenas pelo volume de financiamento.

Mudanças relevantes deverão ser documentadas, submetidas à consulta da comunidade e decididas por processos definidos na constituição. Decisões de segurança emergencial poderão seguir rito acelerado, com justificativa e revisão pública posterior.

## 13. Parceiros e patrocinadores

Parceiros poderão financiar funcionalidades, desafios, infraestrutura e mantenedores. Prioridades poderão considerar o tamanho da base beneficiada, a urgência, o impacto público, o risco técnico e o nível de apoio oferecido.

O financiamento, isoladamente, não garante aprovação automática nem controle sobre o roadmap.

Patrocinadores:

- não poderão impedir funcionalidades apenas por interesse comercial;
- poderão solicitar revisão quando houver risco demonstrável à segurança, privacidade, conformidade ou continuidade operacional;
- poderão receber acesso antecipado quando houver interesse mútuo, critérios transparentes e preservação do lançamento público acordado;
- não poderão exigir exclusividade sobre o núcleo ou sobre capacidades essenciais e gerais;
- poderão negociar exclusividade limitada sobre serviços, integrações ou entregas comerciais específicas, desde que isso não restrinja as liberdades do software aberto;
- não poderão revogar ou reduzir retroativamente direitos já concedidos por versões publicadas;
- deverão ter suas decisões financiadas e potenciais conflitos de interesse divulgados.

Mudanças futuras na arquitetura de licenciamento — incluindo a reclassificação de componentes entre MPL 2.0 e Apache 2.0 — deverão seguir o processo público de governança, apresentar análise de impacto e respeitar os direitos autorais envolvidos. Contratos privados poderão reger edições comerciais, serviços ou componentes separados, mas não reescrever silenciosamente as licenças de componentes já distribuídos.

## 14. Sustentabilidade sem aprisionamento

O OpenCore poderá ser sustentado por patrocínio, doações, consultoria, suporte, treinamento, certificação, desenvolvimento sob encomenda, hospedagem opcional, sincronização gerenciada e outros serviços compatíveis com sua missão.

Nenhum modelo de receita poderá depender de:

- bloquear o acesso do usuário aos próprios dados;
- cobrar para permitir exportação, backup ou migração;
- exigir assinatura para continuar usando recursos locais já instalados;
- tornar a nuvem obrigatória sem necessidade técnica real;
- retirar do núcleo aberto capacidades essenciais apenas para revendê-las;
- esconder coleta de dados ou telemetria;
- negar correções essenciais de segurança de uma versão oficial ainda suportada apenas para forçar uma compra;
- criar incompatibilidades deliberadas para impedir alternativas de suporte ou integração.

Sustentabilidade significa financiar a continuidade do projeto sem transformar dependência em produto.

## 15. O que não construiremos

O OpenCore rejeita:

- lock-in tecnológico, comercial ou de dados;
- dependência obrigatória de nuvem;
- ativação remota recorrente para uso local;
- telemetria oculta ou coleta desproporcional;
- complexidade sem necessidade comprovada;
- crescimento sem governança;
- distribuições oficiais abertas que dependam de componentes proprietários essenciais;
- captura do roadmap por patrocinadores;
- redução dos padrões profissionais para facilitar contribuições;
- dependência permanente de uma única empresa, instituição ou pessoa.

## 16. Evolução deste manifesto

Este manifesto poderá evoluir, mas não por decisão privada ou silenciosa.

Alterações deverão incluir:

1. proposta pública;
2. justificativa e análise de impacto;
3. período de consulta da comunidade;
4. avaliação de conflitos de interesse;
5. aprovação pelo processo qualificado definido na constituição da entidade;
6. registro permanente da decisão e das posições divergentes relevantes.

Nenhuma alteração poderá retirar retroativamente direitos já concedidos aos usuários por licenças de versões publicadas. A substituição da MPL 2.0 ou da Apache 2.0 em componentes futuros exigirá proposta específica, análise jurídica, avaliação de compatibilidade, consulta comunitária e aprovação qualificada conforme a constituição da entidade.

---

## Declaração final

O OpenCore existe para tornar software profissional aberto uma opção real para organizações de qualquer tamanho.

Sua força deverá vir da combinação entre liberdade, simplicidade, robustez, dados sob controle do usuário, arquitetura modular, educação baseada em trabalho real, governança independente e sustentabilidade sem aprisionamento.

O projeto será considerado bem-sucedido quando organizações puderem depender dele sem perder autonomia, desenvolvedores puderem crescer contribuindo para software verdadeiro e nenhuma empresa, patrocinador, instituição ou fundador puder transformar essa dependência em controle unilateral.
