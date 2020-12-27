OBJS=   hd
PREFIX= /usr/local/bin

all: $(OBJS)
install: $(OBJS)
	cp hd $(PREFIX)/
uninstall:
	$(RM) $(PREFIX)/hd
%:%.rs
	rustc $^
	@strip $@
clean:
	$(RM) $(OBJS)
