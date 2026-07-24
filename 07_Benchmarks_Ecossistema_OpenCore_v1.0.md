# Benchmarks do Ecossistema OpenCore — Versão 1.0

**Status:** Referência não normativa  
**Data:** 2026-07-24  
**Natureza:** padrões observados; não é especificação, ADR nem manifesto  
**Documentos relacionados:** Manifesto · Arquitetura · Comunidade e Governança · Plano Institucional · ADR-017 · ADR-022 (quando existir) · Especificação OpenCore Builder  
**Finalidade:** registrar padrões de produtos e ecossistemas open source relevantes para orientar o OpenCore sem copiar código, textos extensos ou estruturas protegidas.

---

## 0. Aviso de uso

Este documento:

- descreve padrões arquiteturais, de produto, licenciamento e comunidade observados publicamente;
- **não** autoriza cópia de código, documentação extensa, assets, marcas ou schemas proprietários;
- **não** afirma superioridade absoluta de nenhum projeto;
- **não** inclui estatísticas de usuários sem fonte, data e natureza (autodeclarada ou auditada);
- trata concorrentes e referências como **benchmarks de padrões**, não como dependências do runtime.

Qualquer número eventualmente citado deve trazer data, origem e se é autodeclarado. Na ausência de fonte verificável nesta versão, números de adoção **não** são incluídos.

---

## 1. Critérios de análise

Para cada projeto:

| Campo | Pergunta |
|---|---|
| Categoria | Que tipo de produto é? |
| Arquitetura | Como se organiza (monólito, módulos, web, desktop, etc.)? |
| Licenciamento | Qual modelo e quais restrições práticas? |
| Público | Quem é o usuário principal? |
| Pontos fortes | O que funciona bem no próprio domínio? |
| Pontos fracos vs OpenCore | Onde o padrão conflita com offline-first, soberania, instalação leiga ou abertura integral? |
| Padrão a estudar | O que o OpenCore pode absorver conceitualmente? |
| Padrão a NÃO incorporar | O que deve ser rejeitado ou adiado? |
| Interoperabilidade | Há caminho realista de adaptador/importação? |
| Risco jurídico de copiar código | Quão sensível é reutilizar implementação? |
| Apelo de marketing | Como o produto se comunica com o usuário final? |

---

## 2. Projetos de referência

### 2.1 Tryton

| Campo | Observação |
|---|---|
| **Categoria** | ERP modular open source |
| **Arquitetura** | Cliente–servidor; módulos Python com dependências declaradas; núcleo e módulos de domínio |
| **Licenciamento** | GPL (núcleo e módulos oficiais tipicamente GPL) |
| **Público** | Organizações que aceitam implantação técnica e personalização |
| **Pontos fortes** | Modularidade madura; declaração de dependências; separação clara entre núcleo e módulos; tradição de extensibilidade |
| **Pontos fracos vs OpenCore** | Modelo servidor + cliente técnico; instalação e operação pouco amigáveis a leigos; GPL no núcleo entra em tensão com a política MPL/Apache do OpenCore se houver cópia; não é offline-first nativo desktop |
| **Padrão a estudar** | Manifestos/metadados de módulo; grafo de dependências; ciclo de ativação de módulos |
| **Padrão a NÃO incorporar** | Exigir servidor como arquitetura única; importar código GPL para o monorepo sem análise por componente |
| **Interoperabilidade** | Adaptador de importação futuro (dados mestres, movimentos) é desejável; não embutir Tryton no runtime |
| **Risco jurídico de copiar código** | **Alto** — GPL; reutilização de código exige análise de copyleft e fronteiras de licença |
| **Apelo de marketing** | Foco em profissionalismo ERP e extensibilidade; pouco discurso de “instale em três cliques” |

---

### 2.2 Frappe / ERPNext

| Campo | Observação |
|---|---|
| **Categoria** | Framework de aplicações (Frappe) + ERP (ERPNext) |
| **Arquitetura** | Stack web (Python/JS); apps/módulos; scaffolding e CLI fortes; banco compartilhado típico de frameworks web |
| **Licenciamento** | GPL (ERPNext e partes do ecossistema) |
| **Público** | Empresas médias, implementadores e desenvolvedores confortáveis com self-hosting web |
| **Pontos fortes** | Experiência do desenvolvedor; CLI e scaffolding; docs e comunidade ativa; catálogo de apps |
| **Pontos fracos vs OpenCore** | Web/server como requisito; instalação frequentemente complexa (containers, serviços); Community vs oferta comercial pode criar percepção de edição limitada; GPL |
| **Padrão a estudar** | CLI (`bench`-like conceitualmente); scaffolds; docs geradas; onboarding de contribuidores |
| **Padrão a NÃO incorporar** | Obrigatoriedade de stack web/servidor; Community “limitada” como modelo das distribuições oficiais OpenCore |
| **Interoperabilidade** | Adaptador de importação ERPNext é candidato natural (CSV/JSON/APIs documentadas), sempre dry-run e relatório de inconsistências |
| **Risco jurídico de copiar código** | **Alto** — GPL; estudar padrões, não portar código |
| **Apelo de marketing** | “ERP completo”, cloud opcional da empresa mantenedora, apps; linguagem mais técnica que a de um instalador leigo |

---

### 2.3 Odoo

| Campo | Observação |
|---|---|
| **Categoria** | Suite empresarial modular (ERP/CRM/etc.) |
| **Arquitetura** | Monólito modular web; apps/módulos; forte organização de catálogo por resultado de negócio |
| **Licenciamento** | Modelo dual / Community vs Enterprise (detalhes variam por versão e módulo; verificar sempre a licença do componente) |
| **Público** | Ampla faixa: PMEs a empresas maiores, com rede grande de parceiros |
| **Pontos fortes** | Descoberta por necessidade (“quero vender”, “quero estoque”); catálogo orientado a resultado; rede de prestadores; UX relativamente acessível para ERP |
| **Pontos fracos vs OpenCore** | Dependência frequente de servidor/web; percepção de Community limitada frente a Enterprise; risco de lock-in de ecossistema e módulos pagos; não alinha com offline-first desktop nativo |
| **Padrão a estudar** | Catálogo por capacidade/resultado; rede de parceiros; páginas por necessidade de negócio |
| **Padrão a NÃO incorporar** | Edição aberta artificialmente incompleta; condicionar capacidades essenciais a nuvem ou Enterprise; marketplace prematuro sem confiança |
| **Interoperabilidade** | Adaptador de importação Odoo é prioridade de ecossistema (muito pedido em migrações) |
| **Risco jurídico de copiar código** | **Alto** — licenças mistas e módulos Enterprise; nunca copiar código Enterprise; Community exige verificação por arquivo |
| **Apelo de marketing** | Extremamente forte em SEO por segmento e resultado (“CRM gratuito”, apps por indústria) |

---

### 2.4 OpenConcerto

| Campo | Observação |
|---|---|
| **Categoria** | ERP/gestão open source com ênfase em operação local |
| **Arquitetura** | Aplicação desktop/Java; operação local típica; modelo de sustentabilidade por serviços |
| **Licenciamento** | GPL (verificar versão e componentes) |
| **Público** | PMEs francófonas e organizações que preferem software instalado localmente |
| **Pontos fortes** | Operação local; narrativa de soberania prática; sustentabilidade por serviços (não só SaaS) |
| **Pontos fracos vs OpenCore** | Ecossistema e UX menos globais; GPL; stack distinta do runtime Rust proposto; instalação ainda pode exigir perfil técnico |
| **Padrão a estudar** | Sustentabilidade por implantação/suporte; posicionamento “local first” de produto |
| **Padrão a NÃO incorporar** | Copiar domínio fiscal/contábil específico sem conformidade local própria |
| **Interoperabilidade** | Adaptador de importação possível para migração regional |
| **Risco jurídico de copiar código** | **Alto** — GPL |
| **Apelo de marketing** | Ênfase em software livre instalado e independência de nuvem obrigatória |

---

### 2.5 Dolibarr

| Campo | Observação |
|---|---|
| **Categoria** | ERP/CRM web modular para PMEs |
| **Arquitetura** | PHP/web; módulos ativáveis; implantação self-hosted relativamente acessível no universo web |
| **Licenciamento** | GPL |
| **Público** | Pequenas e médias empresas, associações, prestadores locais |
| **Pontos fortes** | Baixa barreira relativa entre ERPs web; módulos; comunidade e documentação em várias línguas |
| **Pontos fracos vs OpenCore** | Requer servidor/web; não é offline-first nativo; fronteiras de dados entre módulos menos rígidas que o contrato OpenCore |
| **Padrão a estudar** | Ativação de módulos por necessidade; tom de comunicação para PME |
| **Padrão a NÃO incorporar** | Assumir PHP/LAMP como runtime; misturar dados sem contratos de módulo |
| **Interoperabilidade** | Adaptador de importação Dolibarr é candidato (muito usado em PME) |
| **Risco jurídico de copiar código** | **Alto** — GPL |
| **Apelo de marketing** | “ERP/CRM para pequenas empresas”, gratuito/self-hosted |

---

### 2.6 NocoBase

| Campo | Observação |
|---|---|
| **Categoria** | Plataforma no-code/low-code open source |
| **Arquitetura** | Microkernel / plugins; ciclo de vida de plugins; composição de aplicações |
| **Licenciamento** | Modelo open source com edições/comercial (verificar componentes e edição usada) |
| **Público** | Equipes que montam apps internos sem programar tudo do zero |
| **Pontos fortes** | Ciclo de vida de plugin claro; composição; DX de extensão |
| **Pontos fracos vs OpenCore** | Foco em construção de apps, não em distribuições de domínio instaláveis offline; risco de virar “construtor genérico” — fora do escopo inicial OpenCore |
| **Padrão a estudar** | Ciclo de vida (instalado → ativo → desativado → removido); microkernel conceitual; metadados de plugin |
| **Padrão a NÃO incorporar** | Low-code genérico público no MVP; geração arbitrária de apps sem matriz de compatibilidade |
| **Interoperabilidade** | Baixa prioridade como origem de migração ERP; útil como referência de ciclo de vida |
| **Risco jurídico de copiar código** | **Médio a alto** — verificar licença por pacote/edição antes de qualquer reuso |
| **Apelo de marketing** | “Construa seu sistema”, plugins, no-code |

---

### 2.7 Apache OFBiz

| Campo | Observação |
|---|---|
| **Categoria** | Framework ERP / automação empresarial (Apache) |
| **Arquitetura** | Serviços, entidades de dados, eventos e UI separados conceitualmente; Java |
| **Licenciamento** | Apache License 2.0 |
| **Público** | Desenvolvedores e organizações que customizam fortemente |
| **Pontos fortes** | Separação dados / serviços / eventos / interface; licença permissiva alinhável a SDKs; maturidade conceitual |
| **Pontos fracos vs OpenCore** | Curva de aprendizado alta; UX e instalação distantes de leigos; percepção de complexidade “enterprise” |
| **Padrão a estudar** | Contrato: dados no módulo, serviços expõem operações, eventos notificam, UI consome serviços |
| **Padrão a NÃO incorporar** | Complexidade de framework sem encapsular para o usuário final; jargão OFBiz no produto |
| **Interoperabilidade** | Possível via exportações/custom, menor prioridade que Odoo/ERPNext/Dolibarr |
| **Risco jurídico de copiar código** | **Baixo a médio** — Apache 2.0; ainda assim preferir reimplementar padrões, não copiar trechos grandes |
| **Apelo de marketing** | Técnico/Apache; pouco marketing de PME leiga |

---

### 2.8 Moodle

| Campo | Observação |
|---|---|
| **Categoria** | LMS (aprendizado) open source |
| **Arquitetura** | PHP/web; ecossistema grande de plugins; governança e diretórios de extensão |
| **Licenciamento** | GPL |
| **Público** | Escolas, universidades, treinamentos corporativos |
| **Pontos fortes** | Governança de plugins; comunidade educacional; processo de revisão/confiança de extensões |
| **Pontos fracos vs OpenCore** | Domínio diferente (LMS ≠ gestão offline); web-centric; GPL |
| **Padrão a estudar** | Políticas de plugin; níveis de revisão; formação de mantenedores em comunidade educacional |
| **Padrão a NÃO incorporar** | Modelo de plugin sem matriz de compatibilidade explícita para combinações |
| **Interoperabilidade** | Não prioritária para dados ERP; útil para padrões de comunidade |
| **Risco jurídico de copiar código** | **Alto** — GPL |
| **Apelo de marketing** | Educação, instituições, plugins |

---

### 2.9 OpenMRS

| Campo | Observação |
|---|---|
| **Categoria** | Prontuário eletrônico / saúde open source |
| **Arquitetura** | Modular; forte orientação a implementação em campo; formação de implementadores |
| **Licenciamento** | MPL 2.0 (núcleo tipicamente; verificar módulos) |
| **Público** | Sistemas de saúde, ONGs, implementações em países em desenvolvimento |
| **Pontos fortes** | Comunidade orientada a problemas reais; formação de mantenedores/implementadores; licença MPL alinhável ao núcleo OpenCore |
| **Pontos fracos vs OpenCore** | Domínio clínico regulado; não transferir regras de saúde para o runtime genérico |
| **Padrão a estudar** | Comunidade por problema real; mentoria e promoção de mantenedores; módulos com ownership |
| **Padrão a NÃO incorporar** | Assumir conformidade clínica/regulatória genérica no núcleo |
| **Interoperabilidade** | Fora do foco inicial de migração comercial PME |
| **Risco jurídico de copiar código** | **Médio** — MPL é copyleft de arquivo; reuso exige respeito a arquivos MPL e avisos |
| **Apelo de marketing** | Impacto social, saúde global, implementação |

---

### 2.10 Appsmith

| Campo | Observação |
|---|---|
| **Categoria** | Plataforma low-code para apps internos |
| **Arquitetura** | Builder visual + conectores; runtime de apps gerados |
| **Licenciamento** | Apache 2.0 (projeto principal; verificar edições/cloud) |
| **Público** | Equipes internas que montam painéis e fluxos sobre APIs/bancos |
| **Pontos fortes** | Preview e composição visual; DX de integração; empacotamento de “app pronto” |
| **Pontos fracos vs OpenCore** | Não é ERP offline; tentação de low-code genérico; servidor frequentemente central |
| **Padrão a estudar** | Preview estrutural; explicação do que será gerado; conectores como adaptadores |
| **Padrão a NÃO incorporar** | Construtor low-code público como identidade do OpenCore Builder no MVP |
| **Interoperabilidade** | Conceitual (adaptadores), não migração de dados de gestão |
| **Risco jurídico de copiar código** | **Baixo a médio** — Apache 2.0; preferir padrões a cópia |
| **Apelo de marketing** | “Monte apps internos rapidamente”, drag-and-drop |

---

### 2.11 Axelor

| Campo | Observação |
|---|---|
| **Categoria** | Suite ERP / BPM open source |
| **Arquitetura** | Java; módulos; plataforma de apps empresariais |
| **Licenciamento** | AGPL (verificar edições comerciais) |
| **Público** | Empresas e integradores, sobretudo Europa |
| **Pontos fortes** | Cobertura ERP ampla; modularidade; oferta comercial paralela |
| **Pontos fracos vs OpenCore** | AGPL — copyleft de rede; web/server; Community vs comercial |
| **Padrão a estudar** | Organização modular de domínio; posicionamento de serviços profissionais |
| **Padrão a NÃO incorporar** | AGPL no monorepo OpenCore; dependência de empresa central para capacidades essenciais |
| **Interoperabilidade** | Adaptador possível, prioridade média |
| **Risco jurídico de copiar código** | **Muito alto** — AGPL; não incorporar código Axelor |
| **Apelo de marketing** | ERP/BPM completo, open source com serviços |

---

### 2.12 Budibase (relevante)

| Campo | Observação |
|---|---|
| **Categoria** | Low-code para apps internos / automação |
| **Arquitetura** | Builder + self-host; conectores; geração de apps |
| **Licenciamento** | GPL / modelo open-core (verificar componentes e cloud) |
| **Público** | Equipes que substituem planilhas por apps internos |
| **Pontos fortes** | Velocidade de protótipo; self-host; fluxo “descreva → monte tela” |
| **Pontos fracos vs OpenCore** | Identidade low-code; não substitui distribuições de domínio offline; risco de prometer “qualquer app” |
| **Padrão a estudar** | Onboarding rápido; templates; self-host como opção |
| **Padrão a NÃO incorporar** | Posicionar OpenCore como Budibase/ERP híbrido genérico; GPL no núcleo |
| **Interoperabilidade** | Baixa para migração ERP |
| **Risco jurídico de copiar código** | **Alto** se GPL; analisar por componente |
| **Apelo de marketing** | “Substitua planilhas”, apps internos rápidos |

---

## 3. Síntese transversal

### 3.1 Padrões a incorporar conceitualmente

| Padrão | Origem típica | Uso no OpenCore |
|---|---|---|
| Manifestos e dependências de módulo | Tryton, NocoBase | Metadados, `conflicts`, `recommended_with`, resolução |
| Ciclo de vida de módulo/plugin | NocoBase, Moodle | Estados instalado → ativo → removido + falhas |
| CLI e scaffolding | Frappe | `opencore new*`, `validate`, `doctor` (hipótese) |
| Contratos dados/serviços/eventos/UI | OFBiz | Separação normativa na Arquitetura |
| Módulos e perfis por necessidade | Odoo, Dolibarr | Distribuições + perfis verificados |
| Operação local e serviços | OpenConcerto | Offline-first + sustentabilidade por prestadores |
| Catálogo e descoberta | Odoo, Frappe | Catálogo de capacidades + páginas por necessidade |
| Governança de extensões | Moodle | Níveis de confiança (ADR-017) |
| Formação de mantenedores | OpenMRS, Moodle | Trilhas, mentoria, promoção |
| Rede de prestadores | Odoo | Diretório não exclusivo |
| Instalação guiada / preview | Appsmith (conceitual) | Builder + onboarding, sem low-code genérico |
| Descoberta por necessidade | Odoo SEO | OpenCore Builder + marketing por problema |

### 3.2 Padrões a melhorar (não repetir)

- web/server como requisito único de operação;
- linguagem ou stack única imposta a todo módulo;
- banco compartilhado sem fronteira de módulo;
- edição Community artificialmente limitada;
- dependência estrutural de empresa central;
- instalação complexa (Docker, DB e runtimes manuais no modo monoposto);
- catálogo sem matriz de compatibilidade;
- obrigar o usuário a escolher módulos pelo nome técnico;
- dados difíceis de exportar/restaurar;
- marketplace prematuro sem confiança e assinatura.

### 3.3 Diferenciação OpenCore

O OpenCore deve combinar, de forma verificável:

1. **offline-first** testável (não apenas “self-hosted”);
2. **distribuições nativas instaláveis** para o público-alvo;
3. **soberania** com exportação, restauração e exclusão testadas;
4. **manifesto + lockfile** de composição;
5. **composição guiada** (Builder) para leigos;
6. **IA subordinada a regras** determinísticas (quando existir);
7. **módulos multilíngues** (nativos e em processo), sem confundir isolamento com sandbox;
8. **distribuições oficiais integralmente abertas** (MPL/Apache conforme Manifesto);
9. **educação ligada a manutenção real**;
10. **baixo custo operacional** no modo monoposto;
11. **migração por adaptadores**, sem incorporar concorrentes ao runtime.

---

## 4. Interoperabilidade — prioridades conceituais

Ordem sugerida para adaptadores futuros (não é compromisso de roadmap nesta versão):

1. CSV / JSON / SQLite documentado (base universal);
2. Odoo;
3. ERPNext / Frappe;
4. Dolibarr;
5. Tryton;
6. OpenConcerto;
7. demais conforme demanda de pilotos.

Cada adaptador deverá: preservar IDs externos; mapear campos; gerar relatório de inconsistências; permitir dry-run; não alterar origem sem pedido; declarar limitações; manter operação local independente da integração.

---

## 5. Política de risco jurídico (resumo)

| Situação | Orientação |
|---|---|
| Estudar UI/fluxo/padrões publicamente documentados | Permitido como benchmark |
| Copiar código GPL/AGPL/LGPL para o monorepo | **Proibido** sem análise explícita por componente e decisão registrada |
| Reutilizar trechos Apache 2.0 / MPL | Somente com cumprimento de avisos, atribuição e fronteiras de arquivo |
| Copiar textos de marketing, docs extensas ou assets | **Proibido** |
| Reimplementar ideia (manifesto, CLI, catálogo) | Preferível e alinhado a este documento |

---

## 6. Implicações para o OpenCore Builder

Dos benchmarks, o Builder deve:

- falar a língua do resultado (“padaria”, “estoque offline”), como o marketing de Odoo, sem prometer distribuição inexistente;
- esconder complexidade de instalação como OpenConcerto/self-host bem feito, sem omitir riscos na ficha;
- oferecer DX de contribuição inspirada em Frappe, sem exigir que o usuário final use a CLI;
- usar ciclo de vida e validação inspirados em NocoBase/Moodle;
- nunca virar Appsmith/Budibase genérico na v0.

---

## 7. Fora do escopo deste documento

- rankings de “melhor ERP”;
- tabelas de features feature-by-feature copiadas de sites oficiais;
- números de usuários sem fonte;
- decisões normativas (ficam no Manifesto, Arquitetura, ADRs);
- implementação de adaptadores ou do Builder.

---

## 8. Histórico

| Data | Mudança |
|---|---|
| 2026-07-24 | Versão 1.0 — documento inicial de benchmarks do ecossistema (não normativo). |
