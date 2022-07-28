PREFIX = /usr/local
all: target/release/rsweb-bin container

target/release/rsweb-bin:
	cargo build --release

container:
	docker build -t uludev/rsweb:latest .

install: target/release/rsweb-bin
	mv target/release/rsweb-bin $(PREFIX)/bin/rsweb

clean:
	cargo clean
