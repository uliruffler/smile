.PHONY: all build release debug test clean install uninstall run help check fmt clippy

# Default target
all: release

# Build in release mode
release:
	cargo build --release

# Build in debug mode
debug:
	cargo build

# Build (alias for release)
build: release

# Run tests
test:
	cargo test

# Run in debug mode
run:
	cargo run

# Run in release mode
run-release:
	cargo run --release

# Check code without building
check:
	cargo check

# Format code
fmt:
	cargo fmt

# Run clippy linter
clippy:
	cargo clippy -- -W clippy::all

# Clean build artifacts
clean:
	cargo clean

# Install system-wide (requires sudo)
install: release
	@echo "Installing smile to /usr/local/bin..."
	@sudo cp target/release/smile /usr/local/bin/
	@echo "Installation complete! Run with: smile"

# Uninstall from system
uninstall:
	@echo "Removing smile from /usr/local/bin..."
	@sudo rm -f /usr/local/bin/smile
	@echo "Uninstallation complete!"

# Show help
help:
	@echo "Smile - Emoticon Picker for Gnome/Linux"
	@echo ""
	@echo "Available targets:"
	@echo "  make                 - Build in release mode (default)"
	@echo "  make build           - Build in release mode"
	@echo "  make release         - Build in release mode"
	@echo "  make debug           - Build in debug mode"
	@echo "  make test            - Run tests"
	@echo "  make run             - Run in debug mode"
	@echo "  make run-release     - Run in release mode"
	@echo "  make check           - Check code without building"
	@echo "  make fmt             - Format code"
	@echo "  make clippy          - Run linter"
	@echo "  make clean           - Clean build artifacts"
	@echo "  make install         - Install system-wide (requires sudo)"
	@echo "  make uninstall       - Uninstall from system"
	@echo "  make help            - Show this help"
