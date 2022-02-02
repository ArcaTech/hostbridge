ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

.PHONY: ffi-static
ffi-static:
	cd lib/hostbridge && cargo build --release
	cp lib/hostbridge/target/release/libhostbridge.a lib/
	go build -a -o ./ffi-static ./cmd/ffi-static/main.go

.PHONY: ffi-shared
ffi-shared:
	cd lib/hostbridge && cargo build --release
	cp lib/hostbridge/target/release/libhostbridge.dylib lib/
	go build -a -o ./ffi-shared -ldflags="-r $(ROOT_DIR)lib" ./cmd/ffi-shared/main.go

.PHONY: clean
clean:
	rm -rf ffi-debug ffi-shared ffi-static lib/libhostbridge.dylib lib/libhostbridge.a lib/hostbridge/target