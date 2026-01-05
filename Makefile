.PHONY: all build check-deps diagnostics clean

all: build

build:
	cargo build

check-deps:
	@echo "Checking system dependencies..."
	@which pkg-config > /dev/null || echo "Missing: pkg-config"
	@pkg-config --exists gstreamer-1.0 || echo "Missing: gstreamer-1.0 (dev libs)"
	@pkg-config --exists gstreamer-app-1.0 || echo "Missing: gstreamer-app-1.0"
	@pkg-config --exists gstreamer-video-1.0 || echo "Missing: gstreamer-video-1.0"

diagnostics:
	cargo run -p ndp-inspect

test:
	cargo test

provider:
	cargo run -p ndp-provider-mutter -- --help

sink:
	cargo run -p ndp-sink -- --help
