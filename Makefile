.PHONY: all test clean dbg

BUILD_DIR=build
DBG_BUILD=build/debug
GDB=gdb

DBG_FLAGS=-ex "layout regs" -ex "break main" -ex "start"

all: test
	@echo 'Success, binaries can be found in' $$(cd $(BUILD_DIR) && pwd)/

prepare_dir: src
	test -e $(BUILD_DIR) || mkdir $(BUILD_DIR)
	cd $(BUILD_DIR) && cmake ..

build: prepare_dir
	cd $(BUILD_DIR) && cmake --build .

test: build
	cd $(BUILD_DIR) && (ctest --output-on-failure)

build-dbg: src
	gcc -g3 -o $(DBG_BUILD) src/*.c src/**/*.c

dbg: build-dbg
	$(GDB) $(DBG_FLAGS) --args $(DBG_BUILD) $(DBG_IN)

clean:
	rm -rf $(BUILD_DIR)
