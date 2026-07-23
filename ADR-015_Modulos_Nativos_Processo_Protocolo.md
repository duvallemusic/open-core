> **ERRATA DE NUMERAÇÃO — NÃO CANÔNICO.** Este conteúdo foi renumerado para **ADR-021** (`ADR-021_Modulos_Nativos_Processo_Protocolo_v1.1.md`). O identificador ADR-015 pertence à matriz de classificação arquitetural (`ADR-015_Matriz_Classificacao_Arquitetural.md`).

# ADR-015 — Módulos nativos, módulos em processo e protocolo neutro de linguagem

**Versão desta ADR:** 1.1  
**Status:** Proposto, condicionado a spike  
**Data:** 2026-07-23  
**Base normativa:** Manifesto OpenCore v1.1  
**Documento relacionado:** Arquitetura OpenCore v1.0.2 · Roadmap OpenCore v2.1  
**Histórico:** supersede ADR-015 v1 (`ADR-015_Modulos_Nativos_Processo_Protocolo_v1.md`)  
**Substitui / altera:** complementa ADR-005 e ADR-006; não autoriza plugins in-process arbitrários, interpretadores embutidos, nem sandbox completa de OS na Etapa 1

---

## 0. Glossário

| Termo | Significado |
|---|---|
| **OpenCore Runtime** | Processo principal em Rust: inicialização, registro, ciclo de vida, eventos, storage estrutural, segurança, logs, backup e Module Host |
| **Module Host** | Componente do runtime que lança, supervisiona e fala com módulos em processo via protocolo |
| **Módulo nativo** | Componente Rust com `execution.mode: native`, no mesmo processo do runtime |
| **Módulo em processo** | Processo filho isolado com `execution.mode: process`, falando pelo protocolo |
| **Interpretador / runtime da linguagem** | Python, Node ou equivalente usado para executar um módulo em processo — distinto do OpenCore Runtime |
| **OpenCore Module Protocol** | Contrato público versionado (IPC) entre Module Host e módulos em processo |
| **SDK nativo Rust** | API in-process para módulos nativos |
| **SDK de processo** | Binding do protocolo para uma linguagem externa (ex.: Python) |

---

## 1. Contexto

O OpenCore Runtime tem hipótese principal em Rust (ADR-006), justificada por confiabilidade, desempenho, empacotamento e superfície de segurança controlada.

A Arquitetura v1.0 previa módulos preferencialmente como componentes Rust registrados estaticamente. Essa escolha é adequada para o núcleo e para módulos críticos, mas cria tensão com o Manifesto v1.1 (§8 educação, §9 SDK Apache, §7 contratos abertos, §15 padrões profissionais).

Obrigar todos os módulos a serem escritos em Rust maximizaria a barreira de entrada. Carregar Python/JS ou `.dll`/`.so` arbitrários **dentro** do processo do runtime na Etapa 1 violaria robustez sem excesso e aumentaria risco de crash global e ABI frágil.

A revisão v1.1 desta ADR incorpora correções de segurança semântica, nomenclatura, persistência, empacotamento e escopo do Spike 10.

## 2. Decisão

### 2.1 OpenCore Runtime e módulos nativos — Rust

O OpenCore Runtime permanecerá exclusivamente em Rust.

Módulos que exijam integração profunda, alto desempenho, caminho crítico de segurança ou acesso estrutural serão **módulos nativos** (`execution.mode: native`).

Exemplos típicos: autenticação, criptografia, backup estrutural, atualização, permissões, sincronização crítica, drivers e processamento intensivo.

### 2.2 Módulos em processo — processo isolado

Módulos de domínio, integrações, automações e módulos educacionais poderão ser **módulos em processo** (`execution.mode: process`), supervisionados pelo **Module Host**, comunicando-se pelo **OpenCore Module Protocol**.

Hipótese inicial de transporte: **stdio** com framing por comprimento; protocolo lógico: **JSON-RPC 2.0** (ou subconjunto documentado). Socket/named pipe somente se o spike justificar.

### 2.3 Isolamento de falhas ≠ sandbox de segurança (obrigatório)

**Na v0, o isolamento por processo fornece isolamento de falhas, não uma sandbox completa de segurança.**

Módulos em processo deverão ser oficiais, verificados ou explicitamente confiáveis. As permissões do manifesto controlam capacidades disponibilizadas pelas **APIs do OpenCore** (operações solicitadas pelo protocolo), mas **não garantem** bloqueio de acesso direto a recursos do sistema operacional pelo processo filho (arquivo SQLite, filesystem, rede, outros processos, variáveis de ambiente), enquanto o processo rodar com o mesmo usuário do OpenCore Runtime.

A execução de código não confiável exigirá ADR e mecanismos de sandbox de SO específicos (fora do escopo da Etapa 1).

Consequências práticas:

- o Module Host **nega operações não autorizadas solicitadas pelo protocolo**;
- o módulo **não recebe** caminho, handle ou API de acesso direto ao SQLite da distribuição;
- módulos oficiais **utilizam exclusivamente** o serviço de storage do protocolo/runtime;
- isso é **política e arquitetura**, não garantia absoluta de segurança de OS.

### 2.4 Protocolo primeiro; dois contratos de SDK distintos

O artefato público principal para módulos externos é a especificação **OpenCore Module Protocol** (Apache 2.0).

Há dois contratos distintos que não devem ser misturados:

1. **SDK nativo Rust** — API in-process para módulos com `execution.mode: native`;
2. **OpenCore Module Protocol + SDKs de processo** — mensagens IPC para módulos com `execution.mode: process`.

Um binding Rust do protocolo permitiria um *processo externo* em Rust; isso **não** substitui o SDK nativo.

Organização sugerida:

```text
sdk/
├── native-rust/
├── protocol/
├── process-python/      # ou process-typescript/, conforme Spike 10
└── conformance-tests/
```

Níveis de linguagem:

| Nível | Linguagem | Papel |
|---|---|---|
| Tier 1 | Rust (nativo) | runtime e módulos nativos |
| Tier 1 (após evidência) | Python **ou** TypeScript | primeiro SDK de processo |
| Tier 1 (posterior) | a outra entre Python e TS | após estabilizar o primeiro |
| Tier 2 | Go, C# u outras | comunitário |
| Experimental | qualquer linguagem compatível com o protocolo | sem suporte oficial |

Na Etapa 1, o Spike 10 validará **uma** linguagem externa. Preferência inicial desta ADR: **Python**.

### 2.5 Manifesto: `execution`, não `runtime`

O campo canônico é `execution` (evita colisão com OpenCore Runtime e com interpretador da linguagem):

```yaml
execution:
  mode: process          # native | process
  command: python        # executável, sem shell string
  args:
    - main.py
protocol: opencore-module-v1
```

Para nativos:

```yaml
execution:
  mode: native
```

Não usar `entrypoint: "python main.py"` como string de shell — gera ambiguidades de escape e diferenças entre Windows, Linux e macOS.

### 2.6 Persistência mediada — sem SQL genérico

Módulos em processo **não** recebem acesso SQL arbitrário do tipo `storage.execute` com string SQL livre.

O Spike 10 deverá comparar pelo menos:

**Opção A — Banco SQLite por módulo em processo** (inclinação inicial desta ADR)

```text
data/
├── opencore.db
└── modules/
    ├── visitors.db
    └── deliveries.db
```

Vantagens: isolamento estrutural, migrações próprias, menor risco de alterar tabelas alheias, alinhado à ausência de transações entre módulos.

**Opção B — Banco compartilhado com namespace controlado**

O runtime aplica migrações e operações apenas nas estruturas do módulo. Preserva banco único; é mais difícil de fiscalizar com SQLite.

Módulos nativos podem continuar, na Etapa 1, com a regra atual de banco compartilhado + propriedade lógica (ADR-013), independentemente da opção escolhida para processos.

O protocolo deverá expor operações de storage de alto nível (por exemplo: put/get/query tipada, apply_migration declarada), nunca SQL genérico irrestrito.

### 2.7 Interface gráfica

`ui_schema` é **opcional** e **fora do escopo de aceitação do Spike 10**.

O primeiro módulo externo será **headless**: comando, storage, evento, consulta, falha e recuperação.

UI declarativa (tabelas, formulários, rotas) será validada em spike posterior, para não transformar o protocolo em construtor low-code prematuro.

### 2.8 Empacotamento do interpretador

| Estratégia | Uso recomendado |
|---|---|
| Runtime da linguagem no sistema (PATH) | Desenvolvimento e primeiros spikes |
| Runtime da linguagem empacotado | Distribuições oficiais |
| Executável autônomo do módulo | Alternativa para distribuições oficiais |
| Equivalente nativo temporário | Transição, não solução definitiva |

**Distribuições oficiais não deverão exigir que o usuário final instale Python ou Node.** Runtime do sistema não é estratégia final de produto.

### 2.9 Ciclo operacional (backlog do protocolo)

A especificação do protocolo (pós-spike) deverá definir, no mínimo:

- timeout de inicialização;
- timeout de comandos;
- encerramento gracioso;
- cancelamento;
- heartbeat / health check;
- quantidade máxima de reinícios;
- backoff entre reinícios;
- detecção de crash loop;
- limite de tamanho das mensagens;
- comportamento quando o módulo fica indisponível;
- propagação de logs e erros.

Esses itens não bloqueiam a aceitação conceitual desta ADR, mas bloqueiam o SDK v0 estável.

### 2.10 Fora do escopo imediato

- plugins in-process via `.dll` / `.so` / `.dylib` arbitrários;
- interpretador embutido no OpenCore Runtime (PyO3, V8, Wasmtime etc.);
- download e execução automática de módulos não verificados;
- marketplace;
- transações distribuídas entre processos;
- sandbox completa de OS para código não confiável;
- `ui_schema` como critério do Spike 10;
- suporte oficial simultâneo a múltiplas linguagens externas na Etapa 1.

## 3. Relação com o Manifesto v1.1

| Compromisso | Como esta ADR atende |
|---|---|
| Educação com trabalho real (§8) | Módulo de domínio em Python sem exigir Rust na primeira contribuição |
| Padrões profissionais (§8, §15) | Protocolo, permissões, testes, versionamento; sem fingir sandbox inexistente |
| Contratos abertos (§7) | Protocolo documentado e versionado |
| Licenciamento (§9) | Host/ciclo de vida MPL; protocolo + SDKs + templates Apache |
| Offline-first (§6) | Módulo instalado opera localmente; IPC local |
| Soberania de dados (§5) | Storage mediado; sem SQL genérico; exportação permanece obrigação da distribuição |
| Simplicidade / anti-premature (§7) | Processo antes de ABI/interpretador embutido; UI declarativa adiada; um idioma no spike |
| Usuário acima de conveniência educacional (§2) | Isolamento de falhas; módulos oficiais/confiáveis na v0 |

## 4. Consequências

### Positivas

- reduz barreira educacional sem abandonar Rust no núcleo;
- isola falhas de módulos externos;
- deixa explícitos os limites de segurança (evita falsa confiança);
- separa SDK nativo de SDK de processo;
- permite isolamento estrutural de dados (Opção A);
- exige utilidade real na Portaria se aceito.

### Negativas / custos

- latência e memória de processo extra;
- empacotamento do interpretador é obrigatório para produto oficial;
- superfície de protocolo a versionar;
- duas classes de módulo e dois SDKs;
- código “confiável” ainda pode contornar o protocolo no OS até existir sandbox.

### Obrigações de execução

1. Arquitetura ≥ 1.0.2 e Roadmap ≥ 2.1 devem refletir esta v1.1.
2. Spike 10 antes de status Aceito.
3. Se aceito: ≥ 1 módulo de domínio real da Portaria com `execution.mode: process`.
4. Não iniciar segundo SDK de processo em paralelo na Etapa 1.
5. Não tratar permissões do manifesto como sandbox de OS na documentação pública.

## 5. Critérios do Spike 10 (aceitação condicional)

Status só migra para **Aceito** se demonstrado em **macOS + Windows** (Linux desejável):

1. handshake e registro de um módulo em processo (Python preferencial);
2. publicação e consumo de pelo menos um evento;
3. comando request/response via protocolo (stdio + framing por comprimento);
4. **operações não autorizadas solicitadas pelo protocolo são negadas**;
5. crash do módulo sem derrubar o OpenCore Runtime;
6. reinício controlado do módulo;
7. o módulo **não recebe** caminho, handle ou API de acesso direto ao SQLite; módulos oficiais do spike usam exclusivamente o serviço de storage;
8. storage **sem SQL genérico**; comparação documentada Opção A vs B, com recomendação;
9. medição de memória adicional e latência de IPC;
10. empacotamento: PATH ok para spike; plano documentado para runtime empacotado em distribuição oficial;
11. pessoa externa executa o módulo de exemplo só com documentação;
12. **sem UI** — headless apenas.

## 6. Critério de classificação nativo vs processo

**Nativo** quando atender a ao menos uma condição:

- caminho crítico de segurança, integridade ou autenticação;
- latência incompatível com IPC após medição;
- UI Slint específica não representável no esquema futuro;
- módulo-base estrutural compartilhado pelas distribuições oficiais iniciais;
- isolamento em processo comprometeria a consistência local da distribuição.

Caso contrário, após Spike 10 aceito, preferência para domínio/educação: **processo**.

## 7. Alternativas consideradas

| Alternativa | Veredito |
|---|---|
| Todos os módulos em Rust | Rejeitada como regra exclusiva |
| Plugins in-process (ABI) | Rejeitada na Etapa 1 |
| Interpretador embutido | Adiada |
| Templates Rust “simplificados” como única porta educacional | Rejeitada como solução principal |
| Tratar processo como sandbox | Rejeitada — falsa segurança |
| SQL genérico mediado pelo runtime | Rejeitada — recria acesso irrestrito |
| Runtime da linguagem no PATH como estratégia de produto | Rejeitada para distribuições oficiais |

## 8. Decisões derivadas

- Spike 10: Python + stdio + JSON-RPC (subconjunto) + framing por comprimento + headless + storage tipado.
- Spike separado futuro: `ui_schema`.
- Backlog do protocolo: ciclo operacional (§2.9) + suíte de conformidade.
- Atualizar ADR-005: nativo/estático in-process ≠ módulo em processo.

## 9. Changelog desta ADR

| Versão | Descrição |
|---|---|
| 1 | Decisão híbrida inicial; protocolo primeiro; uma linguagem no spike |
| 1.1 | Isolamento ≠ sandbox; campo `execution`; storage sem SQL genérico (A vs B); SDK nativo ≠ SDK processo; empacotamento oficial; UI fora do Spike 10; ciclo operacional no backlog; critérios e glossário revisados |
