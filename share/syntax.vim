" Vim syntax file
"   Language: omnilinter config file
" Maintainer: Dmitry Marakasov <amdmi3@amdmi3.ru>

if exists("b:current_syntax")
  finish
endif

syn keyword omnilinterTodo      contained TODO FIXME XXX
syn match   omnilinterIgnore    contained "omnilinter: ignore"
syn match   omnilinterComment   "#.*" contains=omnilinterIgnore,omnilinterTodo,@Spell
syn match   omnilinterTag       "[^ \t,]\+" contained

syn region  omnilinterRuleTitle     skipwhite keepend start=+\[+ skip=+\]\]+ end=+\]+ 
syn keyword omnilinterDirective     tags files nofiles match nomatch size lines
syn region  omnilinterDirectiveTags matchgroup=omnilinterDirective start=+^\s*tags+ skip=+,+ end=+\s*$+ contains=omnilinterTag

hi def link omnilinterComment       Comment
hi def link omnilinterRuleTitle     Keyword
hi def link omnilinterDirective     Identifier
hi def link omnilinterIgnore        SpecialComment
hi def link omnilinterTodo          Todo
hi def link omnilinterTag           String

let b:current_syntax = "omnilinter"
