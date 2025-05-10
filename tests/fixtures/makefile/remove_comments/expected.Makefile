

target1:
	@echo "This is a command"

target2: target1
	@echo "This is another command"

VAR = value

%.o: %.c
	@echo "Compiling $<"
