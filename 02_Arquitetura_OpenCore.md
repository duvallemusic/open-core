# Arquitetura (Rascunho)

## Núcleo
Monólito modular.

## Módulos-base
Autenticação
Permissões
Eventos
Configuração
Logs
Backup
Banco
Relatórios
Internacionalização
Atualizações

## Expansão
SDK + API pública + sistema de plugins.

## Stack inicial sugerida
Kernel: Rust
GUI: Slint
Banco: SQLite
Sincronização opcional.

## Perguntas em aberto
- Como versionar plugins?
- Como garantir compatibilidade?
- Como será a comunicação entre módulos?
- Quando migrar partes para microserviços?
- Como assinar módulos distribuídos?
