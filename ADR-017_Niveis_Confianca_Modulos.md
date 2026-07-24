# ADR-017 — Níveis de confiança de módulos (T0–T3)

**Status:** Proposto
**Data:** 2026-07-23
**Base normativa:** Manifesto OpenCore v1.2
**Documento relacionado:** Arquitetura OpenCore v1.3 (origem normativa: v1.1 §17.2; consolidado em v1.2/v1.3)
**Seções de referência:** §17.2 Níveis de confiança de módulos · §17.3 Código externo · §17.4 Política de segurança

---

## Contexto

O ecossistema OpenCore combina runtime e módulos oficiais, módulos comunitários, spikes experimentais e, eventualmente, edições comerciais certificadas. Permissões técnicas no manifesto controlam **o que** um módulo pode solicitar ao runtime; não comunicam **quem** revisou o código, como o artefato é distribuído nem qual garantia de manutenção existe.

Sem uma taxonomia explícita de confiança, usuários e distribuições correm o risco de tratar módulos experimental, comunitário e oficial como equivalentes — ou de confundir isolamento de processo com sandbox de segurança de sistema operacional. A Arquitetura v1.1 (§17.2) define níveis T0 a T3 com requisitos mínimos e restrições de distribuição.

Esta ADR adota essa taxonomia como decisão normativa e deixa explícito que **certificação comercial** é atributo separado do nível de confiança, e que níveis T não implicam isolamento de OS.

---

## Decisão

### 1. Quatro níveis de confiança

O nível de confiança descreve grau de revisão, manutenção e distribuição. **Não substitui** permissões técnicas nem representa garantia absoluta de segurança.

| Nível | Denominação | Requisitos mínimos | Distribuição permitida |
|---|---|---|---|
| **T0** | Experimental | Código local ou spike, manifesto parcial, sem garantia de manutenção | Apenas desenvolvimento; **nunca** em distribuição oficial |
| **T1** | Comunitário | Código-fonte disponível, manifesto completo, licença identificada, testes mínimos e revisão inicial | Instalação manual ou catálogo comunitário com aviso |
| **T2** | Verificado | Revisão técnica, CI completo, contrato de dados, SBOM, compatibilidade testada e artefato assinado quando distribuído | Catálogo verificado e distribuições não oficiais avaliadas |
| **T3** | Oficial | Mantido sob governança OpenCore, licença compatível com módulo oficial, suporte de versão, auditoria e inclusão em matriz oficial | Distribuições oficiais |

### 2. Regras operacionais

- Instalação automática de **T0** será proibida fora do ambiente de desenvolvimento.
- Builds oficiais iniciais aceitarão apenas módulos **T3** registrados estaticamente.
- Spikes e ambientes de desenvolvimento poderão carregar T0 de forma **explícita e isolada**.
- Promoção entre níveis exige cumprimento cumulativo dos requisitos do nível alvo e registro na matriz ou catálogo correspondente.

### 3. Certificação comercial separada

**Certificação comercial** é atributo distinto do nível de confiança. Um módulo ou edição comercial poderá ser certificado sem tornar-se oficial (T3) ou open source, desde que cumpra contratos públicos aplicáveis de:

- segurança;
- portabilidade e exclusão (ADR-016);
- marca;
- interoperabilidade.

Um módulo T3 oficial não implica certificação comercial; um módulo comercial certificado pode permanecer T1 ou T2 se não atender critérios de governança oficial.

### 4. Isolamento de processo ≠ sandbox de OS

Esta ADR **não** afirma que módulos em processo ou níveis T2/T3 operem em sandbox completa de sistema operacional.

- Isolamento por processo fornece, na v0, **isolamento de falhas** e mediação de APIs pelo runtime — não bloqueio garantido de acesso direto a filesystem, rede ou outros recursos do SO pelo processo filho.
- Níveis de confiança medem revisão, CI, assinatura e contratos — **não** capacidade de executar código não confiável com segurança de OS.
- Execução segura de código não confiável exigirá ADR e mecanismos específicos de sandbox, fora do escopo desta decisão.

Documentação pública e manifestos não deverão usar “T2” ou “processo isolado” como sinônimo de sandbox de segurança.

### 5. Relação com código externo e segurança

Código externo não deverá ser baixado e executado automaticamente na primeira versão (§17.3). Futuros sistemas distribuídos deverão considerar assinatura, origem verificável, checksum, revogação, compatibilidade, permissões, isolamento, auditoria e resposta a vulnerabilidades.

O repositório deverá manter processo separado de relato privado, triagem, correção, divulgação coordenada e avisos de segurança (§17.4).

---

## Consequências

### Positivas

- Expectativas claras para desenvolvedores, distribuidores e usuários finais.
- Distribuições oficiais protegidas contra inclusão acidental de spikes T0.
- Certificação comercial possível sem forçar open source ou status oficial.
- Redução de falsa sensação de segurança associada a “módulo verificado”.

### Negativas / custos

- Manutenção de matriz oficial, catálogos e pipelines de promoção T1→T2→T3.
- Módulos comunitários (T1) permanecem com revisão limitada — exige aviso explícito na instalação.
- Certificação comercial adiciona processo paralelo ao nível de confiança.

### Obrigações

1. Manifestos deverão declarar `trust_level: T0|T1|T2|T3` (ou campo equivalente canonizado).
2. CI deverá bloquear inclusão de T0 em builds de release oficial.
3. Módulos T2 e T3 deverão cumprir contrato de dados testável (ADR-016).
4. Documentação de Module Host e módulos em processo deverá repetir limite isolamento ≠ sandbox.

---

## Alternativas consideradas

| Alternativa | Veredito |
|---|---|
| Apenas permissões do manifesto, sem níveis | Rejeitada — não distingue origem e manutenção |
| Certificação comercial como substituto de T3 | Rejeitada — §17.2 exige separação |
| Sandbox de OS implícita em T2+ | Rejeitada — falsa segurança; requer ADR dedicada |
| T0 permitido em distribuições oficiais com aviso | Rejeitada — §17.2 proíbe |
