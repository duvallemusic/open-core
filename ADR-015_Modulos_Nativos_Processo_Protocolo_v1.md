> **ERRATA DE NUMERAÇÃO — NÃO CANÔNICO.** Este conteúdo foi renumerado para **ADR-021** (`ADR-021_Modulos_Nativos_Processo_Protocolo_v1.1.md`). O identificador ADR-015 pertence à matriz de classificação arquitetural (`ADR-015_Matriz_Classificacao_Arquitetural.md`).

# ADR-015 — Módulos nativos, módulos em processo e protocolo neutro de linguagem (v1)

**Status:** Histórico — supersedida pela v1.1  
**Data:** 2026-07-23  
**Base normativa:** Manifesto OpenCore v1.1  
**Documento relacionado:** Arquitetura OpenCore v1.0.1  
**Substitui / altera:** complementa ADR-005 e ADR-006; não autoriza plugins in-process arbitrários nem interpretadores embutidos na Etapa 1  
**Nota:** snapshot preservado para histórico de decisões.

---

## 1. Contexto

O runtime OpenCore tem hipótese principal em Rust (ADR-006), justificada por confiabilidade, desempenho, empacotamento e superfície de segurança controlada.

A Arquitetura v1.0 previa, na primeira versão, módulos preferencialmente como componentes Rust registrados estaticamente. Essa escolha é adequada para o núcleo e para módulos críticos, mas cria tensão com compromissos do Manifesto v1.1:

- educação e contribuição por desenvolvedores em formação (§8);
- trabalho em módulos isolados sem compreender toda a plataforma (Roadmap v2);
- SDK e materiais educacionais sob Apache 2.0, separados do patrimônio MPL 2.0 (§9);
- contratos abertos entre módulos, substituíveis sem conhecimento privado (§7);
- padrões profissionais mantidos — a abertura educacional não reduz qualidade, testes nem segurança (§8, §15).

Obrigar todos os módulos a serem escritos em Rust maximizaria a barreira de entrada exatamente onde o projeto busca ampliar participação verificável.

Ao mesmo tempo, carregar Python, JavaScript ou bibliotecas dinâmicas arbitrárias **dentro** do processo do runtime na Etapa 1 violaria o princípio de robustez sem excesso (§7) e aumentaria riscos de ABI, crash global e superfície de ataque — riscos já rejeitados pela Arquitetura v1 para plugins dinâmicos prematuros.

## 2. Decisão

O OpenCore adotará um modelo híbrido de módulos, com as seguintes regras.

### 2.1 Runtime e módulos nativos — Rust

O runtime permanecerá exclusivamente em Rust.

Módulos que exijam integração profunda, alto desempenho, caminho crítico de segurança ou acesso estrutural ao runtime serão **módulos nativos** em Rust, integrados ao binário da distribuição ou registrados de forma controlada no processo do runtime.

Exemplos típicos: autenticação, criptografia, backup estrutural, atualização, permissões, sincronização crítica, drivers e processamento intensivo.

### 2.2 Módulos externos — processo isolado

Módulos de domínio, integrações, automações e módulos educacionais poderão ser executados como **processos separados**, comunicando-se com o runtime por um **protocolo local versionado** (hipótese inicial: JSON-RPC ou equivalente sobre IPC local — stdio, socket Unix/named pipe; HTTP local somente se o spike justificar).

Exemplos típicos: visitantes, entregas, estoque, relatórios personalizados, importação de planilhas, integrações externas e exercícios de curso.

### 2.3 Protocolo primeiro; SDKs depois

O artefato principal do SDK não será “uma API Rust”, e sim a especificação **OpenCore Module Protocol** (nome provisório), independente de linguagem.

Bindings oficiais sob Apache 2.0 facilitarão o uso do protocolo. Níveis iniciais de suporte:

| Nível | Linguagem | Papel |
|---|---|---|
| Tier 1 | Rust | runtime e módulos nativos |
| Tier 1 (após evidência) | Python **ou** TypeScript/JavaScript | primeiro host externo oficial |
| Tier 1 (posterior) | a segunda entre Python e TypeScript | após estabilização do primeiro host |
| Tier 2 | Go, C# u outras | comunitário, sem suporte oficial imediato |
| Experimental | qualquer linguagem compatível com o protocolo | sem garantia de suporte |

Na Etapa 1, o spike validará **uma** linguagem externa, não três SDKs completos.

### 2.4 Persistência e permissões

Módulos em processo **não** abrirão diretamente o SQLite compartilhado da distribuição.

Leitura, escrita, transações e migrações declaradas ocorrerão por APIs autorizadas do runtime, conforme permissões do manifesto do módulo.

O manifesto declarará capacidades mínimas (eventos, storage, filesystem, network, UI). O runtime concederá ou negará essas capacidades.

### 2.5 Interface gráfica de módulos externos

Na fase inicial, módulos em processo **não** injetarão código Slint arbitrário.

Contribuições de UI externa usarão declarações controladas (rotas, formulários, tabelas e ações) renderizadas por componentes oficiais do runtime/distribuição.

Interfaces altamente específicas permanecerão em módulos nativos Rust/Slint até existir evidência e contrato estável para extensão visual mais rica.

### 2.6 O que permanece fora do escopo imediato

- Modelo A: plugins in-process via `.dll` / `.so` / `.dylib` arbitrários;
- Modelo B: interpretador Python/JS/Wasm embutido no runtime;
- download e execução automática de módulos não verificados;
- marketplace;
- transações distribuídas entre processos;
- promessa oficial de “qualquer linguagem” com suporte pleno.

## 3. Relação com o Manifesto v1.1

| Compromisso | Como esta ADR atende |
|---|---|
| Educação com trabalho real (§8) | Módulo de domínio em Python/TS sem exigir Rust na primeira contribuição |
| Padrões profissionais (§8, §15) | Mesmo protocolo, permissões, testes, versionamento e revisão |
| Contratos abertos (§7) | Protocolo documentado e versionado como contrato público |
| Licenciamento (§9) | Host/ciclo de vida MPL; protocolo + SDKs + templates Apache |
| Offline-first (§6) | Módulo já instalado opera localmente; IPC é local, não nuvem |
| Soberania de dados (§5) | Storage mediado; exportação/portabilidade continuam obrigação da distribuição |
| Simplicidade / anti-premature (§7) | Processo isolado antes de ABI ou interpretador embutido; um idioma no spike |
| Usuário acima de conveniência educacional (§2) | Crash de módulo externo não derruba o runtime; permissões limitam dano |

## 4. Consequências

### Positivas

- reduz barreira educacional sem abandonar Rust no núcleo;
- isola falhas de módulos externos;
- facilita testes e reinício individual;
- alinha SDK com Apache 2.0 e materiais de ensino;
- reforça fronteiras de dados (mitiga isolamento apenas aparente);
- permite que a Portaria prove a plataforma com pelo menos um módulo de domínio externo.

### Negativas / custos

- latência maior que chamada in-process;
- memória adicional por processo;
- necessidade de política clara de empacotamento do interpretador (embutido vs detectado no PATH);
- superfície de protocolo que precisa versionar com cuidado;
- duas classes de módulo exigem documentação e critérios de classificação explícitos;
- UI declarativa limita expressividade inicial dos módulos externos.

### Obrigações de execução

1. Registrar esta decisão na Arquitetura e na tabela de ADRs.
2. Executar spike dedicado antes de tratar o modelo como aceito.
3. Na distribuição de referência, classificar cada módulo como `native` ou `process` com justificativa.
4. Incluir pelo menos um módulo de domínio real da Portaria como `process` se o spike for bem-sucedido — evitar que externos existam só como “exemplo didático”.
5. Não iniciar SDK Python e TypeScript em paralelo na Etapa 1.

## 5. Critérios do spike (aceitação condicional)

O status desta ADR só poderá migrar para **Aceito** se o spike demonstrar, em ao menos dois sistemas operacionais entre Windows, Linux e macOS:

1. handshake e registro de um módulo em processo;
2. publicação e consumo de pelo menos um evento;
3. comando request/response via protocolo;
4. negação de permissão não declarada;
5. crash do módulo sem derrubar o runtime;
6. reinício controlado do módulo;
7. ausência de abertura direta do arquivo SQLite pelo processo filho;
8. medição de memória adicional e latência de IPC;
9. caminho de empacotamento documentado (runtime embutido **ou** pré-requisito explícito);
10. pessoa externa consegue executar o módulo de exemplo só com documentação.

## 6. Critério de classificação nativo vs processo

Um módulo deverá ser **nativo** quando atender a ao menos uma condição:

- estiver no caminho crítico de segurança, integridade ou autenticação;
- exigir latência incompatível com IPC após medição;
- precisar de UI Slint específica não representável no esquema declarativo;
- for módulo-base estrutural compartilhado por todas as distribuições oficiais iniciais;
- não puder ser isolado sem comprometer a consistência local da distribuição.

Caso contrário, a preferência inicial para módulos de domínio e educacionais será **processo**, desde que o spike tenha sido aceito.

## 7. Empacotamento do interpretador

Distribuições oficiais que incluam módulos `process` deverão declarar explicitamente uma das estratégias:

1. **Runtime embutido:** a distribuição empacota o interpretador suportado (maior tamanho, maior previsibilidade);
2. **Runtime do sistema:** exige versão mínima detectada no ambiente (menor pacote, maior atrito de onboarding);
3. **Somente desenvolvimento:** módulos `process` habilitados em modo dev; a distribuição publicada usa equivalente nativo até o empacotamento estar maduro.

A estratégia (3) é aceitável apenas como transição e deve ser comunicada com clareza — não pode fingir suporte educacional completo em produção.

## 8. Alternativas consideradas

### A — Todos os módulos em Rust

Rejeitada como regra exclusiva: conflita com §8 e com a proposta de SDK educacional, embora permaneça válida para runtime e nativos críticos.

### B — Plugins in-process (ABI dinâmica)

Rejeitada para a Etapa 1: risco de crash global, ABI frágil e complexidade multiplataforma prematura.

### C — Interpretador embutido (PyO3, V8, Wasmtime etc.)

Adiada: aumenta tamanho, superfície de segurança e custo de manutenção antes de existir protocolo estável.

### D — Apenas templates Rust “simplificados” para alunos

Rejeitada como solução principal: reduz a barreira só parcialmente e não aproveita ecossistemas Python/JS nem o modelo Apache de SDKs.

## 9. Decisões derivadas imediatas

- Criar backlog do **Spike 10 — Módulo externo em processo**.
- Estender o manifesto de módulo v0 com `runtime: native | process`, `language`, `entrypoint` e `protocol`.
- Atualizar ADR-005 para deixar explícito que “registro estático / nativo” continua a regra para in-process; módulos em processo são classe distinta, não plugins dinâmicos in-process.
- Adiar interpretador embutido e ABI pública para ADRs futuras, se houver evidência.

## 10. Notas de revisão

Enquanto o spike não concluir, esta ADR permanece **Proposto, condicionado a spike**. Implementações definitivas da Portaria não devem depender do host multilíngue até a aceitação formal.
