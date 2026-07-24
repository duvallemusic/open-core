# Especificação funcional — OpenCore Builder v0

**Natureza:** documento de produto e UX  
**Status:** proposta v0  
**Data:** 2026-07-24  
**Subordinação normativa:** Manifesto OpenCore v1.2 · Arquitetura OpenCore v1.3 · ADR-022  
**Documento relacionado:** Roadmap OpenCore v2.3  
**Escopo:** experiência de descoberta, triagem, recomendação, preview, download, instalação e onboarding — sem implementar o OpenCore Runtime, o portal completo ou IA obrigatória nesta etapa

---

## 1. Visão

O OpenCore Builder é o caminho pelo qual uma pessoa ou organização descreve o que precisa e recebe uma composição válida, explicada e instalável — sem montar infraestrutura manualmente e sem aprender a arquitetura do OpenCore.

A experiência-alvo:

> Contar o que precisa → receber recomendação explicada → visualizar a composição → ajustar opções compatíveis → baixar pacote instalável → começar a operar.

O Builder é externo ao OpenCore Runtime. Após o download, a instalação deve operar, fazer backup, restaurar e exportar sem depender do portal.

## 2. Problema do usuário

Usuários de software profissional open source frequentemente enfrentam:

- páginas técnicas que falam de módulos, stacks e bancos em vez de resultados;
- instalação que exige Docker, terminal, SQLite manual ou runtime da linguagem;
- catálogos sem garantia de que a combinação funciona;
- recomendações opacas ou que escondem rede, permissões e componentes externos;
- dependência de nuvem, cadastro ou prestador para continuar usando;
- mensalidade obrigatória ou lock-in disfarçado de “edição comunitária”.

O OpenCore precisa resolver a última milha: descoberta por necessidade, composição verificada, instalação “próximo, próximo, concluir” e onboarding compreensível — preservando offline-first, soberania dos dados e ausência de lock-in.

## 3. Personas

### 3.1 Persona principal de referência (teste de simplicidade)

Pequeno empresário sem conhecimento técnico — por exemplo dono de **padaria**, **oficina**, academia ou pequeno comércio — que:

- precisa de estoque, caixa e gestão;
- procura opção gratuita e open source;
- não sabe o que é OpenCore Runtime, módulo, Docker ou banco;
- utiliza principalmente Windows;
- precisa começar com um computador (monoposto);
- pode evoluir para vários postos ou unidades;
- possui dados em planilha;
- quer suporte opcional;
- não quer mensalidade obrigatória.

**Uso desta persona:** teste de simplicidade da linguagem, da triagem e da instalação. **Não limita** o produto a padaria/oficina nem define a identidade da plataforma.

### 3.2 Personas secundárias

| Persona | Necessidade principal |
|---|---|
| Administrador de condomínio / associação | Distribuição específica, usuários e permissões simples |
| Prestador de serviço / integrador | Modo avançado, lockfile, diagnóstico, adaptadores |
| Mantenedor / contribuidor | Metadados de módulo, perfis, textos de triagem |
| Organização piloto institucional | Instalação reproduzível, ficha, suporte e continuidade |

## 4. Princípios de UX

1. **Linguagem de resultado antes de arquitetura.**
2. **Complexidade invisível, não omitida:** simples na superfície; auditável no detalhe.
3. **“Não sei” é resposta válida.**
4. **Explicabilidade obrigatória** de toda recomendação.
5. **Modo simples por padrão; modo avançado sob demanda.**
6. **Sem cadastro obrigatório** para entender a recomendação ou baixar.
7. **IA opcional;** motor de regras é autoridade (ADR-022).
8. **Não prometer** módulos, telas ou distribuições inexistentes.
9. **Offline após download** no modo monoposto.
10. **Portal não é requisito** para operar, restaurar ou exportar.

## 5. Jornada

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

Em cada etapa o usuário pode voltar, salvar progresso localmente (quando aplicável) e obter ajuda contextual sem jargão.

## 6. Triagem

### 6.1 Objetivo

Traduzir necessidades de negócio em restrições e capacidades, sem exigir nomes de módulos.

### 6.2 Perguntas mínimas

| Tema | Pergunta (exemplos de redação) |
|---|---|
| Tipo de organização | Que tipo de organização você gerencia? |
| Capacidades necessárias | O que você precisa fazer no dia a dia? |
| Sistema operacional | Em qual sistema o computador principal roda? |
| Quantidade de computadores | Quantos computadores vão usar o sistema? |
| Quantidade de usuários | Quantas pessoas vão acessar? |
| Unidades | É uma unidade só ou várias? |
| Acesso remoto | Precisa acessar de fora do local? |
| Offline | Precisa funcionar sem internet? |
| Sistema atual | Você já usa algum sistema hoje? |
| Formato dos dados | Seus dados estão em planilha, outro sistema ou ainda não? |
| Equipamentos | Usa impressora, balança ou outros equipamentos? |
| Idioma / região | Idioma e região / moeda preferidos |
| Nível de ajuda | Prefere fazer sozinho, com guia ou com prestador? |

### 6.3 Opções sempre presentes

- **Não sei**
- **Decidir depois**
- explicação curta de por que a pergunta importa
- recomendação padrão segura quando a resposta for incerteza

### 6.4 Regras

- triagem anônima por padrão;
- não exigir CPF, CNPJ, faturamento ou dados sensíveis sem necessidade;
- respostas “não sei” encaminham para perfil conservador e verificável;
- modo conversacional com IA, se existir, é opcional e deve cair no mesmo validador.

## 7. Catálogo de capacidades

O catálogo traduz linguagem de negócio em capacidades funcionais e módulos candidatos, sem hardcodar o tipo de negócio no OpenCore Runtime.

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

| Camada | Conteúdo |
|---|---|
| Necessidade do usuário | Frase em linguagem comum |
| Capacidade funcional | Identificador estável (`capability`) |
| Implementação | Módulo(s) candidato(s) |
| Perfil de distribuição | Essencial / Completo / Multiestação / outros verificados |

O catálogo inicial pode ser arquivo estático validado em spike; formato final condicionado à Arquitetura 1.3 e aos Spikes 13–14.

## 8. Perfis

Perfis são variantes verificadas de uma distribuição, não combinações livres.

Exemplos conceituais (não exclusivos):

| Perfil | Intenção |
|---|---|
| **Essencial** | Começar rápido no monoposto com o mínimo útil |
| **Completo** | Operação diária ampliada, ainda em limites testados |
| **Multiestação** | Vários postos, usuários, permissões e rede local |

Regras:

- personalização inicial só dentro da matriz do perfil;
- novos perfis exigem testes de composição e experiência;
- páginas de marketing por segmento só apontam para perfis realmente suportados.

### 8.1 Exemplo ilustrativo — Padaria

> **Atenção:** exemplo de UX para teste de simplicidade. Depende de módulos realmente implementados. Não altera a Portaria nem introduce regras de padaria no OpenCore Runtime.

Necessidades possíveis declaradas pelo usuário:

- vendas no balcão; caixa; produtos; estoque; compras; fornecedores; clientes; contas; encomendas; validade; ficha técnica; integração com balança; impressora; múltiplos caixas; múltiplas unidades.

#### Padaria Essencial (exemplo)

- vendas;
- caixa;
- produtos;
- estoque;
- fornecedores;
- backup;
- relatórios básicos.

#### Padaria Completo (exemplo)

- tudo do Essencial;
- financeiro;
- clientes;
- compras;
- encomendas;
- validade;
- relatórios.

#### Padaria Multiestação (exemplo)

- tudo do Completo;
- usuários;
- permissões;
- rede local;
- backup centralizado.

O mesmo padrão de Essencial / Completo / Multiestação aplica-se a outros segmentos (oficina, comércio etc.) quando houver distribuição/perfil verificados.

## 9. Recomendação

### 9.1 Saída mínima

A recomendação deve apresentar:

- distribuição sugerida;
- perfil sugerido;
- modo operacional (monoposto, rede local, etc.);
- lista do que foi incluído;
- opcionais elegíveis;
- requisitos de hardware e SO;
- necessidade de internet;
- próximos passos (preview → personalizar → baixar).

### 9.2 Explicabilidade (obrigatória)

Cada recomendação deve dizer:

- o que foi incluído;
- por que;
- o que ficou opcional;
- o que exige internet;
- o que transmite dados;
- o que exige hardware;
- o que pode ser removido;
- o que é necessário por dependência.

### 9.3 Modos de uso

#### Modo simples (padrão)

- perfil recomendado;
- poucas decisões;
- apenas módulos oficiais/verificados;
- linguagem comum;
- “não sei” → padrão seguro.

#### Modo avançado

- módulos e versões;
- permissões;
- adaptadores;
- requisitos técnicos;
- riscos e níveis de confiança;
- eventual inclusão de módulo comunitário com aceite explícito.

## 10. Preview

Preview estrutural da composição validada:

- navegação representativa;
- módulos ativos;
- fluxos principais;
- permissões e alertas de rede/dados externos.

Não pode:

- prometer funcionalidade inexistente;
- substituir testes;
- ocultar dependências;
- atuar como low-code genérico na v0.

Critério: o que o preview mostra deve corresponder ao manifesto/lockfile da composição.

## 11. Personalização

Permitida apenas dentro dos limites verificados do perfil:

- incluir/remover opcionais compatíveis;
- escolher modo operacional suportado;
- marcar equipamentos relevantes;
- optar por dados de demonstração.

Bloqueada na v0:

- combinação livre fora da matriz;
- módulos experimentais;
- módulos comunitários no modo simples;
- alterações que quebrem dependências sem explicação.

Toda alteração reexecuta o validador antes do download.

## 12. Validação

Autoridade: motor de regras + validador determinístico (ADR-022).

Verifica:

- dependências e conflitos;
- compatibilidade com OpenCore Runtime;
- SO / hardware;
- confiança (ADR-017);
- permissões e transmissões externas;
- consistência preview ↔ manifesto ↔ lockfile ↔ pacote.

Falhas:

- mensagem em linguagem comum;
- detalhe técnico expansível no modo avançado;
- sugestão de ajuste (remover conflito, escolher outro perfil, etc.).

## 13. Download

A tela de download deve mostrar:

| Informação | Obrigatório |
|---|---|
| Sistema operacional | Sim |
| Tamanho aproximado | Sim |
| Versão | Sim |
| Composição (módulos/perfil) | Sim |
| Checksum | Sim |
| Assinatura | Futura; campo reservado |
| Licença | Sim |
| Documentação | Sim |
| Necessidade de internet | Sim |
| Requisitos mínimos | Sim |
| Última atualização | Sim |

Também deve permitir:

- exportar manifesto e lockfile / ficha da composição;
- baixar sem cadastro obrigatório;
- distinguir suporte comunitário de suporte comercial opcional.

Pacote inicial: artefato pré-construído conhecido (Estágio A) ou montagem a partir de artefatos assinados (Estágio B). Sem compilação arbitrária no MVP.

## 14. Instalação

### 14.1 Objetivo de UX

Quando seguro: **próximo → próximo → concluir**.

### 14.2 Modo monoposto — proibições

Nunca exigir do usuário leigo:

- Docker;
- terminal;
- instalação manual de SQLite;
- instalação manual de Python/Node (runtime da linguagem);
- edição manual de arquivos de configuração.

### 14.3 Fluxo esperado

1. verificação de requisitos;
2. escolha de pasta de instalação / dados (com padrão sensato);
3. confirmação da composição;
4. instalação;
5. atalho de inicialização;
6. abertura do onboarding.

A instalação deve funcionar offline após o download no modo monoposto.

## 15. Onboarding

Após a primeira execução, perguntar/configurar:

- nome da organização;
- usuários iniciais;
- localização / idioma / moeda;
- política de backup;
- importação de dados (se houver);
- equipamentos (impressora etc.);
- dados de demonstração (opcional);
- relatório final / ficha da instalação.

Princípios:

- pular com segurança o que for “decidir depois”;
- não sobrecarregar o primeiro uso;
- linguagem simples; detalhes técnicos no modo avançado;
- ao final, caminho claro para o primeiro uso útil (ex.: cadastrar produto / abrir caixa — conforme distribuição real).

## 16. Importação

- formatos iniciais: CSV (mínimo); outros via adaptadores futuros;
- dry-run quando possível;
- relatório de inconsistências;
- preservar IDs externos quando aplicável;
- não alterar origem sem pedido;
- declarar limitações;
- importação não é obrigatória para concluir o onboarding.

Adaptadores para sistemas externos (Odoo, ERPNext, Dolibarr, etc.) são evolução; não fazem parte do MVP obrigatório do Builder.

## 17. Backup e continuidade

- backup ≠ exportação (distinção preservada);
- fluxo compreensível de backup e restauração no produto instalado;
- ficha da instalação com localização dos dados, política de backup e lockfile;
- restauração e exportação **não** dependem do portal;
- telemetria desligada por padrão.

A ficha deve incluir, no mínimo: distribuição, perfil, OpenCore Runtime, módulos, versões, modo operacional, necessidade de internet, localização dos dados, componentes externos, permissões, canal de atualização e status.

## 18. Suporte

- suporte comunitário documentado e gratuito de consultar;
- prestadores opcionais, sem condicionar download;
- deixar claro o que é comunitário vs comercial;
- usuário pode contratar terceiros fora de qualquer rede oficial;
- Builder pode apresentar prestadores como opção, nunca como bloqueio;
- diagnóstico compreensível (e `doctor` / ficha para técnicos).

## 19. SEO e páginas de entrada

Páginas por necessidade real, por exemplo:

- sistema gratuito para padaria;
- controle de estoque offline;
- sistema para oficina sem mensalidade;
- sistema para condomínio;
- software de gestão local;
- sistema para associação;
- sistema para biblioteca.

Regras:

- não prometer distribuição inexistente;
- apontar só para perfis realmente suportados;
- linguagem de resultado antes de arquitetura;
- destacar: sem mensalidade obrigatória; funcionamento local; dados sob controle; instalação guiada; módulos conforme necessidade; código aberto; suporte opcional.

Mensagens de referência (não slogans definitivos):

- “Sistemas que pertencem a quem usa.”
- “Você explica como seu negócio funciona. O OpenCore prepara o sistema certo.”
- “Seu sistema pronto, sem mensalidade e sem nuvem obrigatória.”
- “Não procure módulos. Conte o que seu negócio precisa.”
- “Um sistema do tamanho da sua organização.”

## 20. Privacidade

- triagem anônima por padrão;
- sem CPF/CNPJ/faturamento sem necessidade;
- informar se respostas vão a serviço de IA;
- modo sem IA sempre disponível;
- retenção mínima;
- não usar respostas para anúncios;
- telemetria desativada por padrão;
- cadastro não obrigatório para recomendação ou download;
- dados da triagem não são exigidos para operação local posterior.

## 21. Acessibilidade

- linguagem simples;
- navegação por teclado;
- compatibilidade com leitores de tela;
- contraste adequado;
- explicações curtas e expansíveis;
- não depender somente de cores;
- opção de voltar em todas as etapas da triagem;
- salvar progresso localmente quando possível;
- triagem utilizável em dispositivo móvel, mesmo que o produto instalado seja desktop;
- evitar captchas ou barreiras que impeçam o fluxo principal sem alternativa.

## 22. Métricas

Métricas propostas (coleta apenas com consentimento / telemetria opt-in; muitas podem ser medidas em testes moderados sem telemetria de produção):

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

## 23. MVP

Entregas mínimas do Builder / jornada associada (IA **fora** do MVP obrigatório):

1. páginas por distribuição real;
2. questionário estático;
3. motor de regras;
4. três perfis (Essencial, Completo, Multiestação — ou equivalentes verificados);
5. preview de navegação;
6. personalização limitada;
7. lockfile (e manifesto);
8. instalador pré-construído;
9. onboarding;
10. importação CSV;
11. backup/restauração;
12. documentação.

Fora do MVP:

- compilação arbitrária;
- marketplace;
- IA obrigatória;
- qualquer módulo comunitário por padrão;
- builds ilimitados;
- personalização livre fora da matriz.

## 24. Evolução

Ordem sugerida (alinhada ao Roadmap 2.3 e ADR-022):

1. Spikes 14–16: regras, instalador/onboarding, preview estrutural;
2. alpha com pacotes pré-construídos;
3. Spike 18: montagem por artefatos assinados;
4. catálogo ampliado e segunda distribuição;
5. Spike 17: camada conversacional opcional;
6. rede de prestadores e adaptadores;
7. Estágio C / marketplace / personalização livre — somente após confiança, custo e matriz.

Cada evolução permanece subordinada a Manifesto, Arquitetura e ADR-022; mudanças que afetem recomendações exigem revisão de produto e compatibilidade, não apenas editorial.

## 25. Critérios de aceite

Esta especificação v0 será considerada adequada para orientar implementação quando:

1. A jornada completa (busca → primeiro uso) estiver documentada e coerente com ADR-022.
2. A persona padaria/oficina passar no teste de simplicidade sem limitar o produto.
3. A triagem incluir “não sei”, “decidir depois” e padrões seguros.
4. A recomendação for explicável nos oito pontos da §9.2.
5. Existirem modos simples e avançado com regras de confiança claras.
6. Preview, download, instalação monoposto e onboarding cumprirem as proibições desta especificação.
7. Privacidade e acessibilidade mínimas estiverem definidas.
8. O MVP listar entregas sem IA obrigatória.
9. Exemplos de perfil (Padaria Essencial/Completo/Multiestação) estiverem marcados como exemplos dependentes de módulos reais.
10. Nenhum trecho afirmar que o portal é necessário para operar, que a IA monta qualquer combinação, ou que o Builder faz parte do OpenCore Runtime.

---

## Changelog

| Versão | Descrição |
|---|---|
| 0 | Proposta inicial de especificação funcional/UX do OpenCore Builder, subordinada ao Manifesto 1.2, Arquitetura 1.3 e ADR-022 |
