.PHONY: clean

all: install
	@./install

install:
	@mkdir -p bin
	@cd src && cargo build --release
	@cp src/release/install install

clean:
	@./install clean
	@rm -rf bin src/debug src/release
