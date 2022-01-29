#!/bin/bash

set -e

log() {
	echo "[configure.sh] $@" >&2
}

print_help() {
	echo "./configure.sh -[hgp <prefix>]

OPTIONS
  -h: print this help and exit
  -g: prepare repository for git and exit
  -p <prefix>: set the prefix and create the makefile using that prefix
"
}

update_version() {
	local version_string
	log "updating version"
	version_string="$(grep 'version' Cargo.toml | sed 's/version = "\(.*\)"/\1/')"
	echo "$(sed "s/pub const RSWEB_VERSION: \\&str = \".*\";/pub const RSWEB_VERSION: \\&str = \"$version_string\";/" src/lib.rs)" > src/lib.rs
	echo "$(sed "s/pub const RSWEB_SERVER_STR: \\&str = \".*\"/pub const RSWEB_SERVER_STR: \\&str = \"rsweb\\/$version_string\"/" src/lib.rs )" > src/lib.rs
	log "done!"
}

make_prefix() {
	log "generating makefile"
	local makefile='
all: target/release/rsweb-bin container

target/release/rsweb-bin:
	cargo build --release

container:
	docker build -t uludev/rsweb:latest .

install: target/release/rsweb-bin
	mv target/release/rsweb-bin $(PREFIX)/bin/rsweb'
	local prefix="PREFIX = $1"
	echo "$prefix$makefile" > Makefile
	log "done!"
}

args() {
	local opts
	while getopts "hgp:" opts; do
		case $opts in
			h)
				print_help
				exit 0
				;;
			g)
				update_version
				exit 0
				;;
			p)
				update_version
				make_prefix "$OPTARG"
				;;
		esac
	done
}

main() {
	args "$@"
	exit 0
}

main "$@"
