# gpt-pipe

Execute GPT actions on stdin.

I am currently using this in `vim` like so

```vim
nmap <leader>p :%! gpt-pipe you are a pragmatic planner. give insight/critique tasks and how I should do them. reorder tasks and explain ordering<CR>
```

which reorders my task list according to what GPT-4 thinks I should complete first.

To see the prompt being typed out in vim asyncronously you can do

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