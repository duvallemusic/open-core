# Contribuindo com o OpenCore

Obrigado por considerar uma contribuição.

O OpenCore busca unir software profissional, soberania de dados, colaboração aberta e formação por trabalho real. Abertura à participação não reduz os requisitos de clareza, segurança, testes, documentação ou manutenção.

## 1. Antes de começar

Leia, nesta ordem:

1. [`README.md`](README.md)
2. [`00_Indice_Versoes.md`](00_Indice_Versoes.md)
3. [`01_Manifesto_OpenCore_v1.1_licenciamento.md`](01_Manifesto_OpenCore_v1.1_licenciamento.md)
4. [`02_Arquitetura_OpenCore_v1.2.md`](02_Arquitetura_OpenCore_v1.2.md)
5. [`03_Comunidade_Governanca_OpenCore_v1.0.md`](03_Comunidade_Governanca_OpenCore_v1.0.md)
6. [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md)

Use apenas as versões marcadas como canônicas no índice.

## 2. Estado atual do projeto

O OpenCore ainda está na Etapa 0 documental. Não existe runtime de produção nem SDK público estável.

Contribuições úteis agora incluem:

- revisão de consistência;
- documentação;
- pesquisa técnica;
- propostas de RFC;
- ADRs derivados de decisões aprovadas;
- templates e processo comunitário;
- identificação de riscos, casos de uso e requisitos de pilotos.

Não inicie implementação do runtime ou do Spike 10 como contribuição oficial antes de a Etapa 0 ser encerrada e a atividade ser aberta no roadmap.

## 3. Escolhendo uma contribuição

Prefira uma issue existente com escopo e critérios de aceitação.

Quando não houver issue adequada:

- use o template de bug para erros verificáveis;
- use o template de proposta para nova capacidade ou mudança;
- use o template de documentação para inconsistência ou melhoria textual;
- use RFC quando a mudança for significativa.

Antes de trabalhar em algo grande, registre a proposta. Uma implementação pronta não obriga o projeto a aceitar a decisão arquitetural que ela pressupõe.

## 4. Mudanças que exigem RFC

Consulte [`rfcs/README.md`](rfcs/README.md).

Em geral, RFC é necessária para:

- entrada ou remoção de capacidade do runtime;
- novo contrato público ou mudança incompatível;
- persistência, migração, backup ou exportação;
- protocolo, classe de módulo ou modo de execução;
- nova distribuição oficial;
- política de suporte, confiança ou depreciação;
- telemetria, integração padrão ou coleta de dados;
- mudança de governança, licenciamento ou parceria relevante.

## 5. ADRs

ADRs registram decisões arquiteturais; não são propostas informais.

Leia [`docs/adr/ADR_GUIDE.md`](docs/adr/ADR_GUIDE.md) e use [`docs/adr/ADR_TEMPLATE.md`](docs/adr/ADR_TEMPLATE.md).

Não renumere ADRs existentes. Uma decisão alterada deve ser supersedida por nova ADR, preservando o histórico.

## 6. Fluxo de trabalho

1. Confirme o escopo em uma issue, RFC ou tarefa do roadmap.
2. Crie uma branch curta e descritiva.
3. Faça mudanças focadas; evite misturar assuntos independentes.
4. Atualize documentação e testes aplicáveis.
5. Assine os commits conforme o DCO.
6. Abra a pull request usando o template.
7. Responda aos comentários e diferencie bloqueios de sugestões.
8. Aguarde os checks e a revisão do responsável.

Sugestões de nomes de branch:

```text
docs/ajusta-governanca
fix/corrige-link-roadmap
rfc/storage-modulos-processo
adr/022-decisao-exemplo
```

## 7. Commits e DCO

O projeto utiliza o Developer Certificate of Origin.

Assine cada commit com:

```bash
git commit -s -m "docs: descreve a mudança"
```

Isso adiciona:

```text
Signed-off-by: Seu Nome <seu-email@example.com>
```

Ao assinar, você certifica as condições descritas em [`DCO.md`](DCO.md).

Commits não assinados poderão precisar de correção antes do merge.

## 8. Pull requests

Uma PR deverá, quando aplicável:

- explicar problema, solução e escopo;
- referenciar issue, RFC ou ADR;
- declarar impacto sobre dados, módulos e compatibilidade;
- listar testes ou validações realizadas;
- atualizar documentação e changelog;
- identificar novas dependências e licenças;
- informar riscos, limitações e trabalho futuro;
- resolver comentários bloqueantes.

Mudanças sensíveis não devem ser revisadas apenas pelo próprio autor quando houver pessoa qualificada disponível.

Durante o Estágio F, aplica-se a exceção fundadora registrada na governança.

## 9. Documentação

Documentação é parte do produto.

Ao alterar um comportamento, atualize:

- documentação de uso;
- arquitetura ou ADR relacionada;
- exemplos;
- migração e compatibilidade;
- segurança e privacidade, quando afetadas;
- índice de versões, se o documento canônico mudar.

Snapshots históricos não devem ser reescritos para parecer atuais.

## 10. Licenciamento

Contribuições aceitas seguem a licença do componente de destino:

- patrimônio estrutural e módulos oficiais: MPL 2.0, quando publicado sob essa licença;
- protocolo, SDKs, templates e ferramentas: Apache 2.0, quando publicado sob essa licença;
- outros componentes: licença identificada no próprio diretório ou arquivo.

### Documentos normativos atuais

Conforme a RFC-0001 aceita:

- documentação textual pública: **CC BY 4.0**;
- exemplos e trechos de código em documentação: **Apache 2.0**, salvo aviso diferente;
- marcas, logotipos e selos: direitos reservados (fora da licença documental).

Veja [`LICENSE`](LICENSE) e [`rfcs/0001-licenca-documentacao.md`](rfcs/0001-licenca-documentacao.md).

A abertura ampla de PRs documentais deverá ocorrer após a publicação de uma política de licença documental.

## 11. Segurança

Não publique vulnerabilidade explorável em issue ou discussão aberta.

Siga [`SECURITY.md`](SECURITY.md).

## 12. Conduta

Toda participação está sujeita ao [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md).

Discordância técnica é legítima. Ataque pessoal, assédio, intimidação, discriminação e retaliação não são.

## 13. Mentoria e tarefas educacionais

Issues educacionais devem possuir:

- contexto;
- resultado esperado;
- critérios de aceitação;
- conhecimentos recomendados;
- testes esperados;
- responsável por revisão ou mentoria;
- evidência pública utilizável em portfólio.

Mentoria orienta; não executa a contribuição no lugar do participante.

## 14. Aceitação

Uma contribuição pode ser recusada mesmo quando tecnicamente funcional se:

- conflitar com o Manifesto ou ADRs;
- criar complexidade prematura;
- introduzir risco sem mitigação;
- não possuir responsável ou manutenção viável;
- duplicar trabalho sem benefício claro;
- depender de serviço proprietário essencial;
- não atender às obrigações de licença, documentação ou testes.

A recusa deve ser fundamentada e respeitosa.

## 15. Reconhecimento

Contribuições aceitas poderão ser reconhecidas por histórico Git, changelog, notas de release, arquivo de contribuidores, certificados baseados em evidências e outros mecanismos públicos.

Código não possui valor automático superior a documentação, testes, design, pesquisa, tradução, segurança ou mentoria.
