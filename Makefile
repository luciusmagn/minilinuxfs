.PHONY: clean

all: install
	@./install

install:
	@cd src && cargo build --release
	@cp src/release/install install

clean:
	@./install clean
	@rm -rf src/debug src/release
