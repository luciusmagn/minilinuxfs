all: bin/install
	@bin/install

.PHONY: clean rsrc/cfg.rs rsrc/main.rs all

rsrc/cfg.rs rsrc/main.rs:
	@true

bin/install: rsrc/cfg.rs rsrc/main.rs
	@mkdir -p bin
	@cd rsrc && cargo build --release
	@cp rsrc/release/install bin/install

clean:
	@./bin/install clean
	@rm -rf bin rsrc/debug rsrc/release
