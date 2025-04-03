CC=rustc
TRANSLATOR=src/translator
BINARY=build

.PHONY: clean

translator: $(TRANSLATOR)/main.rs $(TRANSLATOR)/srs_commands.rs $(TRANSLATOR)/asm_commands.rs $(BINARY)
	$(CC) -o $(BINARY)/translator $(TRANSLATOR)/main.rs

$(BINARY):
	mkdir $(BINARY)

clean:
	rm -rf $(BINARY)
