.PHONY: all test clean

BUILD_DIR=build

all: test
	@echo 'Success, binaries can be found in' $(BUILD_DIR)/ 'directory.'

prepare_dir: src
	test -e $(BUILD_DIR) || mkdir $(BUILD_DIR)
	cd $(BUILD_DIR) && cmake ..

build: prepare_dir
	cd $(BUILD_DIR) && cmake --build .

test: build
	cd $(BUILD_DIR) && (ctest || ctest --rerun-failed --output-on-failure)

clean:
	rm -rf $(BUILD_DIR)
