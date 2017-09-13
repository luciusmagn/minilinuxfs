.PHONY: clean

all: bin/install
	@bin/install

bin/install:
	@mkdir -p bin
	@cd rsrc && cargo build --release
	@cp rsrc/release/install bin/install

clean:
	@./bin/install clean
	@rm -rf bin rsrc/debug rsrc/release
