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


.PHONY: run build test release win
