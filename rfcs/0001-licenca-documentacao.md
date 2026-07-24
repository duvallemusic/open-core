# RFC-0001 — Licença da documentação OpenCore

**Autor:** Bruno Costa / comunidade OpenCore  
**Status:** Em consulta  
**Publicação inicial da licença:** 2026-07-23  
**Período de consulta:** 2026-07-23 a 2026-08-22  
**Responsável pela decisão:** Lead Maintainer interino  
**Issue relacionada:** [#2](https://github.com/duvallemusic/open-core/issues/2)  
**ADRs relacionadas:** não aplicável  
**Conflitos de interesse conhecidos:** nenhum declarado  
**Nota (2026-07-24):** a revisão formal da Etapa 0 foi concluída em paralelo; esta RFC **permanece em consulta** até 2026-08-22 e **não** deve ser marcada como aceita antes do encerramento formal.

> **Efeito da publicação inicial:** a licença foi aplicada publicamente aos
> materiais identificados em 2026-07-23 e permanece válida para essas versões.
> A consulta prevista nesta RFC trata da ratificação da decisão dentro da
> governança do OpenCore e de eventuais ajustes para versões futuras. Ela não
> revoga direitos já concedidos.

---

## Resumo

Esta RFC formaliza e submete à consulta pública a decisão inicial de licenciar
a documentação pública do OpenCore sob **Creative Commons Attribution 4.0
International — CC BY 4.0**, mantendo exemplos e trechos de código sob **Apache
License 2.0**, salvo indicação diferente no próprio arquivo.

Marcas, logotipos, selos e elementos de identidade institucional permanecerão fora da licença documental e dependerão de política de marca futura.

## Contexto e problema

O Manifesto v1.1 define MPL 2.0 para o patrimônio estrutural de software e Apache 2.0 para SDKs, ferramentas e materiais de código, mas o repositório documental ainda informa que seus textos estão sob direitos autorais sem licença pública explícita.

Essa lacuna cria problemas para:

- receber contribuições documentais de terceiros;
- aplicar o DCO de forma coerente;
- permitir traduções e materiais educacionais;
- reutilizar guias em distribuições e treinamentos;
- definir o que pode ser copiado, adaptado e redistribuído;
- separar documentação aberta de marca e comunicação oficial.

## Objetivos

- tornar os documentos públicos legalmente reutilizáveis;
- permitir tradução, adaptação e redistribuição com atribuição;
- reduzir atrito para educação, universidades e organizações;
- preservar autoria e histórico;
- diferenciar texto, código e marca;
- habilitar PRs documentais externos sob política clara.

## Não objetivos

- alterar as licenças previstas para runtime, módulos ou SDKs;
- conceder direito de uso de marca ou certificação;
- licenciar dados pessoais, segredos, vulnerabilidades ou materiais de terceiros;
- resolver política final de patentes, CLA ou entidade jurídica;
- relicenciar automaticamente conteúdo sem direito suficiente para fazê-lo.

## Proposta

### 1. Documentação textual — CC BY 4.0

Serão licenciados sob `CC-BY-4.0`, salvo aviso diferente:

- Manifesto;
- Arquitetura;
- Roadmap;
- Comunidade e Governança;
- Plano Institucional;
- ADRs e RFCs;
- guias de contribuição, segurança e operação;
- documentação educacional;
- traduções oficiais;
- diagramas e tabelas produzidos pelo projeto.

A redistribuição e adaptação deverão manter atribuição adequada, referência à licença e indicação razoável de mudanças.

### 2. Código em documentação — Apache 2.0

Exemplos substanciais, templates executáveis, scaffolds, scripts e trechos destinados a incorporação em software serão tratados como `Apache-2.0`, salvo cabeçalho ou aviso diferente.

Trechos meramente ilustrativos e inseparáveis do texto poderão acompanhar a licença documental quando sua reutilização isolada não for objetivo do material.

### 3. Marca e identidade — direitos reservados

A licença documental não concede direito de usar:

- logotipo;
- selos;
- expressões de aprovação oficial;
- certificações;
- identidade visual institucional.

### 4. Forma sugerida de atribuição

Uma atribuição razoável pode utilizar:

> Documentação OpenCore, por Bruno Costa e contribuidores do OpenCore,
> licenciada sob CC BY 4.0. Alterações realizadas quando aplicável.

A atribuição pode incluir um link para o repositório, para a licença e uma
indicação das modificações realizadas.

## Compatibilidade e migração

Após a publicação inicial:

1. confirmar que os autores atuais autorizam o licenciamento;
2. publicar o mapa de licenciamento no arquivo `LICENSE`;
3. adicionar aviso de código Apache quando aplicável;
4. atualizar README e CONTRIBUTING;
5. identificar exceções e conteúdo de terceiros;
6. aplicar a política apenas a conteúdo com cadeia de direitos suficiente.

## Licenciamento e propriedade intelectual

A CC BY 4.0 é uma licença de conteúdo permissiva, com obrigação de atribuição. Não possui cláusula share-alike.

A Apache 2.0 permanece adequada para código reutilizável por incluir termos de copyright e patentes voltados a software.

## Comunidade e educação

A proposta facilita:

- traduções;
- materiais de aula;
- guias internos de organizações;
- cópias offline;
- adaptação para acessibilidade;
- produção de portfólio e estudos de caso.

A atribuição deverá evitar exigências desproporcionais para trechos pequenos, seguindo práticas razoáveis da CC BY 4.0.

## Impacto institucional e financeiro

Parceiros poderão reutilizar documentação em treinamentos e serviços pagos com atribuição, sem adquirir autoridade ou certificação.

O projeto poderá continuar vendendo treinamento, suporte e materiais organizados; a licença permite competição e reduz lock-in.

## Alternativas consideradas

| Alternativa | Benefícios | Custos | Veredito inicial |
|---|---|---|---|
| Manter direitos reservados | controle máximo | impede colaboração e reutilização aberta | rejeitar |
| CC BY 4.0 | ampla adoção, tradução e educação | permite derivados fechados com atribuição | recomendada |
| CC BY-SA 4.0 | derivados permanecem sob mesma licença | maior atrito e dúvidas de compatibilidade | alternativa válida |
| Apache 2.0 para toda documentação | uma licença para código e texto | menos apropriada para conteúdo e atribuição editorial | não preferida |
| MPL 2.0 para documentos | alinhamento nominal ao núcleo | copyleft por arquivo concebido para software | rejeitar |
| Domínio público / CC0 | máxima liberdade | reduz obrigação de atribuição | rejeitar para documentos normativos |

## Plano de implementação

1. [x] confirmar a intenção dos autores atuais de licenciar o conteúdo;
2. [x] publicar o mapa de licenciamento no arquivo `LICENSE`;
3. [x] atualizar README e CONTRIBUTING;
4. [x] separar texto, código reutilizável e marca;
5. [ ] manter consulta pública até 2026-08-22;
6. [ ] registrar contribuições, objeções e alternativas;
7. [ ] publicar decisão motivada ao final da consulta;
8. [ ] identificar continuamente conteúdo de terceiros e exceções;
9. [ ] adicionar identificadores SPDX quando aplicável;
10. [ ] abrir amplamente contribuições documentais sob os termos publicados.

## Critérios de sucesso

- qualquer pessoa consegue identificar a licença de texto e código;
- contribuições documentais possuem termos claros;
- traduções podem ser publicadas legalmente;
- marca permanece separada de conteúdo;
- não existem arquivos canônicos sem licença ou exceção identificada.

## Plano de reversão

Licenças concedidas a versões publicadas não podem ser retiradas retroativamente. Uma política futura poderá alterar a licença de novas versões apenas com direitos suficientes e processo de governança adequado.

## Questões em aberto

- CC BY 4.0 ou CC BY-SA 4.0 para os documentos normativos?
- como atribuir o conjunto do projeto de forma prática?
- quais diagramas, fontes e imagens exigem inventário separado?
- o nome “OpenCore” precisará de aviso de marca antes do registro formal?

## Estado da consulta

A consulta pública foi aberta em 2026-07-23 e permanecerá disponível até
2026-08-22.

A licença documental foi publicada inicialmente em 2026-07-23 para estabelecer
termos claros para uso e contribuição. Direitos concedidos às versões já
publicadas permanecem válidos.

O resultado final, as objeções relevantes e a decisão motivada serão registrados
nesta seção após o encerramento.
