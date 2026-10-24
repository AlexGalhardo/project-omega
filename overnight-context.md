Dá para fazer isso com 4 peças: manter a máquina acordada, deixar o Claude Code rodando num terminal que sobrevive a desconexões, um script que relança o Claude quando ele cair, e um plano em arquivo com commits para ele retomar de onde parou. Você já anotou que tem problemas de conexão em sessões longas, então o script de retry é a parte mais importante.

## 1. Manter o PC acordado

**macOS (M1):**
- Ligue o carregador. Se for notebook com a tampa fechada, precisa de carregador e monitor externo, ou então use `sudo pmset -a disablesleep 1` (desfaça com `0` depois).
- Rode o script com `caffeinate -i ./run-overnight.sh`. Ele impede a suspensão só enquanto o processo estiver rodando.

**Windows 11:**
- Configurações → Energia: "Nunca" para suspensão quando conectado na tomada, e "Não fazer nada" para a tampa.
- Alternativa: PowerToys → Awake (modo "keep awake").

## 2. Terminal que não morre sozinho

Se o terminal fechar ou a SSH cair, o processo morre. Use `tmux`:

```bash
tmux new -s claude        # cria a sessão
# rode seu script dentro dela
# Ctrl+B depois D para desanexar; tmux attach -t claude para voltar
```

No Windows, use o tmux dentro do WSL2.

## 3. Script que relança o Claude quando ele cair

Crie `run-overnight.sh` na raiz do projeto:

```bash
#!/bin/bash
cd /caminho/do/projeto
PROMPT="Continue o trabalho descrito em PLAN.md. Marque cada item concluído, faça commit ao terminar cada etapa. Quando tudo estiver pronto, crie o arquivo DONE.txt."

until [ -f DONE.txt ]; do
  echo "[$(date)] iniciando/retomando" >> run.log
  claude -c -p "$PROMPT" --permission-mode acceptEdits >> run.log 2>&1
  echo "[$(date)] saiu com código $? — aguardando antes de retomar" >> run.log
  sleep 120   # sem internet, o comando falha rápido; o sleep evita loop agressivo
done
```

Pontos importantes:
- `claude -c` retoma a conversa mais recente naquela pasta, então o contexto não se perde quando o processo cai.
- Confirme as flags com `claude --help`, porque elas mudam entre versões.
- Se o limite de uso da sua conta for atingido (a janela é de algumas horas), o comando também falha. O `sleep` faz o script esperar e tentar de novo, então dá para aumentar para 300–600s se você notar muitas falhas no `run.log`.

## 4. Permissões: o ponto de atenção

Para ele não parar pedindo aprovação, use permissões pré-aprovadas em `.claude/settings.json` do projeto, em vez de `--dangerously-skip-permissions`:

```json
{
  "permissions": {
    "allow": ["Bash(npm *)", "Bash(npx *)", "Bash(git add *)", "Bash(git commit *)", "Bash(git status)", "Bash(git diff *)"],
    "deny": ["Bash(rm -rf *)", "Bash(git push *)", "Bash(git reset --hard *)"]
  }
}
```

Assim ele age sozinho só no que você liberou. Ajuste a lista ao seu stack. Evite a flag de pular todas as permissões num PC pessoal sem isolamento, porque um comando errado pode apagar coisas que você não tem de volta.

## 5. Plano e git para retomada segura

Isso é o que realmente faz a retomada funcionar, mesmo depois de reiniciar a máquina ou trocar de sessão:

- **`PLAN.md`** com a lista de tarefas em checkboxes (`- [ ]` / `- [x]`). O Claude lê e atualiza a cada etapa.
- **`CLAUDE.md`** com regras permanentes: "atualize PLAN.md ao concluir cada item", "faça commit por etapa, mensagem descritiva", "se um teste falhar 3 vezes, anote no PLAN.md e siga para o próximo item".
- **Git em branch separada** (ex.: `overnight/xyz`). Commits pequenos e frequentes. Se algo der errado de madrugada, você só faz `git reset` para o último commit bom.

## 6. Reinício do PC ou queda longa

tmux e o script não sobrevivem a um reboot. Para isso:
- **macOS:** crie um LaunchAgent em `~/Library/LaunchAgents/` que roda o script no login.
- **Windows:** Agendador de Tarefas, gatilho "ao fazer logon", executando o `wsl ... run-overnight.sh`.

Se o trabalho for realmente longo e você não quiser depender do PC ligado, a alternativa é rodar num VPS ou container Linux, com o mesmo script.

## 7. Aviso de término (opcional)

Um hook `Stop` no `settings.json` pode disparar uma notificação quando o Claude terminar (`osascript` no macOS, toast do PowerShell no Windows), para você saber ao acordar sem precisar abrir o `run.log`.

## Checklist antes de dormir

1. Projeto em branch nova, com commit limpo.
2. `PLAN.md` com as tarefas e `CLAUDE.md` com as regras.
3. `settings.json` com permissões definidas.
4. Script testado por 2–3 minutos (confira o `run.log`).
5. Carregador ligado, suspensão desativada, `caffeinate` ou tmux ativo.

Se quiser, posso montar o `PLAN.md`, o `CLAUDE.md` e o `settings.json` com base no projeto grande que você tem em mente.