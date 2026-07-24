# Comunidade e Governança OpenCore — Versão 1.0

**Status:** Aprovado  
**Data de aprovação:** 2026-07-23  
**Data:** 2026-07-23  
**Última atualização editorial:** 2026-07-24 — acréscimo da relação com o OpenCore Builder, papéis de distribuição/formação, ownership explícito, alinhamento ADR-017 e formas de contribuição faltantes (sem alterar o status Aprovado).  
**Base normativa:** Manifesto OpenCore v1.1 (evoluções posteriores aplicam-se quando canônicas)  
**Documentos relacionados:** Arquitetura OpenCore · Roadmap OpenCore · Plano Institucional · ADR-015..021 · ADR-017 (níveis de confiança) · ADR-022 / Especificação Builder (quando existirem)  
**Finalidade:** definir como pessoas participam, como responsabilidades são conquistadas, como decisões são tomadas e como o projeto protege sua missão durante a formação da comunidade.  
**Supersede:** `03_Comunidade_OpenCore_rascunho.md` — rascunho inicial.

---

## 1. Objetivo

O OpenCore deve funcionar simultaneamente como:

1. infraestrutura profissional aberta para organizações reais;
2. comunidade de desenvolvimento responsável;
3. ambiente de aprendizado baseado em trabalho real;
4. patrimônio técnico duradouro, que não dependa permanentemente de uma única pessoa, empresa ou instituição.

Esta política transforma esses compromissos em regras operacionais para:

- participação e reconhecimento;
- revisão e aprovação de contribuições;
- formação, atuação e remoção de mantenedores;
- decisões técnicas, comunitárias e institucionais;
- RFCs, ADRs e mudanças no roadmap;
- segurança, conflitos e aplicação do Código de Conduta;
- propriedade e continuidade dos módulos;
- transparência e prevenção de captura do projeto.

A governança deve ser proporcional à maturidade do OpenCore. O projeto não criará comitês sem função real, mas também não deixará poder, responsabilidade ou conflitos sem regras públicas.

---

## 2. Princípios de governança

### 2.1 Usuários e continuidade em primeiro lugar

Quando houver conflito entre conveniência técnica, interesse educacional, financiamento, velocidade de entrega e proteção dos usuários, terão prioridade:

1. segurança e integridade dos dados;
2. continuidade das versões suportadas;
3. soberania, portabilidade e acesso aos dados;
4. manutenção da utilidade das distribuições abertas;
5. transparência das decisões.

### 2.2 Autoridade baseada em responsabilidade comprovada

Influência no projeto será conquistada por contribuições consistentes, revisão responsável, conhecimento demonstrado, cuidado com usuários e capacidade de cooperar.

Popularidade, vínculo institucional, cargo externo, volume de financiamento ou quantidade isolada de commits não concedem autoridade permanente.

### 2.3 Decisões públicas por padrão

Decisões técnicas, comunitárias e institucionais relevantes deverão ocorrer em canais públicos e permanecer registradas.

Conversas privadas podem ser usadas para:

- relatos de segurança;
- aplicação do Código de Conduta;
- dados pessoais ou informações legalmente protegidas;
- negociações comerciais ainda confidenciais;
- preparação operacional que não substitua a decisão pública necessária.

### 2.4 Educação sem redução de padrões

Contribuidores iniciantes receberão documentação, tarefas graduadas, exemplos, mentoria e feedback proporcional ao nível de experiência.

Os requisitos de qualidade, segurança, testes, documentação, licenciamento e revisão não serão removidos para facilitar aprovação. O apoio deve permitir que a pessoa alcance o padrão, não eliminar o padrão.

### 2.5 Independência e diversidade de poder

Nenhum patrocinador, parceiro educacional, empresa, fundador, mantenedor ou grupo de contribuidores deverá controlar sozinho e permanentemente as decisões fundamentais do OpenCore.

Durante a fase inicial, haverá autoridade fundadora transitória para impedir paralisia. Essa autoridade deverá diminuir conforme responsabilidades reais forem distribuídas.

### 2.6 Simplicidade institucional

O projeto adotará apenas estruturas de governança necessárias ao seu estágio atual.

A existência futura de conselho, fundação, associação ou entidade equivalente não justifica criar burocracia prematura antes de existirem comunidade ativa, ativos, contratos, receitas, marca ou responsabilidades que exijam essa estrutura.

---

## 3. Estágios de governança

### 3.1 Estágio F — Fundação

O OpenCore inicia no **Estágio F**, enquanto houver menos de três mantenedores ativos e ainda não existir comunidade recorrente suficiente para decisões colegiadas sustentáveis.

Neste estágio:

- existe um **Lead Maintainer interino**, inicialmente exercido pelo fundador;
- decisões relevantes continuam públicas e sujeitas a consulta;
- o Lead Maintainer pode desempatar ou decidir quando não houver outro responsável habilitado;
- exceções à revisão independente devem ser registradas;
- nenhuma autoridade fundadora é considerada irrevogável ou hereditária;
- patrocinadores e parceiros não possuem voto automático.

O Estágio F evita simular uma democracia sem participantes e, ao mesmo tempo, impede que decisões importantes sejam privadas ou inexplicáveis.

### 3.2 Estágio C — Governança compartilhada

A transição para o **Estágio C** deverá ser proposta por RFC quando existirem, no mínimo:

- três mantenedores ativos;
- responsabilidades distribuídas em pelo menos duas áreas ou módulos;
- contribuições externas recorrentes por pelo menos seis meses;
- processo de revisão funcionando sem dependência constante do fundador;
- registros públicos suficientes para avaliar participação e conflitos de interesse.

No Estágio C será criado um **Conselho de Mantenedores**, inicialmente composto por três a sete pessoas.

O Conselho de Mantenedores será responsável por:

- decisões técnicas e comunitárias de impacto amplo;
- nomeação e remoção de mantenedores;
- resolução de impasses;
- aprovação de releases e políticas de suporte;
- supervisão dos processos de segurança e conduta;
- preparação da transição institucional.

Nenhuma organização poderá ocupar mais de um terço dos assentos do Conselho.

### 3.3 Estágio I — Entidade independente

A criação de fundação, associação ou estrutura equivalente deverá ser avaliada quando o OpenCore passar a administrar de forma recorrente um ou mais dos seguintes elementos:

- marca e certificações;
- contratos institucionais;
- patrocínios ou receitas significativas;
- infraestrutura paga e ativos compartilhados;
- contratação ou remuneração de mantenedores;
- responsabilidade jurídica por eventos, programas educacionais ou distribuição de software.

A criação da entidade exigirá RFC institucional, consulta pública, análise jurídica e constituição própria.

Seu órgão institucional deverá buscar representação equilibrada de:

- fundadores;
- mantenedores e contribuidores técnicos;
- organizações e usuários da plataforma;
- apoiadores e patrocinadores.

Nenhuma categoria ou organização poderá controlar sozinha as decisões fundamentais. A constituição deverá estabelecer mandatos, impedimentos, prestação de contas, transparência financeira e proteção contra captura.

A entidade não substituirá automaticamente a governança técnica. Suas competências sobre marca, finanças e contratos deverão permanecer separadas das decisões sobre arquitetura e aceitação de código, salvo quando houver obrigação legal, risco de segurança ou conflito de missão claramente demonstrado.

---

## 4. Papéis da comunidade

Uma pessoa pode exercer mais de um papel, desde que declare conflitos de interesse e não concentre sozinha todas as etapas de decisões sensíveis.

### 4.1 Usuário

Pessoa ou organização que utiliza, avalia ou depende de uma distribuição OpenCore.

Pode:

- relatar problemas;
- propor melhorias;
- participar de consultas;
- fornecer casos de uso e evidências de impacto;
- contestar decisões que afetem direitos, dados ou continuidade operacional.

### 4.2 Contribuidor

Pessoa que realiza qualquer contribuição aceita. Formas reconhecidas incluem, sem exclusividade:

- código;
- testes;
- documentação;
- tradução;
- design;
- acessibilidade;
- segurança;
- pesquisa;
- suporte;
- triagem de issues e relatos;
- mentoria;
- gestão comunitária;
- validação com usuários;
- criação de dados de demonstração;
- contribuições ao OpenCore Builder (perfis, textos de triagem, catálogo de capacidades, templates), conforme a seção 19.

Não existe hierarquia de valor que torne código automaticamente superior às demais contribuições.

### 4.3 Contribuidor recorrente

Contribuidor que demonstra participação contínua, compreensão das regras e capacidade de concluir trabalho com responsabilidade.

Pode receber:

- atribuição prioritária de issues;
- participação em planejamento de módulos;
- acesso a mentorias avançadas;
- indicação para o papel de revisor.

### 4.4 Revisor

Pessoa autorizada a revisar contribuições em um escopo definido, sem necessariamente possuir permissão de merge.

Responsabilidades:

- verificar clareza, testes, documentação, arquitetura e licenciamento;
- explicar solicitações de mudança;
- diferenciar bloqueios obrigatórios de sugestões opcionais;
- evitar aprovação automática ou revisão hostil;
- declarar quando não possui conhecimento suficiente para aprovar uma parte da mudança.

### 4.5 Mantenedor em formação

Pessoa em período acompanhado (tipicamente o probatório de noventa dias ou trilha `maintainer-track`) com escopo limitado de merge ou coordenação.

Responsabilidades:

- exercer progressivamente as atribuições de mantenedor sob mentoria;
- documentar decisões e transferir conhecimento;
- declarar disponibilidade e conflitos;
- não atuar sozinha em mudanças de segurança, runtime ou distribuição oficial sem co-revisão.

A conclusão bem-sucedida do período, com evidências públicas, habilita nomeação a mantenedor pleno conforme a seção 5.

### 4.6 Mantenedor de módulo ou área

Responsável por um módulo, conjunto de documentos ou área técnica específica.

Pode:

- aprovar e integrar mudanças dentro de seu escopo;
- organizar backlog e releases do módulo;
- representar o módulo em RFCs;
- classificar riscos e necessidades de manutenção;
- indicar novos revisores.

Não pode:

- alterar unilateralmente contratos globais do runtime;
- ignorar ADRs e políticas do projeto;
- tratar o módulo oficial como propriedade pessoal;
- impedir sucessão ou revisão externa legítima.

### 4.7 Responsável por distribuição

Responsável por uma distribuição oficial ou candidata (composição testada de runtime, módulos e perfil).

Pode:

- coordenar manifesto e lockfile da distribuição;
- organizar matriz de compatibilidade e testes de instalação/onboarding;
- representar a distribuição em RFCs e releases;
- indicar inclusão ou remoção de módulos na composição, sujeito a arquitetura e segurança;
- acompanhar estado de manutenção dos módulos incluídos.

Não pode:

- incluir módulo T0/experimental em distribuição oficial;
- alterar unilateralmente contratos do OpenCore Runtime;
- apresentar combinação não verificada como oficial;
- condicionar a distribuição a serviços comerciais exclusivos.

Mudanças em distribuições oficiais exigem o responsável da distribuição **e** revisão de arquitetura e segurança, conforme a seção 7.

### 4.8 Mantenedor do núcleo

Responsável por componentes estruturais, contratos globais, arquitetura ou releases oficiais.

Mudanças no runtime, segurança estrutural, persistência, migrações, protocolo de módulos, compatibilidade e empacotamento exigem revisão de mantenedor habilitado nessas áreas.

### 4.9 Lead Maintainer

Responsável por coordenação geral durante o Estágio F ou quando designado pelo Conselho.

Competências:

- garantir que decisões possuam responsável e conclusão;
- resolver impasses no Estágio F;
- representar publicamente o estado técnico do projeto;
- coordenar releases e resposta a incidentes quando não houver responsável específico;
- iniciar processos de sucessão e distribuição de autoridade.

O Lead Maintainer não possui direito de propriedade sobre contribuições aceitas, módulos oficiais, marca futura ou decisões da comunidade.

### 4.10 Release Manager

Responsável temporário por uma versão específica.

Deverá confirmar:

- aprovação dos responsáveis técnicos;
- resultados de CI e testes requeridos;
- changelog;
- licenças e inventário de componentes;
- migrações, backup e plano de recuperação;
- artefatos e assinaturas quando aplicáveis;
- documentação de instalação e atualização.

### 4.11 Grupo de Resposta de Segurança

Grupo restrito responsável por receber relatos privados, coordenar correções e preparar divulgação responsável.

Enquanto não houver ao menos três pessoas qualificadas e confiáveis, essa função será exercida pelo Lead Maintainer com apoio técnico convocado quando necessário.

Membros do grupo devem possuir acesso mínimo, dever de confidencialidade e obrigação de declarar conflitos.

### 4.12 Conselho de Mantenedores

Órgão do Estágio C, sem existência obrigatória no Estágio F.

Mandato, composição, rotação, quórum e processo eleitoral serão definidos na RFC de transição, respeitando os limites desta política.

---

## 5. Conquista, revisão e perda de responsabilidades

### 5.0 Ladder de progressão

A progressão típica de responsabilidade é:

```text
Primeira contribuição
→ colaborador recorrente
→ responsável por componente (quando aplicável)
→ revisor
→ mantenedor em formação
→ mantenedor
```

Caminhos paralelos válidos incluem: responsável por distribuição, mantenedor de segurança, mentoria e contribuições ao Builder. Progressão lateral (ex.: documentação → revisor de docs) é incentivada.

Promoção deve considerar qualidade, constância, conhecimento, comportamento, capacidade de revisão, documentação, responsabilidade, segurança e ausência de conflito grave. **Não** basear poder apenas em volume de commits ou financiamento.

### 5.1 Critérios gerais

Papéis com autoridade serão concedidos por evidências, não apenas por tempo ou quantidade de contribuições.

Serão avaliados:

- qualidade e continuidade das contribuições;
- capacidade de revisar trabalho de outras pessoas;
- domínio do escopo assumido;
- respeito ao Código de Conduta;
- clareza de comunicação;
- responsabilidade com segurança e usuários;
- documentação e transferência de conhecimento;
- habilidade para reconhecer limites e pedir revisão especializada.

### 5.2 Nomeação de revisores

Um contribuidor recorrente poderá ser indicado por um mantenedor.

A indicação deverá informar:

- escopo de revisão;
- evidências de preparação;
- período inicial de acompanhamento;
- possíveis conflitos de interesse.

No Estágio F, a nomeação é decidida pelo Lead Maintainer após consulta pública mínima de sete dias. No Estágio C, seguirá decisão do Conselho ou processo delegado por ele.

### 5.3 Nomeação de mantenedores

A nomeação exige:

- indicação pública por mantenedor existente;
- histórico de contribuições e revisões relevantes;
- manifestação explícita de disponibilidade e responsabilidades aceitas;
- consulta pública mínima de quatorze dias;
- ausência de violações graves não resolvidas;
- aprovação pelo processo vigente no estágio de governança.

A primeira nomeação poderá ter período probatório de noventa dias, com escopo limitado e acompanhamento documentado.

### 5.4 Inatividade

Inatividade não é punição e não apaga o reconhecimento histórico.

Um mantenedor que permaneça sem atividade ou resposta por noventa dias deverá ter sua disponibilidade confirmada. Após cento e oitenta dias sem atuação e sem acordo de afastamento, suas permissões operacionais poderão ser suspensas e o escopo redistribuído.

Licenças temporárias, saúde, cuidado familiar, trabalho e outras circunstâncias legítimas deverão ser tratados com flexibilidade e privacidade.

### 5.5 Remoção

Um papel poderá ser removido por:

- abandono prolongado sem comunicação;
- uso indevido de acesso;
- violação grave ou reiterada do Código de Conduta;
- conflito de interesse ocultado;
- quebra deliberada de segurança ou integridade;
- bloqueio abusivo de contribuições ou sucessão;
- descumprimento reiterado das responsabilidades assumidas.

A remoção deverá garantir notificação, oportunidade razoável de resposta e registro da decisão, preservando informações pessoais e de segurança.

Acesso poderá ser suspenso imediatamente quando houver risco concreto, com revisão posterior obrigatória.

---

## 6. Modelo de decisão

### 6.1 Busca de consenso

O OpenCore prefere consenso fundamentado, mas consenso não significa unanimidade nem discussão sem prazo.

O responsável pela decisão deverá:

1. identificar a questão;
2. reunir alternativas e evidências;
3. registrar objeções relevantes;
4. determinar quem possui autoridade no escopo;
5. decidir dentro do processo aplicável;
6. documentar resultado, justificativa e consequências.

### 6.2 Decisões rotineiras

Incluem correções, documentação, testes, manutenção e mudanças locais que não alterem contratos públicos, direitos dos usuários ou arquitetura compartilhada.

Podem ser aprovadas por mantenedor do escopo, respeitando revisão e CI aplicáveis.

### 6.3 Decisões significativas

Exigem RFC quando envolverem, entre outros casos:

- entrada ou remoção de capacidade do runtime;
- novo contrato público ou mudança incompatível;
- nova classe de módulo, protocolo ou mecanismo de execução;
- alteração relevante de persistência, migração, backup ou exportação;
- mudança de sistemas operacionais oficialmente suportados;
- coleta de dados, telemetria ou integração externa padrão;
- nova distribuição oficial;
- política de suporte, compatibilidade ou depreciação;
- critérios de confiança, certificação ou distribuição de módulos;
- mudança relevante no processo de contribuição;
- parceria que possa influenciar roadmap, marca ou independência;
- despesas, patrocínios ou compromissos institucionais relevantes.

### 6.4 Decisões fundamentais

São decisões que alteram:

- Manifesto;
- licenças do núcleo, SDKs ou componentes futuros;
- direitos dos usuários;
- estrutura de governança;
- controle de marca e certificação;
- criação ou dissolução de entidade independente;
- princípios de independência e sustentabilidade.

No Estágio F, exigem:

- RFC pública;
- consulta mínima de trinta dias;
- análise de impacto e conflitos de interesse;
- decisão motivada do Lead Maintainer;
- registro das objeções relevantes.

No Estágio C, exigirão aprovação qualificada mínima de dois terços do Conselho, além da consulta pública.

Nenhuma decisão poderá retirar retroativamente direitos já concedidos por versões publicadas.

### 6.5 Situações emergenciais

Falhas críticas de segurança, corrupção de dados, comprometimento de infraestrutura ou risco jurídico imediato podem exigir ação antes da consulta normal.

A autoridade responsável poderá:

- suspender releases ou downloads;
- revogar credenciais;
- aplicar correção temporária;
- ocultar detalhes exploráveis;
- reverter mudança perigosa.

A ação deverá ser limitada ao necessário e receber registro público ou relatório pós-incidente assim que a divulgação for segura, preferencialmente em até sete dias após a estabilização.

---

## 7. Pull requests e revisão de mudanças

### 7.1 Requisitos gerais

Toda pull request deverá, quando aplicável:

- possuir objetivo e escopo claros;
- referenciar issue, RFC ou ADR relacionada;
- declarar impacto sobre módulos, dados e compatibilidade;
- incluir testes proporcionais ao risco;
- atualizar documentação e changelog;
- identificar mudanças de licença ou dependências;
- passar pelos checks obrigatórios;
- resolver comentários bloqueantes antes do merge.

### 7.2 Aprovação

Como regra geral:

- mudanças rotineiras exigem uma aprovação de pessoa habilitada no escopo;
- mudanças no OpenCore Runtime, segurança, migrações, protocolo ou releases exigem ao menos uma aprovação independente de mantenedor qualificado;
- mudanças que implementem RFC deverão demonstrar conformidade com a decisão aprovada;
- o autor não deverá ser a única pessoa a revisar mudança sensível de sua autoria;
- mudanças em distribuições oficiais exigem responsável por distribuição + arquitetura + segurança;
- inclusão ou promoção no catálogo público exige checklist do nível de confiança (ADR-017) e revisão habilitada;
- mudanças que afetem recomendações do OpenCore Builder exigem revisão de produto e compatibilidade (seção 19), não apenas revisão editorial.

### 7.3 Exceção fundadora

Enquanto não existir um segundo mantenedor qualificado, exigir aprovação independente para toda mudança bloquearia o projeto.

No Estágio F, o Lead Maintainer poderá integrar a própria mudança desde que:

- todos os checks aplicáveis estejam concluídos;
- a justificativa e os riscos estejam documentados;
- não existam objeções bloqueantes sem resposta;
- mudanças significativas tenham seguido RFC;
- seja aplicado período mínimo de quarenta e oito horas entre abertura e merge para mudanças sensíveis, salvo emergência.

A exceção deverá deixar de ser usada assim que houver revisor qualificado e disponível.

### 7.4 Mudanças triviais

Correções ortográficas, links, formatação e manutenção sem efeito normativo podem seguir processo simplificado, desde que não alterem significado, código executável ou obrigações do projeto.

### 7.5 Reversão

Mudanças poderão ser revertidas quando causarem falha, risco, incompatibilidade não prevista ou violação de decisão vigente.

A reversão emergencial não encerra a discussão: deverá ser seguida de análise, issue ou ADR quando o problema revelar decisão arquitetural relevante.

---

## 8. Processo de RFC e ADR

### 8.1 RFC

RFC é uma proposta pública para mudança relevante ainda não decidida.

Cada RFC deverá conter:

- contexto e problema;
- objetivos e não objetivos;
- proposta;
- alternativas consideradas;
- impactos técnicos, comunitários, institucionais e de segurança;
- compatibilidade e migração;
- implicações de dados e licenciamento;
- plano de implementação;
- critérios de sucesso e possibilidade de reversão;
- conflitos de interesse conhecidos.

Estados recomendados:

```text
Rascunho → Em consulta → Aceita | Rejeitada | Adiada | Retirada → Implementada
```

Consultas terão prazo padrão de:

- sete dias para mudanças significativas de escopo limitado;
- quatorze dias para arquitetura ou política ampla;
- trinta dias para decisões fundamentais.

O prazo poderá ser ampliado quando houver impacto relevante ou participação insuficiente.

### 8.2 ADR

ADR registra uma decisão arquitetural aceita, seu contexto, alternativas, consequências e obrigações.

Uma ADR:

- não substitui consulta pública quando RFC for obrigatória;
- deve referenciar a RFC, issue ou evidência que originou a decisão;
- não deve ser alterada silenciosamente após aceita;
- pode ser supersedida por nova ADR, preservando histórico;
- pode permanecer proposta quando condicionada a spike ou evidência futura.

### 8.3 Autoridade de aceitação

No Estágio F, o Lead Maintainer aceita ou rejeita RFCs após consulta, devendo justificar a decisão.

No Estágio C, RFCs serão decididas pelo Conselho ou pelo conjunto de mantenedores formalmente delegado para o tema.

A decisão não será determinada apenas por contagem de comentários ou reações.

---

## 9. Roadmap e priorização

O roadmap será público e orientado por evidências.

Prioridades considerarão:

1. segurança, integridade e continuidade;
2. impacto sobre usuários e organizações;
3. alinhamento ao Manifesto;
4. necessidade para validar a plataforma;
5. reutilização entre distribuições;
6. redução de risco técnico;
7. capacidade real de manutenção;
8. valor educacional e comunitário;
9. recursos e financiamento disponíveis.

Financiamento pode aumentar a viabilidade de uma proposta, mas não substitui revisão técnica nem garante prioridade ou aceitação.

Itens patrocinados deverão identificar:

- patrocinador;
- escopo financiado;
- condições e conflitos de interesse;
- responsável técnico;
- impacto sobre roadmap e manutenção futura;
- compromisso de disponibilização pública quando aplicável.

---

## 10. Propriedade e continuidade de módulos

### 10.1 Responsabilidade compartilhada e ownership

Módulos oficiais pertencem ao ecossistema OpenCore nos termos de suas licenças e governança. Mantenedores são responsáveis temporários, não proprietários exclusivos.

Cada módulo (e, quando aplicável, cada distribuição oficial) deverá declarar publicamente:

- **mantenedor principal;**
- **substituto** (ou status explícito de ausência temporária, com plano de cobertura);
- **canais de contato** públicos e, se necessário, privados de segurança;
- **status** de manutenção;
- **prazo de resposta** esperado (ex.: reconhecimento em X dias úteis);
- **política de sucessão** ou referência ao processo de abandono desta seção.

Sempre que possível, um módulo oficial deverá possuir também:

- ao menos dois revisores capazes de compreender seu funcionamento;
- documentação de arquitetura e operação;
- testes e responsáveis identificados;
- processo de release e suporte;
- inventário de dependências e licença;
- plano de transferência ou descontinuação.

### 10.2 Estado de manutenção

Módulos deverão declarar um dos estados (equivalências aceitas no catálogo):

- **mantido / ativo:** mantido e elegível para uso conforme documentação;
- **manutenção limitada:** recebe correções prioritárias, sem novas funcionalidades;
- **procurando mantenedor / em adoção:** sem responsável suficiente, aberto a novos mantenedores;
- **órfão:** sem mantenedor após chamada pública sem sucesso, ainda não arquivado;
- **depreciado / substituído:** possui substituição ou encerramento anunciado;
- **arquivado:** sem suporte ativo, preservado para histórico;
- **removido de distribuições oficiais:** continua eventualmente no catálogo comunitário, mas fora de composições oficiais;
- **experimental:** sem garantia de estabilidade (alinhado a T0 quando aplicável).

O estado de manutenção é separado do nível de confiança T0–T3 (ADR-017).

### 10.3 Abandono

Quando um módulo ficar sem resposta ou manutenção:

1. será aberta chamada pública para adoção;
2. riscos e dependências serão avaliados;
3. a distribuição poderá congelar sua versão ou removê-lo de composições oficiais;
4. um novo mantenedor poderá ser nomeado por processo público;
5. se não houver capacidade segura de continuidade, o módulo será depreciado, arquivado ou marcado órfão;
6. dados, exportação e caminho de migração deverão ser preservados sempre que possível.

O processo de adoção comunitária será público. Nenhum módulo essencial será abandonado silenciosamente.

### 10.4 Forks e sucessão

A governança não impedirá forks permitidos pela licença.

Quando houver divergência legítima, a prioridade será preservar interoperabilidade, histórico e possibilidade de migração, evitando incompatibilidade deliberada ou disputa de marca que prejudique usuários.

### 10.5 Níveis de confiança (ADR-017)

A governança de módulos integra os níveis de confiança da ADR-017. Denominações equivalentes:

| Nível | Nome | Uso típico |
|---|---|---|
| **T0** | Experimental | Apenas desenvolvimento; nunca em distribuição oficial |
| **T1** | Comunitário | Catálogo comunitário com aviso; instalação manual |
| **T2** | Verificado | Catálogo verificado; compatibilidade e artefatos revisados |
| **T3** | Oficial | Governança OpenCore; elegível a distribuições oficiais |

**Certificação comercial** (quando existir política própria) é atributo **separado** do nível T: um módulo pode ser certificado sem ser T3, e um T3 não implica certificação comercial.

Quem pode promover ou rebaixar:

- T0 → T1: mantenedor do escopo + checklist mínimo;
- T1 → T2: revisores habilitados + CI/contratos conforme ADR-017;
- T2 → T3: processo de inclusão oficial (arquitetura, segurança, ownership);
- rebaixamento: responsável do módulo/distribuição ou segurança, com registro público do motivo.

Nível de confiança **não** equivale a sandbox de sistema operacional.

---

## 11. Comunidade, educação e mentoria

### 11.1 Trilhas de contribuição

O projeto deverá oferecer tarefas em diferentes níveis, por exemplo:

- `good first issue` — escopo pequeno e documentação suficiente;
- `help wanted` — contribuição delimitada com apoio disponível;
- `intermediate` — exige compreensão de módulo ou ferramenta;
- `advanced` — arquitetura, segurança, migrações ou desempenho;
- `maintainer-track` — revisão, releases, RFCs e transferência de conhecimento.

Rótulos indicam complexidade, não importância.

### 11.2 Requisitos das issues educacionais

Uma tarefa destinada a aprendizado deverá informar:

- contexto e resultado esperado;
- critérios de aceitação;
- arquivos ou áreas relevantes;
- conhecimentos recomendados;
- riscos e limitações;
- testes esperados;
- responsável por revisão ou mentoria;
- evidência pública que poderá compor portfólio.

### 11.3 Mentoria

Mentoria é orientação, não execução do trabalho pelo mentor.

Mentores deverão:

- explicar contexto e padrões;
- dividir problemas quando necessário;
- oferecer feedback acionável;
- evitar dependência privada de conhecimento;
- transformar dúvidas recorrentes em documentação.

### 11.4 Programas e coortes

Coortes comunitárias ou institucionais deverão possuir:

- escopo e duração definidos;
- responsáveis por revisão;
- limites de quantidade de participantes;
- critérios de conclusão;
- política de autoria e licenciamento;
- métricas e retrospectiva;
- proteção contra uso de trabalho gratuito sem aprendizado ou benefício público verificável.

Parceiros educacionais não poderão exigir aprovação automática de contribuições nem controlar o roadmap do núcleo.

---

## 12. Código de Conduta e resolução de conflitos

### 12.1 Código de Conduta

Toda participação em espaços oficiais estará sujeita a um Código de Conduta público.

O código deverá proteger, entre outros aspectos:

- respeito e segurança pessoal;
- crítica técnica sem ataque pessoal;
- inclusão e acessibilidade;
- proibição de assédio, discriminação, intimidação e retaliação;
- privacidade de denunciantes e envolvidos;
- aplicação proporcional e documentada.

### 12.2 Conflitos técnicos

Conflitos técnicos deverão ser resolvidos por:

1. definição clara da divergência;
2. identificação de critérios e evidências;
3. consulta a decisões vigentes;
4. experimento ou spike quando apropriado;
5. decisão do responsável pelo escopo;
6. possibilidade de recurso para instância superior.

Discordância não é violação de conduta. Comportamento abusivo durante a discordância pode ser.

### 12.3 Relatos de conduta

Relatos poderão ser enviados por canal privado definido no `CODE_OF_CONDUCT.md`.

Quem estiver diretamente envolvido ou possuir conflito não participará da decisão.

Quando ainda não existir grupo independente suficiente, poderá ser convocada pessoa externa de confiança para apoiar a análise.

### 12.4 Medidas possíveis

Medidas podem incluir:

- orientação privada;
- aviso formal;
- mediação;
- limitação temporária de participação;
- remoção de responsabilidades ou acessos;
- suspensão;
- banimento de espaços oficiais;
- encaminhamento jurídico quando necessário.

Toda medida deverá ser proporcional ao risco, gravidade, recorrência e possibilidade de reparação.

### 12.5 Recurso

Pessoas afetadas por decisão de conduta ou remoção de papel poderão solicitar uma revisão, exceto quando isso criar risco adicional à vítima ou expuser informação protegida.

A revisão deverá ser realizada por pessoas diferentes das que tomaram a decisão original, quando a estrutura da comunidade permitir.

---

## 13. Segurança

### 13.1 Divulgação responsável

Vulnerabilidades não deverão ser publicadas inicialmente em issues abertas quando isso aumentar o risco para usuários.

O projeto manterá `SECURITY.md` com:

- canal privado de relato;
- versões suportadas;
- informações necessárias para triagem;
- expectativa de comunicação;
- processo de correção e divulgação;
- política de crédito ao pesquisador.

### 13.2 Autoridade e acesso

Acesso a segredos, infraestrutura, publicação de pacotes, assinatura de artefatos e canais de segurança seguirá privilégio mínimo.

Sempre que possível:

- contas individuais serão usadas em vez de credenciais compartilhadas;
- autenticação multifator será obrigatória;
- acessos serão revisados periodicamente;
- ações críticas deixarão trilha de auditoria;
- nenhuma pessoa controlará sozinha código, assinatura e distribuição de uma release estável.

A separação integral poderá ser implementada progressivamente durante o Estágio F, sem ocultar exceções temporárias.

### 13.3 Incidentes

Incidentes relevantes deverão gerar registro contendo, conforme seguro:

- impacto;
- período afetado;
- causa;
- correção;
- dados ou versões atingidas;
- ações preventivas;
- responsáveis e prazos.

O objetivo da análise é melhorar o sistema, sem eliminar responsabilização por conduta deliberada ou negligência grave.

---

## 14. Conflitos de interesse, parceiros e patrocinadores

### 14.1 Declaração

Pessoas com poder de decisão deverão declarar vínculos que possam influenciar a questão, incluindo:

- emprego ou contratação;
- participação societária;
- financiamento;
- relação com fornecedor ou concorrente;
- autoria de solução avaliada;
- relação pessoal relevante.

Ter conflito não implica má-fé. Ocultá-lo ou participar indevidamente da decisão pode justificar anulação e revisão.

### 14.2 Impedimento

Pessoa com conflito material deverá evitar ser a única aprovadora da decisão.

Quando a comunidade ainda não possuir substituto qualificado, sua participação técnica poderá ocorrer, mas o conflito e a limitação deverão ser registrados, buscando revisão externa quando possível.

### 14.3 Limites de patrocinadores

Patrocinadores e parceiros:

- não recebem autoridade automática;
- não podem exigir exclusividade sobre capacidades essenciais;
- não podem impedir publicação de correções necessárias;
- não podem condicionar acesso aos dados do usuário;
- não podem ocultar participação financiada;
- podem financiar escopo, pesquisa, infraestrutura e manutenção sujeitos às regras públicas;
- podem obter representação institucional apenas por processo próprio e sem controle unilateral.

---

## 15. Transparência e registros

O projeto manterá publicamente, conforme aplicável:

- roadmap;
- RFCs e ADRs;
- lista de mantenedores, revisores e escopos;
- política de suporte e releases;
- atas ou registros de decisões relevantes;
- conflitos de interesse declarados;
- fontes e destinação de patrocínios;
- módulos oficiais e estados de manutenção;
- relatórios agregados de comunidade e segurança;
- histórico de alterações desta política.

Reuniões podem apoiar decisões, mas não substituem o registro escrito. Uma decisão tomada em reunião só será considerada oficial após documentação no repositório ou canal definido.

---

## 16. Reconhecimento

O OpenCore reconhecerá contribuições de forma verificável e proporcional.

Mecanismos podem incluir:

- histórico Git e autoria preservada;
- changelog e notas de release;
- arquivo de contribuidores;
- créditos de documentação, design, pesquisa e tradução;
- badges ou registros de trilha educacional;
- recomendações ou certificados de participação baseados em evidências;
- destaque de mantenedores e mentores;
- remuneração transparente quando houver recursos.

Reconhecimento não concede propriedade exclusiva sobre módulo, cargo permanente ou poder de decisão fora do processo de governança.

---

## 17. Métricas comunitárias

Métricas servem para identificar bloqueios e melhorar o processo, não para classificar pessoas por produtividade bruta.

O projeto poderá acompanhar:

- tempo até executar o projeto localmente;
- tempo até a primeira contribuição aceita;
- percentual de PRs concluídos ou abandonados;
- tempo de primeira resposta e revisão;
- principais causas de retrabalho;
- retenção após a primeira contribuição;
- distribuição de revisões entre mantenedores;
- quantidade de módulos com mais de um responsável;
- dependência operacional do fundador;
- incidentes de segurança e tempo de correção;
- participação em documentação, testes e outras áreas não relacionadas a código;
- satisfação de contribuidores e usuários-piloto;
- documentação necessária para contribuir sem ajuda privada;
- transferências de manutenção bem-sucedidas;
- módulos sem substituto declarado;
- contribuições ao Builder (perfis, triagem, catálogo) aceitas e revertidas por incompatibilidade.

Metas não deverão incentivar aprovação superficial, competição destrutiva ou redução artificial do escopo de testes e revisão.

---

## 18. Documentos operacionais derivados

A aprovação desta versão deverá originar ou atualizar, no mínimo:

- `CONTRIBUTING.md` — fluxo prático de contribuição;
- `CODE_OF_CONDUCT.md` — comportamento e aplicação;
- `SECURITY.md` — reporte e versões suportadas;
- `MAINTAINERS.md` — pessoas, papéis, escopos e situação de atividade;
- `GOVERNANCE.md` ou referência a este documento;
- templates de issue e pull request;
- template e diretório de RFCs;
- política e template de ADRs;
- labels de dificuldade, área, prioridade e estado;
- `CODEOWNERS` quando houver responsáveis suficientes;
- política inicial de contribuição sob as licenças aplicáveis.

### 18.1 Licenciamento das contribuições

Contribuições aceitas seguirão a licença do componente de destino.

O modelo inicial recomendado é utilizar o **Developer Certificate of Origin (DCO)** por meio de `Signed-off-by`, sem exigir CLA próprio na fase inicial.

A adoção futura de CLA exigirá RFC, justificativa jurídica, análise de impacto sobre a comunidade e garantia de que não concederá poder unilateral incompatível com o Manifesto.

---

## 19. Relação com o OpenCore Builder

O OpenCore Builder é ferramenta/serviço auxiliar externo ao OpenCore Runtime. Contribuições ao Builder e aos artefatos que alimentam recomendações são contribuições de primeira classe nesta governança.

### 19.1 Formas de contribuição reconhecidas

Além das formas da seção 4.2, são explicitamente válidas:

- perfis de negócio e variantes (ex.: Essencial, Completo, Multiestação);
- textos de triagem e microcopy em linguagem não técnica;
- entradas e mapeamentos do catálogo de capacidades;
- validação com usuários leigos (roteiros, observações, relatórios);
- traduções da triagem, preview e fichas;
- acessibilidade do fluxo de descoberta e instalação;
- templates de distribuição e configuração inicial;
- dados de demonstração seguros e reversíveis.

### 19.2 Revisão obrigatória de produto e compatibilidade

Mudanças que afetem **recomendações**, composição sugerida, inclusão/exclusão de módulos em perfis, regras de compatibilidade, níveis de confiança padrão ou textos que alterem o significado de uma capacidade **não** podem ser mergeadas apenas com revisão editorial.

Exigem, no mínimo:

1. revisão de produto (clareza, honestidade, ausência de promessa falsa);
2. revisão de compatibilidade (manifesto, conflitos, dependências, matriz ou regras equivalentes);
3. alinhamento aos níveis de confiança (ADR-017) — módulos experimentais (T0) não entram no modo padrão.

Alterações puramente ortográficas, de formatação ou de tradução fiel, sem mudança de sentido normativo, podem seguir o processo de mudanças triviais (seção 7.4).

### 19.3 Limites

- o Builder não concede autoridade sobre o OpenCore Runtime;
- recomendações automatizadas ou assistidas por IA permanecem subordinadas a regras determinísticas;
- contribuidores do Builder não podem contornar ownership de módulos ou de distribuições;
- dados de triagem seguem minimização e privacidade do Plano Institucional e da especificação do Builder.

---

## 20. Alteração desta política

Mudanças editoriais sem efeito normativo podem ser aprovadas como manutenção documental.

Mudanças que alterem papéis, autoridade, votação, direitos de participação, independência, aplicação de conduta, relação com o Builder ou relação com patrocinadores exigem RFC.

Durante o Estágio F:

- consulta mínima de quatorze dias para mudanças operacionais relevantes;
- consulta mínima de trinta dias para mudanças fundamentais;
- decisão motivada do Lead Maintainer.

Durante o Estágio C:

- aprovação mínima de dois terços do Conselho para mudanças relevantes;
- consulta pública conforme impacto;
- preservação do histórico e das posições divergentes relevantes.

---

## 21. Decisões iniciais desta versão

Esta versão estabelece que:

1. o OpenCore inicia com governança fundadora transitória e pública;
2. o fundador não recebe controle permanente ou irrevogável;
3. a transição para governança compartilhada depende de comunidade e responsabilidade reais;
4. autoridade é conquistada por contribuição, revisão e cuidado comprovados;
5. revisões independentes são obrigatórias quando houver pessoa qualificada, com exceção fundadora documentada;
6. RFCs são obrigatórias para arquitetura, políticas e compromissos relevantes;
7. ADRs registram decisões técnicas e não substituem consulta quando ela for necessária;
8. mantenedores são responsáveis temporários por bens comuns, não proprietários dos módulos oficiais;
9. módulos sem manutenção entram em processo público de adoção, depreciação ou arquivamento;
10. contribuições educacionais seguem os mesmos padrões profissionais, com apoio proporcional;
11. patrocinadores financiam trabalho, mas não compram controle do roadmap;
12. segurança e conduta possuem canais privados, processo de revisão e registro proporcional;
13. contribuições não relacionadas a código possuem legitimidade e reconhecimento equivalentes;
14. o modelo inicial de contribuição utilizará DCO, sem CLA próprio;
15. a criação de entidade independente ocorrerá quando houver necessidade institucional concreta e processo público;
16. ownership de módulo/distribuição declara principal, substituto, canais, prazo e sucessão;
17. níveis de confiança seguem ADR-017 (T0–T3); certificação comercial é atributo separado;
18. contribuições ao OpenCore Builder (perfis, triagem, catálogo, a11y, demo data etc.) são válidas e mudanças de recomendação exigem revisão de produto e compatibilidade.

---

## 22. Critério de aprovação da versão 1.0

Esta política estará pronta para ser marcada como **Aprovada** quando:

- estiver alinhada ao Manifesto v1.1, à Arquitetura v1.2 e ao Roadmap v2.2;
- identificar claramente quem decide durante o Estágio F;
- definir caminho verificável para revisores e mantenedores;
- definir PRs, RFCs, ADRs, segurança, conduta e conflitos;
- estabelecer processo contra abandono de módulos;
- proteger o projeto contra captura por patrocinadores ou fundador;
- permitir que uma pessoa externa compreenda como participar, crescer e contestar decisões;
- os documentos operacionais prioritários tiverem responsáveis e ordem de criação definidos.

Após aprovação, o índice de versões deverá marcar **Comunidade e Governança v1.0** como canônico e a próxima entrega documental passará a ser o **Plano Institucional OpenCore v1.0**.

---

## 23. Histórico de alterações

| Data | Mudança |
|---|---|
| 2026-07-23 | Versão 1.0 aprovada. |
| 2026-07-24 | Atualização editorial: relação com o OpenCore Builder (§19); papéis de mantenedor em formação e responsável por distribuição; ladder de progressão; ownership explícito; alinhamento ADR-017; formas de contribuição (a11y, triagem, mentoria, demo data); aprovação de mudanças do Builder e de distribuições. Status permanece **Aprovado**. |
