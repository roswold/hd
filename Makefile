OBJS= hd
all: $(OBJS)
%:%.rs
	rustc $^
	@strip $@
clean:
	$(RM) $(OBJS)
