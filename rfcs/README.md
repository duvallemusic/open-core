# Processo de RFC do OpenCore

RFC — Request for Comments — é uma proposta pública para mudança relevante ainda não decidida.

## Quando usar

RFC é obrigatória para mudanças que afetem, entre outros temas:

- arquitetura e runtime;
- contratos públicos e compatibilidade;
- persistência, migrações, backup ou exportação;
- protocolo, SDK ou classe de módulo;
- segurança, confiança ou telemetria;
- nova distribuição oficial;
- política de suporte ou depreciação;
- governança e contribuição;
- licenciamento e direitos dos usuários;
- parceria, patrocínio ou compromisso institucional relevante;
- marca, certificação ou entidade independente.

Correções rotineiras e manutenção local normalmente não exigem RFC.

## Estrutura do diretório

```text
rfcs/
├── README.md
├── 0000-template.md
└── NNNN-titulo-curto.md
```

## Numeração

- use quatro dígitos;
- solicite o próximo número na issue ou PR;
- não reutilize número;
- preserve RFCs rejeitadas, retiradas e supersedidas;
- nomes devem ser curtos e estáveis.

## Estados

```text
Rascunho → Em consulta → Aceita | Rejeitada | Adiada | Retirada → Implementada
```

## Fluxo

1. Discuta o problema em issue quando necessário.
2. Copie `0000-template.md` e preencha a proposta.
3. Abra PR marcada como rascunho.
4. Obtenha número e responsável pela decisão.
5. Complete impactos, alternativas e conflitos.
6. Mova para **Em consulta**.
7. Mantenha a consulta pelo prazo aplicável.
8. Registre decisão e objeções relevantes.
9. Crie ADR quando a decisão arquitetural exigir.
10. Marque **Implementada** somente após evidência.

## Prazos padrão

- mudança significativa de escopo limitado: 7 dias;
- arquitetura ou política ampla: 14 dias;
- Manifesto, licenças, governança, marca ou direitos: 30 dias.

O prazo pode ser ampliado por impacto, participação insuficiente ou nova evidência.

## Autoridade no Estágio F

O Lead Maintainer decide após consulta e deve publicar justificativa.

A decisão não é determinada apenas por votação, comentários ou reações. Evidências, riscos, manutenção e alinhamento ao Manifesto têm peso central.

## Revisão

Uma RFC deve responder:

- o problema é real e bem delimitado?
- há alternativa mais simples?
- quem mantém a solução?
- há impacto em usuários, dados ou segurança?
- a proposta cria lock-in?
- as licenças permitem a implementação?
- existe plano de migração e reversão?
- critérios de sucesso são verificáveis?
- conflitos de interesse foram declarados?

## Alterações após aceitação

Correções editoriais podem ser feitas sem mudar a decisão. Mudança normativa exige nova RFC ou reabertura formal, preservando o histórico.
