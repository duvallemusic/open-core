# Guia de ADRs do OpenCore

ADR — Architecture Decision Record — registra uma decisão arquitetural, seu contexto, alternativas, consequências e obrigações.

## 1. Quando criar

Crie ADR quando uma decisão afetar:

- runtime, módulos-base, módulos de domínio ou distribuições;
- contratos públicos;
- persistência, migração, backup ou exportação;
- segurança, permissões ou níveis de confiança;
- compatibilidade e suporte;
- protocolo ou SDK;
- empacotamento e atualização;
- dependências estruturais;
- testes arquiteturais;
- classificação de capacidade.

## 2. RFC antes da ADR

Quando a decisão ainda estiver aberta e possuir impacto relevante, use RFC primeiro.

A ADR registra o resultado. Ela não substitui a consulta pública obrigatória.

ADRs condicionadas a spike podem permanecer com status **Proposto** até que critérios de aceitação sejam demonstrados.

## 3. Numeração

- use o próximo número disponível;
- não reutilize número de ADR rejeitada, arquivada ou supersedida;
- não renumere ADRs publicadas;
- preserve erratas e histórico;
- use três dígitos: `ADR-022_Tema.md`.

Antes de criar, consulte `00_Indice_Versoes.md` e a lista atual.

## 4. Estados

- **Proposto:** decisão ainda não aceita ou condicionada a evidência;
- **Aceito:** decisão vigente;
- **Rejeitado:** alternativa formalmente recusada;
- **Adiado:** decisão necessária, mas sem evidência ou prioridade suficiente;
- **Supersedido:** substituído por ADR posterior;
- **Obsoleto:** não se aplica mais, preservado para histórico.

## 5. Conteúdo mínimo

Use [`ADR_TEMPLATE.md`](ADR_TEMPLATE.md).

Toda ADR deve conter:

- status e data;
- documentos relacionados;
- contexto;
- decisão;
- consequências positivas e negativas;
- obrigações de execução;
- alternativas consideradas;
- critérios de validação quando aplicável;
- relação de supersessão.

## 6. Alterações

Após aceita, uma ADR não deve ser reescrita silenciosamente para mudar a decisão.

Correções editoriais podem ser feitas sem alterar significado. Mudança normativa exige nova ADR que:

- referencia a anterior;
- explica a alteração;
- preserva histórico;
- atualiza documentos canônicos afetados.

## 7. Implementação

PRs que implementem uma ADR devem:

- referenciar a ADR;
- mapear requisitos e critérios;
- registrar exceções;
- atualizar testes, documentação e compatibilidade;
- não declarar a ADR implementada sem evidência.

## 8. Índice

ADRs canônicas deverão aparecer no índice de versões ou em catálogo próprio quando o volume justificar.
