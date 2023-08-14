MODULE = $(notdir $(CURDIR))

WATCH  = cargo watch -w config -w lib

all:
	cargo build -p config
	cargo build -p lib
# cargo build -p daemon

.PHONY: format
format:
	cargo fmt &

.PHONY: daemon
daemon:
	$(WATCH) -w $@ -x 'run -p $@'
