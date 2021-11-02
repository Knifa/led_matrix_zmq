PI_ARCH := aarch64-unknown-linux-gnu

.PHONY: build-cross-image
build-cross-image:
	docker build -t led-matrix-zmq/cross-aarch64:latest -f ./Dockerfile.cross-aarch64 .

.PHONY: copy-to-pi
copy-to-pi:
	scp target/$(PI_ARCH)/release/$(TARGET) matryx-pi:/tmp

.PHONY: copy-viewer-to-pi
copy-viewer-to-pi:
	cross build -p viewer --target aarch64-unknown-linux-gnu --release
	$(MAKE) copy-to-pi TARGET=viewer
