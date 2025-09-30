run:
	cargo run

build:
	cargo build --release

release:
	./target/release/tui_stat

test:
	cargo test -- --show-output

win:
	cargo clean
	cargo build --target x86_64-pc-windows-gnu

install:
	install -m 755 target/release/tui_stat /usr/local/bin/

package: build
	# Create a distribution package
	mkdir -p dist
	cp target/release/tui_stat dist/
	cp README.md dist/ 2>/dev/null || true
	cp LICENSE dist/ 2>/dev/null || true
	cp CHANGELOG.md dist/ 2>/dev/null || true

# Cross-compilation targets
cross-build-linux:
	docker run --rm -v "$(pwd)":/usr/src/myapp -w /usr/src/myapp rust:1.70 \
		cargo build --release

clean:
	cargo clean
	rm -rf dist/

.PHONY: run build test release win install package cross-build-linux clean
