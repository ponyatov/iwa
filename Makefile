# var
MODULE  = $(notdir $(CURDIR))
module  = $(shell echo $(MODULE) | tr A-Z a-z)
OS      = $(shell uname -o|tr / _)
NOW     = $(shell date +%d%m%y)
REL     = $(shell git rev-parse --short=4 HEAD)
BRANCH  = $(shell git rev-parse --abbrev-ref HEAD)
CORES  ?= $(shell grep processor /proc/cpuinfo | wc -l)

# src
S += Cargo.toml Cargo.lock src
S += config lib daemon

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

# merge
MERGE += README.md Makefile $(S)
MERGE += .gitignore .gitattributes .stignore
MERGE += apt.dev apt.txt apt.msys

.PHONY: dev
dev:
	git push -v
	git checkout $@
	git pull -v
	git checkout shadow -- $(MERGE)
#	$(MAKE) doxy ; git add -f docs

.PHONY: shadow
shadow:
	git push -v
	git checkout $@
	git pull -v

.PHONY: release
release:
	git tag $(NOW)-$(REL)
	git push -v --tags
	$(MAKE) shadow

ZIP = tmp/$(MODULE)_$(NOW)_$(REL)_$(BRANCH).zip
zip:
	git archive --format zip --output $(ZIP) HEAD
