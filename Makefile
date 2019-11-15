TAG ?= technosophos/tinygw:latest

.PHONY: build
build:
	#cargo build
	docker build -t $(TAG) .
	docker push $(TAG)