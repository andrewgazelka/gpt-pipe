# gpt-pipe
[![GPT pipe](https://img.shields.io/crates/v/gpt-pipe.svg?style=plastic)](http://crates.io/crates/gpt-pipe)

Execute GPT actions on stdin.

## Auto document code

Say I want to add docs to all TypeScript code

```bash
fd -e ts -x sh -c 'cat {} | gpt-pipe -s /Users/yourname/.config/gpt/doc.txt > {}.tmp && mv {}.tmp {}'
```

where `~/.config/gpt/doc.txt` is

```text
you are a helpful and competent programmer. add comments and docs to help document and explain parts of the code
```

## PRs

Say I have a file `pr-format.md`

```markdown
You are a PR creator. You turn a diff into a PR. 
PRs are in markdown and in the following format:

[title]

# Overview

[edit]

## Changes

[edit]

## Tradeoffs

[edit]
```

I can run `git diff | gpt-pipe -s pr-format.md` and get GPT-4 to fill in the blanks.

## vim 
I am currently using this in `vim` to feed my entire vim buffer into ChatGPT.

```vim
nmap <leader>p :%! gpt-pipe you are a pragmatic planner. give insight/critique tasks and how I should do them. reorder tasks and explain ordering<CR>
```

which reorders my task list according to what GPT-4 thinks I should complete first.

To see the prompt being typed out in vim asynchronously, you can do

```vim
function! HandleOutput(job_id, data, event)
  let l:output = join(a:data, "\n")
  let l:output_lines = split(l:output, '\n', 1)
  let l:current_line = getline(line('$'))
  let l:first_line = l:current_line . l:output_lines[0]

  call setline(line('$'), l:first_line)

  if len(l:output_lines) > 1
    call append(line('$'), l:output_lines[1:])
  endif
endfunction

function! StartAsyncCommand(input)
  let l:cmd = ['gpt-pipe', 'you are a pragmatic planner. give insight/critique tasks and how I should do them. reorder tasks and explain ordering.']
  let l:job_opts = {
        \ 'on_stdout': function('HandleOutput'),
        \ 'on_stderr': function('HandleOutput'),
        \ 'in_io': 'pipe',
        \ }
  let l:job_id = jobstart(l:cmd, l:job_opts)
  call jobsend(l:job_id, a:input)
  call jobclose(l:job_id, 'stdin')
endfunction
```

and then map it to `<leader>p` with

```vim
nmap <leader>p :call StartAsyncCommand(getline(1, '$'))<CR>
```

# Installation

```bash
cargo install gpt-pipe
```
