all: container target/release/rsweb-bin

target/release/rsweb-bin:
	cargo build --release

container:
	docker build -t uludev/rsweb:latest .
