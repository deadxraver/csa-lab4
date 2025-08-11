.PHONY: all test clean

BUILD_DIR=build

all: test
	@echo 'Success'

prepare_dir: src
	test -e $(BUILD_DIR) || mkdir $(BUILD_DIR)
	cd $(BUILD_DIR) && cmake ..

build: prepare_dir
	cd $(BUILD_DIR) && cmake --build .

test: build
	cd $(BUILD_DIR) && ctest

clean:
	rm -rf $(BUILD_DIR)
