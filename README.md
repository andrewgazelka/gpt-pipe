# gpt-pipe

Execute GPT actions on stdin.

I am currently using this in `vim` like so

```text
nmap <leader>p :%! gpt-pipe you are a pragmatic planner. give insight/critique tasks and how I should do them. reorder tasks and explain ordering<CR>
```

which reorders my task list according to what GPT-4 thinks I should complete first.