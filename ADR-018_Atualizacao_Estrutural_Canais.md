# ADR-018 — Atualização estrutural no runtime e canais substituíveis

**Status:** Proposto  
**Data:** 2026-07-23  
**Base normativa:** Manifesto OpenCore v1.1  
**Documento relacionado:** Arquitetura OpenCore v1.2 (origem normativa: v1.1)  
**Seções de referência:** §6.5 Matriz de classificação (linha Atualização) · §19 Atualizações e canal de segurança · §19.1–§19.4

---

## Contexto

O Manifesto classifica atualização como capacidade estrutural: toda distribuição OpenCore precisa atualizar runtime, módulos e esquemas com integridade, compatibilidade e possibilidade de recuperação. Ao mesmo tempo, a Arquitetura v1.1 (§6.5) separa claramente **o que** pertence ao runtime (validação, compatibilidade, migração, recuperação) do **o que** permanece substituível (feed, download, provedor de artefatos, interface administrativa).

Acoplar atualização a um servidor, feed comercial ou atualizador automático proprietário violaria soberania operacional e dificultaria espelhamento por terceiros. A Etapa 1, porém, não exige atualizador automático completo — exige contratos estruturais e um **canal mínimo de segurança** publicamente documentado (§19.3).

Esta ADR formaliza essa separação e fixa o escopo mínimo da primeira versão (§19.4).

---

## Decisão

### 1. Responsabilidade estrutural do runtime

O runtime deverá fornecer contratos e garantias para atualizar com segurança:

- identificação das versões instaladas (runtime, módulos, distribuição);
- validação de compatibilidade entre componentes;
- verificação de integridade e autenticidade de metadados ou artefatos;
- coordenação de backup, migração e recuperação;
- bloqueio de downgrade incompatível;
- registro auditável do resultado;
- possibilidade de continuar na versão local quando a atualização não for obrigatória por necessidade técnica comprovada.

O runtime **não** deverá conter provedor comercial, servidor ou feed específico.

### 2. Canais, cliente e obtenção de artefatos

Consulta a canais, download de pacotes e integração com provedores serão implementados por:

- interface administrativa opcional (módulo-base);
- ferramenta standalone; ou
- adaptador substituível.

O usuário deverá poder:

- desativar consultas automáticas;
- configurar canais compatíveis;
- importar manualmente metadados e pacotes assinados;
- consultar versões instaladas sem enviar telemetria;
- verificar avisos de segurança sem permitir atualização automática.

### 3. Canal mínimo de atualização de segurança

Antes de existir atualizador automático, cada versão suportada deverá possuir canal mínimo e **publicamente documentado** contendo:

- matriz de versões suportadas;
- avisos de segurança com identificador e severidade;
- versões afetadas e corrigidas;
- notas de versão;
- checksums e assinaturas dos artefatos publicados;
- instruções de atualização e recuperação;
- opção de download manual;
- formato legível por máquina para futura integração com adaptadores.

O formato deverá permitir **espelhamento por terceiros** e importação manual, evitando que um único servidor da entidade OpenCore seja requisito para continuidade operacional.

### 4. Escopo da Etapa 1 (§19.4)

Na primeira versão, o objetivo será validar:

- identificação de componentes instalados;
- pacote assinado ou checksum verificável;
- compatibilidade e atualização de esquema;
- backup prévio e recuperação;
- leitura local ou remota de aviso de segurança de teste.

Um atualizador automático completo poderá ser adiado até maturidade do formato de distribuição e da política de compatibilidade.

### 5. Classificação arquitetural

Conforme ADR-015 e §6.5:

| Aspecto | Classificação |
|---|---|
| Validação, compatibilidade, migração, recuperação | Runtime estrutural |
| Interface administrativa de atualização | Módulo-base opcional |
| Feed, download, provedor de artefatos | Integração ou adaptador |

---

## Consequências

### Positivas

- Atualização de segurança possível offline ou via espelho, sem dependência de telemetria.
- Provedores alternativos e importação manual preservam soberania.
- Runtime permanece enxuto e testável; canais evoluem independentemente.
- Alinhamento explícito com Manifesto (capacidade estrutural) sem monopolizar distribuição.

### Negativas / custos

- Etapa 1 exige processos manuais ou semi-manuais de atualização.
- Manutenção contínua do canal de segurança por versão suportada.
- Assinatura, checksum e matriz de compatibilidade aumentam complexidade de release.

### Obrigações

1. Releases oficiais deverão publicar artefato do canal mínimo junto com binários.
2. Adaptadores de feed deverão consumir formato legível por máquina documentado.
3. Downgrade incompatível deverá ser bloqueado pelo runtime com mensagem auditável.
4. Testes de arquitetura no CI deverão impedir dependência de domínio ou provedor comercial dentro do runtime (ADR-020).

---

## Alternativas consideradas

| Alternativa | Veredito |
|---|---|
| Atualizador automático obrigatório na v1 | Rejeitada — §19.4 adia até maturidade |
| Feed único centralizado sem espelhamento | Rejeitada — viola continuidade e §19.3 |
| Toda lógica de atualização no runtime | Rejeitada — acopla provedores e UI |
| Atualização apenas manual sem canal de segurança | Rejeitada — insuficiente para resposta a vulnerabilidades |
