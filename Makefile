ENDPOINT_URL ?= http://localhost:18015
START_BLOCK ?= 0
PACKAGE_FILE ?= ./substreams-aleo-v0.1.0.spkg
MODULE_NAME ?= map_ratifications
HOST ?= 127.0.0.1
PORT ?= 8080

.PHONY: build
build:
	cargo build --release

.PHONY: all
all:
	cargo run -- all \
		--endpoint-url $(ENDPOINT_URL) \
		--package-file $(PACKAGE_FILE) \
		--module-name $(MODULE_NAME) \
		--start-block $(START_BLOCK) \
		--host $(HOST) \
		--port $(PORT)

.PHONY: sync
sync:
	cargo run -- sync \
		--endpoint-url $(ENDPOINT_URL) \
		--package-file $(PACKAGE_FILE) \
		--module-name $(MODULE_NAME) \
		--start-block $(START_BLOCK)

.PHONY: serve
serve:
	cargo run -- serve \
		--host $(HOST) \
		--port $(PORT)

