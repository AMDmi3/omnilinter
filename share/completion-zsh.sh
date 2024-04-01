#compdef omnilinter

_arguments \
  '*'{-c+,--config=}'[Path(s) to configuration file(s)]:file:_files' \
  '(-t --tags)'{-t+,--tags=}'[Only process rules tagged with these values]:tag:' \
  '--skip-tags=[Ignore rules tagged with these values]:tag:' \
  '(-f --format)'{-f+,--format=}'[Output format]:format:(by-root full-paths by-rule by-path json)' \
  '--color=[Coloring]:mode:(auto always never)' \
  '--palette=[Palette to use for rule coloring]:palette:(none simple severity true-color)' \
  '--error-exitcode=[If any matches are found, exit with given code]:exit code:' \
  {-j+,--jobs=}'[Number of target directories to process simultaneously]:number:' \
  '(* -)'{-h,--help}'[Print help]' \
  '(* -)'{-V,--version}'[Print version]' \
  '*:directory:_files -/'
