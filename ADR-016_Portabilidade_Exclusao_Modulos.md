# ADR-016 — Portabilidade e exclusão verificáveis por módulo

**Status:** Proposto  
**Data:** 2026-07-23  
**Base normativa:** Manifesto OpenCore v1.1  
**Documento relacionado:** Arquitetura OpenCore v1.2 (origem normativa: v1.1)  
**Seções de referência:** §4.6 Privacidade, LGPD e responsabilidade da implantação · §4.7 Contrato verificável por módulo · §11.7 Contrato de portabilidade e exclusão por módulo · §14.4 Exclusão de dados · §14.5 Portabilidade em módulos proprietários

---

## Contexto

O Manifesto OpenCore v1.1 estabelece soberania de dados como princípio central. A Arquitetura v1.1 reconhece, porém, que o software isoladamente não determina base legal, responsabilidades de operador ou conformidade plena com a LGPD em cada implantação (§4.6). Cabe à arquitetura oferecer **mecanismos** que permitam às distribuições e organizações cumprir normas aplicáveis.

Compromissos genéricos de exportação ou exclusão não bastam (§4.7). Cada módulo persistente deverá fornecer um **contrato verificável** de portabilidade e exclusão, integrado ao manifesto, aos testes e à exportação da distribuição. Esse contrato aplica-se igualmente a módulos open source e a módulos proprietários candidatos a certificação (§14.5): proteção de segredos industriais não pode eliminar exportação, exclusão ou migração de dados essenciais do usuário.

Sem contrato mínimo uniforme, auditorias LGPD, migrações entre distribuições e desinstalação segura de módulos permanecem dependentes de boa vontade documental, incompatível com o modelo modular proposto.

---

## Decisão

### 1. Contrato mínimo obrigatório por módulo persistente

Todo módulo que persista dados deverá declarar e implementar um contrato contendo, no mínimo:

- entidades e categorias de dados sob sua responsabilidade;
- indicação de dados pessoais, sensíveis ou operacionais;
- relações necessárias para preservar significado na exportação;
- formatos de exportação suportados e versão do esquema exportado;
- procedimento de validação da exportação;
- operações de exclusão total e seletiva;
- dependências que podem impedir exclusão imediata;
- retenções legais ou de auditoria configuráveis;
- anexos e arquivos vinculados;
- transferências para adaptadores ou serviços externos;
- comportamento após desativação ou remoção do módulo.

O contrato deverá integrar-se ao manifesto do módulo e ser descoberto pelo runtime ou pela ferramenta de exportação da distribuição.

### 2. Declaração de privacidade no manifesto

Módulos que tratem dados pessoais deverão declarar adicionalmente (§4.6):

- categorias de dados tratados e finalidade funcional;
- dados obrigatórios e opcionais;
- relações e dependências relevantes;
- regras configuráveis de retenção;
- mecanismos de consulta, correção, exportação e exclusão;
- registros de auditoria que possam limitar exclusão imediata;
- integrações e transferências externas possíveis;
- dados sensíveis que não poderão aparecer em logs ou telemetria.

As distribuições oficiais deverão documentar a repartição de responsabilidades entre software, operador da instalação e serviços externos.

### 3. Exclusão com resultado verificável

Mecanismos de exclusão (§14.4) deverão considerar dependências, auditoria, retenção legal, backups existentes, anexos e sincronizações externas configuradas.

A operação deverá produzir relatório verificável indicando:

- dados excluídos;
- dados anonimizados ou pseudonimizados;
- dados mantidos e respectivo motivo;
- anexos processados;
- integrações externas que exigem ação adicional;
- backups sujeitos à política de retenção.

### 4. Testes obrigatórios do contrato

Módulos persistentes deverão possuir testes automatizados que comprovem:

1. exportação completa de conjunto representativo de dados;
2. leitura ou validação da exportação **sem** depender do banco interno;
3. exclusão de registros elegíveis e anexos associados;
4. preservação explícita de registros sujeitos a retenção ou auditoria;
5. ausência de dados de outros módulos na exportação privada;
6. relatório de dependências quando exclusão não puder ser concluída;
7. identificação de dados enviados a integrações externas configuradas.

Falha em qualquer item impede certificação T2/T3 e inclusão em catálogo oficial (complementa ADR-017).

### 5. Módulos proprietários

Para certificação ou catálogo oficial, módulos proprietários deverão fornecer (§14.5):

- exportador documentado para dados essenciais;
- esquema versionado em formato aberto ou amplamente interoperável;
- mecanismo documentado de exclusão;
- inventário de dados enviados a serviços externos;
- testes de conformidade executáveis pela entidade certificadora;
- documentação suficiente para auditoria de segurança sob termos de confidencialidade quando necessário.

O contrato poderá ocultar detalhes internos do produto, mas **não** poderá tornar dados essenciais dependentes de implementação privada insubstituível.

---

## Consequências

### Positivas

- Soberania de dados operacionalizável módulo a módulo, não apenas declarada no Manifesto.
- Conformidade LGPD facilitada por contratos repetíveis e testáveis.
- Migração entre distribuições e desinstalação de módulos com evidência objetiva.
- Módulos proprietários podem coexistir no ecossistema sem lock-in de dados.

### Negativas / custos

- Custo de implementação e manutenção de exportadores, exclusores e suítes de teste por módulo.
- Exclusão completa pode ser impossível de imediato quando há auditoria, backup ou sync externo — exige comunicação clara ao usuário.
- Formatos de exportação múltiplos aumentam superfície de compatibilidade a versionar.

### Obrigações

1. Manifestos de módulos persistentes deverão referenciar o contrato de dados explicitamente.
2. CI deverá exigir presença do contrato e execução dos testes de exportação/exclusão (ADR-020).
3. Distribuições oficiais deverão agregar exportação multi-módulo sem violar fronteiras de propriedade de dados.
4. Adaptadores de sync deverão declarar dados replicados externamente (complementa ADR-019).

---

## Alternativas consideradas

| Alternativa | Veredito |
|---|---|
| Exportação apenas no nível da distribuição | Rejeitada — oculta responsabilidade por módulo e falha em remoções parciais |
| Contrato documental sem testes | Rejeitada — não verificável (§4.7, §11.7) |
| Exclusão física imediata sempre | Rejeitada — conflita com auditoria, backup e retenção legal |
| Isenção de módulos proprietários | Rejeitada — §14.5 exige paridade de contratos essenciais |
