# Política de Segurança do OpenCore

## 1. Estado atual

O OpenCore está na Etapa 0 documental e ainda não possui release de produto suportada.

| Versão | Suporte de segurança |
|---|---|
| Documentação da Etapa 0 | correções de conteúdo e processo |
| Runtime / distribuições | ainda não publicados |

Esta tabela deverá ser atualizada antes da primeira release técnica.

## 2. Como relatar uma vulnerabilidade

Use o **GitHub Private Vulnerability Reporting** deste repositório.

O Private Vulnerability Reporting está habilitado neste repositório.

Não publique em issue, pull request, discussão ou rede social:

- exploit funcional;
- credencial ou segredo;
- dados pessoais;
- caminho de ataque não corrigido;
- detalhes que aumentem o risco para usuários.

Se o formulário privado estiver temporariamente indisponível, não publique os detalhes. Contate o Lead Maintainer por canal seguro e aguarde instruções.

## 3. Informações úteis

Inclua, quando possível:

- componente e versão afetados;
- ambiente e sistema operacional;
- descrição do impacto;
- passos de reprodução;
- prova de conceito mínima e segura;
- pré-condições e privilégios necessários;
- possibilidade de corrupção ou exposição de dados;
- mitigação conhecida;
- disponibilidade para testar correção;
- preferência de crédito ou anonimato.

Não envie dados reais de terceiros quando uma reprodução sintética for suficiente.

## 4. Metas de resposta

Durante o Estágio F, os seguintes prazos são metas de melhor esforço:

- confirmação de recebimento: até 5 dias úteis;
- triagem inicial: até 10 dias corridos;
- atualização de andamento: ao menos a cada 14 dias enquanto o caso estiver ativo;
- divulgação: após correção ou mitigação adequada, considerando risco aos usuários.

Casos críticos poderão exigir comunicação e ação imediatas. A capacidade limitada do projeto será informada com transparência.

## 5. Processo

O Grupo de Resposta de Segurança deverá:

1. confirmar o relato;
2. avaliar impacto, alcance e urgência;
3. limitar acesso às informações;
4. identificar versões e componentes afetados;
5. preparar correção ou mitigação;
6. testar regressão, migração e recuperação;
7. coordenar release e comunicação;
8. registrar análise pós-incidente quando seguro.

Ações emergenciais podem incluir suspensão de downloads, revogação de credenciais, reversão e correção temporária.

## 6. Divulgação coordenada

O projeto buscará acordar uma data de divulgação com o pesquisador.

Divulgação poderá ser antecipada quando:

- houver exploração ativa conhecida;
- usuários precisarem de mitigação imediata;
- a informação já for pública;
- a espera aumentar materialmente o risco.

Poderá ser adiada quando detalhes permitirem exploração ampla antes de existir mitigação viável.

## 7. Crédito

Pesquisadores de boa-fé poderão receber crédito em advisory, changelog ou agradecimento, salvo solicitação de anonimato ou impedimento legal.

Crédito não será condicionado à renúncia de direitos nem à ocultação de risco legítimo.

## 8. Pesquisa responsável

O projeto considera de boa-fé pesquisas que:

- evitem acesso, alteração ou destruição de dados de terceiros;
- não interrompam serviço ou infraestrutura;
- utilizem o mínimo de exploração necessário;
- não realizem extorsão;
- respeitem confidencialidade durante a correção;
- forneçam tempo razoável para resposta;
- cumpram a legislação aplicável.

Esta política não concede autorização para acessar sistemas, contas ou dados que não pertençam ao pesquisador.

## 9. Segredos e credenciais

Credenciais publicadas acidentalmente deverão ser consideradas comprometidas e revogadas, mesmo após remoção do histórico visível.

PRs e issues não devem conter:

- chaves de API;
- tokens;
- senhas;
- certificados privados;
- dados pessoais reais;
- dumps de produção.

## 10. Dependências e cadeia de suprimentos

Quando houver código, o projeto deverá evoluir verificações para:

- inventário de dependências e licenças;
- alertas de vulnerabilidade;
- revisão de lockfiles;
- princípio de menor privilégio no CI;
- proteção de branches e releases;
- assinatura ou verificação de artefatos quando aplicável;
- rotação de acessos;
- testes arquiteturais definidos no ADR-020.

## 11. Incidentes

Incidentes relevantes deverão gerar, quando seguro:

- impacto e período;
- causa;
- versões atingidas;
- correção e mitigação;
- dados potencialmente afetados;
- ações preventivas;
- responsáveis e prazos.

O relatório evitará expor informação que facilite exploração ainda ativa.

## 12. Contato e responsáveis

Consulte [`MAINTAINERS.md`](MAINTAINERS.md) para o responsável interino e o estágio atual do Grupo de Resposta de Segurança.
