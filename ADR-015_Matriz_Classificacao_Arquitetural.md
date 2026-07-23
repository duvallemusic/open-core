# ADR-015 — Matriz de classificação arquitetural

**Status:** Proposto  
**Data:** 2026-07-23  
**Base normativa:** Manifesto OpenCore v1.1  
**Documento relacionado:** Arquitetura OpenCore v1.2 (origem normativa: v1.1)  
**Seções de referência:** §6.4 Critério para entrada no runtime · §6.5 Matriz de classificação · §6.6 Limiar para promoção de capacidades ao runtime

---

## Contexto

O OpenCore organiza-se em runtime estrutural, módulos-base, módulos de domínio, distribuições e adaptadores transversais (Arquitetura §5). Sem critérios explícitos de classificação, capacidades tendem a migrar prematuramente para o runtime — aumentando acoplamento, superfície de segurança e dificuldade de remoção em distribuições específicas.

A Arquitetura v1.1 (§6.5) estabelece uma matriz inicial que distingue três destinos possíveis para cada capacidade: **runtime estrutural**, **módulo-base oficial** e **integração ou adaptador**. Algumas capacidades ocupam posições híbridas: o runtime fornece contratos e orquestração, enquanto implementação, provedores ou interfaces administrativas permanecem modulares.

A promoção de uma capacidade modular ao runtime (§6.6) exige barreira deliberada: RFC, ADR e evidência de necessidade estrutural intrínseca ou de uso transversal comprovado. Antes de existirem duas distribuições de domínios distintos, nenhuma capacidade deverá ser promovida apenas por conveniência.

Esta ADR formaliza a matriz como decisão normativa e fixa exemplos concretos — incluindo atualização, sincronização e autenticação — para orientar RFCs, manifestos e revisões arquiteturais futuras.

---

## Decisão

### 1. Três classes arquiteturais obrigatórias

Toda capacidade nova ou existente deverá ser classificada em exatamente uma das categorias abaixo, podendo o runtime reter apenas **contratos estruturais** sem incorporar a implementação completa:

| Classe | Definição | Critério resumido |
|---|---|---|
| **Runtime estrutural** | Parte mínima, não removível, necessária para boot, integridade ou coordenação uniforme | Independente de domínio; impossível ou inseguro como módulo |
| **Módulo-base oficial** | Capacidade reutilizável entre distribuições, removível ou substituível | Serviço comum; não necessário ao boot mínimo |
| **Integração ou adaptador** | Ponte para protocolos, formatos ou serviços externos | Depende de credencial, provedor ou política externa |

A presença de contratos no runtime **não transforma** a implementação completa da capacidade em parte do runtime (Arquitetura §6.5, parágrafo final).

### 2. Matriz normativa de capacidades

| Capacidade | Runtime estrutural | Módulo-base oficial | Integração ou adaptador | Justificativa |
|---|---:|---:|---:|---|
| Inicialização e encerramento | Sim | Não | Não | Necessário para qualquer distribuição |
| Registro e ciclo de vida de módulos | Sim | Não | Não | Coordenação estrutural uniforme |
| Validação de compatibilidade | Sim | Não | Não | Protege integridade da distribuição |
| Configuração estrutural | Sim | Não | Não | Necessária antes da ativação dos módulos |
| Barramento local de eventos | Sim | Não | Não | Contrato comum entre módulos |
| Coordenação de migrações | Sim | Não | Não | Ordem e integridade globais |
| Persistência de domínio | Não | Sim ou domínio | Não | Dados pertencem ao módulo responsável |
| **Autenticação** | Não | **Sim** | Não | Reutilizável, mas removível em distribuições específicas |
| Permissões de negócio | Não | Sim | Não | Serviço comum consumido por módulos |
| Backup e restauração | Contratos e orquestração | Sim | Adaptadores de destino | Runtime preserva integridade; módulo implementa operação |
| Exportação portátil | Contrato e descoberta | Sim ou domínio | Adaptadores de formato | Cada módulo exporta os próprios dados |
| **Atualização** | Validação, compatibilidade, migração e recuperação | Interface administrativa opcional | Feed, download e provedor de artefatos | Separa segurança estrutural de canais externos |
| **Sincronização** | Não | Não | **Sim** | Depende de protocolo, provedor e política externa |
| Telemetria | Consentimento e bloqueio por padrão | Não | Sim, sempre opcional | Transmissão externa nunca é estrutural obrigatória |
| Relatórios genéricos | Não | Sim | Exportadores externos opcionais | Capacidade reutilizável, não necessária ao boot |
| Integrações de terceiros | Não | Não | Sim | Dependem de serviços e credenciais externos |
| Interface específica | Não | Não | Não | Pertence à distribuição ou ao módulo correspondente |

### 3. Exemplos orientadores

- **Autenticação:** módulo-base removível; o runtime não incorpora regras de identidade ou políticas de sessão de um domínio.
- **Atualização:** o runtime garante validação, compatibilidade, migração e recuperação; feeds, download e provedores comerciais são adaptadores substituíveis (complementa ADR-018).
- **Sincronização:** sempre adaptador; nunca módulo-base genérico nem runtime mínimo (complementa ADR-019).

### 4. Limiar para promoção ao runtime

Uma capacidade modular somente poderá ser promovida ao runtime mediante RFC + ADR e atendendo a **um** dos caminhos:

1. **Necessidade estrutural intrínseca:** indispensável para inicialização, integridade, segurança ou coordenação uniforme; ou
2. **Evidência de uso transversal:** utilizada por pelo menos duas distribuições de domínios distintos e necessária em ≥ 60% das distribuições oficiais mantidas, sem alternativa modular segura e sustentável.

A proposta deverá ainda demonstrar: redução mensurável de duplicação ou risco; ausência de regra de domínio; contrato estável e testado; impacto aceitável sobre tamanho, boot e segurança; plano de migração; e impossibilidade de resolver o problema apenas com contrato, serviço-base ou adaptador.

Antes de existirem duas distribuições distintas, promoções ficam restritas ao caminho de necessidade estrutural intrínseca.

---

## Consequências

### Positivas

- Fronteiras previsíveis entre runtime, módulos-base e adaptadores.
- Distribuições podem omitir autenticação, sincronização ou telemetria sem fork do núcleo.
- Atualização e backup preservam integridade estrutural sem acoplar provedores comerciais.
- RFCs e ADRs futuros partem de uma matriz compartilhada e auditável.

### Negativas / custos

- Classificação inicial exige revisão caso a caso; casos híbridos (backup, exportação, atualização) demandam documentação cuidadosa.
- Capacidades mal classificadas gerarão retrabalho de extração do runtime.
- O limiar de 60% e duas distribuições distintas retardam promoções legítimas até maturidade do ecossistema.

### Obrigações

1. Novas capacidades deverão declarar sua classificação no manifesto ou ADR de introdução.
2. Divergências da matriz exigem ADR de alteração, não decisão informal.
3. Testes de arquitetura no CI (ADR-020) deverão reforçar dependências proibidas por camada conforme esta classificação.
4. A Arquitetura v1.2, quando consolidada, deverá manter esta matriz como referência normativa cruzada.

---

## Alternativas consideradas

| Alternativa | Veredito |
|---|---|
| Classificação implícita por convenção de diretório | Rejeitada — insuficiente para revisão e CI |
| Toda capacidade transversal no runtime | Rejeitada — viola simplicidade e removibilidade |
| Sincronização como módulo-base | Rejeitada — dependência externa e opcionalidade (§20) |
| Promoção ao runtime por voto ou conveniência | Rejeitada — exige RFC, ADR e evidência (§6.6) |
