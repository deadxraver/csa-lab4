CC=rustc
TRANSLATOR=src/translator
BINARY_DIR=build
TEST_DIR=tests

.PHONY: clean

test:
	make build_test
	$(BINARY_DIR)/test1
	# TODO: run another tests

build_test: $(TEST_DIR) $(BINARY_DIR)
	$(CC) --out-dir $(BINARY_DIR) $(TEST_DIR)/test1.rs

translator: $(TRANSLATOR)/main.rs $(TRANSLATOR)/srs_commands.rs $(TRANSLATOR)/asm_commands.rs $(BINARY_DIR)
	$(CC) -o $(BINARY_DIR)/translator $(TRANSLATOR)/main.rs

$(BINARY_DIR):
	mkdir $(BINARY_DIR)

clean:
	rm -rf $(BINARY_DIR)
