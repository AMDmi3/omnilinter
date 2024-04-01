PREFIX?=	/usr/local

all:
	@echo 'run `${MAKE} install` to install additional files'

install:
	mkdir -p "${DESTDIR}${PREFIX}/share/zsh/site-functions/"
	install -m 644 completion-zsh.sh "${DESTDIR}${PREFIX}/share/zsh/site-functions/_omnilinter"
	mkdir -p "${DESTDIR}${PREFIX}/share/vim/vimfiles/syntax/"
	install -m 644 syntax.vim "${DESTDIR}${PREFIX}/share/vim/vimfiles/syntax/omnilinter.vim"
