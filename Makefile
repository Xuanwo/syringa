SHELL := /bin/bash

.PHONY: build release

help:
	@echo "Please use \`make <target>\` where <target> is one of"
	@echo "  build      to create bin directory and build syringa"
	@echo "  release    to release syringa"


build:
	@echo "build syringa"
	@cargo build
	@echo "ok"

release:
	@echo "release syringa"
	@cargo build --release
	@echo "ok"
