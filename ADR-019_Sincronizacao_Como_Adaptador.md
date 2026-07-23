# ADR-019 — Sincronização como adaptador

**Status:** Proposto  
**Data:** 2026-07-23  
**Base normativa:** Manifesto OpenCore v1.1  
**Documento relacionado:** Arquitetura OpenCore v1.2 (origem normativa: v1.1)  
**Seções de referência:** §6.5 Matriz de classificação (linha Sincronização) · §20 Sincronização como adaptador · §11.7 Contrato de portabilidade e exclusão (transferências externas)

---

## Contexto

Sincronização multi-dispositivo ou multi-instalação é desejável em muitos cenários, mas depende de protocolo externo, provedor de nuvem, política de conflito e credenciais que variam por implantação. Tratá-la como módulo-base genérico ou como parte do runtime mínimo forçaria dependências externas, telemetria implícita ou acoplamento à entidade OpenCore — em contradição com operação offline-first e soberania de dados.

A Arquitetura v1.1 (§6.5 e §20) classifica sincronização exclusivamente como **integração ou adaptador**: opcional, substituível e dependente de provedor. A primeira distribuição poderá operar integralmente em instalação local única, sem sync.

Esta ADR consolida essa classificação e define limites do que o runtime pode oferecer como contrato neutro compartilhado entre adaptadores.

---

## Decisão

### 1. Classificação normativa

**Sincronização será opcional e classificada como integração ou adaptador**, não como:

- módulo-base genérico obrigatório;
- parte do runtime mínimo;
- capacidade estrutural necessária ao boot.

Qualquer proposta de promover sync ao runtime ou módulo-base deverá seguir ADR-015 (limiar §6.6) e será presumida **rejeitada** enquanto depender de provedor ou política externa específica.

### 2. Contratos neutros no runtime (limitados)

O runtime **poderá** oferecer contratos neutros de identificação, eventos, conflito e integridade **somente** quando comprovadamente úteis a **mais de um** adaptador de sincronização.

O runtime **não** deverá:

- implementar protocolo de sync específico (ex.: proprietário da entidade OpenCore);
- exigir serviço de nuvem para operação local;
- replicar dados de domínio sem mediação do módulo responsável.

### 3. Requisitos para adaptadores de sincronização

Uma solução de sync deverá preservar:

- **operação local** plena quando sync estiver desativado ou indisponível;
- **exportação independente** dos dados (ADR-016), sem depender do adaptador ativo;
- **resolução documentada de conflitos**, incluindo estratégia padrão e opções configuráveis;
- **criptografia apropriada** em trânsito e, quando aplicável, em repouso no destino;
- **controle do usuário** sobre ativação, frequência, escopo e provedor;
- **possibilidade de provedores alternativos** quando viável;
- **ausência de dependência obrigatória** da entidade OpenCore.

### 4. Privacidade, exclusão e inventário externo

Adaptadores de sync deverão integrar-se ao contrato de dados dos módulos que replicam (§11.7):

- declarar quais entidades ou categorias são enviadas a destinos externos;
- identificar dados replicados em relatórios de exportação e exclusão;
- indicar quando exclusão local não remove cópias remotas sem ação adicional.

Módulos de domínio permanecem proprietários dos dados; o adaptador atua sobre contratos e eventos autorizados, não sobre tabelas alheias.

### 5. Escopo da primeira distribuição

A primeira distribuição oficial poderá **não incluir** adaptador de sincronização. Ausência de sync não constitui lacuna arquitetural na Etapa 1.

---

## Consequências

### Positivas

- Instalações offline e air-gapped permanecem first-class.
- Provedores de sync competem ou coexistem sem fork do runtime.
- Matriz arquitetural (ADR-015) permanece coerente: sync ao lado de telemetria e integrações de terceiros.
- Reduz risco de “sync obrigatório” como vetor de lock-in.

### Negativas / custos

- Cada adaptador implementa resolução de conflito e inventário externo — possível duplicação entre provedores.
- Contratos neutros no runtime exigem disciplina para não crescer até virar “sync lite” embutido.
- Usuários que esperam sync out-of-the-box dependerão de adaptador ou distribuição que o empaque.

### Obrigações

1. Manifestos de adaptadores de sync deverão declarar permissões de rede, escopo de dados e provedor.
2. Distribuições que incluam sync deverão documentar comportamento offline e procedimento de desativação.
3. CI deverá impedir imports de sync ou clientes de nuvem dentro do runtime (ADR-020).
4. RFCs futuras de contratos neutros de conflito deverão citar esta ADR e demonstrar uso por ≥ 2 adaptadores.

---

## Alternativas consideradas

| Alternativa | Veredito |
|---|---|
| Sync como módulo-base oficial | Rejeitada — §20 e §6.5 |
| Sync no runtime mínimo | Rejeitada — acopla provedor e viola offline-first |
| Sync implícita via backup | Rejeitada — backup ≠ replicação bidirecional; classificações distintas na matriz |
| Apenas sync proprietária OpenCore | Rejeitada — viola provedores alternativos e §20 |
