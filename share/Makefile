PREFIX?=	/usr/local

all:
	@echo 'run `${MAKE} install` to install additional files'

install:
	install -m 644 completion-zsh.sh "${DESTDIR}${PREFIX}/share/zsh/site-functions/_omnilinter"
	install -m 644 syntax.vim "${DESTDIR}${PREFIX}/share/vim/vimfiles/syntax/omnilinter.vim"
