# OpenCore — Roadmap e Arquitetura

# Fase 0 — Fundação

- Manifesto
- Missão
- Governança
- Licença
- Código de conduta
- Guia de contribuição
- ADRs

# Fase 1 — Kernel

Componentes:

- Configuração
- Eventos
- Banco
- Autenticação
- Permissões
- Backup
- Logs
- Internacionalização
- Atualizações
- Plugin Loader

Preferência arquitetural:
Monólito modular com fronteiras claras.

# Fase 2 — SDK

Criar API pública para módulos.

Cada módulo deve possuir:

- Manifest
- Contratos
- Hooks
- Versionamento
- Testes

# Fase 3 — Interface

Interface gráfica modular.

O usuário escolhe módulos.

O sistema gera uma distribuição personalizada.

# Fase 4 — Primeiro produto

Sistema de Portaria.

Objetivo:
Validar a plataforma, não apenas resolver o problema de portaria.

# Fase 5 — Portal

Portal com:

- documentação
- downloads
- gerador de builds
- marketplace de módulos
- showcase

# Fase 6 — Comunidade

- good first issue
- mentorias
- roadmap público
- reuniões abertas
- RFCs

# Fase 7 — Parcerias

Prioridade estratégica:

1. Plataformas de ensino
2. Empresas patrocinadoras
3. Universidades
4. Comunidades open source

## Avaliação da estratégia

A parceria com plataformas semelhantes pode reduzir drasticamente o
risco inicial.

Benefícios:

- comunidade já existente
- fluxo contínuo de novos desenvolvedores
- mentores experientes
- empresas trazendo problemas reais
- geração de portfólio para alunos
- identificação de talentos
- visibilidade institucional

Riscos:

- perda parcial de autonomia
- necessidade de alinhar interesses
- burocracia
- dependência de parceiros

Recomendação:

Buscar essas instituições como parceiras de adoção, e não como proprietárias do
projeto.

A governança deve permanecer independente para garantir longevidade.

## Stack sugerida

Kernel:
Rust

GUI:
Slint (preferencialmente)

Banco:
SQLite inicialmente

Arquitetura:
Monólito modular

Expansão futura:
Plugins + SDK + APIs estáveis.
