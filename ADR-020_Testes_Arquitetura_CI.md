# ADR-020 — Testes de arquitetura no CI

**Status:** Proposto  
**Data:** 2026-07-23  
**Base normativa:** Manifesto OpenCore v1.1  
**Documento relacionado:** Arquitetura OpenCore v1.2 (origem normativa: v1.1)  
**Seções de referência:** §23.4 Testes de arquitetura no CI · §11.7 Contrato de portabilidade e exclusão · §17.2 Níveis de confiança · §6.5 Matriz de classificação

---

## Contexto

Fronteiras arquiteturais do OpenCore — runtime versus módulos, propriedade de dados, licenças MPL/Apache, permissões declaradas versus uso real — degradam rapidamente se dependem apenas de revisão humana. A Arquitetura v1.1 (§23.4) exige que o CI verifique progressivamente regras estruturais, complementando ADRs sobre classificação (ADR-015), contratos de dados (ADR-016), níveis de confiança (ADR-017), atualização (ADR-018) e sync como adaptador (ADR-019).

Spikes e Etapa 1 podem começar com subconjunto de regras, mas cada exceção deverá ser temporária, rastreada e substituída por verificação automatizada ou checklist obrigatório com responsável identificado.

---

## Decisão

### 1. Princípio geral

**As fronteiras arquiteturais não deverão depender apenas de revisão humana.** O pipeline de CI deverá, de forma incremental e falha-bloqueante onde viável, verificar conformidade com decisões registradas em ADRs e na Arquitetura v1.1.

Quando uma regra não puder ser verificada estaticamente, deverá existir **teste automatizado de integração** ou **checklist obrigatório de revisão** com responsável nomeado e registro no PR.

### 2. Catálogo de verificações obrigatórias (progressivo)

O CI deverá evoluir para cobrir, no mínimo:

| Área | Verificação |
|---|---|
| **Camadas e dependências** | Imports e dependências proibidas por camada (runtime ↔ domínio ↔ adaptador) |
| **Acoplamento** | Ausência de ciclos proibidos entre módulos |
| **Propriedade de dados** | Proibição de acesso direto a tabelas ou migrações de outro módulo |
| **Manifesto** | Presença, schema válido e campos obrigatórios (incl. permissões e trust level) |
| **Compatibilidade** | Coerência de versões declaradas (runtime, módulo, distribuição) |
| **Permissões** | Divergência entre permissões declaradas e capacidades utilizadas (rede, storage, eventos) |
| **Rede** | Acesso à rede sem declaração explícita no manifesto |
| **Licenciamento** | Fronteiras MPL 2.0 e Apache 2.0 por diretório ou crate |
| **Contrato de dados** | Presença do contrato em módulos persistentes (ADR-016) |
| **Portabilidade / exclusão** | Execução dos testes de exportação e exclusão obrigatórios |
| **Runtime puro** | Proibição de dependência de domínio ou adaptador específico dentro do runtime |
| **Distribuições oficiais** | Proibição de componente proprietário essencial não certificado; bloqueio de T0 em release |
| **Classificação** | Sinais heurísticos ou allowlists: sync/telemetria/feed fora do runtime (ADR-015, ADR-018, ADR-019) |

### 3. Fases de adoção

**Fase A — Etapa 1 mínima (obrigatória antes de release oficial):**

- validação de manifesto;
- testes de exportação/exclusão em módulos persistentes;
- bloqueio de T0 em builds de release;
- verificação básica de licença por diretório raiz;
- smoke tests em sistemas suportados (complemento §23.3).

**Fase B — Endurecimento:**

- detecção de imports proibidos runtime→domínio;
- verificação permissões vs uso estático (rede, filesystem);
- ausência de ciclos entre módulos na matriz de dependências.

**Fase C — Maturidade:**

- conformidade completa da tabela §23.4;
- SBOM e artefato assinado para T2/T3;
- regras de proprietário/certificação em distribuições oficiais.

Transição entre fases deverá ser registrada em ADR ou changelog de CI, não apenas tacitamente.

### 4. Exceções e débito técnico

Exceções temporárias deverão:

- referenciar issue ou RFC com prazo;
- restringir-se a crate, módulo ou regra específica;
- falhar visivelmente (warn → error) conforme prazo expira.

Revisão humana substituta exige checklist publicado, item assinado no PR e auditoria periódica.

### 5. Relação com outros ADRs

| ADR | Reforço no CI |
|---|---|
| ADR-015 | Dependências por camada; domínio fora do runtime |
| ADR-016 | Contrato de dados + testes exportação/exclusão |
| ADR-017 | trust_level; T0 bloqueado em release; requisitos T2/T3 |
| ADR-018 | Provedor/feed de atualização fora do runtime |
| ADR-019 | Clientes de sync/nuvem apenas em adaptadores |

---

## Consequências

### Positivas

- Fronteiras arquiteturais enforced de forma repetível e auditável.
- Regressões de acoplamento detectadas antes de merge.
- Contratos LGPD e portabilidade deixam de ser aspiracionais.
- Onboarding de contribuidores alinhado a regras explícitas, não folklore.

### Negativas / custos

- Investimento contínuo em linters, testes de arquitetura e manutenção de allowlists.
- Falsos positivos em detecção estática exigem tuning e exceções rastreadas.
- Módulos legados podem bloquear CI até adequarem manifesto e contratos.

### Obrigações

1. Todo repositório runtime, módulo oficial e distribuição deverá integrar job de arquitetura no CI.
2. Módulos persistentes sem testes de exportação/exclusão não progridem além de T1.
3. Novas regras §23.4 deverão ter owner e data-alvo de implementação automática.
4. Resultados falhos de arquitetura deverão ser tratados como falha de merge, salvo exceção registrada.

---

## Alternativas consideradas

| Alternativa | Veredito |
|---|---|
| Apenas code review para fronteiras | Rejeitada — §23.4 exige CI progressivo |
| Big-bang: todas as regras no dia zero | Rejeitada — inviável na Etapa 1; preferir fases A→C |
| Testes manuais sem checklist | Rejeitada — não rastreável |
| Regras só no runtime, não nos módulos | Rejeitada — contratos e permissões são por módulo |
