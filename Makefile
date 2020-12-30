OBJS=   hd
PREFIX= /usr/local/bin

all: $(OBJS)
install: $(OBJS)
	cp hd $(PREFIX)/
uninstall:
	$(RM) $(PREFIX)/hd
hd: main.rs
	rustc $^ -o $@
	@strip $@
clean:
	$(RM) $(OBJS)
