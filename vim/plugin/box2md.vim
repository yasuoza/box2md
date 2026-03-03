if exists('g:loaded_box2md') | finish | endif
let g:loaded_box2md = 1

let g:box2md_path = get(g:, 'box2md_path', 'box2md')

command! -range Box2html <line1>,<line2>call s:ToHtml()
command! Box2md call s:ToMd()

function! s:ToHtml() range
  let lines = getline(a:firstline, a:lastline)
  let input = join(lines, "\n")
  let output = system(g:box2md_path . ' to-html -c', input)
  if !v:shell_error
    echo 'HTML copied to clipboard'
  else
    echoerr 'box2md: ' . output
  endif
endfunction

function! s:ToMd()
  let output = system(g:box2md_path . ' to-md -p')
  if !v:shell_error
    let text = substitute(output, '\n$', '', '')
    call setreg('"', text, 'c')
    normal! p
  else
    echoerr 'box2md: ' . output
  endif
endfunction
