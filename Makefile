REST_API ?= https://vm.aleo.org/api
ENDPOINT_URL ?= http://localhost:18015
START_BLOCK ?= 0
PACKAGE_FILE ?= ./substreams-nexus-dao-v0.1.0.spkg
MODULE_NAME ?= map_records
HOST ?= 127.0.0.1
PORT ?= 8081

.PHONY: build
build:
	cargo build --release

.PHONY: all
all:
	cargo run -- all \
		--rest-api $(REST_API) \
		--endpoint-url $(ENDPOINT_URL) \
		--package-file $(PACKAGE_FILE) \
		--module-name $(MODULE_NAME) \
		--start-block $(START_BLOCK) \
		--host $(HOST) \
		--port $(PORT)

.PHONY: sync
sync:
	cargo run -- sync \
		--rest-api $(REST_API) \
		--endpoint-url $(ENDPOINT_URL) \
		--package-file $(PACKAGE_FILE) \
		--module-name $(MODULE_NAME) \
		--start-block $(START_BLOCK)

.PHONY: serve
serve:
	cargo run -- serve \
		--rest-api $(REST_API) \
		--host $(HOST) \
		--port $(PORT)

